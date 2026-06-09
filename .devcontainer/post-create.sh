#!/bin/bash
set -e

# Update kicad-happy to latest
cd /opt/kicad-happy && git pull

# Set up skills in ~/.agents/skills/ (discovered by Copilot)
mkdir -p ~/.agents/skills
for skill in kicad spice emc datasheets bom digikey mouser lcsc element14 jlcpcb pcbway kidoc; do
  ln -sf "/opt/kicad-happy/skills/$skill" "$HOME/.agents/skills/$skill"
done

# Install probe-rs for flashing/debugging nRF52840
if ! command -v probe-rs &> /dev/null; then
  cargo install probe-rs-tools
fi

# Add thumbv7em-none-eabihf target if not already present
rustup target add thumbv7em-none-eabihf

echo "kicad-happy skills installed. Available skills:"
ls ~/.agents/skills/
