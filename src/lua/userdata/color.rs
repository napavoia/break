use mlua::{FromLua, Lua, UserData, Value};

use crate::{Color, consts, getset};

impl UserData for Color {
    fn add_fields<F: mlua::prelude::LuaUserDataFields<Self>>(fields: &mut F) {
        getset!(getset fields; r is ColorValue);
        getset!(getset fields; g is ColorValue);
        getset!(getset fields; b is ColorValue);
        getset!(getset fields; a is ColorValue);
    }

    fn add_methods<M: mlua::prelude::LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method(
            "rgb",
            |_, _, (r, g, b): (ColorValue, ColorValue, ColorValue)| {
                Ok(color(r, g, b, ColorValue(0xff)))
            },
        );
        methods.add_method(
            "rgba",
            |_, _, (r, g, b, a): (ColorValue, ColorValue, ColorValue, ColorValue)| {
                Ok(color(r, g, b, a))
            },
        );
        methods.add_method("invert", |_, this, _: Value| Ok(this.invert()));

        consts!(methods; WHITE);
        consts!(methods; BLACK);
        consts!(methods; GREY);
        consts!(methods; RED);
        consts!(methods; GREEN);
        consts!(methods; BLUE);
        consts!(methods; MAGENTA);
        consts!(methods; YELLOW);
        consts!(methods; CYAN);
    }

    fn register(registry: &mut mlua::prelude::LuaUserDataRegistry<Self>) {
        Self::add_fields(registry);
        Self::add_methods(registry);
    }
}

impl FromLua for Color {
    fn from_lua(value: Value, _: &Lua) -> mlua::prelude::LuaResult<Self> {
        let table = match value {
            Value::UserData(userdata) => return userdata.borrow::<Color>().map(|v| *v),
            Value::Table(table) => table,
            _ => return Err(mlua::Error::UserDataTypeMismatch),
        };

        let r: ColorValue = table.get("r")?;
        let g: ColorValue = table.get("g")?;
        let b: ColorValue = table.get("b")?;
        let a: ColorValue = table.get("a")?;

        Ok(color(r, g, b, a))
    }
}

fn color(r: ColorValue, g: ColorValue, b: ColorValue, a: ColorValue) -> Color {
    Color::RGBA(r.0, g.0, b.0, a.0)
}

#[derive(Debug, Clone, Copy)]
struct ColorValue(pub u8);
impl FromLua for ColorValue {
    fn from_lua(value: Value, _: &Lua) -> mlua::prelude::LuaResult<Self> {
        match value {
            Value::Integer(v) => Ok(Self(v.clamp(0, 255) as u8)),
            Value::Number(v) => Ok(Self((v.clamp(0.0, 1.0) * 255.0) as u8)),
            _ => Err(mlua::Error::UserDataTypeMismatch),
        }
    }
}
