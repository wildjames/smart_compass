# Smart Compass

The product is going to be something similar to the compass in the Pirates of the Caribbean.

It has a ring of LEDs that indicate the direction you need to walk in to get to the place you want to be, which can be other devices in a tracking group, waypoints on a navigation route (e.g. hiking), Atlas Obscura sites for the city you're in, or some location you pinned (e.g. where you parked your car, or something like that).
Locations will be either pins dropped by the compass itself, or loaded to the device using a companion app on a phone - communicating with that means the device needs BLE. The nRF52840 module gives that.

The compass has a GNSS module for getting its own location, and a 9-axis IMU for the compass orientation.
It has a LoRa radio for kilometer-range mesh networking, allowing it to maintain an off-grid communication layer in cases where the cell network is either unavailable (e.g. rural locations, mountains), or overloaded (e.g. music festivals and other large events) - or if you just don't want to be carrying a phone (e.g. a kid being set loose in a shopping centre).

For typical single-device operation, however, the compass will rely on a companion phone app. Using the app, the user can navigate to desired location without referring to their phone. These can be user-defined, like setting the navigation destination on Maps, or get a list of Locations of Interest (LoI) from something like Atlas Obscura. I could also do a "hot/cold" thing with phone RSSI signal strength to get a kind of phone finder feature, but that may not be very useful.

## Design requirements

The device has a few core requirements:

- It must operate with no companion phone or PC. It's fine to have a phone for initial setup, or for additional features, but peer-to-peer operation must be entirely on-device.
- Tracking must be done by the device. The GNSS network must be used for this.
- The device must be secure - there must be no way for non-paired devices to be able to discern the location of a device, even if they are part of the same mesh network.
- Communication between devices must be long-range. Kilometer ranges between peers is a requirement, and longer ranges with a mesh network is also necessary.
- The battery must last for at least a day, but 3 day battery life is the target. Deep sleep, low power, and high polling modes should be used to assist this.
- The device must be handheld. No external antenna is ideal, but the option to add one may be worthwhile.
- The device must have flash storage
- The device needs an on-board way to communicate the direction and distance of paired devices.
  - The device must have a ring of addressable RGB LEDs for peer indication.
  - The device must have a haptic engine for feedback
  - The device may have a speaker for feedback.


I'm writing down user flows as I think of them in the [User Flow Library](UserFlowLibrary.md) page.

-----


## Archived KiCAD packages

- 0.1: First edition.
- 0.2: Submitted to reddit for hope of review. First hierarchical schematic.
- 0.3: Reviewed connections, fixed a few bugs with USB and SPI.
- 0.4: Rerouted the board to place the e-ink connector correctly, and the MCU more centrally. Also made the antenna placement more seriously, with proper clearance zone and via stitching.


-----

## Thoughts

The component cost per device is up to ~£50. The boards are about £5 per board for the first run (some heavy discounts, I shouldn't squander that). Assembled, the first shipment of 5 boards would cost $115 plus ~$60 in components per board makes that up to $400. That's a hefty chunk of cash. I could try getting just two assembled for this initial run, which brings it down to like, $200, but that's still pricey.

## TODO list

- I can apparently use a pi pico 2 as an SWD programmer, to flash the bootloader, and from there I can just upload via the native USB.
- I will do the programming in Rust, using the Embassy ecosystem. This will give me the memory safety and speed of rust (nice when there is no easy way to see crashes on the device), but also the fearless concurrency will be a massive boon with all the ICs that need to be interfaced with.
