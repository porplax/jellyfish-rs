use clap::Parser;
use color::ColorOption;
use neobridge_rust::{Neobridge, RGB};
use screenshots::Screen;
use std::{process::exit, thread, time::Duration};

mod color;
mod render;

const RECOMMENDED_DEPTH_LIMIT: usize = 300;
const VERSION: &str = env!("CARGO_PKG_VERSION");

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
    #[arg(short, long)]
    port: String,

    /// How many colors to calculate for each LED on the strip
    #[arg(short, long, default_value_t = 16)]
    depth: usize,

    /// Refresh rate of the program
    #[arg(short, long, default_value_t = 60)]
    refresh_rate: u64,

    /// Brightness of LED strip
    #[arg(long, default_value_t = 1.0)]
    brightness: f32,

    /// Saturation of LED strip
    #[arg(long, default_value_t = 5.0)]
    saturation: f32,

    /// Remove warnings
    #[arg(long, default_value_t = false)]
    no_warnings: bool,

    /// Disables color operations such as brightness, saturation, ...
    #[arg(long, default_value_t = false)]
    disable_color_operations: bool,
}

struct CalculationOption {
    disable_color_operations: bool,
}

impl CalculationOption {
    fn new(disable_color_operations: bool) -> CalculationOption {
        CalculationOption {
            disable_color_operations,
        }
    }
}

enum Level {
    Info,
    Warning,
    Error,
}

impl Level {
    fn colorize(self) -> String {
        match self {
            Self::Info => String::from(""),
            Self::Warning => String::from("\u{001b}[33m"),
            Self::Error => String::from("\u{001b}[31m"),
        }
    }
}

fn cli_print(level: Level, msg: &str) {
    println!("{}{}\u{0001b}[0m", level.colorize(), msg);
}

fn main() {
    let args = Args::parse();

    cli_print(
        Level::Info,
        &format!(
"jellyfish v{} | swish swish🪼",
            VERSION
        ),
    );

    let monitors = Screen::all().unwrap();

    let width = monitors[args.monitor].display_info.width;
    let height = monitors[args.monitor].display_info.height;

    if args.depth > RECOMMENDED_DEPTH_LIMIT && !(args.no_warnings) {
        cli_print(
            Level::Warning,
            &format!(
                "depth ({}) might be too expensive! consider lowering if CPU usage is too high.",
                args.depth
            ),
        );
    }

    if args.depth == 0 {
        cli_print(
            Level::Error,
            "you cannot have a depth of 0, this leaves nothing for me to process! >:(",
        );
        exit(0)
    }

    if args.brightness > 1.0 || args.brightness < 0.0 {
        cli_print(
            Level::Error,
            &format!(
                "brightness ({}) must be between 0.0 and 1.0!",
                args.brightness
            ),
        );
        exit(0);
    }

    cli_print(
        Level::Info,
        &format!(
            "using monitor: {}; width: {}, height: {}",
            args.monitor, width, height
        ),
    );

    // first, connect to board with neobridge. jelly just calculates what colors are on the monitor.
    // then returns those values to the board.
    let mut neobridge = Neobridge::new(&args.port, args.n_of_leds.try_into().unwrap());
    let mut jelly: render::JellyRenderer = render::JellyRenderer::new(
        width,
        height,
        args.n_of_leds,
        args.depth,
        ColorOption::new(args.brightness, args.saturation),
        CalculationOption::new(args.disable_color_operations),
    );

    cli_print(
        Level::Info,
        &format!(
            "connected to {} that has a number of {} LEDs.",
            args.port, args.n_of_leds
        ),
    );

    // reset LEDs to black.
    neobridge.set_all(RGB(0, 0, 0));
    neobridge.show();

    cli_print(
        Level::Info,
        &format!("sent reset commands to {}, assuming board works", args.port),
    );

    // start loop here.
    let screen = monitors[args.monitor];

    cli_print(Level::Info, "started capture!");
    loop {
        // don't put image into a separate var, this prevents errors.
        if let Ok(image) = screen.capture_area(
            0,
            height as i32 - ((args.depth + 1) as i32),
            width,
            args.depth as u32,
        ) {
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
