<p align="center">
  <a href="" rel="noopener">
 <img width=200px height=200px src="https://u.cubeupload.com/ihavecandy/c77jellyfish.png" alt="oh look, a jellyfish"></a>
</p>

<h3 align="center">jellyfish.</h3>
<p align="center"> ambient lighting on your desktop using neopixels!
    <br>
</p>
<div align="center">

![Status](https://img.shields.io/badge/status-active-success.svg)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/porplax/jellyfish-rs?style=for-the-badge)
![GitHub Issues or Pull Requests](https://img.shields.io/github/issues/porplax/jellyfish-rs)

---

![IMG_1036-ezgif com-speed](https://github.com/porplax/jellyfish-rs/assets/66521670/bfba4df8-9ba1-43da-8313-4a69f154d7dc)
</div>

# 💫about this project💫

[NeoPixels](https://www.adafruit.com/category/168) is a great way to implement lighting effects of your own. It is easily programmable and cheap to do. So using a strip and a Pi Pico W I had lying around, I made this project.

This project was just a experiment of mines to see if it is possible to add reactive lighting to my monitor and do so without the cost of performance. Feel free to look at the source code to see how I did it.

If you want, you can download it. I've made a lot of optimizations and features into Jellyfish. **You can even tune the brightness, or saturation of the colors!**

*Ever wanted to add extra lighting to your setup?* This project is a great way to do so. It uses serial communication to send lighting data from the PC to circuitpython, and rust to process it via the PC.

# 🧰made with🧰

- ![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
- ![Raspberry Pi](https://img.shields.io/badge/-RaspberryPi-C51A4A?style=for-the-badge&logo=Raspberry-Pi)
- ![Python](https://img.shields.io/badge/python-3670A0?style=for-the-badge&logo=python&logoColor=ffdd54)

## crates used

- [nalgebra](https://github.com/dimforge/nalgebra) | v1.2.0 and under
- [clap](https://github.com/clap-rs/clap) | v1.5.0 and under
- [egui](https://github.com/emilk/egui) | v2.0.0^
- [screenshots](https://github.com/nashaofu/xcap)
- [image](https://github.com/image-rs/image) | v2.0.0^
- [serde](https://github.com/serde-rs/serde) | v2.0.0^
- [serde_json](https://github.com/serde-rs/json) | v2.0.0^
- [neobridge-rust](https://github.com/porplax/neobridge)
- [serialport](https://github.com/serialport/serialport-rs)

# 🛠️building🛠️

**Before building you must have `git` and `rust` installed on your system!**

```bash
git clone https://github.com/porplax/jellyfish-rs
cd jellyfish-rs
cargo build
```

*PS: This was bulit on windows. I am unsure if other operating systems are supported.*

# 🖋️getting started🖋️

Jellyfish is a DIY project. All you need is a circuitpython board, few wires, and a neopixel LED strip (*i don't know if other LEDs can work, this is just from experience*).

## getting things right

If you really wanna get things right, it is essential to have the right size LED for your desk setup. Here was my setup.

- [Adafruit NeoPixel Digital RGB LED Strip - Black 30 LED 1m - BLACK](https://www.adafruit.com/product/2552)
- [Pi Pico W](https://www.adafruit.com/product/5526)

**You can use other boards, but they must be able to run on circuitpython.** Any other firmware that has the NeoPixels library on it can be used, but Neobridge doesn't support that :(

## prerequisites

Before you can run the jellyfish program, you'll need a circuitpython board to run **Neobridge**.

**If you have a RPI Pico/RPI Pico W board,** [you can use the automated installer on windows](https://github.com/porplax/neobridge/raw/master/neobridge-install.py).

**If you have another board or want to do manual installation, here are the instructions:**

- Download a [Circuitpython 8.x/9.x .UF2 file](https://circuitpython.org/downloads), the library bundle and [code.py](https://github.com/porplax/neobridge/raw/master/src/neobridge/code.py).
- Flash the Circuitpython 8.x/9.x .UF2 file onto your board.
- Move `neopixel.mpy` from the bundle to `\lib`.
- Modify `code.py` by changing the pinout, number of pixels, and order.
- Make sure it is running and will run each bootup.

# installing the project

![gui](https://github.com/porplax/jellyfish-rs/assets/66521670/92b3cb30-0e21-44ef-91d9-ed14438e36f3)

- Download `jellyfish-v2.0.0.zip` into a suitable location.
- Extract
- Simply run the `.exe` file, and you can start using Jellyfish!

**Smartscreen will mistake it for malware. if it does, click *run anyway*.**

## Supported platforms

- Windows 10 and up

# 📃to-do list📃

- [X] Automated installation script for circuitpython.
- [ ] Contribution section.
- [ ] Detailed step-by-step tutorial with pictures.
- [ ] Switch to an event-loop driven UI library.
- [ ] 'Run in background' option, I've tried multiple times but it is pretty hard to do atm. 
- [ ] Switch to `async`, instead of using `threads`

## known issues and bugs

There are no known bugs or issues! If you find one, please make an issue.

## those involved

- [@porplax](https://github.com/porplax), @Localis9 - Creator of the project. (@zaynes_starr on discord), (Localis is just my old acc)

See also the list of [contributors](https://github.com/porplax/jellyfish-rs/contributors) who participated in this project.
