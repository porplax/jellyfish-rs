use screenshots::image::{ImageBuffer, Pixel, Rgba};
use neobridge_rust::RGB;
// without using any list, there was a 72.930% increase in performance.
pub struct ChannelStorage {
    expecting_size: usize,

    sum_of_r: u16,
    sum_of_g: u16,
    sum_of_b: u16
}

impl ChannelStorage {
    pub fn new(expecting_size: usize) -> ChannelStorage {
        ChannelStorage {
            expecting_size,

            sum_of_r: 0,
            sum_of_g: 0,
            sum_of_b: 0
        }
    }

    fn push(&mut self, r: u8, g: u8, b: u8) {
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

    result: Vec<RGB>
}

impl JellyRenderer {
    pub fn new(depth: usize, width: u32, height: u32, n_of_leds: usize) -> JellyRenderer {
        JellyRenderer {
            width,
            _height: height,
            
            n_of_leds,

            depth,

            result: Vec::with_capacity(0)
        }
    }

    pub fn grab(&mut self, image: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> &Vec<RGB> {
        self.result.clear();
        let mut x: u32 = 0;

        // we want to get a certain amount of pixels at the bottom.
        for _row in 0..self.n_of_leds {

            // we are getting a column of RGB values for each LED on a strip.
            // for example, if I have a depth of 10, 
            // then there will be 10 RGB values for each LED on a strip.
            let mut column: ChannelStorage = ChannelStorage::new(self.depth);
            for y in 0..self.depth {
                let rgb_val: &Rgba<u8> = image.get_pixel(x, y as u32);
       
                // to average over these later, go ahead and store these.
                column.push(rgb_val.channels()[0], rgb_val.channels()[1], rgb_val.channels()[2]);
            }

            // every value (r, g, b) in the column is averaged into a single RGB. 
            let rgb: RGB = RGB(column.compile_r_channel_to_u8(), column.compile_g_channel_to_u8(), column.compile_b_channel_to_u8());

            self.result.push(rgb);
            x += self.width / self.n_of_leds as u32;
        }

    &self.result
    }

  
}
