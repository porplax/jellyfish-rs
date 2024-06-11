use clap::Parser;
use neobridge_rust::{Neobridge, RGB};
use std::{thread, time::Duration};
use xcap::Monitor;

mod render;

#[derive(Parser, Debug)]
#[command(
    author = "cutesunshine",
    version,
    about = "Ambient lighting on neopixel devices."
)]
struct Args {
    #[arg(short, long)]
    width: u32,
    height: u32,
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

    let mut neobridge = Neobridge::new(&args.port, args.n_of_leds.try_into().unwrap());
    let mut jelly: render::JellyRenderer =
        render::JellyRenderer::new(args.depth, args.width, args.height, args.n_of_leds);

    neobridge.set_all(RGB(0, 0, 0));
    neobridge.show();

    loop {
        if let Ok(image) = monitors.get(0).unwrap().capture_image() {
            let colors = jelly.grab(&image);
            neobridge.set_list(colors);
            neobridge.show();
        }
        thread::sleep(Duration::from_millis(1000 / args.refresh_rate));
    }
}
