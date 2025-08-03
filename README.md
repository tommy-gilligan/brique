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

## Assuming Rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
select nightly
load into shell

## Setting up web environment
cargo install --locked trunk
cd web
trunk serve

## Setting up rp environment

- Setting up board
    - Install Hardware Test Program
    - Reassemble 3310
    - Manual Test

### TODO

- document app API

- give more control over display flushing, keep track of what needs updating, what needs flushing
	- display can do 90deg rotated addressing
- allow setting time
- rp2350 very low power state
- make all system work live on primary core.  give apps dedicated secondary core.
- 'pre-emption'
- priveledged, background blocking system calls  
- defmt should be used if possible (instead of log).  currently it is used for uart/rtt but for usb i'm using log.  is there a way to use usb logging from defmt?  will probably require a bit of a bit deep dive
- placement of some parts is off (rotation for U*)
- switch off after 5 minutes idle (switching on takes no time)
- power button: can this be triggered by 'any key'?  ie. any keypad press turns the device on.  there's enough GPIO to spare that we should have a dedicated GPIO for any key too)
- power button should turn on device but also function as gpio?

https://serdisplib.sourceforge.net/ser/pcd8544.html





Ordering from JLCPCB
https://github.com/tommy-gilligan/brique/releases/latest
Download:
- gerbers.zip
- cpl.csv
- bom.csv

Upload gerbers.zip
Form with many options
The board needs to be 1mm (this is the thickness of the original board it is replacing)
Select 1mm thickness
Select PCB assembly
Assemble bottom side
PCB assembly significantly affects price but minimum quantity is 2.  If you are cheap and only need 1 board: select quantity 2
Select standard assembly (I think currently 
Click Next
You'll be taken to PCB tab of a review page
Click next
Upload bom
Uplaod cpl
Click Process BOM & CPL
Click continue on missing parts (I need to change these to NC: M1 J4 LS1 U2)
The inclusion of 01005 is limiting me to 'Standard' (part C161362... seems to be something wrong with bom.csv output from kibot): upsize
