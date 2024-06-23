use std::sync::{atomic::{AtomicBool, Ordering}, Arc};

use egui::RichText;
use screenshots::Screen;

use crate::VERSION;

#[derive(Clone)]
pub struct JellyfishApp {
    pub monitors: Vec<Screen>,
    pub m_idx: usize,
    pub n_of_leds: usize,

    pub depth: usize,
    pub refresh_rate: u64,

    pub brightness: f32,
    pub saturation: f32,

    pub port: String,
    pub running: Arc<AtomicBool>,
}

impl Default for JellyfishApp {
    fn default() -> Self {
        Self {
            monitors: Screen::all().unwrap(),
            m_idx: 0,
            n_of_leds: 30,

            depth: 12,
            refresh_rate: 30,

            brightness: 1.0,
            saturation: 0.2,

            port: String::from("COM3"),
            running: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl eframe::App for JellyfishApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("jellyfish!");
            ui.label(&format!("v{}", VERSION));
            if self.running.load(Ordering::Relaxed) {
                ui.label(
                    RichText::new("running!")
                        .color(egui::Color32::GREEN)
                        .monospace()
                        .small(),
                );
            } else {
                ui.label(
                    RichText::new("waiting for user...")
                        .color(egui::Color32::RED)
                        .monospace()
                        .small(),
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
                ui.add(egui::Slider::new(
                    &mut self.m_idx,
                    0..=self.monitors.len() - 1,
                ))
                .on_hover_text_at_pointer("Which monitor to use.");
            });

            ui.horizontal(|ui| {
                ui.label("depth:");
                ui.add(
                    egui::DragValue::new(&mut self.depth)
                        .speed(1)
                        .clamp_range(0..=self.monitors[self.m_idx].display_info.height / 2),
                )
                .on_hover_text_at_pointer("How many pixels to calculate for each LED.");

                ui.label("# of leds:");
                ui.add(egui::DragValue::new(&mut self.n_of_leds).speed(1))
                    .on_hover_text_at_pointer("How many LEDs are present on the strip.");

                ui.label("refresh rate:");
                ui.add(egui::DragValue::new(&mut self.refresh_rate).speed(1))
                    .on_hover_text_at_pointer("How fast the program should run.");
            });
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("brightness:");
                ui.add(
                    egui::DragValue::new(&mut self.brightness)
                        .speed(0.005)
                        .clamp_range(0.0..=1.0),
                )
                .on_hover_text_at_pointer("Brightness of each LED.");

                ui.label("saturation:");
                ui.add(
                    egui::DragValue::new(&mut self.saturation)
                        .speed(0.005)
                        .clamp_range(0.0..=1.0),
                )
                .on_hover_text_at_pointer("Saturation of each LED.");
            });

            ui.horizontal(|ui| {
                if ui.button(RichText::new("run!")).clicked()
                    && !self.running.load(Ordering::Relaxed)
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
