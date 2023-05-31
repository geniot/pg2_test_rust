use sdl2::pixels::Color;
use crate::error::PixResult;
use crate::event::KeyEvent;
use crate::state::PixState;
use log::{debug, error, info};

#[allow(unused_variables)]
pub trait PixEngine {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        Ok(())
    }
    fn on_update(&mut self, s: &mut PixState) -> PixResult<()>;
    fn on_stop(&mut self, s: &mut PixState) -> PixResult<()> {
        Ok(())
    }
    fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
        Ok(false)
    }
    fn on_key_released(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
        Ok(false)
    }
}

pub struct Engine {
    state: PixState,
}

impl Engine {
    pub fn builder() -> EngineBuilder {
        EngineBuilder::default()
    }

    pub fn run<A>(&mut self, app: &mut A) -> PixResult<()>
        where
            A: PixEngine,
    {
        self.state.canvas.set_draw_color(Color::BLACK);
        self.state.canvas.clear();

        let on_start = app.on_start(&mut self.state);
        if on_start.is_err() || self.state.quit {
            debug!("Quitting during startup with `Engine::on_stop`");
            if let Err(ref err) = on_start {
                error!("Error: {}", err);
            }
            return app.on_stop(&mut self.state).and(on_start);
        }
        self.state.canvas.present();

        'on_stop: loop {
            let result = 'running: loop {
                self.handle_events(app)?;
                if self.state.quit {
                    break 'running Ok(());
                }
                self.state.canvas.set_draw_color(Color::BLACK);
                self.state.canvas.clear();
                self.state.canvas.present();
            };
            let on_stop = app.on_stop(&mut self.state);
            if self.state.quit {
                break 'on_stop on_stop.and(result);
            }
        }
    }

    #[inline]
    fn handle_events<A>(&mut self, app: &mut A) -> PixResult<()>
        where
            A: PixEngine,
    {
        let state = &mut self.state;
        while let Some(event) = state.event_pump.poll_event() {}
        Ok(())
    }
}

#[must_use]
#[derive(Debug)]
pub struct EngineBuilder {}

impl Default for EngineBuilder {
    fn default() -> Self {
        Self {}
    }
}

impl EngineBuilder {
    /// Constructs a `EngineBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> PixResult<Engine> {
        Ok(Engine {
            state: PixState::new()?,
        })
    }
}