extern crate strum;

use std::collections::HashMap;
use std::ops::Deref;
use abserde::*;
use serde::{Serialize, Deserialize};
use std::string::ToString;
use SettingKey::*;
use strum_macros::Display;
use crate::state::PixState;

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
        let (width, height) = state.canvas.window().size();
        self.set(Width, width.to_string());
        self.set(Height, height.to_string());

        self.config.save_config(&self.my_abserde).unwrap();
    }
}