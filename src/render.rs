use image::{ImageBuffer, Pixel, Rgba};
use neobridge_rust::RGB;

use crate::buffer::{BufferStorage, DMatrixf32};

pub struct JellyRenderer {
    width: u32,
    height: u32,
    n_of_leds: usize,

    depth: usize,
    buf: BufferStorage,
}

impl JellyRenderer {
    pub fn new(depth: usize, width: u32, height: u32, n_of_leds: usize) -> JellyRenderer {
        JellyRenderer {
            width,
            height,
            n_of_leds,

            depth,
            buf: BufferStorage::new(Vec::with_capacity(n_of_leds), DMatrixf32::zeros(0, 0)),
        }
    }

    fn init(&mut self) {
        self.buf.current.clear();
        self.buf.channel = DMatrixf32::identity(self.depth, 3);
    }

    pub fn grab(&mut self, image: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> &Vec<RGB> {
        // clear so we don't run into errors.
        self.init();

        let mut x: u32 = 0;
        let mut _y: u32 = 0;

        // we want to get a certain amount of pixels at the bottom.
        for _row in 0..self.n_of_leds {
            _y = self.height - 1;

            // we are getting a column of RGB values for each LED on a strip.
            // for example, if I have a depth of 10, 
            // then there will be 10 RGB values for each LED on a strip.
            for _column in 0..self.depth {
                let rgb_val: &Rgba<u8> = image.get_pixel(x, _y);

                // to average over these later, go ahead and store these in a matrix.
                self.buf.channel[(_column, 0)] = rgb_val.channels()[0] as f32; // R
                self.buf.channel[(_column, 1)] = rgb_val.channels()[1] as f32; // G
                self.buf.channel[(_column, 2)] = rgb_val.channels()[2] as f32; // B

                _y -= 1;
            }

            // every value (r, g, b) in the column is averaged into a single RGB struct. 
            let rgb: RGB = RGB(
                self.buf.compile_column_to_u8(0),
                self.buf.compile_column_to_u8(1),
                self.buf.compile_column_to_u8(2),
            );

            self.buf.current.push(rgb);
            x += self.width / self.n_of_leds as u32;
        }

        &self.buf.current
    }
}

