use std::{ sync::atomic::Ordering, thread, time::Duration };
use neobridge_rust::{ Neobridge, RGB };
use screenshots::Screen;
use user_interface::app::JellyfishApp;

use crate::{color, renderer, user_interface};

impl JellyfishApp {
    pub fn run(&mut self) {
        let b: JellyfishApp = self.clone();
        let r: thread::JoinHandle<_> = std::thread::spawn({
            let running = self.running.clone();
            move || {
                let width: u32 = b.monitors[b.monitor_index].display_info.width;
                let height: u32 = b.monitors[b.monitor_index].display_info.height;

                let mut neobridge: Neobridge = Neobridge::new(
                    &b.port,
                    b.number_of_leds.try_into().unwrap()
                );
                let mut jelly: renderer::JellyRenderer = renderer::JellyRenderer::new(
                    width,
                    height,
                    b.number_of_leds,
                    b.depth_per_led,
                    color::ColorOption::new(b.brightness, b.saturation)
                );

                neobridge.set_all(RGB(0, 0, 0));
                neobridge.show();

                let screen: Screen = b.monitors[b.monitor_index];

                while running.load(Ordering::SeqCst) {
                    if
                        let Ok(image) = screen.capture_area(
                            0,
                            (height as i32) - ((b.depth_per_led + 1) as i32),
                            width,
                            b.depth_per_led as u32
                        )
                    {
                        let colors: &Vec<RGB> = jelly.grab(&image);

                        // then send it to the board.
                        neobridge.set_list(colors);
                        neobridge.show();
                    }

                    thread::sleep(Duration::from_millis(1000 / b.tick_rate));
                }
                neobridge.set_all(RGB(0, 0, 0));
                neobridge.show();
            }
        });
        drop(r);
    }
}