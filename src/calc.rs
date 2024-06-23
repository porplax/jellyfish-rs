use neobridge_rust::RGB;
pub struct ColorOption {
    pub brightness: f32,
    pub saturation: f32
}

impl ColorOption {
    pub fn new(brightness: f32, saturation: f32) -> ColorOption {
        ColorOption { brightness, saturation }
    }
}

pub struct ColorOperation;
impl ColorOperation {
    pub fn set_brightness(r: &RGB, scale: f32) -> RGB {
        // it doesn't allow me to directly multiply by scale, so I have to do this.
        // converts from u8 to f32, scales it, then goes back from f32 to u8.
        RGB(
            (f32::from(r.0) * scale) as u8,
            (f32::from(r.1) * scale) as u8,
            (f32::from(r.2) * scale) as u8,
        )
    }

    // https://stackoverflow.com/questions/13806483/increase-or-decrease-color-saturation (thank you @Hoffmann!!!)
    pub fn set_saturation(r: &RGB, scale: f32) -> RGB{
        // i have to convert each u8 value to a float.
        let r_f32: f32 = f32::from(r.0) / 255.0; 
        let g_f32: f32 = f32::from(r.1) / 255.0; 
        let b_f32: f32 = f32::from(r.2) / 255.0; 
        let max: f32 = f32::max(f32::max(r_f32, g_f32), b_f32);
        let min: f32 = f32::min(f32::min(r_f32, g_f32), b_f32);
    
        
        /* let store: Arc<[f32; 3]> = Arc::new([
            f32::from(r.0) / 255.0,
            f32::from(r.1) / 255.0,
            f32::from(r.2) / 255.0,
        ]);
    
        let max: f32 = store.into_iter().reduce(f32::max).unwrap();
        let min: f32 = store.into_iter().reduce(f32::min).unwrap();
         */
        let mut _hue: f32 = 0.0;
        let mut _saturation: f32 = 0.0;
        let value: f32 = max;
        
    
        let delta: f32 = max - min;
    
        if max != 0.0 {
            _saturation = delta / max;
        } else {
            return *r
        }
    
        if r_f32 == max {
            _hue = (g_f32 - b_f32) / delta;
        } else if g_f32 == max {
            _hue = 2.0 + (b_f32 - r_f32) / delta;
        } else {
            _hue = 4.0 + (r_f32 - g_f32) / delta;
        }
        _hue *= 60.0;
        if _hue < 0.0 {
            _hue += 360.0;
        }
    
        _saturation *= 10.0*scale;
    
        if _saturation == 0.0 {
            return *r
        }
    
        _hue /= 60.0;
        let i = f32::floor(_hue);
        let f = _hue - i;
        let p = value * (1.0 - _saturation);
        let q = value * (1.0 - _saturation * f);
        let t = value * (1.0 - _saturation * (1.0 - f));
    
        let mut _r: u8 = 0;
        let mut _g: u8 = 0;
        let mut _b: u8 = 0;
        let (r, g, b) = match i as i32 {
            0 => (value, t, p),
            1 => (q, value, p),
            2 => (p, value, t),
            3 => (p, q, value),
            4 => (t, p, value),
            _ => (value, p, q),
        };
    
        RGB((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
    }
}

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

