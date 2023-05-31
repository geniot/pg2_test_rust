use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use rand::prelude::*;

pub struct Emerald {}

impl Emerald {
    pub fn new() -> Emerald {
        Emerald {}
    }

    pub fn render(&self, rng: &mut ThreadRng, texture: &Texture, canvas: &mut WindowCanvas) {
        let rect = Rect::new(rng.gen_range(20..300),
                             rng.gen_range(20..220),
                             20, 20);
        canvas.copy(texture, None, rect).unwrap();
    }
}