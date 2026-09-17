# Component sourcing review

Every part on all three boards checked against the Digi-Key catalogue, with the
manufacturer part number, Digi-Key part number and key parameters written back
into the schematics as BOM properties. Where a component's reference circuit was
missing from [`../datasheets/`](../datasheets/) it has been downloaded, and the
schematic checked against it.

Stock figures are a snapshot taken during this review. Treat them as a signal
about which parts are comfortable and which are tight, not as a quote — re-check
anything marked as a concern before you place an order.

**Prices below are single-unit Digi-Key cut-tape.** Most drop 30–50% by qty 10.

## What changed in the schematics

Each placed component now carries `MPN`, `Manufacturer` and, where verified,
`DigiKey_PN`. Passives also carry `Voltage` / `Dielectric` / `Tolerance`. Parts
with a sourcing problem carry a `Sourcing_Note` property that travels with the
BOM export, so the warning reaches whoever places the order.

Two footprint references were changed. The ring LEDs moved to a different
package as part of the Inolux substitution in §1, and both ends of the ring
interconnect moved from the 6-way to the 10-way FH12 land when the two
board-to-board links were standardised on one connector, also in §1. Every other
footprint already matches the package of the part specified, and the cases where
a better-sourced part would need a different land pattern are called out below
as layout decisions rather than made silently.

All five schematics validate clean, as does the symbol library. Main board ERC
now stands at **9 errors / 24 warnings**, down from 9 / 95: the errors are the
same pre-existing count recorded in [`README.md`](README.md), and the warnings
fell because the main board's 24 symbols were lifted out of the sheets' caches
into [`../libs/smart_compass_main.kicad_sym`](../libs/smart_compass_main.kicad_sym)
and registered in a project `sym-lib-table`, so symbol references resolve for
the first time. Every remaining warning is a *footprint* library warning, plus
one unconnected wire endpoint; the footprint side is untouched.
The display board has 1 warning (a symbol/library mismatch on the panel
connector that predates this work). The ring board is at 0 violations.

---

## 1. Blocking problems — resolve before ordering

### Ring LEDs — ~~WS2812B-2020 is not a Digi-Key line~~ → now Inolux IN-PI15TAT5R5G5B

**Largely resolved.** Digi-Key does not carry Worldsemi at all, so the original
WS2812B-2020 had no Digi-Key part number. The ring has been changed to
**Inolux `IN-PI15TAT5R5G5B`** (`1830-IN-PI15TAT5R5G5BCT-ND`), which is Active,
~72,000 in stock, 5-week lead, $0.53/1 and $0.26/100.

Why it fits:

- **Same protocol.** Single-wire return-to-zero, 800 kbps, 24-bit **GRB**,
  MSB first. Timing is the usual WS2812 envelope — T0H 0.2–0.4 µs, T1H
  0.58–1.0 µs, code period ≥1.2 µs. Reset is **>200 µs**, shorter than the
  WS2812B-2020's >280 µs, so firmware written for the old part already
  satisfies it. No firmware change expected.
- **Cheaper and smaller** — 1515 (1.5 × 1.5 × 0.65 mm) against 2020, which
  gives the ring more room, not less.
- Inolux's "cascading enhancement" reshapes the signal at each hop, which is
  helpful down a 16-LED chain on a flex cable.

**The V_IH question is now closed — by design change, not by an answer.** The
datasheet gives V_IH = **3.4 V minimum at VDD = 5.0 V** as a flat number and
does *not* state that it scales with VDD. The old `Vdrive` diode-drop scheme
depended entirely on it scaling, because 3.3 V logic can never meet a fixed
3.4 V threshold at any supply, and Inolux had not written the ratio down.

That assumption has been removed rather than confirmed. The ring now runs at
**VDD = 5.0 V** from its own boost converter (§7), which is the exact condition
the 3.4 V figure is specified at, and a 74AHCT1G125 buffer drives the data line
from that same 5 V rail. The number is used as written. No email to Inolux is
needed.

**The supply headroom problem is gone with it.** Recommended operating range is
**3.5–5.5 V**, typical 5.0 V (the absolute-maximum table confusingly says
3.7–5.5 V, which contradicts it). At 5.0 V the part sits at its typical
operating point across the whole battery discharge curve instead of sliding
under its minimum below half charge.

**Library work this needed.** It is a 1515 part with a **completely different
pin assignment** from the WS2812B-2020:

| Pin | WS2812B-2020 | IN-PI15TAT5R5G5B |
| --- | --- | --- |
| 1 | DO | **DIN** |
| 2 | GND | **VDD** |
| 3 | DI | **DOUT** |
| 4 | VDD | **GND** |

Every pin differs, so reusing the old symbol would have silently swapped power
and data. A new symbol `smart_compass:IN-PI15TAT5R5G5B` has been added to
[`../libs/smart_compass.kicad_sym`](../libs/smart_compass.kicad_sym) with the
same pin *positions* (so the existing ring wiring is untouched) and the correct
pin *numbers*. The netlist verifies as intended: the incoming data line lands on
pin 1 of the first LED, each LED's pin 3 feeds the next LED's pin 1, the 5 V
rail reaches every pin 2, and ground every pin 4.

The land pattern `smart_compass:LED_Inolux_IN-PI15TAT5R5G5B_1515` has since been
installed from a vendor component pack into
[`../libs/smart_compass.pretty/`](../libs/smart_compass.pretty/), and the
matching STEP model into `../libs/smart_compass.3dshapes/`. Ring-board ERC is
back to 0 violations.

The vendor land was checked against the datasheet before use: 0.5588 mm pads on
0.95 mm pitch gives 0.39 mm gaps and a 1.51 mm overall span, matching the
datasheet's 0.55 / 0.40 / 0.55 recommendation. Pad *numbering* runs 1→2→3→4
counter-clockwise, the same sense as the datasheet, though the whole part is
drawn rotated 90° from the datasheet's top view. That is harmless on a
four-pad square package, but it does mean **the STEP model's rotation should be
eyeballed in the 3D viewer** — the vendor pack gave no model alignment, so the
`(rotate)` in the footprint is currently 0,0,0 and may need a 90° Z correction.

The vendor pack also contained a symbol, which was **not** used. It places the
pins 40 mm apart in a completely different arrangement, which would have broken
the existing ring wiring, and it declares GND as `power_out` rather than
`power_in` — with sixteen of them on one net that produces spurious ERC
conflicts. The hand-built symbol described above is kept instead.

### IMU — BNO085 deprecated, swapped to BNO086 ✅ done

The BNO085 is deprecated and was in any case unbuyable from Digi-Key: zero
stock, 16-week factory lead. **BNO086** (`1888-BNO086CT-ND`, Active, ~9,600 in
stock, £11.62/1, £9.10/100) is the successor.

CEVA's BNO08X datasheet (rev 1.17, in [datasheets/](../datasheets/)) states
outright that the BNO086 "may be used as a drop-in replacement for the BNO085,
with an identical pinout and software feature set" — same 28-LGA
5.2 × 3.8 × 1.1 mm package, same SH-2 firmware and SHTP transport. Over the
BNO085 it adds an on-die temperature sensor, 14-bit accelerometer fusion, lower
idle power and Interactive Calibration.

The schematic now carries the BNO086. The IMU's symbol, `Value`, `MPN`, `MP`,
`MANUFACTURER`, `DigiKey_PN`, `PRICE` and `Description` were all updated; no
wiring changed and no layout change is needed. The `Footprint` field still
points at `BNO085:IC_BNO085` deliberately — that is the land pattern for the
shared 28-LGA package and it is the footprint already placed on the board, so
renaming it would only orphan the PCB. The SnapEDA links on the cached symbol
also still say BNO085, which is accurate: that is where the symbol drawing came
from.

### QSPI flash — ~~every 4×4 mm MX25R6435F variant is unstocked~~ → now Infineon S25FL064L ✅ done

**Resolved, and with no layout consequence.** `MX25R6435FZAIH0` is Active but
shows zero stock with a 52-week factory lead, as do `FZAIL0` (4×4) and `FZBIH3`
(4×3). The only stocked member of the family, `MX25R6435FZNIL0`, is 8-WSON
6×5 mm — a different land pattern.

Rather than accept the bigger package, the flash has been changed to **Infineon
`S25FL064LABNFI043`** (`448-S25FL064LABNFI043CT-ND`, Active, ~50,000 in stock,
cut tape, $1.67/1 and $1.46/100 — cheaper than the part it replaces).

**It is a true drop-in.** The package drawings were compared directly, Macronix
USON 8L 4×4 against Infineon UNF008:

| | MX25R6435F USON 4×4 | Infineon UNF008 |
| --- | --- | --- |
| Body | 4.00 × 4.00 × 0.55 nom | 4.00 × 4.00 × 0.55 nom |
| Lead pitch `e` | 0.80 | 0.80 BSC |
| Terminal `b` / `L` | 0.30 / 0.40 | 0.30 / 0.40 |
| Exposed pad | 3.00 × 2.30 | 3.00 × 2.30 |

Identical to the hundredth of a millimetre, so the `SON80P400X400X60-9N` land is
still correct. The pinout matches as well — 1 `CS#`, 2 `SO/IO1`, 3 `WP#/IO2`,
4 `VSS`, 5 `SI/IO0`, 6 `SCK`, 7 `IO3/RESET#`, 8 `VCC`. Pin 7 is the detail that
mattered: the board drives `FLASH_RST` from an MCU GPIO into pin 7, and the
S25FL064L is one of the few alternatives where pin 7 really is `RESET#` rather
than `HOLD#`. No wiring change, no footprint change, no layout change.

**Longevity.** This was chosen to be a part with production life left, not just
one in stock today. In its favour: Active with ~50,000 on the shelf; Infineon
offers AEC-Q100 grades of the same die (`…NFA040`) plus 105 °C and 125 °C
industrial grades, which implies a long supply commitment; datasheet still under
revision (rev \*H, 2023-04). Against: it is a 65 nm floating-gate part of
2016 vintage, and Infineon's strategic push is toward SEMPER. **Infineon's flash
longevity-program document is behind a login, so a published end-of-supply date
could not be verified** — treat that as unconfirmed. For comparison, the family
being left behind is the one visibly decaying: `MX25R6435FM2IH0` and
`MX25R6435FM2KL0` both read **"Not For New Designs"** at Digi-Key today.

**Second source on the identical land:** GigaDevice `GD25Q64EQIGR`
(`1970-GD25Q64EQIGRCT-ND`, ~5,900 in stock, $2.10). Its USON8 4×4 drawing was
checked too and matches exactly — 4.00 × 4.00, 0.80 pitch, 0.30/0.40 terminals,
2.30 × 3.00 e-pad. One functional difference: GigaDevice's pin 7 is
`IO3/HOLD#`, not `RESET#`, so `FLASH_RST` would become a hold line and reset
would have to go through the 66h/99h software command. Fine as a fallback, not
as the primary.

**Two consequences worth carrying forward:**

1. *Supply range narrows to 2.7–3.6 V* (the MX25R was 1.65–3.6 V). The rail is
   the RT9080's 3.3 V, and on battery the LDO drops out to roughly
   V_batt − 0.25 V, so a 3.0 V cell gives ~2.85 V — above 2.7 V, but with much
   less margin than before. Worth checking against the chosen LiPo cutoff.
2. *It is not an ultra-low-power part.* Deep power-down is 2 µA typ against the
   MX25R's 0.35 µA, which is immaterial, but **standby is 20 µA typ**, which is
   not, against a 3-day battery target. The firmware must actually issue the
   Deep Power Down command (`B9h`) after each flash access rather than leaving
   the part in standby. This is a note for the driver — there is no flash driver
   in the firmware yet.

**Library work this needed.** Only a rename and a relabel; the symbol geometry
is unchanged, so the existing wiring is untouched. `smart_compass_main:MX25R6435FZAIH0`
was renamed to `smart_compass_main:S25FL064L`, the four multi-I/O pin *names*
were respelled to Infineon's (`SO/SIO1` → `SO/IO1`, `WP#/SIO2` → `WP#/IO2`,
`SI/SIO0` → `SI/IO0`, `RESET#/SIO3` → `IO3/RESET#`) with pin *numbers* and
positions untouched, and `Value`, `Footprint`, `Datasheet`, `Description`, `MF`,
`MP`, `MANUFACTURER`, `Manufacturer`, `MPN`, `DigiKey_PN`, `PRICE`,
`Purchase-URL` and `Sourcing_Note` were updated on both the library symbol and
the schematic's cached copy. `Check_prices` was cleared and `Purchase-URL`
repointed at Digi-Key, because a stale link that buys a Macronix part is an
ordering hazard. `SnapEDA_Link` is deliberately left pointing at the Macronix
part, on the same reasoning as the BNO086 above: that is genuinely where the
symbol drawing came from.

Main board ERC is unchanged at **9 errors / 24 warnings** — the same
pre-existing counts as before the swap.

**The footprint is a separate, pre-existing problem.** The `Footprint` field
said `MX25R6435FZAIH0:SON80P400X400X60-9N`, but that library is registered
nowhere — there is no project `fp-lib-table` on the main board and no such entry
in the global tables, and no matching `.kicad_mod` anywhere in the repository.
The reference was already dangling before this change. The field now reads
`smart_compass:SON80P400X400X60-9N`, pointing at the project's own footprint
library in [`../libs/smart_compass.pretty/`](../libs/smart_compass.pretty/),
which does exist — but **the `.kicad_mod` itself still has to be created there
before layout.** The land pattern it must implement is the one in the table
above, and it is the same one either part needs.

Related and worth knowing: [`main/SmartCompass.kicad_pcb`](main/SmartCompass.kicad_pcb)
currently contains **no footprints at all** — only layer definitions, three net
declarations and some embedded 3D models. The gerbers under
[`main/gerber/`](main/gerber/) were produced from an earlier version of the
board and no longer describe this design.

### Ring interconnect — ~~FH12-6S-0.5SH(55) shows zero stock, 16-week lead~~ → now 10-way on both links ✅ done

**Resolved, and it removes a BOM line.** The 6-way FFC connector for the LED
ring link was unstocked at Digi-Key with a 16-week factory lead. The 10-way part
already used for the display link (`FH12-10S-0.5SH(55)`, `HFJ110CT-ND`) has
~37,000 in stock.

Both interconnects now use the 10-way part. `README.md` already stated the goal
of keeping the cable and connector on one BOM line; this achieves it properly,
at the cost of a slightly wider connector on two boards. Neither daughter board
is laid out yet, so the timing was good.

**The four spare ways were not left floating.** Three double up the supply and
the return; the fourth became the ring's enable line when the ring moved to a
5 V boost (§7). The ring link pinout, identical at both ends:

| Pin | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 |
| --- | - | - | - | - | - | - | - | - | - | -- |
| Net | `Vdrive` | `Vdrive` | `Vdrive` | `RING_EN` | GND | GND | `NEOPIX` | GND | GND | `NEOPIX_RET` |

Three conductors for `Vdrive` and four for `GND`, up from two and two. The
IR-drop argument is thin on its own — at the ring's ~370 mA of battery-side
current a 50 mm 0.5 mm-pitch FFC drops only a few millivolts on two conductors —
so the real reasons are the return path and the loop area. `NEOPIX` keeps a
ground either side of it, which is what the 6-way pinout was already doing;
`NEOPIX_RET` now gets one too; and the supply and return conductors sit adjacent
across the pin 4 / pin 5 boundary rather than at opposite ends of the cable.

`NEOPIX_RET` is at pin 10 rather than tucked between grounds so that its run on
the main board does not have to cross the ground bus in `user_io.kicad_sch`.
`NEOPIX` is the signal that matters and it keeps grounds on both sides.

**Schematic work this needed.** The ring connector in
[`main/user_io.kicad_sch`](main/user_io.kicad_sch) and its partner in
[`led_ring/LedRing.kicad_sch`](led_ring/LedRing.kicad_sch) both moved from
`Connector_Generic:Conn_01x06` to `Conn_01x10`, with `Value`, `Footprint`
(`Connector_FFC-FPC:Hirose_FH12-10S-0.5SH_1x10-1MP_P0.50mm_Horizontal`), `MPN`,
`DigiKey_PN` and `Sourcing_Note` updated on both. The main-board end was rewired
against a `Vdrive` bus and a `GND` bus; the ring end is label-driven like the
rest of that sheet. The netlists on the two boards were checked pin-for-pin
against each other. Main board ERC is unchanged at **9 errors / 24 warnings**;
the ring board is still at **0 violations**.

Both links are now the same connector at both ends, so one cable part serves
both. Neither pinout is a palindrome, so a cable fitted the wrong way round puts
`Vdrive` onto ground pins — the conductor order belongs on the assembly drawing.
The stiffener face is no longer a trap: the panel connector has since been
corrected to the contacts-bottom part as well (§3), so every FFC connector on
the product is contacts bottom.

Drop-in-ish alternatives if you would rather stay at 6 ways: Molex `5051100697`
(`900-5051100697CT-ND`, $0.68) or Hirose `FH19C-6S-0.5SH(10)`
(`H125824CT-ND`). Both are 6-position, 0.5 mm, bottom-contact, right-angle, but
neither shares the FH12 land pattern.

---

## 2. Errors found in the existing BOM data

These were wrong in the schematic before this review.

| Part | Was | Now | Why it mattered |
| --- | --- | --- | --- |
| The 0 Ω antenna-feed link | MPN `RT0603BRE075KL` on a part valued `0R` | `RC0603JR-070RL` | `RT0603BRE075KL` is a **5 kΩ** resistor. That link connects the chip antenna to the GNSS `RF_IN` pin. Building to the old BOM would have put 5 kΩ in series with the antenna feed and killed GNSS reception. |
| The GNSS chip antenna | MPN `1575AT43A0040E` | `1575AT43A0040001E` | The old string is not an orderable Johanson part number — you cannot buy it. The catalogue part is `1575AT43A0040001E` (`712-1575AT43A0040001ECT-ND`). |
| The series diode that made `Vdrive` | Datasheet → Diotec **BAV99** PDF | Nexperia/onsemi BAV199 | BAV99 is a different device. The wrong datasheet is how the current-rating problem in §4 stayed hidden. |
| The power-switch, three button and haptic connectors | Datasheet → JST **PH** series PDF | JST **GH** series PDF | These are `SM02B-GHS-TB`, 1.25 mm GH series. The linked document described the 2.00 mm PH series, which is what the battery connector uses. Easy way to order the wrong crimps. |

### Generic passives were specified as unbuyable marketplace parts

Every generic passive MPN in the design resolved to a Digi-Key *marketplace*
listing with a prohibitive minimum order, flagged NCNR and no-backorder:

| Value | Old MPN | Problem |
| --- | --- | --- |
| 0.1 µF 0402 | `0402B104K160HI` (Aillen) | MOQ **20,000**, NCNR |
| 12 pF 0402 | `C0402C0G120J500NTB` (EYang) | MOQ **100,000**, NCNR |

All generic passives have been respecified to mainstream, catalogue-stocked
parts, listed in §6.

---

## 3. Reference-circuit checks

Datasheets added to [`../datasheets/`](../datasheets/) during this review:
MAX-M10S datasheet **and integration manual** (the reference design is in the
integration manual, not the datasheet), MX25R6435F, MCP73831, TPD1E05U06,
USB4085, ABS07, Si1308EDL, Johanson 1575AT43A0040, and the correct Nexperia
BAV199. The TI **TPS61023** datasheet was added later for the ring's boost
converter (§7); its Figure 8-1 is the reference design that stage is built
from. The GNSS folder previously held only a **Quectel LC86L** datasheet,
left over from an earlier design choice — the part actually fitted is a u-blox
MAX-M10S, which had no datasheet in the repo at all.

One download failed: Diodes Inc. dropped the connection for `DMG3415U.pdf`.
Everything needed to check the power-mux P-FET was available from the Digi-Key
parametrics, but the PDF itself is still missing.

### MAX-M10S GNSS — matches the reference

Checked against Integration Manual §B.1 "Typical design", Figure 35 (3.3 V):

- `VCC`, `V_IO`, `V_BCKP` commoned to +3.3 V with 0.1 µF + 10 µF — correct.
  Tying `V_BCKP` to the main rail rather than a separate backup is a legitimate
  choice; it keeps the RTC and orbit data alive whenever the rail is up, and
  loses hot-start data when the rail drops.
- `VIO_SEL` **left open** — correct, and this is the detail that is easy to get
  wrong. It must be grounded only for a 1.8 V design.
- `VCC_RF`, `LNA_EN`, `EXTINT`, `SAFEBOOT`, `TIMEPULSE`, `SDA`, `SCL` all left
  open — explicitly allowed for a UART-only minimum design with a passive
  antenna, which is what this is.
- The π-network on the antenna feed (all three positions unpopulated) with the
  0 Ω link fitted gives a clean chip-antenna-or-external-IPEX selection: fit the
  0 Ω link for the onboard chip antenna, or fit the series position of the
  π-network instead for the IPEX receptacle. Nicely done.

### Wio-SX1262 LoRa module — matches, with an enclosure consequence

Pinout verified pin-by-pin against datasheet §2.1. `ANT` (pin 9) is left
unconnected, and the datasheet footnote confirms this is correct: *"The RF pin
defaults to the IPEX interface and does not require soldering."* The reference
design in Figure 10 shows the π-network on pin 9 only for the SMT-pin variant,
which is a different orderable part.

The consequence is that **the LoRa antenna hangs off the module's own onboard
IPEX connector**, not off a connector you place. The GNSS has its own separate
IPEX receptacle. Two antennas, attached at two points you only partly control,
inside a circular handheld — this is a real constraint on the enclosure and on
the antenna keep-out, and it is worth drawing before committing the layout.

### DRV2605L haptic driver — matches the reference

`VDD` and `VDD/NC` both to +3.3 V with a 1 µF; `REG` decoupled by its own 1 µF,
which the datasheet requires; `IN/TRIG` tied to GND, correct for I²C mode; `EN`
driven from the MCU; `OUT+`/`OUT−` to the actuator connector. All correct.

### MCP73831 charger — matches the reference

`VDD` from VBUS with 4.7 µF, `VBAT` to +BATT with 4.7 µF, `PROG` to GND through
its programming resistor, `STAT` sinking the charge LED from VBUS through a
series resistor. The 2 kΩ programming resistor gives I_REG = 1000/2000 =
**500 mA**, matching the on-sheet note.

**Two considerations at 500 mA:**

1. *Thermal.* At VBUS 5 V charging a cell at 3.0 V, the SOT-23-5 dissipates
   (5 − 3) × 0.5 = **1 W**. That is far beyond what the package can shed. The
   MCP73831 has thermal regulation and will fold the current back rather than
   fail, so it is safe — but the real charge rate will be well under 500 mA and
   the part will run hot next to a LiPo. The existing "MAKE SURE THIS IS
   THERMALLY WELL CONNECTED" note on the power sheet is aimed at the LDO; the
   charger deserves the same attention, or a larger programming resistor.
2. *USB budget.* With plain 5.1 kΩ Rd pull-downs and no CC monitoring, the
   device is only entitled to the USB default — 500 mA from a USB 2.0 host.
   500 mA of charge plus system current exceeds that. Fine with a real USB-C
   charger, potentially out of spec on a laptop port.

### MAX17048 fuel gauge — matches the reference

`CELL` and `VDD` both to +BATT with 0.1 µF on `CELL`, `CTG` to GND, `EP` to GND,
`QSTRT` to GND, `ALERT` out to the MCU. This is the typical operating circuit.
The on-sheet note questioning whether `VDD` should be connected that way — it
should; commoning `VDD` and `CELL` at the pack positive is exactly what the
datasheet shows for a 1-cell gauge.

### RT9080 LDO and the power mux — correct, but one capacitor does nothing

The VBUS/battery mux around the P-FET and its blocking diode is the standard
arrangement and is wired correctly: with VBUS present the P-FET gate sits above
its source so the battery is isolated and VBUS feeds the LDO through the diode;
with VBUS gone the gate pull-down pulls the gate down, the FET conducts and the
battery takes over, with the diode blocking back-feed. Maximum V_GS is 5 V
against the DMG3415U's ±8 V limit.

**Finding:** a 0.1 µF capacitor is connected between the LDO's **pin 4 and
GND**. Datasheet §8 is explicit that on the `RT9080` — as distinct from the
`RT9080N` — pin 4 is `NC`, *"No internal connection. Leaving this pin floating
does not affect the functionality of the device."* The ordering-information
table on page 1 confirms: `RT9080` = without SNS pin.

So that capacitor is electrically inert. The regulator is stable and correct
because a 10 µF sits on `VOUT`, which satisfies the datasheet's requirement. But
if the intent was high-frequency output decoupling, it is on the wrong pin.
Either delete it, or move it to `VOUT` where it will do its job. Worth noting
the datasheet actively suggests tying pin 4 to GND copper for thermal reasons —
so a direct connection there is better than a capacitor.

### E-paper panel connector — ~~contacts top, unlike every other FFC on the product~~ → corrected to contacts bottom ✅ done

The SSD1681 charge-pump review in [`README.md`](README.md) stands.

An earlier pass recorded the panel connector as `FH12A-24S-0.5SH(55)`,
**contacts top**, against the `FH12-xS` interconnects which are **contacts
bottom**, and filed that under "the panel flex and the interconnect cable need
their stiffeners on opposite faces". **That was a part-number error, not a
design decision**, and it has been corrected rather than documented around.

The evidence is all inside the symbol itself. The shared library part in
[`../libs/smart_compass.kicad_sym`](../libs/smart_compass.kicad_sym) is named
`FH12-24S-0.5SH_55_`, its `MP` and `MPN` both read `FH12-24S-0.5SH(55)`, its
`Purchase-URL` is a SnapEDA link for `FH12-24S-0.5SH(55)`, its land pattern is
`HRS_FH12-24S-0.5SH_55_`, and its `Description` — still on the placed symbol —
reads "24 Position FFC, FPC Connector **Contacts, Bottom** …". Only `Value`,
`MPN` and `DigiKey_PN` on the *placed* instance said `FH12A` / `HFK124CT-ND`.
The sourcing pass then verified that string against Digi-Key, found a real and
stocked part, and recorded the resulting contradiction as a constraint instead
of a typo.

The panel connector is now **`FH12-24S-0.5SH(55)`** (`HFJ124CT-ND`, Active,
~15,300 in stock, $2.41/1 cut tape, $1.74/100). It is the same FH12 series, same
0.5 mm pitch, same 24 positions, same right-angle SMT mounting, same 2.00 mm
height above board, same flip-lock actuator and the same 0.30 mm FFC thickness
as the part it replaces — only the contact face differs, and the land pattern is
unchanged, so **there is no layout consequence**. The `FH12A` variant is $0.07
cheaper; that is not a reason to keep the wrong contact face.

The `Value` and `MPN` fields on the footprint in
[`../libs/smart_compass.pretty/HRS_FH12-24S-0.5SH_55_.kicad_mod`](../libs/smart_compass.pretty/HRS_FH12-24S-0.5SH_55_.kicad_mod)
carried the same `FH12A` string and were corrected too, so the error cannot leak
back out through a position file or a fab drawing. Pad geometry was not touched.

**Every FFC connector on the product is now contacts bottom** — the panel
connector, both main-board interconnects and both daughter-board interconnects —
so every cable on the device presents its contacts to the board and takes a
stiffener on the same face. That is one less way to build a board that looks
right and does not work.

Display-board ERC is unchanged at **1 warning**, the pre-existing
symbol/library mismatch on the panel connector: the library symbol's `Value` is
the symbol *name* (`FH12-24S-0.5SH_55_`) rather than the part number, so the
cached copy and the library copy still differ. That predates this work and is
cosmetic.

---

## 4. The `Vdrive` diode was overstressed — ✅ resolved, the diode is gone

**Resolved by §7.** The diode has been replaced by a ferrite bead, because the
job it was doing no longer exists. The analysis below is kept because it is what
drove the decision to boost the ring.

### What was wrong

A BAV199 made `Vdrive` by dropping `+BATT` about 0.7 V, so that 3.3 V logic
could clear the LED's V_IH at a full battery charge. The intent was sound. The
part was not.

Tracing it against the Nexperia datasheet that is now in the repo: pin 1 is
`A1`, pin 2 is `K2`, and **pin 3 is the common `K1, A2` node**. The schematic
used pin 1 (`+BATT`) and pin 3 (`Vdrive`), leaving pin 2 open — so only one
diode was in circuit and the drop was one junction. That part was correct.

The problem is the rating. BAV199 is a *low-leakage signal* diode:

- **I_F max = 160 mA** single-diode-loaded, 140 mA double-loaded.
- Total package dissipation 250 mW.

The ring draws roughly **250 mA at full white** with the Inolux LEDs — 5 mA per
colour plus 0.5 mA of static IC current, times sixteen. (`README.md` quotes
~300 mA for the WS2812B-2020 it replaces; the conclusion is the same either
way.) That is well over the diode's maximum forward current, dissipating around
0.25 W in a 250 mW package.

There is a second-order consequence that matters more than the rating. At
300 mA a BAV199 does not drop 0.7 V; its V_F is specified at **1.25 V at
150 mA**. So `Vdrive` collapses to roughly 3.0–3.1 V at a full 4.2 V battery,
against the LED's minimum supply of **3.5 V**. The ring would brown out at
high brightness — and it would do it intermittently, at full white, which is a
miserable fault to chase.

A 0.5 A Schottky would have fixed the rating — `MBR0540-TP`
(`MBR0540TPMSCT-ND`) is already on the BOM in the power mux and the display
charge pump, and drops ~0.45 V at 300 mA. But it would not have fixed the real
problem.

Even with a Schottky the headroom was thin by design. At a 3.7 V nominal cell
`Vdrive` is ~3.25 V, below the Inolux 3.5 V minimum; near the bottom of the
discharge curve it is worse. The diode-drop approach only strictly satisfied
the datasheet near full charge.

The trap was that removing the diode did not rescue it either. With VDD tied
straight to `+BATT`, V_IH at a full 4.2 V cell is 0.7 × 4.2 = 2.94 V, against an
nRF52840 V_OH of roughly 2.9–3.0 V — marginal, which is precisely why the diode
was there. The two ends of the discharge curve pulled in opposite directions:
the diode was needed at the top for logic levels and hurt at the bottom for
supply compliance.

### What was done instead

The ring was boosted to 5 V on the ring board and `NEOPIX` level-shifted, which
is in spec across the whole discharge curve — §7. With the boost taking
0.5–5.5 V in, the cable should carry as much voltage as it can, so the drop
became pure loss and `Vdrive` is now simply `+BATT`.

The diode is therefore replaced by a TDK `MPZ1608S601ATA00` **ferrite bead**
(`445-MPZ1608S601ATA00CT-ND`, Active, ~1.27 M in stock, $0.10, 0603). 600 Ω at
100 MHz, 1 A rated, 150 mΩ DCR — about 70 mV and 32 mW at the 460 mA worst case.
It is a bead rather than a 0 Ω link because with the new 22 µF bulk capacitor
beside it, it forms an LC filter that keeps the boost's 1 MHz switching current
off `+BATT`, which also feeds the GNSS LNA and the LoRa module.

The reference to the old `MBR0540` swap in the summary table is withdrawn.

---

## 5. Other considerations

### The USB-C CC pull-downs were not the spec value

These are the CC1/CC2 pull-downs on the USB-C receptacle, which the Type-C spec
requires to be **5.1 kΩ**. They were 5 kΩ — not an E-series value, and
specified as a 0.1% thin-film part at $0.10 each where a 1% thick-film is $0.01.

I have set them to 5.1 kΩ / `RC0603FR-075K1L`, and standardised the other three
5 kΩ parts onto the same value — a reset pull-up, an LED series resistor and the
GNSS pull-down, all non-critical. That removes a BOM line and replaces five 0.1%
precision parts with one ordinary one. The 2% value change is immaterial in all
three of those circuits.

### The ring data-line damping resistor is a 0.1% precision part doing a rough job

The 500 Ω series damping resistor on `NEOPIX` is a problem of availability, not
function. 500 Ω is not an E24 value, so the only parts that exist are precision
ones — `RT0603BRC07500RL` is 0.1%, 15 ppm/°C, $0.25. For source-end damping,
510 Ω at 5% (`RC0603JR-07511RL`) does the identical job for about a cent. Left
as-is since it is correct and in stock; flagged in its `Sourcing_Note`.

Worth noting the Inolux datasheet independently endorses the value: its typical
application circuit calls for protective series resistors of **about 500 Ω** on
the data line, to stop live plugging and unplugging damaging the internal
signal pins. So that resistor is doing exactly the right job at exactly the
right value.

### The display board's boost inductor — the suggested replacement is obsolete

`README.md` suggests `NR3015T100M` for that inductor. Digi-Key lists it as
**Obsolete**, zero stock, with a last-buy date of 2026-03-31 that has already
passed.

Specified instead: **Bourns `SRN3015TA-100M`** (`SRN3015TA-100MCT-ND`, ~2,900 in
stock, $0.40) — 10 µH, 800 mA rated / 750 mA saturation, shielded, AEC-Q200,
−40 to +125 °C, and the same **3.0 × 3.0 × 1.5 mm** outline, so it stays in the
existing `L_Taiyo-Yuden_NR-30xx` land. Verify the pad geometry against the
Bourns drawing before layout; the outline matches but the pads are not
guaranteed identical. KiCad has no `SRN3015` footprint, so the existing one is
the right starting point.

The value field has been changed from `10uH 1A` to `10uH` because the part is
800 mA, not 1 A. That is ample — the 3 Ω current-sense resistor sets a peak
inductor current well under 500 mA.

### The ±20 V gate-rail capacitors sit at a 25 V rating

The 1 µF parts on `PREVGH` (~+20 V) and `PREVGL` (~−20 V) are specified at 25 V.
Class-II ceramics lose most of their capacitance near their rated voltage, so a
25 V X7R at 20 V is delivering a fraction of its nominal value. A 50 V part in
the same 0603 land would be the safer choice. Flagged per-part.

### The display board's switching FET has the longest lead time on the design

`SI1308EDL-T1-GE3` has exactly one reel (3,000) in Digi-Key stock and a
**55-week** factory lead. Fine for prototypes, a genuine risk for anything
beyond. Worth qualifying a second source now rather than later.

### DRV2605L stock is thin

~105 pieces in stock, 16-week lead. Enough for a 5-board run; watch it.

### The SWD and expansion headers have no part number

Both are generic 2.54 mm pin headers with no MPN assigned. They are almost
certainly not fitted on a finished unit, so this is deliberate-looking rather
than an omission — but if you want them populated on the prototypes they need a
part.

### Rough cost

At single-unit pricing the three boards come to roughly **$87 per device** in
components — including the ring LEDs, now that they have a price (16 × $0.53 =
$8.48, falling to ~$4.20 at qty 100), and the ring's 5 V boost stage from §7
(~$1.75, ~$1.10 at qty 100). That is against the ~£50/$60 estimate in the
top-level `README.md`.

The gap is almost entirely the three expensive modules: IMU ($15.79), GNSS
($11.42) and MCU ($9.60) are nearly half the total on their own. Quantity 10
brings the whole thing to roughly $70. Assembly is extra.

---

## 6. Respecified passives

The generic passive values now point at mainstream catalogue parts:

| Value / package | Part | Digi-Key | Notes |
| --- | --- | --- | --- |
| 0.1 µF 0402 | `CL05B104KO5VPNC` | `1276-6844-1-ND` | 16 V X7R, AEC-Q200, ~448 k stock |
| 12 pF 0402 | `CL05C120JB5NNNC` | `1276-1178-1-ND` | 50 V C0G, ~95 k stock |
| 4.7 µF 0402 | `CL05A475MP5NRNC` | `1276-1482-1-ND` | 10 V X5R |
| 1 µF 0603 | `CL10B105KA8NNNC` | `1276-1184-1-ND` | 25 V X7R |
| 10 µF 0805 | `CL21A106KPFNNNG` | `1276-6456-1-ND` | 10 V X5R, ~172 k stock |
| 22 µF 0805 | `CL21A226KPCLRNC` | `1276-6786-1-ND` | 10 V X5R, ~117 k stock. Chosen over the 6.3 V part because it has to serve both the battery-side `Vdrive` rail and the ring's 5 V output, where a 6.3 V X5R would lose heavily to DC bias |
| 0 Ω 0603 | `RC0603JR-070RL` | `311-0.0GRCT-ND` | ~10.8 M stock |
| 3 Ω 0603 | `RC0603FR-073RL` | `13-RC0603FR-073RLCT-ND` | 1% |
| 500 Ω 0603 | `RT0603BRC07500RL` | `13-RT0603BRC07500RLCT-ND` | 0.1%, see §5 |
| 732 k 0603 | `RC0603FR-07732KL` | `311-732KHRCT-ND` | 1%, boost feedback upper leg, see §7 |
| 1 k / 2 k / 2.2 k / 3.9 k / 4.7 k / 10 k / 27 Ω / 100 k 0603 | Yageo `RC0603FR-07…L` | 10 k = `311-10.0KHRCT-ND` | 1% thick film |
| 5.1 k 0603 | `RC0603FR-075K1L` | `311-5.10KHRCT-ND` | 1% |

Digi-Key part numbers are recorded only where I verified them directly. The
27 Ω, 2.2 k, 3.9 k, 4.7 k, 100 k, 2 k and 1 k resistors carry a verified
manufacturer part number but no Digi-Key number — they are ordinary members of
the Yageo `RC0603FR-07` line and will resolve at order time.

Several of the MLCCs above returned a zero stock figure during this review while
other members of the same series showed hundreds of thousands. MLCC stock
rotates fast and the API's per-variation figures are patchy, so for the
capacitors treat the MPN as "this value, voltage and dielectric, from this
series" rather than a hard commitment to that exact suffix.

---

## 7. The LED ring's 5 V boost

The ring board now makes its own 5 V. This is what closes out both §1's V_IH
question and §4's diode problem, and it is checked against TI's own reference
design for exactly this case: **Figure 8-1, "Li-ion Battery to 5-V Boost
Converter"**, in the datasheet now saved at
[`../datasheets/TPS61023_TI.pdf`](../datasheets/TPS61023_TI.pdf) — 2.7–4.35 V
in, 5 V out, 10 µF input, 1 µH, 732 k / 100 k feedback divider, 2 × 22 µF out.

### The converter — TI TPS61023

`TPS61023DRLR` (`296-TPS61023DRLRCT-ND`, Active, ~20,000 in stock, $1.22/1 and
$0.70/100), SOT-563, 1.2 × 1.6 mm. Input 0.5–5.5 V, output adjustable
2.2–5.5 V, 1 MHz, synchronous, ~93% efficient at our operating point.

Two properties decided it over the alternatives:

- **True input-to-output disconnect in shutdown**, 0.1 µA. Most boost
  converters pass the input through to the output via the rectifier even when
  disabled, so disabling them does not de-power the load. This one does, and
  that is what makes `RING_EN` worth a cable conductor: the sixteen LEDs draw
  **0.5 mA each even when dark**, 8 mA for the ring, which on a 3-day battery
  target is not something to leave running.
- **EN thresholds are low-voltage** — logic high max 1.2 V — so a 3.3 V GPIO
  drives it directly with no shifter of its own.

Values follow the reference, with one deliberate departure:

| | Reference (Fig. 8-1) | Here | Why |
| --- | --- | --- | --- |
| C_IN | 10 µF | 10 µF | as reference |
| L | 1 µH | **2.2 µH** | see below |
| R1 / R2 | 732 k / 100 k | same | V_OUT = 0.595 × (1 + 732/100) = **4.95 V** |
| C_OUT | 2 × 22 µF | 2 × 22 µF | as reference |

The inductor is the one departure. The reference targets 1.5 A out; the ring
needs 250 mA. At 1 µH that would give about 1.1 A peak-to-peak of ripple
current, which is a lot of 1 MHz magnetic noise to put inside a handheld with a
GNSS antenna and a LoRa module in it. At 2.2 µH the ripple halves and the peak
inductor current is ~0.68 A. The datasheet supports 0.37–2.9 µH, so this is
inside the specified range, not a liberty.

Specified: **Murata `DFE252010F-2R2M=P2`** (`490-13042-1-ND`, ~8,100 in stock,
$0.25) — 2.2 µH, 2.3 A rated, 3.1 A saturation, 97 mΩ, 2.5 × 2.0 × **1.0 mm**,
which matters for a thin medallion. The footprint is the generic
`Inductor_SMD:L_1008_2520Metric` land; **check the pads against the Murata
drawing before layout**, the same caveat as the display board's boost inductor.

No feedforward capacitor is fitted. The datasheet recommends one only above
about 40 µF of effective output capacitance; a pair of 10 V 22 µF X5Rs at 5 V
of DC bias lands near 20 µF, so it is not needed.

### The level shifter — 74AHCT1G125

`SN74AHCT1G125DBVR` (`296-4708-1-ND`, Active, ~53,500 in stock, $0.14),
SOT-23-5. A single non-inverting buffer on `NEOPIX`, powered from the boosted
5 V, `OE` tied low, with its own 0.1 µF supply bypass — the TI reference design
contains only the converter, so it has no logic IC in it and no bypass for one,
but the buffer still needs local charge for its nanosecond edges.

It is mandatory, not optional: at VDD = 5.0 V the LED needs V_IH ≥ 3.4 V and
the nRF52840 cannot produce it. **AHCT** is the right family because its TTL
input thresholds (V_IH 2.0 V at VCC 5 V) accept a 3.3 V signal directly; the
plain AHC part would want 3.5 V and would not work.

The SC-70-5 variant `SN74AHCT1G125DCKR` reads zero stock at Digi-Key for both
TI and UMW; the SOT-23-5 part is the stocked one and is easier to hand-solder
anyway. Diodes Inc. `74AHCT1G125W5-7` (`74AHCT1G125W5-7DICT-ND`, ~30,000,
$0.17) is a same-land second source.

### The return path — a 2.2 k / 3.9 k divider

The last LED's `DOUT` now swings to 5 V, and it crosses the cable to a 3.3 V
board. A 2.2 kΩ upper leg (the old 500 Ω damping resistor re-valued) and a new
3.9 kΩ lower leg divide it to **3.20 V** at the connector. The upper leg still
sits at the LED's output, so it goes on damping the return line at its source
end as before.

### What this cost

Five new BOM lines on the ring board — the converter, the level shifter, the
inductor, the 732 k feedback resistor and the 3.9 kΩ divider leg; the 100 k
feedback resistor and the capacitors reuse values already on the BOM — and one
on the main board for the ferrite bead, against one line deleted for the diode.
About **$1.75** per unit at quantity one, $1.10 at quantity 100.

The running cost is the conversion loss. At 93% efficiency the ring's 1.24 W at
full white draws about 1.33 W from the battery instead of the ~0.9 W the
under-volted arrangement drew — but the under-volted arrangement was dim and
out of spec, so it is not really the same product. The 20 µA the converter
burns while enabled is immaterial next to the 8 mA the LEDs burn, and `RING_EN`
removes both.

### What the layout has to achieve

Not attempted here — it is layout — but the boost imposes real constraints, and
datasheet §10 is specific:

- The high-current loop is the switch FET → rectifier FET → output capacitors →
  ground. Keep it short and tight; both output capacitors want to be close to
  `VOUT` **and** `GND`, not just `VOUT`.
- The input capacitor belongs hard against `VIN` **and** `GND`.
- Minimise the copper area of the `SW` node, and keep a ground plane under the
  converter.
- The feedback divider is high-impedance — keep the `FB` trace short and away
  from `SW`.
- The level shifter's bypass capacitor belongs at the buffer's own supply pin,
  not merely somewhere on the 5 V rail.
- The whole converter is a 1 MHz noise source sitting inside the LED ring,
  which sits inside a medallion with a GNSS antenna and a LoRa antenna. Where
  it lands relative to those is a judgement call worth making deliberately.

---

## Summary of what needs a decision

| # | Item | Decision needed |
| --- | --- | --- |
| 1 | Ring LEDs | Done (Inolux, symbol + footprint + STEP installed). The V_IH question is closed — the ring runs at VDD = 5.0 V, the condition the 3.4 V figure is specified at (§7). Outstanding: check the STEP's rotation in the 3D viewer |
| 2 | IMU | Done (BNO086 swapped in; drop-in, no wiring or layout change) |
| 3 | QSPI flash | Done (Infineon `S25FL064LABNFI043`; identical land, identical pinout, no layout change). Outstanding: create the `SON80P400X400X60-9N` land in `libs/smart_compass.pretty/` — it never existed; and have the firmware issue Deep Power Down (`B9h`) so standby costs 2 µA rather than 20 µA |
| 4 | Ring and display interconnects | Done (both links standardised on the 10-way FH12). Outstanding: get the conductor order onto the assembly drawing |
| 4b | E-paper panel connector | Done (corrected to the contacts-bottom `FH12-24S-0.5SH(55)`, `HFJ124CT-ND` — the `FH12A` string was a part-number error; same land, no layout change). Every FFC connector on the product is now contacts bottom |
| 5 | The `Vdrive` diode | Done. The diode-drop scheme was abandoned rather than patched: the ring is boosted to 5 V on its own board (§7), so the diode became a 600 Ω ferrite bead with a 22 µF bulk capacitor alongside it |
| 5b | Ring 5 V boost | Done (TPS61023, 2.2 µH inductor, 732 k/100 k divider and the input and output capacitors; a 74AHCT1G125 shifts `NEOPIX`; a 2.2 k/3.9 k divider brings `NEOPIX_RET` back down). Outstanding: check the inductor's pads against the Murata drawing, and lay the converter out to datasheet §10 |
| 5c | `RING_EN` | Done (FFC pin 4 changed from a fourth `Vdrive` way to `RING_EN`, driven from the MCU's `P1.00`). Outstanding: the conductor order on the assembly drawing must reflect it, and firmware must hold `NEOPIX` low whenever `RING_EN` is low |
| 6 | The inert capacitor on the LDO's pin 4 | Delete, or move to `VOUT` |
| 7 | Charge-status LED | Done (kept the red `LTST-C191KRKT`; value relabelled `RED`) |
| 8 | GNSS pull-down | Keep 5.1 k, raise, or remove |
| 9 | Charger current | Keep 500 mA and accept thermal foldback, or raise the programming resistor |
