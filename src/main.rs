use clap::Parser;
use neobridge_rust::{Neobridge, RGB};
use screenshots::Screen;
use std::{fmt::format, process::exit, thread, time::Duration};

mod color;
mod engine;
mod term;

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
    #[arg(long, default_value_t = 0.0)]
    saturation: f32,

    /// Remove warnings
    #[arg(long, default_value_t = false)]
    no_warnings: bool,

    /// Enables slient mode
    #[arg(long, default_value_t = false)]
    slient_mode: bool,

    /// Disables color operations such as brightness, saturation, ...
    #[arg(long, default_value_t = false)]
    disable_color_operations: bool,

    /// Run process in background
    #[arg(long, default_value_t = false)]
    run_in_background: bool,
}

fn main() {
    let args: Args = Args::parse();
    let term = term::Terminal::new(args.slient_mode);

    term.cli_print(
        term::Level::Info,
        &format!(
"jellyfish v{} | swish swish🪼",
            VERSION
        ),
    );

    let monitors: Vec<Screen> = Screen::all().unwrap();

    let width: u32 = monitors[args.monitor].display_info.width;
    let height: u32 = monitors[args.monitor].display_info.height;

    if args.depth > RECOMMENDED_DEPTH_LIMIT && !(args.no_warnings) {
        term.cli_print(
            term::Level::Warning,
            &format!(
                "depth ({}) might be too expensive! consider lowering if CPU usage is too high.",
                args.depth
            ),
        );
    }

    if args.depth == 0 {
        term.cli_print(
            term::Level::Error,
            "you cannot have a depth of 0, this leaves nothing for me to process! >:(",
        );
        exit(0)
    }

    if args.brightness > 1.0 || args.brightness < 0.0 {
        term.cli_print(
            term::Level::Error,
            &format!(
                "brightness ({}) must be between 0.0 and 1.0!",
                args.brightness
            ),
        );
        exit(0);
    }

    if args.saturation > 1.0 || args.saturation < 0.0 {
        term.cli_print(
            term::Level::Error, 
            &format!(
                "saturation ({}) must be between 0.0 and 1.0!",
                args.saturation
            ),
        );
        exit(0);
    }

    term.cli_print(
        term::Level::Info,
        &format!(
            "using monitor: {}; width: {}, height: {}",
            args.monitor, width, height
        ),
    );

    // first, connect to board with neobridge. jelly just calculates what colors are on the monitor.
    // then returns those values to the board.
    let mut neobridge: Neobridge = Neobridge::new(&args.port, args.n_of_leds.try_into().unwrap());
    let mut jelly: engine::JellyRenderer = engine::JellyRenderer::new(
        width,
        height,
        args.n_of_leds,
        args.depth,
        color::ColorOption::new(args.brightness, args.saturation),
        term::CalculationOption::new(args.disable_color_operations),
    );

    term.cli_print(
        term::Level::Info,
        &format!(
            "connected to {} that has a number of {} LEDs.",
            args.port, args.n_of_leds
        ),
    );

    // reset LEDs to black.
    neobridge.set_all(RGB(0, 0, 0));
    neobridge.show();

    term.cli_print(
        term::Level::Info,
        &format!("sent reset commands to {}, assuming board works", args.port),
    );

    if args.run_in_background {
        term.cli_print(
            term::Level::Warning, 
            "running in background..."
        );

        std::mem::drop(neobridge);
        std::process::Command::new(format!(r"C:\Users\sayne\Programming\jellyfish-rs\target\release\jellyfish-rs.exe"))
                                    .args([
                                        format!("-p"),
                                        args.port,
                                        format!("-n"),
                                        args.n_of_leds.to_string(), 
                                        format!("-r"),
                                        args.refresh_rate.to_string(),
                                        format!("-d"),
                                        args.depth.to_string(),
                                        format!("-m"),
                                        args.monitor.to_string(),
                                        format!("--brightness"),
                                        args.brightness.to_string(),
                                        format!("--saturation"),
                                        args.saturation.to_string(),
                                        "--slient-mode".to_string()
                                        ],
                                    )
                                    .spawn()
                                    .expect("failed to run in background");
        exit(0)
    }

    // start loop here.
    let screen: Screen = monitors[args.monitor];

    term.cli_print(term::Level::Info, "started capture!");
    loop {
        // don't put image into a separate var, this prevents errors.
        if let Ok(image) = screen.capture_area(
            0,
            height as i32 - ((args.depth + 1) as i32),
            width,
            args.depth as u32,
        ) {
            // first get the colors.
            let colors: &Vec<RGB> = jelly.grab(&image);

            // then send it to the board.
            neobridge.set_list(colors);
            neobridge.show();
        }

        // just so it doesn't put a lot of load onto the CPU.
        thread::sleep(Duration::from_millis(1000 / args.refresh_rate));
    }
}
