# SPICE simulation of the main board

The main board schematic is set up for KiCad's built-in simulator (ngspice).
It simulates the power tree: USB input, charger, load-sharing power path,
3.3 V regulator, and the LED ring and haptics, which hang off the power path
output.
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
  load-sharing P-FET (DMP2045U), the VBUS Schottky (SS1040-AU), the charge LED
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
| nRF52840 module | 6 mA on 3.3 V, 1 µA on its VBUS pin | Estimate. The VBUS figure is kept small so VBUS decays slowly after unplugging, which is the pessimistic case |
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

## Not simulated

- Thermal regulation in the charger.
- The LDO's current limit and over-temperature shutdown.
- The MLCCs' loss of capacitance with DC bias.
- PCB trace resistance and inductance.
- Anything on the LED ring or display boards beyond their input capacitance
  and load.
