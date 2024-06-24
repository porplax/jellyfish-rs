use std::sync::{ atomic::{ AtomicBool, Ordering }, Arc };
use egui::RichText;
use crate::*;

pub mod app;

impl eframe::App for JellyfishApp {
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.save_struct_to_config();
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("jellyfish!");
            ui.label(&format!("v{}", VERSION));
            if self.running.load(Ordering::Relaxed) {
                ui.label(RichText::new("running!").color(egui::Color32::GREEN).monospace().small());
            } else {
                ui.label(
                    RichText::new("waiting for user...")
                        .color(egui::Color32::RED)
                        .monospace()
                        .small()
                );
            }
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("port:");
                ui.text_edit_singleline(&mut self.port)
                    .highlight()
                    .on_hover_text_at_pointer("Which port serial is located on.");
            });

            ui.horizontal(|ui| {
                ui.label("monitor:");
                ui.add(
                    egui::Slider::new(&mut self.monitor_index, 0..=self.monitors.len() - 1)
                ).on_hover_text_at_pointer("Which monitor to use.");
            });

            ui.horizontal(|ui| {
                ui.label("depth:");
                ui.add(
                    egui::DragValue
                        ::new(&mut self.depth_per_led)
                        .speed(1)
                        .clamp_range(0..=self.monitors[self.monitor_index].display_info.height / 2)
                ).on_hover_text_at_pointer("How many pixels to calculate for each LED.");

                ui.label("# of leds:");
                ui.add(
                    egui::DragValue::new(&mut self.number_of_leds).speed(1)
                ).on_hover_text_at_pointer("How many LEDs are present on the strip.");

                ui.label("tick rate:");
                ui.add(egui::DragValue::new(&mut self.tick_rate).speed(1)).on_hover_text_at_pointer(
                    "How fast the program should run."
                );
            });
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("brightness:");
                ui.add(
                    egui::DragValue
                        ::new(&mut self.brightness)
                        .speed(0.005)
                        .clamp_range(0.0..=1.0)
                ).on_hover_text_at_pointer("Brightness of each LED.");

                ui.label("saturation:");
                ui.add(
                    egui::DragValue
                        ::new(&mut self.saturation)
                        .speed(0.005)
                        .clamp_range(0.0..=1.0)
                ).on_hover_text_at_pointer("Saturation of each LED.");
            });

            ui.horizontal(|ui| {
                if
                    ui.button(RichText::new("run!")).clicked() &&
                    !self.running.load(Ordering::Relaxed)
                {
                    self.running = Arc::new(AtomicBool::new(true));
                    <JellyfishApp as Clone>::clone(&self).run();
                }

                if self.running.load(Ordering::Relaxed) {
                    if ui.button("stop!").clicked() {
                        self.running.store(false, Ordering::SeqCst);
                    }
                }
            });
        });
    }
}
