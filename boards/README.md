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

Parts that are not on any board - the haptic actuator, the panel, the cell, the
case switches and buttons, the antennas, and the cables and housings joining
them - are listed in [EXTRA_PARTS_BOM.md](EXTRA_PARTS_BOM.md), because they never
appear in a BOM exported from a schematic. Outstanding sourcing work and open
risks are in [COMPONENT_SOURCING.md](COMPONENT_SOURCING.md).

## Libraries

`../libs/smart_compass.kicad_sym` and `../libs/smart_compass.pretty` hold the
parts the boards share. They were extracted from the main board, where they had
only ever existed embedded inside the `.kicad_sch` and `.kicad_pcb` files - they
were never in any library table, so a new project could not place them.

Only the parts the daughter boards needed were extracted: `WS2812B2020`,
`FH12-24S-0.5SH_55_` and `LQM18DN100M70L`. A fourth symbol,
`IN-PI15TAT5R5G5B`, was added later for the Inolux ring LED that replaced the
WS2812B - its pin numbering is completely different, so it needed its own
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
