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

> **A note on how this document refers to parts.** Nothing here is identified by
> its schematic reference designator. Designators are renumbered whenever the
> schematic is re-annotated, so any that were written into this file would
> quietly come to point at the wrong component. Parts are named by what they do,
> and by value or part number where that is needed to pick one out. Net names,
> pin names and manufacturer part numbers are stable, so those are used as-is.

## Why the display board carries so much

It would be convenient to put only the FPC connector on the display board and
keep its support circuitry on the main board. That does not work.

The panel's driver IC generates its own high-voltage gate rails using an
external boost stage, and four of those nodes are unsuitable for a cable:

- `GDR` and `RESE` are the gate drive and current-sense nodes of the switching
  FET. They carry fast edges in a current-sense loop.
- `EPD_SW` is the inductor switch node.
- `PREVGH` and `PREVGL` are the rectified high-voltage output rails.

So the entire charge-pump cluster — the boost inductor, the switching FET, its
gate pulldown and current-sense resistor, the three rectifier and charge-pump
diodes, and the rail capacitors — moves to the display board next to the panel
connector, and only the digital SPI side crosses the cable. The upside is that
this frees a large area in the centre of the main board, where the panel
connector and its support used to sit ringed by the LEDs.

## Interconnects

Both use **`FH12-10S-0.5SH(55)`**, 10-way 0.5 mm FFC — the same family as the
24-way panel connector already uses — so the cable and connector really do stay
on one line in the BOM. Each daughter board carries one; on the main board the
two mating connectors both live in `user_io.kicad_sch`.

The ring link used to use the 6-way `FH12-6S-0.5SH(55)`, but that part shows
zero Digi-Key stock and a 16-week factory lead, while the 10-way has ~37,000 on
the shelf — see [COMPONENT_SOURCING.md](COMPONENT_SOURCING.md) §1.
Standardising on the 10-way costs a slightly wider connector on two boards and
buys four spare conductors. Three of them are spent doubling up supply and
return; the fourth became `RING_EN` when the ring moved to a 5 V boost, and is
what lets the MCU switch the whole ring off.

Both links are now the same connector on both ends, so the same cable part
serves both. Neither pinout is a palindrome, so a cable fitted the wrong way
round puts `Vdrive` onto ground pins — the conductor order belongs on the
assembly drawing.

The stiffener face no longer differs anywhere on the product: the panel
connector, both main-board interconnects and both daughter-board interconnects
are all **contacts bottom**. The panel connector used to be specified as the
contacts-top `FH12A-24S-0.5SH(55)`, which would have meant the panel flex and
the interconnect cables needing stiffeners on opposite faces; that turned out to
be a part-number error rather than a decision — the symbol, its land pattern and
its own description were always the contacts-bottom `FH12-24S-0.5SH(55)`. See
[COMPONENT_SOURCING.md](COMPONENT_SOURCING.md) §3.

### Ring interconnect — FH12-10S-0.5SH(55)

| Pin | Net | Notes |
| --- | --- | ----- |
| 1–3 | `Vdrive` | Three conductors in parallel. Battery rail; the ring boosts it to 5 V locally and draws roughly 370 mA here at full white. |
| 4 | `RING_EN` | Enables the ring's boost converter, from the MCU's `P1.00`. Low disconnects the LEDs entirely. |
| 5, 6 | `GND` | Return, adjacent to the supply block so the power loop stays narrow. |
| 7 | `NEOPIX` | 3.3 V data in, with `GND` on both sides (pins 6 and 8). |
| 8, 9 | `GND` | |
| 10 | `NEOPIX_RET` | `DOUT` of the last LED in the chain, divided back down to 3.2 V before it crosses. |

Three ways for `Vdrive` and four for `GND` is more than the current needs — at
370 mA the IR drop down a 50 mm FFC is only a few millivolts on two conductors.
The reason to spend the spares this way is the return path: it puts a ground
either side of `NEOPIX` and one next to `NEOPIX_RET`, and it keeps the supply
and return conductors adjacent across the pin 4 / pin 5 boundary instead of at
opposite ends of the cable.

`NEOPIX_RET` sits at pin 10 rather than between the grounds so that its run on
the main board does not have to cross the ground bus. `NEOPIX` is the signal
that matters, and it keeps grounds on both sides.

The 500 Ω series damping resistor on the ring data line stays on the **main**
board, so the damping sits at the source end of the cable, which is where it
does any good. It has a second job now: while `RING_EN` is low the ring's level
shifter is unpowered, and that resistor limits the current into the shifter's
input clamp if firmware drives `NEOPIX` high with the ring off.

### The ring runs at 5 V, boosted on the ring board

The LEDs are specified for 3.5–5.5 V with 5.0 V typical, and their input
threshold is **3.4 V at VDD = 5.0 V**. The old arrangement fed them `+BATT`
minus a diode drop and relied on that threshold scaling with VDD — an
assumption Inolux never wrote down, and one that still left the ring
under-volted below about half charge. It is gone.

The cable now carries the raw battery rail as `Vdrive`, and a TI TPS61023 boost
converter on the ring board steps it up to 5.0 V for the LEDs alone. A
74AHCT1G125 buffer running off that same 5 V rail shifts `NEOPIX` up to meet the
3.4 V threshold; its TTL input thresholds accept 3.3 V directly. Coming back the
other way, the last LED's `DOUT` swings to 5 V, so a 2.2 kΩ / 3.9 kΩ divider
brings it to 3.2 V before `NEOPIX_RET` crosses to the 3.3 V board.

Each LED draws 0.5 mA even when dark, so the ring costs 8 mA continuously
whenever it is powered. `RING_EN` exists because of that: the TPS61023 offers
true input-to-output disconnect in shutdown, so pulling it low takes the ring
to 0.1 µA rather than merely dark. Firmware must hold `NEOPIX` low whenever
`RING_EN` is low.

On the main board the series diode that used to make `Vdrive` is gone. It
existed only to make the logic levels work, and the boost does that job properly
now, so the drop was pure loss. In its place a 600 Ω ferrite bead works with a
new 22 µF bulk capacitor as an LC filter that keeps the ring boost's 1 MHz
switching current off `+BATT`, which also feeds the GNSS LNA and the LoRa
module. The boost's own 10 µF input capacitor sits on the ring board beside the
converter, where the datasheet wants it.

The 22 µF bulk capacitor that had already moved **to** the ring board is now the
bulk capacitance on the boost's input. Each LED keeps its own 0.1 µF, on the 5 V
rail.

`NEOPIX_RET` is new. The last LED's data output was unconnected in the original
design; bringing it back lets firmware confirm the chain is intact. On the main
board it lands on a test point — swap that for a spare GPIO if you ever want
firmware to read it.

### Display interconnect — FH12-10S-0.5SH(55)

| Pin | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 |
| --- | - | - | - | - | - | - | - | - | - | -- |
| Net | +3.3V | GND | `SPI_SCK` | `SPI_MOSI` | GND | `DISP_CS` | `DISP_DC` | `DISP_RES` | `DISP_BUSY` | GND |

`SPI_SCK` and `SPI_MOSI` have the fastest edges, so they sit next to grounds;
the slow control signals are at the far end.

`BS` (pin 8 of the panel connector) is tied to GND on the display board for
4-wire SPI, as it was before, so it does not use up a conductor. Because of that
the main board no longer needs `DISP_BS` at all: the net, its sheet pin and its
hierarchical label are gone. The MCU pin that drove it, `P1.00`, was left free
with a no-connect flag, and now drives `RING_EN` to the LED ring's boost
converter.

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

Pin 16 of the panel connector (`VCI`, "Power Supply pin for the chip") and pin
15 (`VDDIO`, "Power for interface logic pins") were connected to their 1 µF
capacitor and to each other and to *nothing else*. KiCad had auto-named the net
after the connector pin instead of after a rail, which is the giveaway — a net
with a power symbol on it would have taken the rail's name.

The reference circuit ties both pins to 3.3 V with a 1 µF. They are now on
`+3.3V`. As drawn, the main board cannot power the panel at all.

### 2. `PREVGH` and `PREVGL` were swapped at the connector

Per §1.5, pin 21 is `VGH`, "Positive Gate driving voltage", and pin 23 is
`VGL`, "Negative Gate driving voltage". Tracing the boost:

- `PREVGH` is the rectified **positive** rail — the cathode of the rectifier
  diode fed from the inductor switch node. It belongs on **pin 21**; it was on
  pin 23.
- `PREVGL` is the pumped **negative** rail — the anode of the charge-pump output
  diode, driven below ground through the pump capacitor and its partner diode.
  It belongs on **pin 23**; it was on pin 21.

So roughly +20 V was going into the negative gate pin and −20 V into the
positive one.

### 3. `PREVGL` had no decoupling capacitor

`PREVGH` carried two 1 µF capacitors and `PREVGL` none. The reference circuit
puts one on each rail, so one of the pair was moved to `PREVGL`.

### 4. Boost component values had drifted off the reference

None of these were intentional, so all three are back at the reference values:

| Part | Was | Now | Role |
| ---- | --- | --- | ---- |
| `GDR` gate pulldown | 1 M | **10 K** | Was 100× weaker, slowing FET turn-off |
| `RESE` current sense | 2.2 Ω | **3 Ω** | Sets peak inductor current |
| Boost inductor | 47 µH, 500 mA | **10 µH, 1 A** | |

The inductor also carried the symbol `smart_compass:LQM18DN100M70L` — a Murata
0603 chip inductor rated about 70 mA, which never matched its 3×3 mm `NR-30xx`
wirewound footprint. It was a stale library artifact, and harmless while the
value read 47 µH because the mismatch was obvious. With the value corrected to
10 µH the wrong part becomes *plausible* to order, so it is now a generic
`Device:L`.

~~Suggested part: **Taiyo Yuden NR3015T100M**~~ — Digi-Key now lists that part as
**Obsolete**, with a last-buy date of 2026-03-31 that has already passed. The
schematic specifies **Bourns SRN3015TA-100M** instead: 10 µH, 800 mA / 750 mA
saturation, shielded, same 3×3×1.5 mm outline. See
[COMPONENT_SOURCING.md](COMPONENT_SOURCING.md) §5.

## The main board split

The main board schematic has been updated to match. `user_io.kicad_sch` no
longer contains any of the moved circuitry:

- The LED ring — the sixteen LEDs, their per-LED decoupling capacitors and the
  ring's bulk capacitor — is gone, replaced by the ring interconnect.
- The display cluster — the panel connector, the boost inductor, the switching
  FET, its gate pulldown and sense resistor, the rectifier and charge-pump
  diodes and all the rail capacitors — is gone, replaced by the display
  interconnect.

The 500 Ω data-line damping resistor stays, as intended, still damping `NEOPIX`
at the source end of the cable and feeding pin 7 of the ring interconnect. The
series diode has been replaced by the ferrite bead and its bulk capacitor,
because the ring now makes its own 5 V — see above.

Deleting the panel connector and its cluster removed all four faults above from
the main board as a side effect. If the split is ever abandoned they need fixing
in place — particularly the missing `VCI` / `VDDIO` supply, which stops the
panel working at all.

### ERC

Main board ERC is unchanged at 9 errors and drops from about 160 warnings to
95. The drop is just the deleted parts taking their warnings with them; the 9
errors and the 95 remaining warnings all predate the split, and most of the
warnings are the "configuration does not include the symbol library" noise
described under [Libraries](#libraries).

### Still to do

Update the PCB from the schematic. That is layout work and has deliberately
been left alone — the two interconnects need placing against the enclosure, and
the centre of the board, where the panel connector and its charge pump used to
sit ringed by the LEDs, is now free.

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
dropped once you are happy with the substitution. A fifth, `TPS61023`, was
drawn by hand for the ring's boost converter; KiCad ships no symbol for it.
Its footprint, `Package_TO_SOT_SMD:SOT-563`, is a stock KiCad land, so nothing
had to be added to `smart_compass.pretty` for it.

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
