use std::{io::{Read, Write}, sync::{atomic::AtomicBool, Arc}};

use screenshots::Screen;
use serde_json::{Value, json};
use serde::Deserialize;

use crate::{constants::*, gui::JellyfishApp};


#[derive(Deserialize)]
struct GetConfigData {
    pub monitor_index: usize,
    pub number_of_leds: usize,

    pub depth_per_led: usize,
    pub tick_rate: u64,

    pub brightness: f32,
    pub saturation: f32,

    pub port: String,
}

impl JellyfishApp {
    fn retrive_struct_as_json() -> Value {
        json!({
            "monitor_index": DEFAULT_MONITOR_INDEX,
            "number_of_leds": DEFAULT_NUMBER_OF_LEDS,
            
            "depth_per_led": DEFAULT_DEPTH_PER_LED,
            "tick_rate": DEFAULT_TICK_RATE,

            "brightness": DEFAULT_BRIGHTNESS,
            "saturation": DEFAULT_SATURATION,

            "port": DEFAULT_PORT,
        })
    }

    // should be saved as 'config.json'
    pub fn save_struct_to_config(&mut self){
        let mut file = std::fs::File::create(CONFIG_NAME).unwrap();
        let contents: Value;
        if !(std::path::Path::new(CONFIG_NAME).exists()) {
            contents = JellyfishApp::retrive_struct_as_json();
            let mut writer = std::io::BufWriter::new(file);
            let _ = serde_json::to_writer(&mut writer, &contents);
            writer.flush().unwrap();
            return;
        }
        contents = json!({
            "monitor_index": self.monitor_index,
            "number_of_leds": self.number_of_leds,
            
            "depth_per_led": self.depth_per_led,
            "tick_rate": self.tick_rate,

            "brightness": self.brightness,
            "saturation": self.saturation,

            "port": self.port,
        });
        let mut writer = std::io::BufWriter::new(file);
        let _ = serde_json::to_writer(&mut writer, &contents);
        writer.flush().unwrap();
        return;        
    }
}

impl Default for JellyfishApp {
    fn default() -> Self {
        let file: std::fs::File;
        let contents: Value;
        let config: GetConfigData;
        if !(std::path::Path::new(CONFIG_NAME).exists()) {
            file = std::fs::File::create(CONFIG_NAME).unwrap();
            contents = JellyfishApp::retrive_struct_as_json();
            let mut writer = std::io::BufWriter::new(file);
            let _ = serde_json::to_writer(&mut writer, &contents);
            writer.flush().unwrap();
            return JellyfishApp {
                monitors: Screen::all().unwrap(),
                monitor_index: DEFAULT_MONITOR_INDEX,
                number_of_leds: DEFAULT_NUMBER_OF_LEDS,
    
                depth_per_led: DEFAULT_DEPTH_PER_LED,
                tick_rate: DEFAULT_TICK_RATE,
    
                brightness: DEFAULT_BRIGHTNESS,
                saturation: DEFAULT_SATURATION,
    
                port: String::from(DEFAULT_PORT),
                running: Arc::new(AtomicBool::new(false)),
            }
        } else {
            file = std::fs::File::open(CONFIG_NAME).unwrap();
            let mut reader = std::io::BufReader::new(file);
            let mut str = String::new();
            let _ = reader.read_to_string(&mut str);
            config = serde_json::from_str(&str).unwrap();
        }
        return JellyfishApp {
            monitors: Screen::all().unwrap(),
            monitor_index: config.monitor_index,
            number_of_leds: config.number_of_leds,

            depth_per_led: config.depth_per_led,
            tick_rate: config.tick_rate,

            brightness: config.brightness,
            saturation: config.saturation,

            port: String::from(config.port),
            running: Arc::new(AtomicBool::new(false)),
        };
    }
}