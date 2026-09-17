# Smart Compass

A handheld GNSS + LoRa mesh compass that points at people and places with a ring
of addressable LEDs, plus a 1.54" e-ink panel. nRF52840 (BMD-340) MCU, firmware
in Rust on Embassy. See [README.md](README.md) for the product rationale and
[UserFlowLibrary.md](UserFlowLibrary.md) for user flows.

## Tool use: shell is a last resort

**Prefer MCP tools and the dedicated file tools over shell commands, even when
the shell route would be quicker.** A slower call through the proper tool is the
right trade — it surfaces in the permission UI, produces clickable file links,
and returns structured data instead of text to re-parse.

In order of preference:

1. **MCP tools** — the `mcp__kicad__*` server covers essentially the whole KiCad
   toolchain. Use `ToolSearch` to find the right one before assuming there isn't
   one; most are deferred and not listed until you search.
2. **Dedicated file tools** — `Read` for file contents, `Grep` for content
   search, `Glob` for finding files by pattern, `Edit`/`Write` to modify. Never
   `Get-Content`, `Select-String`, `cat`, `grep`, or `find`.
3. **Shell** (`Bash` / `PowerShell`) — only when nothing above can do the job:
   `git`, `cargo`, process and lock-file checks, and file-system mutations such
   as moving or deleting files.

Specifically, **do not call `kicad-cli` directly.** Every export, ERC run and
netlist generation has an MCP equivalent (`run_erc`, `generate_netlist`,
`export_*`, and the schematic query and edit families).

## Hardware

Three KiCad projects, one per board. KiCad has no multi-board project, so the
shared parts live in [libs/](libs/) and each board has a project-scope
`sym-lib-table` / `fp-lib-table` pointing at `${KIPRJMOD}/../../libs/`.

| Directory | Project | What it is |
| --------- | ------- | ---------- |
| [boards/main/](boards/main/) | `SmartCompass` | MCU, GNSS, LoRa, IMU, flash, power tree, haptics, buttons, USB |
| [boards/led_ring/](boards/led_ring/) | `LedRing` | The 16-LED ring (Inolux IN-PI15TAT5R5G5B) and its 5 V boost |
| [boards/display/](boards/display/) | `Display` | E-ink connector and its charge-pump support |

[boards/README.md](boards/README.md) has the interconnect pinouts and the
outstanding work on the main board. Datasheets for every part are in
[datasheets/](datasheets/) — check the reference circuit there before trusting a
migrated schematic.

### Schematic work only

Edit `.kicad_sch` files, hierarchical sheets, symbols, nets, and library
scaffolding. **Do not create or modify `.kicad_pcb` files** — no footprint
placement, board outlines, or routing. Layout on this product depends on
judgement that is hard to exercise from the outside: antenna keep-out, the
circular medallion form factor, enclosure fit, LED ring geometry, and how the
boards stack. Reading the `.kicad_pcb` for context is fine and often useful.

When a task runs into layout, stop at the schematic and describe what the layout
needs to achieve.

### Never name parts by reference designator in documentation

In `boards/README.md`, `boards/COMPONENT_SOURCING.md` and any other prose,
commit message or table, refer to a component by **what it does** — "the QSPI
flash", "the ring's boost converter", "the 500 Ω series damping resistor on the
ring data line" — never by its designator. Add the value or manufacturer part
number when that is what distinguishes one part from several like it.

Designators are guaranteed to change. KiCad re-annotation renumbers parts, and
tools that re-serialise a schematic can renumber them wholesale without being
asked, so a designator written into prose silently comes to point at a different
component. Manufacturer part numbers, net names, pin names and footprint names
are stable and can be used freely.

This applies to documentation only. Designators remain the identity of a
placement inside the schematic files and in a BOM.

### Before editing a schematic

KiCad holds a live lock on open files. Check for `~*.kicad_sch.lck` in the
project directory first — if one exists, the file is open in Eeschema and any
edit written to disk will be silently overwritten when the user next saves.
There is no way to make Eeschema reload from disk (`discard_or_reload` is
PCB-only), so ask the user to close it rather than editing under the lock.

After any schematic edit, run `validate_schematic` and `run_erc`.

## Firmware

A Cargo workspace in [firmware/](firmware/) with two members:
`smart_compass_rs` (the device firmware) and `compass_sim` (a host-side
simulator for exercising the firmware logic).

```bash
make build_firmware   # cargo build -p smart_compass_rs --features embedded --target thumbv7em-none-eabihf
make run_sim          # cargo run -p compass_sim
make test             # both test suites
```
