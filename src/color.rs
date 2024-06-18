use neobridge_rust::RGB;

pub struct ColorOption {
    pub brightness: f32,
}

impl ColorOption {
    pub fn new(brightness: f32) -> ColorOption {
        ColorOption {
            brightness
        }
    }
}

pub struct ColorOperation;
impl ColorOperation {
    pub fn set_brightness(r: &RGB, scale: f32) -> RGB {
        // it doesn't allow me to directly multiply by scale, so I have to do this.
        // converts from u8 to f32, scales it, then goes back from f32 to u8.
        RGB((f32::from(r.0)*scale) as u8, (f32::from(r.1)*scale) as u8, (f32::from(r.2)*scale) as u8)
    }

    pub fn set_saturation(r: &RGB) {
        // i have to convert each u8 value to a float.
        let store: Vec<f32> = vec![f32::from(r.0) / 255.0, f32::from(r.1) / 255.0, f32::from(r.2) / 255.0];

        // TODO: i got move errors here, i cloned for now but this isn't efficient!!!
        let max: f32 = store.clone().into_iter().reduce(f32::max).unwrap();
        let min: f32 = store.clone().into_iter().reduce(f32::min).unwrap();
        let luminance: f32 = (1.0 / 2.0) * (max + min);
        let mut saturation: f32 = 0.0;
    }
}