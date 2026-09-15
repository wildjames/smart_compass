# Board set

The Smart Compass is built as three PCBs. KiCad has no multi-board project, so
each one is its own project directory here, and the parts they share live in
`../libs/`.

| Directory | Project | What it is |
| --------- | ------- | ---------- |
| `main/` | `SmartCompass` | The engine board: MCU, GNSS, LoRa, IMU, flash, power tree, haptics, buttons, USB |
| `led_ring/` | `LedRing` | The 16-LED addressable RGB ring |
| `display/` | `Display` | The e-ink panel connector and its charge-pump support |

The ring and display were split off so the housing can be assembled and
modified without the main board's position dictating where they sit.

## Why the display board carries so much

It would be convenient to put only the FPC connector on the display board and
keep its support circuitry on the main board. That does not work.

The panel's driver IC generates its own high-voltage gate rails using an
external boost stage, and four of those nodes are unsuitable for a cable:

- `GDR` and `RESE` are the gate drive and current-sense nodes of the switching
  FET, `Q2`. They carry fast edges in a current-sense loop.
- `EPD_SW` is the inductor switch node.
- `PREVGH` and `PREVGL` are the rectified high-voltage output rails.

So the entire cluster — `L1`, `Q2`, `R18`, `R19`, `D19`–`D21`, `C36`–`C45` —
moves to the display board next to `J3`, and only the digital SPI side crosses
the cable. The upside is that this frees a large area in the centre of the main
board, where `J3` and its support currently sit ringed by the LEDs.

## Interconnects

Both use **`FH12-10S-0.5SH(55)`**, 10-way 0.5 mm FFC — the same family as the
panel connector `J3` already uses — so the cable and connector really do stay on
one line in the BOM. On each daughter board the interconnect is `J10`. On the
main board the mating parts are `J6` (ring) and `J7` (display), both in
`user_io.kicad_sch`.

The ring link only needs six conductors. It used to use the 6-way
`FH12-6S-0.5SH(55)`, but that part shows zero Digi-Key stock and a 16-week
factory lead, while the 10-way has ~37,000 on the shelf — see
[COMPONENT_SOURCING.md](COMPONENT_SOURCING.md) §1. Standardising on the 10-way
costs a slightly wider connector on two boards and buys four spare conductors,
which are spent on supply and return rather than left floating.

Both links are now the same connector on both ends, so the same cable part
serves both. Neither pinout is a palindrome, so a cable fitted the wrong way
round puts `Vdrive` onto ground pins — the conductor order belongs on the
assembly drawing.

The stiffener face no longer differs anywhere on the product: `J3`, `J6`, `J7`
and both `J10`s are all **contacts bottom**. `J3` used to be specified as the
contacts-top `FH12A-24S-0.5SH(55)`, which would have meant the panel flex and
the interconnect cables needing stiffeners on opposite faces; that turned out to
be a part-number error rather than a decision — the symbol, its land pattern and
its own description were always the contacts-bottom `FH12-24S-0.5SH(55)`. See
[COMPONENT_SOURCING.md](COMPONENT_SOURCING.md) §3.

### Ring board — `J10`, FH12-10S-0.5SH(55)

| Pin | Net | Notes |
| --- | --- | ----- |
| 1–4 | `Vdrive` | Four conductors in parallel. The ring draws roughly 250 mA at full white. |
| 5, 6 | `GND` | Return, adjacent to the supply block so the power loop stays narrow. |
| 7 | `NEOPIX` | Data in, with `GND` on both sides (pins 6 and 8). |
| 8, 9 | `GND` | |
| 10 | `NEOPIX_RET` | `D18` `DOUT`, returned to the main board. |

Four ways each for `Vdrive` and `GND` is more than the current needs — at
250 mA the IR drop down a 50 mm FFC was already only a few millivolts on two
conductors. The reason to spend the spares this way is the return path: it puts
a ground either side of `NEOPIX` and one next to `NEOPIX_RET`, and it keeps the
supply and return conductors adjacent across the pin 4 / pin 5 boundary instead
of at opposite ends of the cable.

`NEOPIX_RET` sits at pin 10 rather than between the grounds so that its run on
the main board does not have to cross the ground bus. `NEOPIX` is the signal
that matters, and it keeps grounds on both sides.

`R13` (500R) stays on the **main** board so the series damping sits at the
source end of the cable, which is where it does any good.

`D2` also stays on the main board. It drops `+BATT` by about 0.7 V to make
`Vdrive`, which is deliberate: the LEDs need V<sub>IH</sub> = 0.7 × VDD, and
dropping VDD is what lets 3.3 V logic drive the data line reliably at a full
battery charge.

**`D2` needs changing.** It is currently a BAV199, a signal diode rated 160 mA,
carrying the ring's ~250 mA. See
[COMPONENT_SOURCING.md](COMPONENT_SOURCING.md) §4 — the recommendation is the
`MBR0540` already used elsewhere on the design, and the same section covers why
the diode-drop trick only strictly works near full charge.

`C25` (22 µF) moved **to** the ring board so the bulk capacitance sits at the
load. Each LED keeps its own 0.1 µF.

`NEOPIX_RET` is new. `D18`'s data output was unconnected in the original design;
bringing it back lets firmware confirm the chain is intact. On the main board it
lands on test point `TP1` — swap that for a spare GPIO if you ever want firmware
to read it.

### Display board — `J10`, FH12-10S-0.5SH(55)

| Pin | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 |
| --- | - | - | - | - | - | - | - | - | - | -- |
| Net | +3.3V | GND | `SPI_SCK` | `SPI_MOSI` | GND | `DISP_CS` | `DISP_DC` | `DISP_RES` | `DISP_BUSY` | GND |

`SPI_SCK` and `SPI_MOSI` have the fastest edges, so they sit next to grounds;
the slow control signals are at the far end.

`BS` (`J3` pin 8) is tied to GND on the display board for 4-wire SPI, as it was
before, so it does not use up a conductor. Because of that the main board no
longer needs `DISP_BS` at all: the net, its sheet pin and its hierarchical label
are gone, and the MCU pin that drove it — `U1` pin 56, `P1.00` — is now free and
carries a no-connect flag. Delete the flag and wire it up if you want the GPIO
back.

`DISP_DC` was `DISP_D/C` on the main board, and has been renamed there too. The
old name is awkward because `/` is KiCad's hierarchical path separator and has
to be escaped as `{slash}` in the file, which makes the net hard to match across
projects.

## Faults found and fixed on the display board

The panel is a **Waveshare 1.54inch e-Paper V2** (200×200, SSD1681 driver).
Checking the migrated circuit against
[`../datasheets/1.54inch_e-paper_V2_Datasheet.pdf`](../datasheets/1.54inch_e-paper_V2_Datasheet.pdf)
§1.5 (Input/Output Terminals) and §1.6 (Reference Circuit) turned up four
faults. All four are **fixed on the display board** and all four are **still
present on the main board**.

### 1. The panel had no supply — `VCI` and `VDDIO` were floating

`J3` pin 16 (`VCI`, "Power Supply pin for the chip") and pin 15 (`VDDIO`,
"Power for interface logic pins") were connected to `C37` and to each other and
to *nothing else*. KiCad had auto-named the net `Net-(J3-VCI)`, which is the
giveaway — a net with a power symbol on it would have been named for the rail.

The reference circuit ties both pins to 3.3 V with a 1 µF. They are now on
`+3.3V`. As drawn, the main board cannot power the panel at all.

### 2. `PREVGH` and `PREVGL` were swapped at the connector

Per §1.5, pin 21 is `VGH`, "Positive Gate driving voltage", and pin 23 is
`VGL`, "Negative Gate driving voltage". Tracing the boost:

- `PREVGH` is the rectified **positive** rail — `D21`'s cathode, fed from the
  inductor switch node. It belongs on **pin 21**; it was on pin 23.
- `PREVGL` is the pumped **negative** rail — `D19`'s anode, below ground via
  the `C36` / `D20` charge pump. It belongs on **pin 23**; it was on pin 21.

So roughly +20 V was going into the negative gate pin and −20 V into the
positive one.

### 3. `PREVGL` had no decoupling capacitor

`PREVGH` carried two (`C42` and `C44`) and `PREVGL` none. The reference circuit
puts one 1 µF on each rail, so `C44` moved to `PREVGL`.

### 4. Boost component values had drifted off the reference

None of these were intentional, so all three are back at the reference values:

| Part | Was | Now | Role |
| ---- | --- | --- | ---- |
| `R18` | 1 M | **10 K** | `GDR` gate pulldown — was 100× weaker, slowing FET turn-off |
| `R19` | 2.2 Ω | **3 Ω** | `RESE` current sense — sets peak inductor current |
| `L1` | 47 µH, 500 mA | **10 µH, 1 A** | Boost inductor |

`L1` also carried the symbol `smart_compass:LQM18DN100M70L` — a Murata 0603
chip inductor rated about 70 mA, which never matched its 3×3 mm `NR-30xx`
wirewound footprint. It was a stale library artifact, and harmless while the
value read 47 µH because the mismatch was obvious. With the value corrected to
10 µH the wrong part becomes *plausible* to order, so `L1` is now a generic
`Device:L`.

~~Suggested part: **Taiyo Yuden NR3015T100M**~~ — Digi-Key now lists that part as
**Obsolete**, with a last-buy date of 2026-03-31 that has already passed. The
schematic specifies **Bourns SRN3015TA-100M** instead: 10 µH, 800 mA / 750 mA
saturation, shielded, same 3×3×1.5 mm outline. See
[COMPONENT_SOURCING.md](COMPONENT_SOURCING.md) §5.

## The main board split

The main board schematic has been updated to match. `user_io.kicad_sch` no
longer contains any of the moved circuitry:

- The LED ring — `D3`–`D18`, their decoupling `C15`–`C22` and `C27`–`C34`, and
  the bulk cap `C25` — is gone, replaced by `J6`.
- The display cluster — `J3`, `L1`, `Q2`, `R18`, `R19`, `D19`–`D21` and
  `C36`–`C45` — is gone, replaced by `J7`.

`D2` and `R13` stay, as intended: `D2` still makes `Vdrive` from `+BATT`, and
`R13` still damps `NEOPIX` at the source end of the cable, feeding `J6` pin 7.

Deleting `J3` and its cluster removed all four faults above from the main board
as a side effect. If the split is ever abandoned they need fixing in place —
particularly the missing `VCI` / `VDDIO` supply, which stops the panel working
at all.

### ERC

Main board ERC is unchanged at 9 errors and drops from about 160 warnings to
95. The drop is just the deleted parts taking their warnings with them; the 9
errors and the 95 remaining warnings all predate the split, and most of the
warnings are the "configuration does not include the symbol library" noise
described under [Libraries](#libraries).

### Still to do

Update the PCB from the schematic. That is layout work and has deliberately
been left alone — `J6` and `J7` need placing against the enclosure, and the
centre of the board, where `J3` and its charge pump used to sit ringed by the
LEDs, is now free.

## Libraries

`../libs/smart_compass.kicad_sym` and `../libs/smart_compass.pretty` hold the
parts the boards share. They were extracted from the main board, where they had
only ever existed embedded inside the `.kicad_sch` and `.kicad_pcb` files — they
were never in any library table, so a new project could not place them.

Only the parts the daughter boards needed were extracted: `WS2812B2020`,
`FH12-24S-0.5SH_55_` and `LQM18DN100M70L`. A fourth symbol,
`IN-PI15TAT5R5G5B`, was added later for the Inolux ring LED that replaced the
WS2812B — its pin numbering is completely different, so it needed its own
symbol rather than a value change. `WS2812B2020` is now unused and can be
dropped once you are happy with the substitution.

Footprints live alongside in `../libs/smart_compass.pretty/`, and 3D models in
`../libs/smart_compass.3dshapes/`, which was added for the Inolux STEP. Both
daughter boards already register the footprint library through their
project-scope `fp-lib-table`, so a `.kicad_mod` dropped into the `.pretty`
folder is immediately available as `smart_compass:<filename>`.

3D models are referenced from inside the footprint with a
`${KIPRJMOD}/../../libs/smart_compass.3dshapes/...` path, which resolves the
same way from any of the three board directories and survives the repo being
cloned elsewhere. The main board still resolves its
other custom parts from its embedded copies, which is why its ERC reports a
long list of "configuration does not include the symbol library" warnings.
Those predate the split. Extracting the rest into `libs/` would clear them.

Each daughter board has a project-scope `sym-lib-table` and `fp-lib-table`
pointing at `${KIPRJMOD}/../../libs/`, so the paths survive the repo being
cloned somewhere else.
