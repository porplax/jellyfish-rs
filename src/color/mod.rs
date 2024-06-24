pub mod color_ops;

pub struct ColorOption {
    pub brightness: f32,
    pub saturation: f32
}

impl ColorOption {
    pub fn new(brightness: f32, saturation: f32) -> ColorOption {
        ColorOption { brightness, saturation }
    }
}
