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

Wanted to***⭐spice up your setup?⭐*** Use jellyfish to add ambient lighting to your set-up! It uses serial communication and NeoPixels to achieve this effect. 

The PC does the color calculation at a *very low CPU cost*, and the NeoPixel only has to match itself to your computer monitor. 

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
