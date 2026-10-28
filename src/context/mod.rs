use mlua::Lua;
use sdl2::{Sdl, video::Window as SdlWindow};
use std::path::Path;

pub use self::window::Window;
use crate::{
    error::Result,
    game::{Game, Status},
};

pub mod builder;
pub mod error;
mod window;

pub struct Context {
    game: Shared<Game>,
    lua: Lua,
    _sdl: Sdl,
}

impl Context {
    fn init(_sdl: Sdl, window: SdlWindow) -> Result<Self> {
        let canvas = window.into_canvas().build()?;
        let event = _sdl.event_pump()?;
        let canvas = Window::from(canvas);
        let game = shared(Game::new(canvas, event));

        let lua = Lua::new();
        lua.globals().set("game", game.clone())?;

        Ok(Self { lua, game, _sdl })
    }

    pub fn run<P: AsRef<Path>>(&mut self, main: P) -> Status {
        let result = self.lua.load(main.as_ref()).exec();

        dbg!(&result);

        loop {
            let mut game = self.game.borrow_mut();
            if let Some(runtime) = game.tick() {
                dbg!(&runtime);
                return Some(runtime);
            }
        }
    }
}

use std::{cell::RefCell, rc::Rc};

pub type Shared<T> = Rc<RefCell<T>>;
pub fn shared<T>(v: T) -> Shared<T> {
    Rc::new(RefCell::new(v))
}
