pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CONFIG_NAME: &str = "config.json";

pub const DEFAULT_MONITOR_INDEX: usize = 0;
pub const DEFAULT_NUMBER_OF_LEDS: usize = 30;
pub const DEFAULT_DEPTH_PER_LED: usize = 12;
pub const DEFAULT_TICK_RATE: u64 = 30;
pub const DEFAULT_BRIGHTNESS: f32 = 1.0;
pub const DEFAULT_SATURATION: f32 = 0.2;
pub const DEFAULT_PORT: &str = "COM3";