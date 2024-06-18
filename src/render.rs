use screenshots::image::{ImageBuffer, Pixel, Rgba};
use neobridge_rust::RGB;

use crate::{color::{self, ColorOption}, CalculationOption};


// without using any list, there was a 72.930% increase in performance.
pub struct ChannelStorage {
    expecting_size: usize,

    sum_of_r: u16,
    sum_of_g: u16,
    sum_of_b: u16,

    high_r: u8,
    high_g: u8,
    high_b: u8,

    low_r: u8,
    low_g: u8,
    low_b: u8
}

impl ChannelStorage {
    pub fn new(expecting_size: usize) -> ChannelStorage {
        ChannelStorage {
            expecting_size,

            sum_of_r: 0,
            sum_of_g: 0,
            sum_of_b: 0,

            high_r: 0,
            high_g: 0,
            high_b: 0,

            low_r: 255,
            low_g: 255,
            low_b: 255
        }
    }

    fn clear(&mut self) {
        self.sum_of_r = 0;
        self.sum_of_g = 0;
        self.sum_of_b = 0;

        self.high_r = 0;
        self.high_g = 0;
        self.high_b = 0;

        self.low_r = 255;
        self.low_g = 255;
        self.low_b = 255;
    }

    fn push(&mut self, r: u8, g: u8, b: u8) {
        if r > self.high_r && g > self.high_g && b > self.high_b {self.high_r = r;self.high_g = g;self.high_b = b}
        else if r < self.low_r && g < self.low_g && b < self.low_b {self.low_r = r;self.low_g = g;self.low_b = b}

        self.sum_of_r = self.sum_of_r + r as u16;
        self.sum_of_g = self.sum_of_g + g as u16;
        self.sum_of_b = self.sum_of_b + b as u16;
    }

    fn compile_r_channel_to_u8(&mut self) -> u8 {
        (self.sum_of_r / self.expecting_size as u16) as u8
    }

    fn compile_g_channel_to_u8(&mut self) -> u8 {
        (self.sum_of_g / self.expecting_size as u16) as u8
    }

    fn compile_b_channel_to_u8(&mut self) -> u8 {
        (self.sum_of_b / self.expecting_size as u16) as u8
    }

}

pub struct JellyRenderer {
    width: u32,
    _height: u32,

    n_of_leds: usize,

    depth: usize,

    color_option: color::ColorOption,
    calc_option: CalculationOption,

    result: Vec<RGB>
}

impl JellyRenderer {
    pub fn new(width: u32, height: u32, n_of_leds: usize, depth: usize, color_option: ColorOption, calc_option: CalculationOption) -> JellyRenderer {
        JellyRenderer {
            width,
            _height: height,
            
            n_of_leds,
            depth,

            color_option,
            calc_option,

            result: Vec::with_capacity(0)
        }
    }

    pub fn grab(&mut self, image: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> &Vec<RGB> {
        self.result.clear();
        let mut x: u32 = 0;

        // we want to get a certain amount of pixels at the bottom.
        let mut channels: ChannelStorage = ChannelStorage::new(self.depth);
        for _row in 0..self.n_of_leds {

            // we are getting a column of RGB values for each LED on a strip.
            // for example, if I have a depth of 10, 
            // then there will be 10 RGB values for each LED on a strip.
            channels.clear();
            for _column in 0..self.depth {
                let rgb_val: &Rgba<u8> = image.get_pixel(x, _column as u32);
       
                // to average over these later, go ahead and store these.
                channels.push(rgb_val.channels()[0], rgb_val.channels()[1], rgb_val.channels()[2]);
            }

            // every value (r, g, b) in the column is averaged into a single RGB. 
            let mut rgb: RGB = RGB(channels.compile_r_channel_to_u8(), channels.compile_g_channel_to_u8(), channels.compile_b_channel_to_u8());
            
            if !(self.calc_option.disable_color_operations) {
                rgb = color::ColorOperation::set_brightness(&rgb, self.color_option.brightness);
            }

            self.result.push(rgb);
            x += self.width / self.n_of_leds as u32;
        }

    &self.result
    }

  
}
