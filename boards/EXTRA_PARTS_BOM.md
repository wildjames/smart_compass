# Extra parts BOM

Everything the finished compass needs that is **not** on a board, and so never
appears in a BOM exported from a schematic.

Quantities are per finished unit unless stated. Stock and lead figures were read
from Digi-Key and will drift — re-check before ordering.

## Ready to order

| Part | What it is | Qty | Digi-Key | Notes |
| ---- | ---------- | --- | -------- | ----- |
| Vybronics `VLV101040A` | Haptic actuator, 10 × 10 × 4 mm linear resonant | 1 | `1670-VLV101040A-ND` | $7.18/1, ~11,600 in stock, 9-week lead. [Datasheet](../datasheets/VLV101040A_Vybronics.pdf) |
| 28 AWG stranded hook-up wire, silicone insulated | All five case leads, soldered straight to the board | ~1 m | - | Silicone rather than PVC: the case halves stay tethered once the connectors are gone, so this wire gets flexed every time the unit is opened. Insulation OD should be about 1 mm to suit the pad footprints |
| 32 or 34 AWG UL3302 | The short stub from the actuator pads to the splice | ~100 mm | - | Vybronics only permits this gauge against the actuator's pressure pads — see the notes below |
| Waveshare 1.54inch e-Paper V2 | 200 × 200 panel, SSD1681 driver | 1 | - | The display board is built against this exact module — see [`../datasheets/1.54inch_e-paper_V2_Datasheet.pdf`](../datasheets/1.54inch_e-paper_V2_Datasheet.pdf) |

## Decided, but not yet sourced

| Part | What it is | Qty | Notes |
| ---- | ---------- | --- | ----- |
| 10-way 0.5 mm FFC cable | Board-to-board, ring link and display link | 2 | Both links use the same connector at both ends, so one cable part serves both. **Length and contact side still to be decided** — see the warning below |
| LiPo cell, 504050 or 505050 | ~1000 mAh, fits behind the PCB | 1 | Almost always ships with a JST PH 2-pin plug already fitted, which mates the board's battery header directly — no crimping needed |

## Still undecided

| Part | What it is | Notes |
| ---- | ---------- | ----- |
| LoRa antenna | 1 off | A flexible PCB antenna is the plan. It plugs into the **radio module's own onboard IPEX connector**, not a connector on the main board. Band must match the module variant fitted |
| GNSS antenna | 0 or 1 | **Optional.** The GNSS feeds an IPEX receptacle through a pi-network that can instead select the onboard chip antenna. Only needed if the external route is taken |

## Notes

**The FFC cables are not symmetrical.** Both board-to-board links use the same
10-way 0.5 mm connector at both ends, so one cable part number covers both — but
neither pinout is a palindrome, and a cable fitted the wrong way round puts the
LED ring's supply onto ground pins. The ring link also carries the ring's enable
signal on a conductor that used to be a spare supply, which makes getting it
right more consequential than it once was. Both pinouts are in
[`README.md`](README.md), and the conductor order belongs on the assembly
drawing.

**The haptic actuator's contacts are not solder tabs.** It ships with pressure
contacts intended for pogo pins or spring fingers against a PCB. Vybronics
explicitly permits thin flexible flying leads instead — UL3302, AWG 32 or 34 —
which is what the pigtail should use at the actuator end. Do not assume ordinary
hook-up wire will do; it is too stiff for the contact pads and will load the
moving mass.

**The pigtail still needs a splice.** 32/34 AWG is too fine to be the whole run.
It would be fragile against the board pads and is far below the 1 mm insulation
the pad footprint is drawn for. So the thin wire stays a short stub at the
actuator and is spliced to a 28 AWG run, and that run is what solders into the
board. Two things follow,

- **Keep the splice short and put it at the actuator end**, within roughly
  10 mm of the contact pads, so the great majority of the flying lead is the
  stiffer 28 AWG and only a stub of 32/34 AWG is unsupported.
- **Bond the splice down.** It is the weakest joint in the whole assembly — a
  hand-soldered butt joint in a flying lead, on the one lead attached to a
  vibrating mass, inside a device that gets carried in a pocket. Adhesive at
  the splice keeps the vibration out of the joint and stops the moving mass
  being loaded by the heavier wire.
