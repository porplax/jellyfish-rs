<p align="center">
  <a href="" rel="noopener">
 <img width=200px height=200px src="https://u.cubeupload.com/ihavecandy/c77jellyfish.png" alt="oh look, a jellyfish"></a>
</p>

<h3 align="center">jellyfish.</h3>

<div align="center">

[![Status](https://img.shields.io/badge/status-active-success.svg)]()
![GitHub Issues or Pull Requests](https://img.shields.io/github/issues/porplax/jellyfish-rs)

![IMG_1036-ezgif com-speed](https://github.com/porplax/jellyfish-rs/assets/66521670/bfba4df8-9ba1-43da-8313-4a69f154d7dc)
</div>

---

<p align="center"> ambient lighting on your desktop using neopixels!
    <br> 
</p>

## 🪼table of contents🪼

- [🪼table of contents🪼](#table-of-contents)
- [💫about this project💫 ](#about-this-project-)
- [🖋️getting started🖋️ ](#️getting-started️-)
  - [prerequisites](#prerequisites)
  - [installing the project](#installing-the-project)
    - [Supported platforms](#supported-platforms)
- [🚥usage🚥 ](#usage-)
  - [⏳optimizing⌛](#optimizing)
- [📃to-do list📃 ](#to-do-list-)
  - [detailed optimization list](#detailed-optimization-list)
- [those involved ](#those-involved-)
- [crates used ](#crates-used-)

## 💫about this project💫 <a name = "about"></a>

Wanted to***⭐spice up your setup?⭐*** Jellyfish reacts to what's on your monitor and controls Neopixels to copy what's on it. In other words, it's ambient lighting. 

It does this via **Neobridge**. It lets your PC to communicate to a RP2040 board _(other circuitpython boards should work but it's not tested)_. Sends RGB data and tells the board to control the Neopixels connected to the board.

## 🖋️getting started🖋️ <a name = "getting started"></a>

### prerequisites

Before you can run the jellyfish program, you'll need a circuitpython board to run **Neobridge**. 
- Flash circuitpython 8.x onto a compatible board (**Download the library bundle as well**)
- [Load the Neobridge script](https://github.com/porplax/neobridge)

After following the instructions, you will need to hook up a Neopixels LED strip, and to make sure that it works, you will need to modify the Neobridge script if the pinout is different from what it codes for.

Jellyfish only captures what is at the bottom of your monitor at the moment, so you should put the LED strip at the back of your desk.
```py
neo = neopixel.NeoPixel(
    PIXEL_PIN, NUMBER_OF_PIXELS, brightness=1, auto_write=False, pixel_order=ORDER)
```
*MAKE SURE CODE.PY IS CORRECT!*
### installing the project

#### Supported platforms
- Windows 10 and up

Download `jellyfish-rs.exe` into a suitable location.

## 🚥usage🚥 <a name="usage"></a>
```
./jellyfish-rs.exe [OPTIONS] --width <WIDTH> <HEIGHT> <N_OF_LEDS>
```
**arguments:**
- `-w, --width <WIDTH>` - Width of capture (*usually monitor's width*)
- `<HEIGHT>` - Height of capture (*usually monitor's height*)
- `<N_OF_LEDS>` - Number of LEDS on the strip
  
**options:**
- `-d, --depth <DEPTH> [default: 132]` - How many colors to capture (*less is more accurate, higher is more ambient*)
- `-r, --refresh-rate <REFRESH_RATE> [default: 60]` - FPS of capture

**example:** (*30 LEDs, depth of 16, FPS set to 165*)
```
./jellyfish-rs.exe --width 1920 1080 30 --depth 16 --refresh-rate 165
```
### ⏳optimizing⌛
Jellyfish is a early release and has ways to go in terms of optimization. Here's what I found when testing on my system.
```
[AMD Ryzen 5 7600 6-Core] Depth=16, FPS=165, AVG ~3% of CPU Usage 
```
If you have a decent PC build, jellyfish should not take up an abundance of resources. However if you're suffering from heavy CPU load, here are some tips:
- **Turn down FPS** (*usually this doesn't matter but can help on low-end systems*) 
- **Lower depth** (*creates less computation for the CPU*)
- **Force jellyfish on low priority or ECO** 
- **Set width and height to your monitor's spec** (*any higher is useless*)

## 📃to-do list📃 <a name = "to-do"></a>
- [ ] Create a automated installation script for circuitpython.
- [ ] Create a contribution section.

### detailed optimization list
- [ ] Replace `Vec<>, String` with static-sized variables.

## those involved <a name = "those involved"></a>

- [@porplax](https://github.com/porplax) - Creator of the project.

See also the list of [contributors](https://github.com/porplax/jellyfish-rs/contributors) who participated in this project.

## crates used <a name = "acknowledgement"></a>

- `xcap` [@nashaofu](https://github.com/nashaofu) | [Screen capture](https://crates.io/crates/xcap)
- `image` [@image-rs](https://github.com/image-rs), [@theotherphil](https://github.com/theotherphil), [@fintelia](https://github.com/fintelia), [@HeroicKatora](https://github.com/HeroicKatora) | [Image processing](https://crates.io/crates/image)
- `nalgebra` [@sebcrozet](https://github.com/sebcrozet), [@milibopp](https://github.com/milibopp) | [Math stuff](https://crates.io/crates/nalgebra)
- `clap` [@clap-rs](https://github.com/clap-rs), [@Rust CLI WG](https://github.com/rust-cli), [@kbknapp](https://github.com/kbknapp) | [Command Line](https://crates.io/crates/clap)
- and of course rust🦀🦀🦀!!!
