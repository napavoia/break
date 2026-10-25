mod context;
pub mod math;
pub mod prelude;

use prelude::*;

pub fn run(ctx: Context) {
    println!("Hello, world!");
    ctx.run();
}
