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

    pub fn clear(&mut self) {
        self.sum_of_r = 0;
        self.sum_of_g = 0;
        self.sum_of_b = 0;
    }

    pub fn push(&mut self, r: u8, g: u8, b: u8) {
        self.sum_of_r = self.sum_of_r + (r as u16);
        self.sum_of_g = self.sum_of_g + (g as u16);
        self.sum_of_b = self.sum_of_b + (b as u16);
    }

    pub fn to_rgbu8(&mut self) -> neobridge_rust::RGB {
        return neobridge_rust::RGB((self.sum_of_r / (self.expecting_size as u16)) as u8, (self.sum_of_g / (self.expecting_size as u16)) as u8, (self.sum_of_b / (self.expecting_size as u16)) as u8);
    }
}
