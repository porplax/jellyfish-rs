use neobridge_rust::{ Neobridge, RGB };
use xcap::Monitor;
use image::{ self, Pixel };
use std::{ thread, time::Duration };
use nalgebra::{ DMatrix, Dyn, OMatrix };
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author="cutesunshine", version, about="Ambient lighting on neopixel devices.")]
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
    refresh_rate: u64
}

type DMatrixf32 = OMatrix<f32, Dyn, Dyn>;

struct JellyRenderer {
    neobridge: Neobridge,
    monitors: Vec<Monitor>,

    width: u32, 
    height: u32,
    n_of_leds: usize,

    x: u32, 
    y: u32,

    channels: DMatrixf32,

    depth: usize,
}

impl JellyRenderer {
    pub fn new(neobridge: Neobridge, monitors: Vec<Monitor>, depth: usize, width: u32, height: u32, n_of_leds: usize) -> JellyRenderer {
        JellyRenderer {
            neobridge,
            monitors,

            width,
            height,
            n_of_leds,

            x: 0,
            y: 0,

            channels: DMatrixf32::zeros(0, 0),
            
            depth
        }
    }

    pub fn initialize(&mut self) {
        self.neobridge.set_all(RGB                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          (0, 0, 0));
        self.neobridge.show();
    }

    pub fn grab_colors(&mut self) -> Vec<RGB> {
        let mut colors: Vec<RGB> = Vec::with_capacity(self.n_of_leds);
        self.x = 0;

        self.channels = DMatrix::identity(self.depth, 3);
        if let Ok(image) = self.monitors.get(0).unwrap().capture_image() {
            for _x in 0..self.n_of_leds {
                self.y = self.height-1;
    
                for _y in 0..self.depth {
                    let rgb_val = image.get_pixel(self.x, self.y);
                    self.channels[(_y, 0)] = rgb_val.channels()[0] as f32; // R
                    self.channels[(_y, 1)] = rgb_val.channels()[1] as f32; // G
                    self.channels[(_y, 2)] = rgb_val.channels()[2] as f32; // B
    
                    self.y -= 1;
                }
    
                let rgb_convert = RGB(
                    self.channels.column(0).mean() as u8,
                    self.channels.column(1).mean() as u8,
                    self.channels.column(2).mean() as u8
                );
                
                colors.push(rgb_convert);
                self.x += self.width / self.n_of_leds as u32;
            }
        } 
        colors
    }
}

fn main() {
    let args = Args::parse();

    let neobridge = Neobridge::new(&args.port, 30);
    let monitors = Monitor::all().unwrap();

    let mut jelly = JellyRenderer::new(
        neobridge,
        monitors,
        args.depth,
        args.width,
        args.height,
        args.n_of_leds
        );

    jelly.initialize();

    loop {
        let colors = jelly.grab_colors();

        jelly.neobridge.set_list(colors);
        jelly.neobridge.show();
        thread::sleep(Duration::from_millis(1000 / args.refresh_rate as u64));
    }
}