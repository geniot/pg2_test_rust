#![windows_subsystem = "windows"]

mod settings;
mod fps_counter;

extern crate sdl2;
#[macro_use]
extern crate strum_macros;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
// use sdl2::image::{InitFlag};
use settings::{Settings, SettingKey};
use fps_counter::{FpsCounter};


pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings: Settings = Settings::new();

    let sdl_context = sdl2::init()?;
    // let image_context = sdl2::image::init(InitFlag::PNG)?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let video_subsystem = sdl_context.video()?;
    let timer_subsystem = sdl_context.timer()?;

    let icon_bytes = include_bytes!("res/pg2_test_rust.png");
    let font_bytes = include_bytes!("res/pixelberry.ttf");

    let icon_rw_ops = sdl2::rwops::RWops::from_bytes(icon_bytes).unwrap();
    let font_rw_ops = sdl2::rwops::RWops::from_bytes(font_bytes).unwrap();

    let texture = sdl2::image::ImageRWops::load_png(&icon_rw_ops).unwrap();
    let mut font = ttf_context.load_font_from_rwops(font_rw_ops, 18)?;
    // font.set_style(sdl2::ttf::FontStyle::BOLD);

    let mut fps_counter = FpsCounter::new(timer_subsystem.ticks());


    let mut window = video_subsystem
        .window("PG2 Hardware Test", settings.get(SettingKey::Width) as u32,
                settings.get(SettingKey::Height) as u32)
        .position(settings.get(SettingKey::XPos) as i32,
                  settings.get(SettingKey::YPos) as i32)
        .resizable()
        .set_window_flags(settings.get(SettingKey::Flags) as u32)
        .build()
        .map_err(|e| e.to_string())?;
    window.set_icon(texture);

    let mut canvas = window.into_canvas().present_vsync().build().map_err(|e| e.to_string())?;

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

        canvas.set_draw_color(Color::RGB(100, 0, 100));
        canvas.clear();
        fps_counter.render(timer_subsystem.ticks(), &mut font, &mut canvas);
        canvas.present();
    }

    settings.save(canvas.window());

    Ok(())
}