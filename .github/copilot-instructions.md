# Copilot Instructions for Medallion Project

This is a KiCad hardware design project for an off-grid peer tracking device.

## kicad-happy Analysis Tools

The kicad-happy skills are installed at `/opt/kicad-happy/skills/`. Use these Python scripts to analyze KiCad design files:

### Schematic Analysis
```bash
python3 /opt/kicad-happy/skills/kicad/scripts/analyze_schematic.py Medallion_Board/Medallion_Board.kicad_sch
```

### PCB Analysis
```bash
python3 /opt/kicad-happy/skills/kicad/scripts/analyze_pcb.py Medallion_Board/Medallion_Board.kicad_pcb --full
```

### EMC Analysis
Run schematic and PCB analysis first to produce JSON, then:
```bash
python3 /opt/kicad-happy/skills/emc/scripts/analyze_emc.py -s schematic.json -p pcb.json
```

### SPICE Simulation
```bash
python3 /opt/kicad-happy/skills/spice/scripts/run_spice.py <netlist>
```

### Reference Documentation
Detailed methodology guides are in `/opt/kicad-happy/skills/*/references/*.md`.

### Skill Instructions
Each skill's full instructions (triggers, usage patterns, output formats) are in `/opt/kicad-happy/skills/*/SKILL.md`.

## Project Context

- KiCad schematic: `Medallion_Board/Medallion_Board.kicad_sch`
- KiCad PCB: `Medallion_Board/Medallion_Board.kicad_pcb`
- Component list: `Medallion_Board/board_components.md`
- Block diagram: `Medallion_Board/block_diagram.md`
- Datasheets: `Medallion_Board/datasheets/` and `datasheets/`
