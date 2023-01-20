#![windows_subsystem = "windows"]

mod settings;

use pix_engine::prelude::*;
use crate::settings::{SettingKey, Settings};

struct Pg2Test {
    settings: Settings,
}

impl Pg2Test {
    fn new() -> Self {
        Self { settings: Settings::new() }
    }
}

impl PixEngine for Pg2Test {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        s.show_window().unwrap();
        Ok(())
    }

    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        Ok(())
    }

    fn on_stop(&mut self, s: &mut PixState) -> PixResult<()> {
        self.settings.save(s);
        Ok(())
    }

    fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
        match event.key {
            Key::Escape | Key::Home => s.quit(),
            _ => (),
        }
        Ok(false)
    }
}

fn main() -> PixResult<()> {
    let mut app = Pg2Test::new();
    let mut engine = Engine::builder()
        .dimensions(app.settings.get(SettingKey::Width).unwrap().parse().unwrap(),
                    app.settings.get(SettingKey::Height).unwrap().parse().unwrap())
        .position(app.settings.get(SettingKey::XPos).unwrap().parse().unwrap(),
                  app.settings.get(SettingKey::YPos).unwrap().parse().unwrap())
        .vsync_enabled()
        .resizable()
        .title("PG2 Hardware Test")
        .show_frame_rate()
        .hidden()
        .build()?;

    engine.run(&mut app)
}
