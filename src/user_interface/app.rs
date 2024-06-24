use std::sync::{ Arc, atomic::AtomicBool };

use screenshots::Screen;

#[derive(Clone)]
pub struct JellyfishApp {
    pub monitors: Vec<Screen>,
    pub monitor_index: usize,
    pub number_of_leds: usize,

    pub depth_per_led: usize,
    pub tick_rate: u64,

    pub brightness: f32,
    pub saturation: f32,

    pub port: String,
    pub running: Arc<AtomicBool>,
}
