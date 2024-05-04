use neobridge_rust::{ Neobridge, RGB };
use toml::Table;
use xcap::Monitor;
use image::{ self, Pixel };
use std::{ thread, time::Duration };
use nalgebra::{ DMatrix, Dyn, OMatrix };
use std::env;
use std::fs::File;
use std::io::prelude::*;

type DMatrixu16 = OMatrix<f32, Dyn, Dyn>;

struct JellyRenderer {
    neobridge: Neobridge,
    monitors: Vec<Monitor>,

    x: u32, 
    y: u32,

    height_capture: usize,
}

impl JellyRenderer {
    pub fn new(neobridge: Neobridge, monitors: Vec<Monitor>, height_capture: usize) -> JellyRenderer {
        JellyRenderer {
            neobridge,
            monitors,

            x: 0,
            y: 0,
            
            height_capture
        }
    }

    pub fn initialize(&mut self) {
        self.neobridge.set_all(RGB(0, 0, 0));
        self.neobridge.show();
    }

    pub fn grab_colors(&mut self) -> Vec<RGB> {
        let image = self.monitors.get(0).unwrap().capture_image().unwrap();

        let mut colors: Vec<RGB> = vec![];
        self.x = 0;

        let mut channels: DMatrixu16 = DMatrix::identity(self.height_capture, 3);

        for _x in 0..30 {
            self.y = 1079;

            for _y in 0..self.height_capture {
                let rgb_val = image.get_pixel(self.x, self.y);
                channels[(_y, 0)] = rgb_val.channels()[0] as f32; // R
                channels[(_y, 1)] = rgb_val.channels()[1] as f32; // G
                channels[(_y, 2)] = rgb_val.channels()[2] as f32; // B

                self.y -= 1;
            }

            let rgb_convert = RGB(
                channels.column(0).mean() as u8,
                channels.column(1).mean() as u8,
                channels.column(2).mean() as u8
            );
            
            colors.push(rgb_convert);
            self.x += 66;
        }
        colors
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("expected file name, got {}", args.len());
    }

    let file = File::open(&args[1]);
    let mut contents = String::new();
    file.expect("file aint openin").read_to_string(&mut contents);

    let toml = contents.parse::<Table>().unwrap();
    let frame_rate = toml["loop"]["frame_rate"].as_integer().unwrap();
    let height_capture = toml["jellyfish"]["height_capture"].as_integer().unwrap();

    println!("Set frame rate to {}", frame_rate);
    println!("Set height capture to {}", height_capture);

    let neobridge = Neobridge::new("COM3", 30);
    let monitors = Monitor::all().unwrap();

    let mut jelly = JellyRenderer::new(
        neobridge, 
        monitors,
        height_capture as usize);
    jelly.initialize();

    loop {
        let colors = jelly.grab_colors();

        jelly.neobridge.set_list(colors);
        jelly.neobridge.show();
        thread::sleep(Duration::from_millis(1000 / frame_rate as u64));
    }
}