use mlua::{Function, Lua, MultiValue, ObjectLike, Table};
use tracing::Level;

use crate::error::{Error, Result};

pub fn construct(lua: &Lua) -> Result<Table> {
    let table = lua.create_table()?;

    table.set("info", info(lua)?)?;
    table.set("debug", debug(lua)?)?;
    table.set("warn", warn(lua)?)?;
    table.set("error", error(lua)?)?;

    table.set("debug_mode", cfg!(debug_assertions))?;

    Ok(table)
}

macro_rules! lua_tracing {
    ($lua:ident, $lvl:expr) => {
        $lua.create_function(|lua: &Lua, args: MultiValue| {
            let mut result = String::with_capacity(args.len() * 3);
            for arg in args {
                result.push_str(&(lua.globals().call_function::<String>("tostring", arg)?));
                result.push('\t');
            }
            tracing::event!(target: "Lua", $lvl, "{result}");
            Ok(())
        })
    };

    (fn $name:ident, $lvl:expr) => {
        pub fn $name(lua: &Lua) -> Result<Function> {
            lua_tracing!(lua, $lvl).map_err(Error::from)
        }
    };
}

lua_tracing!(fn error, Level::ERROR);
lua_tracing!(fn warn, Level::WARN);
lua_tracing!(fn info, Level::INFO);

pub fn debug(lua: &Lua) -> Result<Function> {
    cfg_select! {
        debug_assertions => {
            lua_tracing!(lua, Level::DEBUG)
        }
        _ => {
            lua.create_function(|lua: &Lua, args: MultiValue| { Ok(()) })
        }
    }
    .map_err(Error::from)
}
