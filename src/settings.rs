extern crate strum;

use std::collections::HashMap;
use abserde::*;
use serde::{Serialize, Deserialize};
use std::string::ToString;
use sdl2::sys::SDL_WindowFlags;
use sdl2::video::Window;
use SettingKey::*;

pub struct Settings {
    my_abserde: Abserde,
    config: MyConfig,
}

#[derive(Serialize, Deserialize)]
struct MyConfig {
    user_data: HashMap<String, String>,
}

#[derive(Display)]
pub enum SettingKey {
    XPos = 0x00,
    YPos = 0x01,
    Width = 0x02,
    Height = 0x03,
    Flags = 0x04,
}

impl Settings {
    pub fn new() -> Settings {
        let my_abserde = Abserde {
            app: "pg2_test_rust".to_string(),
            location: Location::Auto,
            format: Format::Json,
        };

        let mut conf = match MyConfig::load_config(&my_abserde) {
            Ok(my_config) => my_config,
            Err(..) => MyConfig { user_data: HashMap::new() },
        };

        let mut default_window_flags: u32 = SDL_WindowFlags::SDL_WINDOW_SHOWN as u32;
        default_window_flags |= SDL_WindowFlags::SDL_WINDOW_RESIZABLE as u32;

        conf.user_data.entry(XPos.to_string()).or_insert_with(|| 100.to_string());
        conf.user_data.entry(YPos.to_string()).or_insert_with(|| 100.to_string());
        conf.user_data.entry(Width.to_string()).or_insert_with(|| 320.to_string());
        conf.user_data.entry(Height.to_string()).or_insert_with(|| 240.to_string());
        conf.user_data.entry(Flags.to_string()).or_insert_with(|| default_window_flags.to_string());

        Settings { my_abserde, config: conf }
    }

    pub fn get(&self, setting_key: SettingKey) -> u32 {
        let val_str: Option<&String> = self.config.user_data.get(&*setting_key.to_string());
        val_str.unwrap().parse().unwrap()
    }

    pub fn set(&mut self, setting_key: SettingKey, val: u32) {
        self.config.user_data.insert(setting_key.to_string(), val.to_string());
    }

    pub fn save(mut self, window: &Window) {
        if window.window_flags() as u32 & SDL_WindowFlags::SDL_WINDOW_MAXIMIZED as u32 <= 0 {
            self.set(XPos, window.position().0 as u32);
            self.set(YPos, window.position().1 as u32);
            self.set(Width, window.size().0);
            self.set(Height, window.size().1);
        }
        self.set(Flags, window.window_flags());

        self.config.save_config(&self.my_abserde).unwrap();
    }
}