use std::time::Duration;

use sdl2::{Sdl, event::Event, keyboard::Keycode, pixels::Color, render::Canvas, video::Window};

pub mod builder;
pub mod error;

pub struct Context {
    canvas: Canvas<Window>,
    sdl: Sdl,
}

impl Context {
    pub fn run(mut self) {
        let mut event_pump = self.sdl.event_pump().unwrap();

        'running: loop {
            // Handle events
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

            // Render
            self.canvas.set_draw_color(Color::RGB(0, 255, 255));
            self.canvas.clear();
            self.canvas.present();

            // Control frame rate (approx 60 FPS)
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
