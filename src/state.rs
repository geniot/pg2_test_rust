use sdl2::EventPump;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::error::{Error, PixResult};


pub struct PixState {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub quit: bool,
}

impl PixState {
    #[inline]
    pub(crate) fn new() -> PixResult<Self> {
        let sdl2_context = sdl2::init().map_err(Error::Renderer)?;
        let event_pump = sdl2_context.event_pump().map_err(Error::Renderer)?;
        let video_subsys = sdl2_context.video().map_err(Error::Renderer)?;
        let mut window_builder = video_subsys.window("some", 200, 300);
        let window = window_builder.build()?;
        let mut canvas_builder = window.into_canvas().accelerated().target_texture();
        canvas_builder = canvas_builder.present_vsync();
        let mut canvas = canvas_builder.build()?;
        canvas.set_logical_size(200, 300)?;

        let mut state = Self {
            canvas,
            event_pump,
            quit: false,
        };
        Ok(state)
    }
}

