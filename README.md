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
- The device must be secure - there must be no way for non-paired devices to be able to discern the location of a device, even if they are part of the same mesh network.
