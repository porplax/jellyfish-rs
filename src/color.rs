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
        let mut hue: f32 = 0.0;
        let mut saturation: f32 = 0.0;
        let value: f32 = max;
        
    
        let delta: f32 = max - min;
    
        if max != 0.0 {
            saturation = delta / max;
        } else {
            return *r
        }
    
        if r_f32 == max {
            hue = (g_f32 - b_f32) / delta;
        } else if g_f32 == max {
            hue = 2.0 + (b_f32 - r_f32) / delta;
        } else {
            hue = 4.0 + (r_f32 - g_f32) / delta;
        }
        hue *= 60.0;
        if hue < 0.0 {
            hue += 360.0;
        }
    
        saturation *= scale;
    
        if saturation == 0.0 {
            return *r
        }
    
        hue /= 60.0;
        let i = f32::floor(hue);
        let f = hue - i;
        let p = value * (1.0 - saturation);
        let q = value * (1.0 - saturation * f);
        let t = value * (1.0 - saturation * (1.0 - f));
    
        let mut r: u8 = 0;
        let mut g: u8 = 0;
        let mut b: u8 = 0;
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
