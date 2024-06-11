use std::mem::take;
use image::{ImageBuffer, Pixel, Rgba};
use nalgebra::{Dyn, OMatrix};
use neobridge_rust::RGB;

type DMatrixf32 = OMatrix<f32, Dyn, Dyn>;

pub struct JellyRenderer {
    width: u32,
    height: u32, 
    n_of_leds: usize,

    depth: usize,
    buf: BufferStorage
}

// i made this struct so we don't have to re-allocate, or define a variable each time we grab colors.
// this makes it easier for the CPU to handle memory resources.
struct BufferStorage {
/*  last: Vec<RGB>,*/    
    current: Vec<RGB>,
    channel: DMatrixf32
}

impl BufferStorage {
    fn new(current: Vec<RGB>, channel: DMatrixf32) -> BufferStorage {
        BufferStorage {
            current,
            channel
        }
    }

    // so we can average an entire column of RGB values down to a u8 value (0-255).
    fn compile_column_to_u8(&self, index: usize) -> u8 {
        self.channel.column(index).mean() as u8
    }
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

    // get colors from provided imagebuffer.
    pub fn grab(&mut self, image: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> Vec<RGB> {
        // we clear what was in the buffer.        
        self.buf.current.clear();
        self.buf.channel = DMatrixf32::identity(self.depth, 3);

        // position values given here.
        let mut x = 0;
        let mut _y = 0;

        // we want to get a certain amount of pixels at the bottom.
        for _row in 0..self.n_of_leds {
            _y = self.height-1;

            // so we do this, get RGB values of pixels in a column.
            // delimited by depth given by user.
            for _column in 0..self.depth {
                // get pixel at position.
                // TODO: there are methods in `image` that can make this way easier and efficient.
                let rgb_val = image.get_pixel(x, _y);

                // store this in the matrix, so we don't have to create 3 vectors.
                // TODO: i feel like there can be other ways to do this without matrices.
                self.buf.channel[(_column, 0)] = rgb_val.channels()[0] as f32; // R
                self.buf.channel[(_column, 1)] = rgb_val.channels()[1] as f32; // G
                self.buf.channel[(_column, 2)] = rgb_val.channels()[2] as f32; // B

                _y -= 1;
            }

            // we have to convert this to RGB from neobridge so the board can read the values.
            // also we average everything that was stored in the matrix so it looks more ambient.
            let rgb: RGB = RGB(
                self.buf.compile_column_to_u8(0),
                self.buf.compile_column_to_u8(1),
                self.buf.compile_column_to_u8(2)
            );
            
            // push and move to next column.
            self.buf.current.push(rgb);
            x += self.width / self.n_of_leds as u32;
        }
        
        // take ownership. so that rust doesn't throw an error.
        let re = take(&mut self.buf.current);
        re
    }
}