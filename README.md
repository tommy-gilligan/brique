<p align="center">
  <a href="https://github.com/tommy-gilligan/brique">
    <img src="https://raw.githubusercontent.com/tommy-gilligan/brique/refs/heads/main/logo.svg" alt="Logo" width="200"/>
  </a>
  <h3 align="center">Brique</h3>
  <p align="center">
    An RP2350-based replacement mainboard for Nokia 3310.<br />
    Rust firmware with browser-based simulator included. 
    <br />
    <br />
    Software: <a href="https://tommy-gilligan.github.io/brique/simulation">Simulation</a>
    ·
    <a href="https://tommy-gilligan.github.io/brique/doc/shared">API</a>
    <br />
    Hardware: <a href="https://github.com/tommy-gilligan/brique/releases/latest/download/gerbers.zip">Gerbers</a>
    ·
    <a href="https://github.com/tommy-gilligan/brique/releases/latest/download/bom.csv">BOM</a>
    ·
    <a href="https://github.com/tommy-gilligan/brique/releases/latest/download/cpl.csv">CPL</a>
    ·
    <a href="https://github.com/tommy-gilligan/brique/releases/latest/download/schematic.pdf">Schematic</a>
    ·
    <a href="https://github.com/tommy-gilligan/brique/releases/latest/download/pcb.svg">PCB Layout</a>
  </p>
</p>
<hr/>
This started as a cute "what if?":

> What if I could type out messages on my smart phone by using tactile buttons like I used to have on my phone back in high school?

Because the board would need to fit into the chassis of a Nokia 3310 and because there are existing doodads that would be sitting alongside the board anyways, scope has crept to include connecting to these other thingamajigs.

![3D render of front of main board](https://github.com/tommy-gilligan/brique/releases/latest/download/3D_blenderfront.png)
![3D render of back of main board](https://github.com/tommy-gilligan/brique/releases/latest/download/3D_blenderback.png)

- Setting up web environment
- Setting up rp environment
- Setting up board
    - Ordering Board
    - Disassemble 3310
    - Install Hardware Test Program
    - Reassemble 3310
    - Manual Test
    - Install Custom Software

### TODO
#### Sooner

- keyboard is kinda broken (multitap)
- terse, unfriendly instructions (ie. README)
- create example that plays RTTTL (and writes it to screen)
- Snake
- give more control over display flushing, keep track of what needs updating, what needs flushing
	- display can do 90deg rotated addressing
- exclude files from zip that do not need to be there
- scrolling menu
- using ringtone menu async with playback
- split out ringtone name for menu
- allow setting time
- grid menu from keyboard
- allow stopping ringtone while it's playing 
- copious debuggin statements
- menu should label select button
- options label for select button in keyboard
- document app API

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
- how should software versions synchronize with hardware versions. what level of compatibility should be supported.
- institute changelog
- use 'Issues' instead of README for tracking
- connection plate for 3d printing
- increase flash capacity? i'd prefer to remove this part altogether by using rp2354 due for release later in the year
- 'pre-emption'
- priveledged, background blocking system calls  
- persistenece/flash
- allow setting alarms
- charging IC should communicate state with rp2350 (charging, full, should be able to just sense 'LED' outputs)
- power button should turn on device but also function as gpio?
- defmt should be used if possible (instead of log).  currently it is used for uart/rtt but for usb i'm using log.  is there a way to use usb logging from defmt?  will probably require a bit of a bit deep dive
- 3d models
- placement of some parts is off (rotation for U*)

#### Much Later
- LTE modem?
- e-ink display?
- power button used for BOOT/RUN?

https://serdisplib.sourceforge.net/ser/pcd8544.html
