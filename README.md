# Brique: RP235x Nokia 3310 Adapter Board

This started as a cute "what if?":

> What if I could type out messages on my smart phone by using tactile buttons like I used to have on my phone back in high school?

Because the board would need to fit into the chassis of a Nokia 3310 and because there are existing doodads that would be sitting alongside the board anyways, scope has crept to include connecting to these other thingamajigs.

![3D render of front of adapter board](./front.jpg)
![3D render of back of adapter board](./back.jpg)
## [Simulation](https://tommy-gilligan.github.io/brique/simulation)
## [API](https://tommy-gilligan.github.io/brique/doc/shared)

## Setting up web environment

## Setting up rp environment

## Setting up board
### Ordering Board
### Disassemble 3310
### Install Hardware Test Program
### Reassemble 3310
### Manual Test

### TODO
#### v0.3
focus on single app binaries for now?
don't worry about main menu and preemption.  put everything on device &mut instead of transfering control all around the place.  eventually i want a nicer system but it's an unnecessary complexity right now.  very hard to know which way of doing syscalls is the right way.  a way that will continue to work in the future?  i'm very bad at planning apis.  i design them by making them and seeing how they feel.  should probably try to change this.

### Install Custom Software

[BOM](https://github.com/tommy-gilligan/brique/releases/latest/download/bom.csv)
[CPL](https://github.com/tommy-gilligan/brique/releases/latest/download/cpl.csv)
[Gerbers](https://github.com/tommy-gilligan/brique/releases/latest/download/gerbers.zip)

- placement of some parts is off (rotation for U*)
- make output file names more generic (for reuse on other projects)
- exclude files from zip that do not need to be there
- don't zip BOM/CPL
- make file names match where they upload
- make CI outputs -> release outputs

### TODO
#### Sooner
##### Hardware
- dress up repo
- Come up with better name
- add back supercap rtc
- double check power regulation
- add jlcpcb part numbers, 3d models

##### Software
- create example that plays RTTTL (and writes it to screen)
- Snake
- give more control over display flushing, keep track of what needs updating, what needs flushing
	- display can do 90deg rotated addressing

- terse, unfriendly instructions (ie. README)
- document app API

defects/small improvements:
scrolling menu
using ringtone menu async with playback
split out ringtone name for menu

persistenece/flash
allow setting time
allow setting alarms
keyboard is kinda broken (multitap)
grid menu from keyboard

defmt should be used if possible (instead of log).  currently it is used for uart/rtt but for usb i'm using log.  is there a way to use usb logging from defmt?  will probably require a bit of a bit deep dive

allow stopping ringtone while it's playing 
copious debuggin statements
menu should label select button
options label for select button in keyboard

power button should turn on device but also function as gpio?
- optional pico-w for wifi/bluetooth (using a module avoids need for recertification?)
    - looks like RP will release such a module (RM2) so go ahead with designing with that in mind
- charging IC should communicate state with rp2350 (charging, full, should be able to just sense 'LED' outputs)

#### Later
- watchdog should involve both cores
- switch off after 5 minutes idle (switching on takes no time)
- rp2350 very low power state
- UI component model?
- power button: can this be triggered by 'any key'?  ie. any keypad press turns the device on.  there's enough GPIO to spare that we should have a dedicated GPIO for any key too)
- make all system work live on primary core.  give apps dedicated secondary core.
- detect battery type to refuse NiMH
- battery gauge
- mic connection
- use text_input for inputing secret for TOTP (drives the need for inputting numeric digits easily and RTC)
- check against minimal design

- how should software versions synchronize with hardware versions. what level of compatibility should be supported.
- institute changelog
- use 'Issues' instead of README for tracking
- optimise GPIO pin mapping.  shorten traces etc.
- connection plate for 3d printing
- increase flash capacity? i'd prefer to remove this part altogether by using rp2354 due for release later in the year
- bring more rigour to 'scheduler'

#### Much Later
- flex pcb for keypad?
- LTE modem?
- e-ink display?
- power button used for BOOT/RUN?

https://serdisplib.sourceforge.net/ser/pcd8544.html
