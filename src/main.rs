#![windows_subsystem = "windows"]

mod settings;
mod engine;
mod error;
mod state;
mod event;

use crate::engine::{Engine, PixEngine};
use crate::error::PixResult;
use crate::event::{Key, KeyEvent};
use crate::settings::{SettingKey, Settings};
use crate::state::PixState;

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
        s.canvas.window_mut().show();
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
            Key::Escape | Key::Home => s.quit = true,
            _ => (),
        }
        Ok(false)
    }
}

fn main() -> PixResult<()> {
    let mut app = Pg2Test::new();
    let mut engine = Engine::builder().build()?;
    engine.run(&mut app)
}
