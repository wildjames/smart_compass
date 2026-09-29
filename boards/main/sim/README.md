# SPICE simulation of the main board

The main board schematic is set up for KiCad's built-in simulator (ngspice).
It simulates the power tree: USB input, charger, load-sharing power path,
3.3 V regulator, and the battery rail that feeds the LED ring and haptics.
Everything else on the board appears as the current it draws.

## Running it

1. Open `SmartCompass.kicad_pro` and the schematic, then **Inspect → Simulator**.
2. Run the simulation. If KiCad asks for an analysis, choose a transient with
   a 10 µs step and a 3 s stop time. That is the `.tran` directive on the root
   sheet.
3. Add signals to the plot. The useful ones are `VBUS`, `/Power and USB/VSYS`,
   `+BATT`, `+3.3V` and `Vdrive`.

A run takes about 15 seconds.

The scenario is the block of `.param` directives on the root sheet, next to
the flash. Edit those values to change it: cell voltage (`VBAT`), cable
(`RCABLE`, `LCABLE`, `VUSB`), and the time of each event.

| Time | Event |
| ---- | ----- |
| 0 | On battery, idle: MCU running, GNSS acquiring, LoRa receiving, IMU on |
| 0.10 s | LoRa transmits at +22 dBm for 100 ms |
| 0.15 s | LED ring on (0.45 W, the schematic's "typical") until 2.5 s |
| 0.25 s | Haptic buzz, 100 ms |
| 0.40 s | USB cable plugged in |
| 0.55 s | LoRa transmits again, now on USB |
| 0.70 s | e-ink refresh, 1 s |
| 1.00 s | USB cable pulled out |
| 1.02 s | LoRa transmits while the power path switches back to the cell |

## How it is put together

- **Part models:** [`libs/spice/smart_compass.spice`](../../../libs/spice/smart_compass.spice).
  These cover the charger (MCP73831-2), the 3.3 V LDO (RT9080-33), the
  load-sharing P-FET (DMP2045U), the VBUS Schottky (MBR0540), the charge LED
  (LTST-C191KRKT) and the LED-ring ferrite bead (MPZ1608S601A). Each one is
  fitted to its datasheet in `datasheets/`, and the file says which figures.
  The two ICs are behavioural. Their comments list what is modelled and what
  is not.
- **Fixtures:** [`fixtures.spice`](fixtures.spice). These stand in for what
  plugs into the board:
  - a Li-Po cell behind 0.15 Ω on the battery connector;
  - a USB host on the far end of a cable (resistance plus inductance) on the
    USB-C connector;
  - the LED-ring and e-ink boards on their flat-cable connectors.

  The digital and RF modules are modelled as the current they draw.
- **Passives:** resistors and capacitors need no model. KiCad simulates them
  from their values.
- Everything is attached through each symbol's `Sim.*` fields. You can see
  and edit these under Properties → Simulation Model. Load currents are
  subcircuit parameters, so one symbol can be overridden there.

### Load currents

| Load | Draw | Source |
| ---- | ---- | ------ |
| nRF52840 module | 6 mA on 3.3 V, 1 µA on its VBUS pin | Estimate. The small VBUS figure is deliberately pessimistic, see below |
| LoRa module | 7.6 mA receiving, 125 mA transmitting | Datasheet |
| GNSS module | 14 mA | Datasheet, acquisition at 3 V |
| IMU | 5 mA | Estimate |
| QSPI flash | 20 µA | Estimate, standby |
| Fuel gauge | 23 µA | Datasheet, active mode |
| Haptic driver + LRA | 60 mA while buzzing | Estimate |
| e-ink board | 8 mA while refreshing | Estimate |
| LED ring board | Constant power from `Vdrive` behind 32 µF | Derived from the LED datasheet: 0.45 W typical, about 1.4 W with every LED full white |

### Parts excluded from simulation

Every symbol must either have a model or be excluded. ngspice refuses a
circuit in which any node has no DC path to ground. The digital modules are
current sinks between their supply pins, so their signal pins are
unconnected in the simulation. Anything wired only to those pins would
float.

| Excluded | Why |
| -------- | --- |
| Both 32.768 kHz crystals and their four 10 pF load capacitors | The oscillators are inside the MCU and IMU, which are only loads here. The capacitors would hang on nodes with no DC path. |
| The two 27 Ω USB data resistors, the 500 Ω NeoPixel data resistor, the 1 kΩ GNSS UART resistor | Each connects two pins that have no model, so both ends float. Signal integrity is not what this simulation looks at. |
| The GNSS antenna, its u.FL connector, the antenna ESD diode, the 0 Ω antenna link and the not-fitted matching parts | RF parts with no meaning at these frequencies. A value of "NM" cannot be simulated at all. |
| The IMU's 0.1 µF CAP capacitor and the haptic driver's 1 µF REG capacitor | Each decouples an internal regulator of an IC modelled only as a load, so it would sit on a floating node. |
| The power-switch pads | Excluding them leaves the switch open, which is the device switched on (the LDO enable is pulled up). |
| The SWD header, reset button, button pads, haptic LRA pads and the NeoPixel-return test point | Connect only to signal pins or to parts already modelled elsewhere. Excluding the buttons leaves them released. |

When you add a symbol to the main board, give it a `Sim.*` model or tick
**Exclude from simulation**. Otherwise the simulator stops with a "no
simulation model" or "singular matrix" error.

## Results

These are from the nominal scenario, plus sweeps of the exported netlist
through the same ngspice build, on 2026-09-28. The models are fitted to
typical datasheet figures. Treat the numbers as good for spotting a problem,
not as a guarantee.

### On battery: fine

- From a 3.7 V cell with the ring lit, a LoRa transmit and a haptic buzz, the
  cell supplies 274 mA and 3.3 V holds at 3.295 V.
- Worst case, a 3.4 V cell with the ring full white and everything else on
  (654 mA peak): the LDO drops out and 3.3 V sags to 3.22 V.
- In the same worst case, Vdrive stays at 3.26 V.

### USB hot-plug overshoot: a real risk

Plugging a cable into a live supply rings the cable's inductance against the
board's ceramic capacitors. VBUS has 14.7 µF on it directly, plus the 10 µF
on VSYS through the Schottky.

The limits it can break:

- the nRF52840's VBUS pin: **5.8 V** absolute max;
- the charger: 7.0 V;
- the LDO input, reached through VSYS: 6.5 V.

| Cable, round trip | 0.5 µH | 1 µH | 2 µH |
| ----------------- | ------ | ---- | ---- |
| 0.1 Ω (short, heavy-gauge) | 6.08 V | 6.58 V | 7.04 V |
| 0.2 Ω (typical) | 5.15 V | 5.62 V | 6.15 V |
| 0.4 Ω (thin or long) | 4.98 V | 4.98 V | 5.18 V |

Peak VBUS from an ideal 5.0 V source with nominal capacitance.

- A 5.25 V host raises every figure by 0.25–0.35 V.
- The real capacitance of the MLCCs at 5 V, which is roughly half their
  nominal value, raises the low-loss cases by up to another 0.5 V.
- The worst case in the sweep is 7.8 V on VBUS and 7.4 V on VSYS.
- The ideal source makes these pessimistic, but a good cable on a phone
  charger is exactly the everyday case.
- A standard 5 V TVS only starts conducting above about 6 V, so on its own it
  will not hold VBUS under 5.8 V.
- The usual remedies are a damping (lossy) bulk capacitor across VBUS, or a
  VBUS overvoltage-protection switch. Either way it is a schematic change to
  weigh, and it cannot be checked on the bench without a scope.

### Pulling USB out: a 3.3 V dip, worst at low charge

The P-FET's gate is VBUS itself. After unplugging, VBUS stays up on its
14.7 µF, discharged only by the 100 kΩ gate pull-down (about a 1.5 s time
constant) and by leakage into VSYS.

- Until the gate falls about half a volt below VSYS, the system runs through
  the P-FET's body diode. That costs about 0.6 V.
- With the typical threshold this lasts about 105 ms.
- With the datasheet's worst-case −1.0 V threshold it lasts about 290 ms.

| Cell | 3.3 V minimum (transmit during the dip) |
| ---- | --------------------------------------- |
| 4.0 V | 3.26 V |
| 3.7 V | 2.96 V |
| 3.5 V | 2.76 V |
| 3.4 V | 2.66 V, below 2.7 V for 79 ms |

2.7 V is the minimum for the GNSS I/O supply (VIO_SEL left open) and for the
QSPI flash. Without the transmit, a 3.4 V cell bottoms out at 2.78 V.

### Hot board on battery: the P-FET can switch itself off

With no USB, the Schottky's reverse leakage flows from VSYS into VBUS and out
through the 100 kΩ pull-down. That lifts the P-FET's gate:

| Board temp | Gate | VSYS | 3.3 V |
| ---------- | ---- | ---- | ----- |
| 25 °C | 0.04 V | 3.69 V | 3.30 V |
| 60 °C | 1.60 V | 3.69 V | 3.30 V |
| 70 °C | 3.13 V | 3.26 V | 3.25 V |
| 85 °C | 3.23 V | 3.25 V | 3.23 V |

This is a 3.7 V cell at idle load.

- Above about 60–70 °C, which a device left in the sun can reach, the FET
  turns off and the whole system runs through its body diode.
- The leakage curve comes from the datasheet's typical plot for the MBR05xx
  family, so the onset temperature is approximate.

### A smaller gate pull-down fixes both

Rerunning with 10 kΩ in place of the 100 kΩ pull-down:

- **Unplug:** the body-diode window shrinks from about 105 ms to about 12 ms.
  A 3.4 V cell's 3.3 V rail now bottoms out at 2.78 V.
- **Temperature:** at 85 °C the gate sits at 0.83 V and the FET stays fully
  on.

The cost is 0.5 mA drawn from USB while plugged in. The design is unchanged;
this is only a simulated option.

### Charging

- The charger delivers its programmed 256 mA (3.9 kΩ on PROG).
- The LED ring and haptic driver hang directly on the cell. With the ring at
  its typical 0.45 W, the cell itself receives only about 135 mA.
- The charger ends a charge when its current falls to 7.5 % of 256 mA
  (19 mA). While the ring draws more than that, the charger never sees it
  and holds the cell at 4.2 V instead. This follows from the datasheet; the
  model does not simulate termination.
- The charge LED runs at about 0.6 mA through its 5.1 kΩ resistor. It will be
  dim. The datasheet's example uses 470 Ω.

## Not simulated

- Thermal regulation in the charger.
- The LDO's current limit and over-temperature shutdown.
- The MLCCs' loss of capacitance with DC bias (except in the hot-plug sweep
  above).
- PCB trace resistance and inductance.
- Anything on the LED ring or display boards beyond their input capacitance
  and load.
