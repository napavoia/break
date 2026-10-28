mod color;
mod context;
mod error;
mod lua;

pub mod game;
pub mod math;
pub use color::Color;

pub use context::{Context, builder::*};
use game::Status;
use std::path::Path;

pub fn run<P: AsRef<Path>>(ctx: &mut Context, script: P) -> Status {
    ctx.run(script)
}
