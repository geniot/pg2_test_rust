#![windows_subsystem = "windows"]
extern crate sdl2;
extern crate preferences;

use std::collections::HashMap;
use preferences::{AppInfo, PreferencesMap, Preferences};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;


const APP_INFO: AppInfo = AppInfo { name: "pg2_test_rust", author: "geniot" };


pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut prefs: PreferencesMap<String> = PreferencesMap::new();
    prefs.insert("xpos".into(), 200.to_string());
    prefs.insert("ypos".into(), 200.to_string());

    let load_result = PreferencesMap::<String>::load(&APP_INFO, "settings");
    assert!(load_result.is_ok());
    let map: HashMap<String, String> = load_result.unwrap();
    let x_pos_str: Option<&String> = map.get("xpos");
    let y_pos_str: Option<&String> = map.get("ypos");

    let x_pos: i32 = x_pos_str.unwrap().parse().unwrap();
    let y_pos: i32 = y_pos_str.unwrap().parse().unwrap();

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("PG2 Hardware Test", 320, 240)
        .position(x_pos, y_pos)
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
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }

    prefs.insert("xpos".into(), canvas.window().position().0.to_string());
    prefs.insert("ypos".into(), canvas.window().position().1.to_string());
    let save_result = prefs.save(&APP_INFO, "settings");
    assert!(save_result.is_ok());

    Ok(())
}