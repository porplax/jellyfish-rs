use clap::Parser;
use neobridge_rust::{Neobridge, RGB};
use std::{thread, time::Duration};
use screenshots::Screen;

mod render;

#[derive(Parser, Debug)]
#[command(
    author = "cutesunshine",
    version,
    about = "Ambient lighting on neopixel devices."
)]
struct Args {               
    /// Monitor to screen record
    #[arg(short, long, default_value_t = 0)]
    monitor: usize,

    /// Number of LEDs present on the strip
    #[arg(short, long)]
    n_of_leds: usize,

    /// Port of the board
    #[arg(short, long, default_value_t = String::from("COM3"))]
    port: String,

    /// How many colors to calculate for each LED on the strip
    #[arg(short, long, default_value_t = 16)]
    depth: usize,

    /// Refresh rate of the program
    #[arg(short, long, default_value_t = 60)]
    refresh_rate: u64,
}

fn main() {
    let args = Args::parse();

    let monitors = Screen::all().unwrap();

    let width = monitors[args.monitor].display_info.width;
    let height = monitors[args.monitor].display_info.height;

    // first, connect to board with neobridge. jelly just calculates what colors are on the monitor.
    // then returns those values to the board.
    let mut neobridge = Neobridge::new(&args.port, args.n_of_leds.try_into().unwrap());
    let mut jelly: render::JellyRenderer =
        render::JellyRenderer::new(args.depth, width, height, args.n_of_leds);

    // reset LEDs to black.
    neobridge.set_all(RGB(0, 0, 0));
    neobridge.show();

    // start loop here.
    let screen = monitors[args.monitor];

    loop {
        // don't put image into a separate var, this prevents errors.
        if let Ok(image) = screen.capture_area(0, height as i32-((args.depth+1) as i32), width, args.depth as u32) {

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