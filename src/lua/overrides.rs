use mlua::{Lua, Table};

use crate::{
    error::{Error, Result},
    lua,
};

pub fn print(lua: &Lua) -> Result<()> {
    let function = lua::utils::tracing::info(lua)?;

    lua.globals().set("print", function).map_err(Error::from)
}

pub fn math(lua: &Lua) -> Result<()> {
    let math: Table = lua.globals().get("math")?;

    math.set("vector", crate::math::vector::zeroed())?;
    math.set("color", crate::color::Color::ZERO)?;

    Ok(())
}
