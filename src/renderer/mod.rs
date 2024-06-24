use neobridge_rust::RGB;
use screenshots::image::{ImageBuffer, Pixel, Rgba};

use crate::{
    channel_storage, color
};

pub struct CalculationOption {
    pub disable_color_operations: bool,
}

impl CalculationOption {
    pub fn new(disable_color_operations: bool) -> CalculationOption {
        CalculationOption {
            disable_color_operations,
        }
    }
}

pub struct JellyRenderer {
    width: u32,
    _height: u32,

    n_of_leds: usize,

    depth: usize,

    color_option: color::ColorOption,
    calc_option: CalculationOption,

    result: Vec<RGB>,
}

impl JellyRenderer {
    pub fn new(
        width: u32,
        height: u32,
        n_of_leds: usize,
        depth: usize,
        color_option: color::ColorOption,
        calc_option: CalculationOption,
    ) -> JellyRenderer {
        JellyRenderer {
            width,
            _height: height,

            n_of_leds,
            depth,

            color_option,
            calc_option,

            result: Vec::with_capacity(0),
        }
    }

    pub fn grab(&mut self, image: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> &Vec<RGB> {
        self.result.clear();
        let mut x: u32 = 0;

        // we want to get a certain amount of pixels at the bottom.
        let mut channels: channel_storage::ChannelStorage = channel_storage::ChannelStorage::new(self.depth);
        for _row in 0..self.n_of_leds {
            // we are getting a column of RGB values for each LED on a strip.
            // for example, if I have a depth of 10,
            // then there will be 10 RGB values for each LED on a strip.
            channels.clear();
            for _column in 0..self.depth {
                let rgb_val: &Rgba<u8> = image.get_pixel(x, _column as u32);

                // to average over these later, go ahead and store these.
                channels.push(
                    rgb_val.channels()[0],
                    rgb_val.channels()[1],
                    rgb_val.channels()[2],
                );
            }

            // every value (r, g, b) in the column is averaged into a single RGB.
            let mut rgb: RGB = RGB(
                channels.compile_r_channel_to_u8(),
                channels.compile_g_channel_to_u8(),
                channels.compile_b_channel_to_u8(),
            );

            if !(self.calc_option.disable_color_operations) {
                rgb = color::color_ops::ColorOperation::set_saturation(&rgb, self.color_option.saturation);
                rgb = color::color_ops::ColorOperation::set_brightness(&rgb, self.color_option.brightness);
            }

            self.result.push(rgb);
            x += self.width / self.n_of_leds as u32;
        }

        &self.result
    }
}
