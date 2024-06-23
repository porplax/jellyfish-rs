use std::{sync::atomic::Ordering, thread, time::Duration};

use neobridge_rust::{Neobridge, RGB};
use screenshots::Screen;

use crate::{calc, engine, gui::JellyfishApp};

pub struct CalculationOption {
    pub disable_color_operations: bool,
}

impl CalculationOption {
    pub fn new(disable_color_operations: bool) -> CalculationOption {
        CalculationOption {
            disable_color_operations,
        }
    }
}


impl JellyfishApp {
    pub fn run(self) {
        let b: JellyfishApp = self.clone();
        let r: thread::JoinHandle<_> = std::thread::spawn({
            let running = self.running.clone();
            move || {
                let width: u32 = b.monitors[b.m_idx].display_info.width;
                let height: u32 = b.monitors[b.m_idx].display_info.height;

                let mut neobridge: Neobridge =
                    Neobridge::new(&b.port, b.n_of_leds.try_into().unwrap());
                let mut jelly: engine::JellyRenderer = engine::JellyRenderer::new(
                    width,
                    height,
                    b.n_of_leds,
                    b.depth,
                    calc::ColorOption::new(b.brightness, b.saturation),
                    CalculationOption::new(false),
                );

                neobridge.set_all(RGB(0, 0, 0));
                neobridge.show();

                let screen: Screen = b.monitors[b.m_idx];

                loop {
                    if !running.load(Ordering::SeqCst) {
                        neobridge.set_all(RGB(0, 0, 0));
                        neobridge.show();
                        return;
                    }
                    if let Ok(image) = screen.capture_area(
                        0,
                        height as i32 - ((b.depth + 1) as i32),
                        width,
                        b.depth as u32,
                    ) {
                        let colors: &Vec<RGB> = jelly.grab(&image);

                        // then send it to the board.
                        neobridge.set_list(colors);
                        neobridge.show();
                    }

                    thread::sleep(Duration::from_millis(1000 / b.refresh_rate));
                }
            }
        });
        drop(r);
    }
}
