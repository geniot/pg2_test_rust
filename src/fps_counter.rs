use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::ttf::Font;

pub struct FpsCounter {
    start_ticks: u32,
    current_second: u32,
    frame_count: u32,
    current_fps: u32,
}

impl FpsCounter {
    pub fn new(t: u32) -> FpsCounter {
        FpsCounter {
            start_ticks: t,
            current_second: t / 1000,
            frame_count: 0,
            current_fps: 0,
        }
    }

    pub fn render(&mut self, current_ticks: u32, font: &mut Font, canvas: &mut WindowCanvas) {
        self.frame_count += 1;
        let mut new_fps = 0;
        if current_ticks != 0 {
            let ticks_delta = current_ticks - self.start_ticks;
            let frame_duration = ticks_delta / self.frame_count;
            if frame_duration != 0 {
                new_fps = 1000 / frame_duration;
            }
        }

        let sec = current_ticks / 1000;
        if sec > self.current_second {
            self.current_fps = new_fps;
            self.frame_count = 0;
            self.start_ticks = current_ticks;
            self.current_second = sec;
        }

        let text_surface = font
            .render(&*self.current_fps.to_string())
            .blended(Color::RGBA(255, 0, 0, 255))
            .map_err(|e| e.to_string()).unwrap();
        let texture_creator = canvas.texture_creator();
        let text_texture = texture_creator
            .create_texture_from_surface(&text_surface)
            .map_err(|e| e.to_string()).unwrap();
        let rect = Rect::new(0, 0, text_surface.width(), text_surface.height());
        canvas.copy(&text_texture, None, rect).unwrap();
    }
}