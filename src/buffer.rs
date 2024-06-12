use neobridge_rust::RGB;
use nalgebra::{Dyn, OMatrix};

pub type DMatrixf32 = OMatrix<f32, Dyn, Dyn>;

// i made this struct so we don't have to re-allocate, or define a variable each time we grab colors.
// easier to store values.
pub struct BufferStorage {
    /*  last: Vec<RGB>,*/
    pub current: Vec<RGB>,
    pub channel: DMatrixf32,
}

impl BufferStorage {
    pub fn new(current: Vec<RGB>, channel: DMatrixf32) -> BufferStorage {
        BufferStorage { current, channel }
    }

    // so we can average an entire column of RGB values down to a u8 value (0-255).
    pub fn compile_column_to_u8(&self, index: usize) -> u8 {
        self.channel.column(index).mean() as u8
    }
}