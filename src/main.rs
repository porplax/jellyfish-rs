#![windows_subsystem = "windows"]
use eframe::{egui, HardwareAcceleration};
use egui::{IconData, RichText};
use neobridge_rust::{Neobridge, RGB};
use screenshots::Screen;
use tray_icon::TrayIconEvent;
use winit::{raw_window_handle::HasWindowHandle, window::Icon};
use std::{ops::Deref, process::exit, thread, time::Duration};

mod color;
mod engine;
mod term;

const RECOMMENDED_DEPTH_LIMIT: usize = 300;
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone)]
struct JellyfishApp {
    monitors: Vec<Screen>,
    m_idx: usize,
    n_of_leds: usize,

    depth: usize,
    refresh_rate: u64,

    brightness: f32,
    saturation: f32,

    port: String
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

            port: String::from("COM3")
        }
    }
}

impl eframe::App for JellyfishApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("jellyfish!");
            ui.label(&format!("v{}", VERSION));
            ui.separator();
            
            ui.horizontal(|ui| {
                ui.label("port:");
                ui.text_edit_singleline(&mut self.port).highlight().on_hover_text_at_pointer("Which port serial is located on.");
            });

            ui.horizontal(|ui| {
                ui.label("monitor:");
                ui.add(egui::Slider::new(&mut self.m_idx, 0..=self.monitors.len()-1)).on_hover_text_at_pointer("Which monitor to use.");
            });

            ui.horizontal(|ui| {
                ui.label("depth:");
                ui.add(egui::DragValue::new(&mut self.depth).speed(1).clamp_range(0..=self.monitors[self.m_idx].display_info.height/2)).on_hover_text_at_pointer("How many pixels to calculate for each LED.");
                
                ui.label("# of leds:");
                ui.add(egui::DragValue::new(&mut self.n_of_leds).speed(1)).on_hover_text_at_pointer("How many LEDs are present on the strip.");

                ui.label("refresh rate:");
                ui.add(egui::DragValue::new(&mut self.refresh_rate).speed(1)).on_hover_text_at_pointer("How fast the program should run.");
            });
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("brightness:");
                ui.add(egui::DragValue::new(&mut self.brightness).speed(0.005).clamp_range(0.0..=1.0)).on_hover_text_at_pointer("Brightness of each LED.");

                ui.label("saturation:");
                ui.add(egui::DragValue::new(&mut self.saturation).speed(0.005).clamp_range(0.0..=1.0)).on_hover_text_at_pointer("Saturation of each LED.");
            });

            if ui.button(RichText::new("run!").heading().italics().color(egui::Color32::GREEN)).clicked() {
                let b: JellyfishApp = self.clone();
                let r: thread::JoinHandle<_> = std::thread::spawn(move || {
                    let width: u32 = b.monitors[b.m_idx].display_info.width;
                    let height: u32 = b.monitors[b.m_idx].display_info.height;

                    let mut neobridge: Neobridge = Neobridge::new(&b.port, b.n_of_leds.try_into().unwrap());
                    let mut jelly: engine::JellyRenderer = engine::JellyRenderer::new(
                        width,
                        height,
                        b.n_of_leds,
                        b.depth,
                        color::ColorOption::new(b.brightness, b.saturation),
                        term::CalculationOption::new(false),
                    );

                    neobridge.set_all(RGB(0, 0, 0));
                    neobridge.show();

                    let screen: Screen = b.monitors[b.m_idx];

                    loop {
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
                });
                drop(r);
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_icon(egui::IconData {
            rgba: image::open(r"assets\jellyfish.png").expect("failed").to_rgba8().into_raw(),
            width: 160,
            height: 160,
        }).with_inner_size([400.0, 200.0]).with_resizable(false),
        ..Default::default()
        
    };
    eframe::run_native(
        &format!("jellyfish! v{}", VERSION),
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            // https://github.com/emilk/egui/discussions/737#discussioncomment-8830140
                let winit::raw_window_handle::RawWindowHandle::Win32(handle) = cc.window_handle().unwrap().as_raw() else {
                    panic!("Unsupported platform");
                };
        
                let context: egui::Context = cc.egui_ctx.clone();

                TrayIconEvent::set_event_handler(Some(move |event: TrayIconEvent| {
                    println!("TrayIconEvent: {:?}", event);
        
                    // Just a static Mutex<bool>
                    let mut visible: bool = true;
        
                    if visible {
                        let window_handle = windows::Win32::Foundation::HWND(handle.hwnd.into());
                        let hide = windows::Win32::UI::WindowsAndMessaging::SW_HIDE;
                        unsafe {
                            windows::Win32::UI::WindowsAndMessaging::ShowWindow(window_handle, hide);
                        }
                        visible = false;
                    } else {
                        let window_handle = windows::Win32::Foundation::HWND(handle.hwnd.into());
                        // You can show the window in all sorts of ways:
                        // https://learn.microsoft.com/en-gb/windows/win32/api/winuser/nf-winuser-showwindow
                        let show = windows::Win32::UI::WindowsAndMessaging::SW_SHOWDEFAULT;
                        unsafe {
                            windows::Win32::UI::WindowsAndMessaging::ShowWindow(window_handle, show);
                        }
                        visible = true;
                    
                }
                }));
            

            Box::<JellyfishApp>::default()
        }),
    )

    /* let monitors: Vec<Screen> = Screen::all().unwrap();

    let width: u32 = monitors[args.monitor].display_info.width;
    let height: u32 = monitors[args.monitor].display_info.height;

    // first, connect to board with neobridge. jelly just calculates what colors are on the monitor.
    // then returns those values to the board.
    let mut neobridge: Neobridge = Neobridge::new(&args.port, args.n_of_leds.try_into().unwrap());
    let mut jelly: engine::JellyRenderer = engine::JellyRenderer::new(
        width,
        height,
        args.n_of_leds,
        args.depth,
        color::ColorOption::new(args.brightness, args.saturation),
        term::CalculationOption::new(args.disable_color_operations),
    );

    // reset LEDs to black.
    neobridge.set_all(RGB(0, 0, 0));
    neobridge.show();

    // start loop here.
    let screen: Screen = monitors[args.monitor];

    loop {
        // don't put image into a separate var, this prevents errors.
        if let Ok(image) = screen.capture_area(
            0,
            height as i32 - ((args.depth + 1) as i32),
            width,
            args.depth as u32,
        ) {
            // first get the colors.
            let colors: &Vec<RGB> = jelly.grab(&image);

            // then send it to the board.
            neobridge.set_list(colors);
            neobridge.show();
        }

        // just so it doesn't put a lot of load onto the CPU.
        thread::sleep(Duration::from_millis(1000 / args.refresh_rate));
    } */
}
