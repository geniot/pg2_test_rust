extern crate strum;

use std::collections::HashMap;
use abserde::*;
use serde::{Serialize, Deserialize};
use std::string::ToString;
use pix_engine::prelude::PixState;
use SettingKey::*;
use strum_macros::Display;

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

        conf.user_data.entry(XPos.to_string()).or_insert_with(|| 100.to_string());
        conf.user_data.entry(YPos.to_string()).or_insert_with(|| 100.to_string());
        conf.user_data.entry(Width.to_string()).or_insert_with(|| 320.to_string());
        conf.user_data.entry(Height.to_string()).or_insert_with(|| 240.to_string());

        Settings { my_abserde, config: conf }
    }

    pub fn get(&self, setting_key: SettingKey) -> Option<&String> {
        self.config.user_data.get(&*setting_key.to_string())
    }

    pub fn set(&mut self, setting_key: SettingKey, val: String) {
        self.config.user_data.insert(setting_key.to_string(), val.to_string());
    }

    pub fn save(&mut self, state: &mut PixState) {
        self.set(Width, state.window_width().unwrap().to_string());
        self.set(Height, state.window_height().unwrap().to_string());
        self.set(XPos, state.window_position_x().unwrap().to_string());
        self.set(YPos, state.window_position_y().unwrap().to_string());

        self.config.save_config(&self.my_abserde).unwrap();
    }
}