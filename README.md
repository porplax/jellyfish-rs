<p align="center">
  <a href="" rel="noopener">
 <img width=200px height=200px src="https://u.cubeupload.com/ihavecandy/c77jellyfish.png" alt="oh look, a jellyfish"></a>
</p>

<h3 align="center">jellyfish.</h3>
<p align="center"> ambient lighting on your desktop using neopixels!
    <br> 
</p>
<div align="center">


[![Status](https://img.shields.io/badge/status-active-success.svg)]()
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/porplax/jellyfish-rs?style=for-the-badge)
![GitHub Issues or Pull Requests](https://img.shields.io/github/issues/porplax/jellyfish-rs)

![IMG_1036-ezgif com-speed](https://github.com/porplax/jellyfish-rs/assets/66521670/bfba4df8-9ba1-43da-8313-4a69f154d7dc)
</div>

---



## 💫about this project💫 <a name = "about"></a>

Wanted to***⭐spice up your setup?⭐*** Jellyfish reacts to what's on your monitor and controls Neopixels to copy what's on it. In other words, it's ambient lighting. 

It does this via **Neobridge**. It lets your PC to communicate to a circuitpython board. Sends RGB data and tells the board to control the Neopixels connected to the board.

## 🖋️getting started🖋️ <a name = "getting started"></a>

### DIY supplies
Jellyfish is a DIY project. All you need is a circuitpython board, few wires, and a neopixel LED strip (*i don't know if other LEDs can work, this is just from experience*).
#### getting things right
If you really wanna get things right, it is essential to have the right size LED for your desk setup. Here was my setup.
- <a href="https://www.adafruit.com/product/2552"> Adafruit NeoPixel Digital RGB LED Strip - Black 30 LED 1m - BLACK </a>
- <a href="https://www.adafruit.com/product/5526">Pi Pico W</a>

**Other boards should work with serial, but they must be circuitpython.**
### prerequisites

Before you can run the jellyfish program, you'll need a circuitpython board to run **Neobridge**. 

**If you have a RPI Pico/RPI Pico W board,** [you can use the automated installer on windows](https://github.com/porplax/neobridge/raw/master/neobridge-install.py).

**If you have another board or want to do manual installation, here are the instructions:**
- Download a [Circuitpython 8.x/9.x .UF2 file](https://circuitpython.org/downloads), the library bundle and [code.py](https://github.com/porplax/neobridge/raw/master/src/neobridge/code.py).
- Flash the Circuitpython 8.x/9.x .UF2 file onto your board.
- Move `neopixel.mpy` from the bundle to `\lib`.
- Modify `code.py` by changing the pinout, number of pixels, and order.
- Make sure it is running and will run each bootup.
### installing the project

#### Supported platforms
- Windows 10 and up

Download `jellyfish-rs.exe` into a suitable location.

## 🚥usage🚥 <a name="usage"></a>
```
./jellyfish-rs.exe [OPTIONS] --monitor <MONITOR> <N_OF_LEDS>
```
**arguments:**
- `-m, --monitor <MONITOR>` - Which monitor to capture from
- `<N_OF_LEDS>` - Number of LEDS on the strip
  
**options:**
- `-d, --depth <DEPTH> [default: 132]` - How many colors to capture (*less is more accurate, higher is more ambient*)
- `-r, --refresh-rate <REFRESH_RATE> [default: 60]` - FPS of capture
- `-p, --port <PORT> [default: COM3]` - Port of board
  
**example:** (*30 LEDs, depth of 16, FPS set to 165*)
```
./jellyfish-rs.exe --monitor 0 30 --depth 16 --refresh-rate 165
```
### ⏳optimizing⌛
If you have a decent PC build, jellyfish should not take up an abundance of resources. However if you're suffering from heavy CPU load, here are some tips:
- **Turn down FPS** (*usually this doesn't matter but can help on low-end systems*) 
- **Lower depth** (*creates less computation for the CPU*)
- **Force jellyfish on low priority or ECO** 

## 📃to-do list📃
- [X] Automated installation script for circuitpython.
- [ ] Contribution section.
- [ ] Detailed step-by-step tutorial with pictures.

## those involved

- [@porplax](https://github.com/porplax) - Creator of the project.

See also the list of [contributors](https://github.com/porplax/jellyfish-rs/contributors) who participated in this project.

## crates used

- `screenshots` [@nashaofu](https://github.com/nashaofu) | [Screen capture](https://crates.io/crates/screenshots)
- `clap` [@clap-rs](https://github.com/clap-rs), [@Rust CLI WG](https://github.com/rust-cli), [@kbknapp](https://github.com/kbknapp) | [Command Line](https://crates.io/crates/clap)
- and of course rust🦀🦀🦀!!!
