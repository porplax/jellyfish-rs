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

    // https://stackoverflow.com/questions/13806483/increase-or-decrease-color-saturation
    pub fn set_saturation(r: &RGB, scale: f32) -> RGB{
        // i have to convert each u8 value to a float.
        let store: Vec<f32> = vec![
            f32::from(r.0) / 255.0,
            f32::from(r.1) / 255.0,
            f32::from(r.2) / 255.0,
        ];

        // TODO: i got move errors here, i cloned for now but this isn't efficient!!!
        let max: f32 = store.clone().into_iter().reduce(f32::max).unwrap();
        let min: f32 = store.clone().into_iter().reduce(f32::min).unwrap();
        
        let mut _hue: f32 = 0.0;
        let mut _saturation: f32 = 0.0;
        let value: f32 = max;
        

        let delta: f32 = max - min;

        if max != 0.0 {
            _saturation = delta / max;
        } else {
            _saturation = 0.0;
            _hue = -1.0;
            return RGB(0, 0, 0)
        }

        if store[0] == max {
            _hue = (store[1] - store[2]) / delta;
        } else if store[1] == max {
            _hue = 2.0 + (store[2] - store[0]) / delta;
        } else {
            _hue = 4.0 + (store[0] - store[1]) / delta;
        }
        _hue *= 60.0;
        if _hue < 0.0 {
            _hue += 360.0;
        }

        if f32::is_nan(_hue) {
            _hue = 0.0;
        }

        _saturation *= scale;

        if _saturation == 0.0 {
            return RGB(0, 0, 0)
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
        match i {
            0.0 => {
                _r = (value*255.0) as u8;
                _g = (t*255.0) as u8;
                _b = (p*255.0) as u8;
            },
            1.0 => {
                _r = (q*255.0) as u8;
                _g = (value*255.0) as u8;
                _b = (p*255.0) as u8;
            }
            2.0 => {
                _r = (p*255.0) as u8;
                _g = (value*255.0) as u8;
                _b = (t*255.0) as u8;
            }
            3.0 => {
                _r = (p*255.0) as u8;
                _g = (q*255.0) as u8;
                _b = (value*255.0) as u8;
            }
            4.0 => {
                _r = (t*255.0) as u8;
                _g = (p*255.0) as u8;
                _b = (value*255.0) as u8;
            }
            _ => {_r=(value*255.0) as u8;_g=(p*255.0) as u8;_b=(q*255.0) as u8}
        }
        RGB(_r, _g, _b)
    }
}
