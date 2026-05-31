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

## Thoughts

Should I add haptics or sound for the first version? More complexity and the board is already pretty complex... I'm getting doubtful of my ability to one-shot this with no mistakes!

The component cost per device is up to ~£50. The boards are about £5 per board for the first run (some heavy discounts, I shouldn't squander that). Assembled, the first shipment of 5 boards would cost $115 plus ~$60 in components per board makes that up to $400. That's a hefty chunk of cash!!! I could try getting just two assembled for this initial run, which brings it down to like, $200, but that's still pricey.

## TODO list

- There are meshtastic boards already out there that are close to what I'm doing here. I should not ignore this, and use them for help in seeing what works and what is cost-effective. e.g. this [card format I like a lot](https://thepihut.com/products/sensecap-card-tracker-t1000-e-for-meshtastic), perhaps more of a pokedex looking thing?
- Haptics might be easy-ish to add. [Driver](https://www.digikey.co.uk/en/products/detail/texas-instruments/DRV2605LDGSR/5014144), [LDA](https://www.digikey.co.uk/en/products/detail/vybronics-inc/VG1040003D/10285886). However, the actuator is 10mm thick, so may contribute meaningfully to the package size. Remember, I still need to fit in a lipo to the back of this thing
