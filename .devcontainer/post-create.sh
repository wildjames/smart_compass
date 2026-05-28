#!/bin/bash
set -e

# Update kicad-happy to latest
cd /opt/kicad-happy && git pull

# Set up skills in ~/.agents/skills/ (discovered by Copilot)
mkdir -p ~/.agents/skills
for skill in kicad spice emc datasheets bom digikey mouser lcsc element14 jlcpcb pcbway kidoc; do
  ln -sf "/opt/kicad-happy/skills/$skill" "$HOME/.agents/skills/$skill"
done

echo "kicad-happy skills installed. Available skills:"
ls ~/.agents/skills/
