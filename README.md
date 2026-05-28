# Off-grid peer tracking

I need to eventually name the project.

The goal is to create a device that allows the off-grid tracking of a group of people, for example at festivals where cell networks are overloaded, or in remote places (e.g. camping)

## Design requirements

The device has a few core requirements:

- It must operate with no companion phone or PC. It's fine to have a phone for initial setup, or for augmented features, but typical operation must be entirely on-device.
- Tracking must be done by the device. The GNSS network must be used for this.
- Communication between devices must be long-range. Kilometer ranges between peers is a requirement, but longer ranges with a mesh network is the goal.
- The battery must last for at least a day, but 3 day battery life is the target. Deep sleep, low power, and high polling modes should be used to get this.
- The device must be handheld. No requirement for an external antenna, but the option to add one would be worthwhile.
- The device needs an on-board way to communicate the direction and distance of paired devices.
  - The device should have a ring of addressable RGB LEDs for peer indication.
  - The device should have a haptic engine for feedback
  - The device should have a buzzer for feedback.
  - The device may have an OLED screen for detail, if appropriate.
- The device must be secure - there must be no way for non-paired devices to be able to discern the location of a device, even if they are part of the same mesh network.

## TODO list

- There are meshtastic boards already out there that are close to what I'm doing here. I should not ignore this, and use them for help in seeing what works and what is cost-effective. e.g. this [card format I like a lot](https://thepihut.com/products/sensecap-card-tracker-t1000-e-for-meshtastic), perhaps more of a pokedex looking thing?
- I should think about if I need an ESP32, or if I can use the lower-power [nRF52](https://www.digikey.co.uk/en/products/detail/nordic-semiconductor-asa/NRF52840-CKAA-R/15929796).
  - I don't need wifi, but I do need GPIO pins...
- I need to look at LoRa ICs, and get something I can throw on my schematic. I don't think the module is gonna be feasible.
  - I think this one is better for my case: [E22-900M22S](https://www.lcsc.com/product-detail/C411293.html). Wire it in. It's SPI communication and the example schematic is crazy simple.
