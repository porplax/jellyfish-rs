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
    low_b: u8,
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
            low_b: 255,
        }
    }

    pub fn clear(&mut self) {
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

    pub fn push(&mut self, r: u8, g: u8, b: u8) {
        if r > self.high_r && g > self.high_g && b > self.high_b {
            self.high_r = r;
            self.high_g = g;
            self.high_b = b
        } else if r < self.low_r && g < self.low_g && b < self.low_b {
            self.low_r = r;
            self.low_g = g;
            self.low_b = b
        }

        self.sum_of_r = self.sum_of_r + r as u16;
        self.sum_of_g = self.sum_of_g + g as u16;
        self.sum_of_b = self.sum_of_b + b as u16;
    }

    pub fn compile_r_channel_to_u8(&mut self) -> u8 {
        (self.sum_of_r / self.expecting_size as u16) as u8
    }

    pub fn compile_g_channel_to_u8(&mut self) -> u8 {
        (self.sum_of_g / self.expecting_size as u16) as u8
    }

    pub fn compile_b_channel_to_u8(&mut self) -> u8 {
        (self.sum_of_b / self.expecting_size as u16) as u8
    }
}

