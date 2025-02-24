# RP235x Nokia 3310 Adapter Board

This started as a cute "what if?":

> What if I could type out messages on my smart phone by using tactile buttons like I used to have on my phone back in high school?

Because the board would need to fit into the chassis of a Nokia 3310 and because there are existing doodads that would be sitting alongside the board anyways, scope has crept to include connecting to these other thingamajigs.

![3D render of front of adapter board](./front.jpg)
![3D render of back of adapter board](./back.jpg)

## Setting up web environment

## Setting up rp environment

## Setting up board
### Ordering Board
### Disassemble 3310
### Install Hardware Test Program
### Reassemble 3310
### Manual Test
### Install Custom Software

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
- USB text entry
- use text_input for inputing secret for TOTP (drives the need for inputting numeric digits easily and RTC)

#### Later
- optional pico-w for wifi/bluetooth (using a module avoids need for recertification?)
    - looks like RP will release such a module (RM2) so go ahead with designing with that in mind
- detect battery type to refuse NiMH
- battery gauge
- flex pcb for keypad?
- LTE modem?
- e-ink display?
- power button used for BOOT/RUN?

https://serdisplib.sourceforge.net/ser/pcd8544.html
