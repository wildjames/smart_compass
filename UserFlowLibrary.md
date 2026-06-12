# User Flows

This is a project where the product will have a fair few things to do with it. Enough that, in order to help me be confident I've thought it through, it's worth making a library of what I want to have in the end.

I'm gonna split this into two parts - first, from the users' perspective, and then from the technical perspective.

For IO, the compass will have 3 user buttons, which will generally fill the role of up, down, and select. There is the e-ink screen for low-cadence information, and the LED ring for high-cadence information. There is also a haptic engine for vibrations, which I won't define here and instead just go by feel later on.

## User perspective

As far as the user is concerned, the device has:
- a name
- a list of devices it is tracking

and a few modes:
- Tracking mode (where the LEDs light up to show the intended direction of travel)
  - Devices/phones
  - Navigation waypoints
  - LoI
- Silent mode (where the device stops reporting its location, but can still track other devices)
- Low-frequency polling mode
- high frequency polling mode

### Phone pairing

- I turn on the device
- I open the bluetooth menu on my phone
- I navigate to the "Set up Compass" settings page in the device screen using the user buttons
- I see the device as "SmartCompass XYZ123" in the available devices menu and connect to it
- I open the app on my phone
- The app knows that a new device has been connected and needs pairing
- The app prompts me for a device name
- I enter it and submit
- After a few moments, I see a confirmation page telling me the device is ready to use

### On-device device pairing

- I have a device that is already set up
- I navigate to the "Add friend" page on the device screen on device A
- The other party navigates to the "Add friend" page on the device screen on device B
- I hold the two devices close to each other
- The devices each send a pairing request to each other, and the LEDs indicate a pairing request by flashing the same random colour and pattern
- I press the "select" button to accept, or the "up/down" buttons to reject
  - If either party rejects, then neither device saves the other
  - If the parties take longer than 60s to accept/reject, consider this a rejection
  - If the devices move out of range of each other within the timeout, consider this a rejection
- If/when connected to a companion phone, the device updates it with the current tracked device list


### In-app device pairing

- I have paired a phone to the device
- I open the app and navigate to the "add friend" page
- I search for that friend's username or email
- The other party recieves a request to track
- The other party accepts the request
- I can now track the other party

### Stop location broadcasting (silent mode)

- I have a device set up and paired with a phone
- I turn on the power switch for the device
- The device is broadcasting its location
- I simultaneously press the "up" and "down" user buttons together for 1s
- The device screen indicates that broadcasting has stopped
- The device LED ring indicates that broadcasting has stopped

### Start location broadcasting

- I have a device set up and paired with a phone
- I turn on the power switch for the device
- Location broadcasting is on by default, but the user has turned on silent mode
- The device is not currently broadcasting its location
- I simultaneously press the up and "down" user buttons together for 1s
- The device screen indicates that broadcasting has begun
- The device LED ring indicates that broadcasting has begun

### Start HTR broadcasting

- I have a device set up and paired with a phone
- The device is currently broadcasting a location
- I use the 3 user buttons to navigate the on-screen menu to the location broadcasting page
- I select the "Start HTR sharing" option

### Stop HTR broadcasting

- I have a device set up and paired with a phone
- The device is currently broadcasting a location
- I use the 3 user buttons to navigate the on-screen menu to the location broadcasting page
- I select the "Stop HTR sharing" option

### Set navigation route

- I have a device set up and paired with a phone
- The device is currently connected to the phone
- I open the app and navigate to the "Navigation" page
- I type in a destination to the search box
- The app returns results from Google Maps
- I select a target location
- The app indicates that navigation has started
- The app shows the route
- The device screen indicates that navigation has started, and displays key information about the route
- The device LED ring indicates that navigation has started, then points to the first waypoint along the route
- The screen updates key information on a 60s cadence
- The user is able to cancel navigation by either powering off the device (no resume on reboot), or double-tapping the "select" button
- When the user arrives at the waypoint, the LED ring indicates a waypoint was reached and begins tracking the next waypoint
- When the user arrives at the destination, the LED ring indicates arrival and the screen updates to do the same.

### Track LoI nearby

- I have a device set up and paired with a phone
- The device is currently connected to the phone, which is used to fetch data
- I use the 3 user buttons to navigate to "Track nearby LoI"
- The LED ring indicates the direction of the 3 nearest LoI
- The screen shows the name of the LoI that is currently closest to 0 degrees, and the distance to it
- When the user arrives at an LoI, the device LED ring indicates arrival
- The screen shows the LoI name
- When the user arrives at an LoI, the phone app sends a notification and opens the wikipedia page for it

### Start tracking a specific person

- I have a device set up
- I use the 3 user buttons to navigate to "Track person"
- I see a list of paired people, with untrackable people grayed out
  - "Untrackable" means no location data currently available for that person
  - I may add a "Patchy signal" mode, where the last known location can still be tracked?
- I use the user buttons to navigate to the person to track, and select them
- The LED ring indicates the direction of the person being tracked
- The device screen shows the name of the person being tracked

### Define a tracking group

- I have a device set up and paired with a phone
- The device is currently connected to the phone
- I open the app and navigate to the "Groups" page
- I select "Create new group"
- I enter a group name
- I select members from my list of paired devices/friends
- I confirm the group
- The app shows the new group in my groups list
- The device receives the updated group information

### Start tracking a group of people

- I have a device set up with at least one group defined
- I use the 3 user buttons to navigate to "Track group"
- I use the up/down buttons to scroll through available groups
- I press "select" to choose a group
- The LED ring indicates the direction of each group member with a different colour
- The device screen shows the group name and a list of members with distances

### Stop tracking

- I am currently tracking a person or group
- I long-press the "select" user button
- The LED ring stops indicating directions
- The device screen confirms that tracking has stopped and returns to the main menu

### Power on

TODO

### Power off

TODO

### Low battery indication

TODO


### Firmware update

TODO - Is OTA updating feasible? Perhaps that's the first feature to implement. Can I do it from the app? Directly from a PC?

### Remove a friend

TODO

### Edit a tracking group

TODO

### Delete a tracking group

#### On-device

- I have set up a tracking group previously
- I navigate to the device settings > "Delete tracked thing" menu on the device screen using the 3 user buttons
- I see a menu of "Groups", "Friends", "Locations"
- I select "Groups", and see a list of my tracking group names
- I select the group to delete, and see a "Are you sure?" screen, which contains the group name
- I select "yes" or "no".
- After selection, I return to the list of groups

#### In-app

- I have set up a tracking group previously
- I navigate to the "Manage Tracking Groups" page in the app
- I select the group to delete
- I select the "Delete" button
- I am presented with a confirmation page, and choose an option
- I am returned either to the selected group management page if I do not delete it, or the list of groups page if I did delete the group.

### Tracked person goes offline

TODO

### Tracked person enters silent mode

TODO - should this look the same as the person going offline, or should I communicate the silent mode to the tracker?


## Technical perspective

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


### Phone pairing

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

### On-device device pairing

This is the process to take two physical devices that are nearby, and set it up so that one tracks the other (device-to-device pairing)

### In-app device pairing

This is the process to use the app to search for another device that is not nearby, and request that partner's information from the remote server (app-to-app pairing)

### Device location broadcasting (LTR)
### Device location broadcasting (HTR)
### Device group tracking
### Device individual tracking

