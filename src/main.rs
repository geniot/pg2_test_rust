#![windows_subsystem = "windows"]

mod settings;

extern crate sdl2;
#[macro_use]
extern crate strum_macros;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::image::{InitFlag};
use std::time::Duration;
use settings::{Settings, SettingKey};


pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut settings: Settings = Settings::new();

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let bytes = include_bytes!("res/pg2_test_rust.png");
    let _image_context = sdl2::image::init(InitFlag::PNG)?;
    let rw_ops = sdl2::rwops::RWops::from_bytes(bytes).unwrap();
    let texture = sdl2::image::ImageRWops::load_png(&rw_ops).unwrap();


    let mut window = video_subsystem
        .window("PG2 Hardware Test", settings.get(SettingKey::Width) as u32, settings.get(SettingKey::Height) as u32)
        .position(settings.get(SettingKey::XPos) as i32, settings.get(SettingKey::YPos) as i32)
        .resizable()
        .set_window_flags(settings.get(SettingKey::Flags) as u32)
        .build()
        .map_err(|e| e.to_string())?;
    window.set_icon(texture);

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

    Ok(())
}