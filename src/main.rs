#![windows_subsystem = "windows"]

use constants::VERSION;
use eframe::egui;
use user_interface::app::JellyfishApp;

mod channel_storage;
mod color;
mod config;
mod renderer;
mod user_interface;
mod constants;
mod event_loop;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder
            ::default()
            .with_icon(egui::IconData {
                rgba: image::open(r"assets\jellyfish.png").expect("failed").to_rgba8().into_raw(),
                width: 160,
                height: 160,
            })
            .with_inner_size([400.0, 200.0])
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        &format!("jellyfish! v{}", VERSION),
        options,
        Box::new(|cc: &eframe::CreationContext| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<JellyfishApp>::default()
        })
    )
}
