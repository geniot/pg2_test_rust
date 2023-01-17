extern crate strum;

use std::collections::HashMap;
use abserde::*;
use serde::{Serialize, Deserialize};
use std::string::ToString;

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
}

impl Settings {
    pub fn new() -> Settings {
        let my_abserde = Abserde {
            app: "pg2_test_rust".to_string(),
            location: Location::Auto,
            format: Format::Json,
        };

        let mut default_config = MyConfig { user_data: HashMap::new() };
        default_config.user_data.insert(SettingKey::XPos.to_string(), 100.to_string());
        default_config.user_data.insert(SettingKey::YPos.to_string(), 100.to_string());

        let conf = match MyConfig::load_config(&my_abserde) {
            Ok(my_config) => my_config,
            Err(..) => default_config,
        };

        Settings { my_abserde: my_abserde, config: conf }
    }

    pub fn get(&self, setting_key: SettingKey) -> i32 {
        let val_str: Option<&String> = self.config.user_data.get(&*setting_key.to_string());
        val_str.unwrap().parse().unwrap()
    }

    pub fn set(&mut self, setting_key: SettingKey, val: i32) {
        self.config.user_data.insert(setting_key.to_string(), val.to_string());
    }

    pub fn save(&self) {
        self.config.save_config(&self.my_abserde).unwrap();
    }
}