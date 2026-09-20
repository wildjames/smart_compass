# Outstanding component and sourcing work

Every part on all three boards has been checked against the Digi-Key catalogue,
and each one against its manufacturer reference circuit where there is one. The
results — manufacturer part number, Digi-Key part number, key parameters, and a
`Sourcing_Note` on anything with a problem — are written into the schematics as
BOM properties, so they travel with a BOM export and reach whoever places the
order.

## 2. Footprints and library work

### boost inductor needs a footprint

| Where | Part | Land currently used |
| --- | --- | --- |
| Display board, 10 µH | Bourns `SRN3015BTA-100M` | `L_Taiyo-Yuden_NR-30xx` — KiCad has no `SRN3015` footprint |

---

## 5. Checks that depend on decisions made elsewhere

### The chosen haptic actuator sits at the driver's minimum load

The Vybronics `VLV101040A` draws 317 mA typical at its rated 2.5 V, which works
out at roughly **7–8 Ω** — right at the haptic driver's specified 8 Ω minimum
load, possibly a hair under. It is well clear of the 4 Ω over-current trip, and
the 8 Ω figure is specified at 5.2 V where dissipation is worst, so at 3.3 V it
ought to be fine. **Measure it on a sample before committing.**

Two related points fall out of the same number:

- **317 mA lands on the shared 3.3 V rail**, alongside the MCU, the radio module,
  the GNSS and the IMU. The energy per buzz is trivial but the transient is not.
  The driver has a single 1 µF at its supply pin, which is the datasheet minimum
  and nothing more; at this current it wants real bulk capacitance there. Worth
  deciding whether the actuator should instead run from the battery through its
  own regulator.
- **The actuator is rated 2.5 V against the ~2.3 V the rail can deliver**, falling
  to ~2.0 V as the LDO drops out late in a discharge. Expect roughly 90% of rated
  force at full battery and ~80% at the end — still well above any alternative,
  but the headline 2.75 G will not be reached.

### The flash's supply range against the battery cutoff

The Infineon part wants **2.7–3.6 V**, where the Macronix part it replaced
accepted 1.65–3.6 V. The rail is the LDO's 3.3 V, and on battery the LDO drops
out to roughly V_batt − 0.25 V, so a 3.0 V cell gives ~2.85 V — above 2.7 V, but
with much less margin than before. **Check this against the LiPo cutoff you
choose.**

---

## 6. Carried forward into firmware

- **Deep power down.** The flash is not an ultra-low-power part: standby is 20 µA
  typical, against 2 µA in deep power down. Against a 3-day battery target that
  difference matters, so the driver must issue the Deep Power Down command
  (`B9h`) after each access rather than leaving the part in standby. There is no
  flash driver in the firmware yet.
- **`NEOPIX` must be held low whenever `RING_EN` is low.** With the ring's boost
  in shutdown the level shifter is unpowered, and driving its input high would
  push current into its input clamp. The 500 Ω series damping resistor on the main
  board limits that current, but firmware should not rely on it.

---

## 7. Documentation and missing files

### The FFC conductor order belongs on the assembly drawing

Both board-to-board links use the same 10-way `FH12-10S-0.5SH(55)` at both ends,
so one cable part serves both — but **neither pinout is a palindrome**, and a
cable fitted the wrong way round puts `Vdrive` onto ground pins. The ring link's
pin 4 is `RING_EN`, which makes getting the orientation right more consequential
than it was when that way was a spare supply conductor. Both pinouts are in
[README.md](README.md).

### One datasheet is still missing

Diodes Inc. dropped the connection for `DMG3415U.pdf`, and it is still absent
from [`../datasheets/`](../datasheets/). Everything needed to check the power-mux
P-FET was available from the Digi-Key parametrics, so nothing is blocked on it,
but the PDF should be collected.

---

## 8. What the layout has to achieve

None of this has been attempted — it is layout — but the constraints are known
and worth having in one place.

### The main board has no PCB

[`main/SmartCompass.kicad_pcb`](main/SmartCompass.kicad_pcb) contains **no
footprints at all** — only layer definitions, three net declarations and some
embedded 3D models. The gerbers under [`main/gerber/`](main/gerber/) were
produced from an earlier version of the board and no longer describe this design.
Neither daughter board is laid out either.

On the main board, the centre — where the panel connector and its charge pump
used to sit ringed by the LEDs — is now free, and the two interconnects need
placing against the enclosure.

### The ring's boost converter, per TPS61023 datasheet §10

- The high-current loop is switch FET → rectifier FET → output capacitors →
  ground. Keep it short and tight; both output capacitors want to be close to
  `VOUT` **and** `GND`, not just `VOUT`.
- The input capacitor belongs hard against `VIN` **and** `GND`.
- Minimise the copper area of the `SW` node, and keep a ground plane under the
  converter.
- The feedback divider is high-impedance — keep the `FB` trace short and away
  from `SW`.
- The level shifter's bypass capacitor belongs at the buffer's own supply pin,
  not merely somewhere on the 5 V rail.
- The whole converter is a 1 MHz noise source sitting inside the LED ring, which
  sits inside a medallion with a GNSS antenna and a LoRa antenna in it. Where it
  lands relative to those is a judgement call worth making deliberately.

### Two antennas, attached at points you only partly control

The LoRa module's RF pin defaults to the module's **own onboard IPEX connector**
and is not soldered, so that antenna hangs off the module rather than off a
connector you place. The GNSS has its own separate IPEX receptacle, fed through a
π-network that also allows the onboard chip antenna instead. Two antennas inside
a circular handheld is a real constraint on the enclosure and on the antenna
keep-out, and it is worth drawing before committing the layout.
