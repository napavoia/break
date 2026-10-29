use mlua::{Lua, Table};

use crate::error::Result;

pub(super) mod tracing;

macro_rules! public {
    ($module:ident, $name:ident) => {
        pub fn $name(lua: &Lua) -> Result<Table> {
            $module::construct(lua)
        }
    };
}

public!(tracing, tracing);
