#![windows_subsystem = "windows"]

mod settings;

extern crate sdl2;
#[macro_use]
extern crate strum_macros;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use settings::{Settings, SettingKey};


pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut settings: Settings = Settings::new();

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("PG2 Hardware Test", settings.get(SettingKey::Width) as u32, settings.get(SettingKey::Height) as u32)
        .position(settings.get(SettingKey::XPos) as i32, settings.get(SettingKey::YPos) as i32)
        .resizable()
        .set_window_flags(settings.get(SettingKey::Flags) as u32)
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(100, 0, 100));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.clear();
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }

    settings.save(canvas.window());

    // prefs.insert("xpos".into(), canvas.window().position().0.to_string());
    // prefs.insert("ypos".into(), canvas.window().position().1.to_string());
    // let save_result = prefs.save(&APP_INFO, "settings");
    // assert!(save_result.is_ok());

    Ok(())
}