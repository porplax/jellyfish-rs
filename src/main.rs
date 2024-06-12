use clap::Parser;
use neobridge_rust::{Neobridge, RGB};
use std::{thread, time::Duration};
use xcap::Monitor;

mod render;
mod buffer;

#[derive(Parser, Debug)]
#[command(
    author = "cutesunshine",
    version,
    about = "Ambient lighting on neopixel devices."
)]
struct Args {
    #[arg(short, long)]
    monitor: usize,
    n_of_leds: usize,

    #[arg(short, long, default_value_t = String::from("COM3"))]
    port: String,

    #[arg(short, long, default_value_t = 132)]
    depth: usize,

    #[arg(short, long, default_value_t = 60)]
    refresh_rate: u64,
}

fn main() {
    let args = Args::parse();

    let monitors = Monitor::all().unwrap();

    let width = monitors.get(args.monitor).unwrap().width();
    let height = monitors.get(args.monitor).unwrap().height();

    // first, connect to board with neobridge. jelly just calculates what colors are on the monitor.
    // then returns those values to the board.
    let mut neobridge = Neobridge::new(&args.port, args.n_of_leds.try_into().unwrap());
    let mut jelly: render::JellyRenderer =
        render::JellyRenderer::new(args.depth, width, height, args.n_of_leds);

    // reset LEDs to black.
    neobridge.set_all(RGB(0, 0, 0));
    neobridge.show();

    // start loop here.
    loop {
        // don't put image into a separate var, this prevents errors.
        if let Ok(image) = monitors.get(args.monitor).unwrap().capture_image() {
            // first get the colors.
            let colors = jelly.grab(&image);

            // then send it to the board.
            neobridge.set_list(colors);
            neobridge.show();
        }

        // just so it doesn't put a lot of load onto the CPU. 
        thread::sleep(Duration::from_millis(1000 / args.refresh_rate));
    }
}