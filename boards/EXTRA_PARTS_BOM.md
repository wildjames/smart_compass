# Extra parts BOM

Everything the finished compass needs that is **not** on a board, and so never
appears in a BOM exported from a schematic.

Quantities are per finished unit unless stated. Stock and lead figures were read
from Digi-Key and will drift — re-check before ordering.

## Ready to order

| Part | What it is | Qty | Digi-Key | Notes |
| ---- | ---------- | --- | -------- | ----- |
| Vybronics `VLV101040A` | Haptic actuator, 10 × 10 × 4 mm linear resonant | 1 | `1670-VLV101040A-ND` | $7.18/1, ~11,600 in stock, 9-week lead. [Datasheet](../datasheets/VLV101040A_Vybronics.pdf) |
| JST `GHR-02V-S` | 2-way 1.25 mm housing, mates the board's GH headers | 5 | `455-1592-ND` | **Zero stock, 16-week lead.** Order early or find another distributor |
| JST `MINI-SSHL-002T-P0.2` | Crimp socket for the above, 26–30 AWG | 10 + spares | `455-1607-1-ND` | $0.28 each. The plain `SSHL-002T-P0.2` is the same contact but only sold in 1,000-piece strips |
| Waveshare 1.54inch e-Paper V2 | 200 × 200 panel, SSD1681 driver | 1 | - | The display board is built against this exact module — see [`../datasheets/1.54inch_e-paper_V2_Datasheet.pdf`](../datasheets/1.54inch_e-paper_V2_Datasheet.pdf) |

## Decided, but not yet sourced

| Part | What it is | Qty | Notes |
| ---- | ---------- | --- | ----- |
| 10-way 0.5 mm FFC cable | Board-to-board, ring link and display link | 2 | Both links use the same connector at both ends, so one cable part serves both. **Length and contact side still to be decided** — see the warning below |
| LiPo cell, 504050 or 505050 | ~1000 mAh, fits behind the PCB | 1 | Almost always ships with a JST PH 2-pin plug already fitted, which mates the board's battery header directly — no crimping needed |

## Still undecided

| Part | What it is | Notes |
| ---- | ---------- | ----- |
| User buttons | 3 off, on the case | Two candidates in [`main/board_components.md`](main/board_components.md): side-mounted SMD tactile, or through-hole. Whichever is chosen wires back through a GH lead |
| Power switch | 1 off, on the case | Candidates are an SMD slide switch or a larger through-hole one. Also wires back through a GH lead |
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
which is what the hand-soldered pigtail into the GH housing should use. Do not
assume ordinary hook-up wire will do; it is too stiff for the contact pads and
will load the moving mass.
