# Smart Compass

The product is a smart compass, something similar to the compass in the Pirates of the Caribbean. It has a ring of LEDs that indicate the direction you need to walk in to get to the place you want to be, which can be other devices in a tracking group, waypoints on a navigation route (e.g. hiking), Atlas Obscura sites for the city you're in, or some location you pinned (e.g. where you parked your car, or something like that).
Locations will be either pins dropped by the compass itself, or loaded to the device using a companion app on a phone - communicating with that means the device needs BLE. The nRF52840 module gives that.

The compass has a GNSS module for getting its own location, and a 9-axis IMU for the compass orientation.
It has a LoRa radio for kilometer-range mesh networking, allowing it to maintain an off-grid communication layer in cases where the cell network is either unavailable (e.g. rural locations, mountains), or overloaded (e.g. music festivals and other large events) - or if you just don't want to be carrying a phone (e.g. a kid being set loose in a shopping centre).

For typical single-device operation, however, the compass will rely on a companion phone app to augment it. Using the app, the user can navigate to desired location without referring to their phone. These can be user-defined, like setting the navigation destination on Maps, or get a list of Locations of Interest (LoI) from something like Atlas Obscura. I plan on also having

## Design requirements

The device has a few core requirements:

- It must operate with no companion phone or PC. It's fine to have a phone for initial setup, or for additional features, but peer-to-peer operation must be entirely on-device.
- Tracking must be done by the device. The GNSS network must be used for this.
- The device must be secure - there must be no way for non-paired devices to be able to discern the location of a device, even if they are part of the same mesh network.
- Communication between devices must be long-range. Kilometer ranges between peers is a requirement, and longer ranges with a mesh network is also necessary.
- The battery must last for at least a day, but 3 day battery life is the target. Deep sleep, low power, and high polling modes should be used to assist this.
- The device must be handheld. No external antenna is ideal, but the option to add one may be worthwhile.
- The device needs an on-board way to communicate the direction and distance of paired devices.
  - The device must have a ring of addressable RGB LEDs for peer indication.
  - The device must have a haptic engine for feedback
  - The device must have a speaker for feedback.
- The device must have flash storage



### Implementation order

TODO: There are a lot of features here, so I want to roadmap from the beginning which features go in first.


## User interactions

I'm gonna split this into two parts - first, from the users' perspective, and then from the technical perspective.

### User perspective

As far as the user is concerned, the device has:
- a name
- a list of devices it is tracking

and a few modes:
- Tracking mode (where the LEDs light up to show the intended direction of travel)
  - LoI
  - Devices/phones
  - Navigation waypoints
- Silent mode (where the device stops reporting its location, but can still track other devices)
- Low-frequency polling mode
- high frequenct polling mode

#### Phone pairing

I turn on the device
I open the bluetooth menu on my phone
I long-press the pairing button on the device
I see the device as "Medallion XYZ123" in the available devices menu and connect to it
I open the app on my phone
The app knows that a new device has been connected and needs pairing
The app prompts me for a device name
I enter it and submit
After a few moments, I see a confirmation page telling me the device is ready to use

#### On-device device pairing

I have a device that is already set up
I long-press the pairing button on device A
I long-press the pairing button on device B
I hold the two devices close to each other
The devices each send a pairing request to each other, and the LEDs indicate a pairing request by flashing the same random colour and pattern
I long press the pairing button to accept, or tap the pairing button to reject
If/when connected to a companion phone, the device updates it with the current tracked device list


#### In-app device pairing
#### Start location broadcasting
#### Stop location broadcasting
#### Start HTR broadcasting
#### Stop HTR broadcasting
#### Set navigation route
#### Track LoI nearby


### Techical perspective

Each device contains the following information about itself:
- The device name (human-readable)
- The device UUID
- A sigining key pair (pub/priv pair)
- An encryption key pair (pub/priv pair)

Each device tracks the following information about other "partner" devices, for each partner:
- The partner name
- The partner UUID
- The partner public signing key
- The partner public encryption key

Devices have three modes of location broadcasting:
- No broadcasting
- Low time resolution location broadcasting (LTR) (default)
- High time resolution location broadcasting (HTR)


#### Phone pairing

This is the initial setup for a user with a new device, or the resetting process for a device that is already set up.

1. The user signals that the device is entering pairing mode
1. The controller clears all previous pairing associations
1. The controller starts the bluetooth radio, if it is not already enabled
1. The controller starts advertising
1. The user pairs their phone to the device
1. The user opens a mobile app, and configures a device name
1. The mobile app sends a message to the device, updating its name
1. The device clears its list of tracked devices, and rotates its cryptographic keys
1. The device sends a message to the app indicating that the setup was successful and informs the app of the device's UUID and public key

#### On-device device pairing

This is the process to take two physical devices that are nearby, and set it up so that one tracks the other (device-to-device pairing)

#### In-app device pairing

This is the process to use the app to search for another device that is not nearby, and request that partner's information from the remote server (app-to-app pairing)

#### Device location broadcasting (LTR)
#### Device location broadcasting (HTR)
#### Device group tracking
#### Device individual tracking


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
