use mlua::{FromLua, Lua, UserData, Value};

use crate::{Color, impl_const, impl_display, impl_gs};

impl UserData for Color {
    fn add_fields<F: mlua::prelude::LuaUserDataFields<Self>>(fields: &mut F) {
        impl_gs!(getset fields; r is ColorValue);
        impl_gs!(getset fields; g is ColorValue);
        impl_gs!(getset fields; b is ColorValue);
        impl_gs!(getset fields; a is ColorValue);
    }

    fn add_methods<M: mlua::prelude::LuaUserDataMethods<Self>>(methods: &mut M) {
        impl_display!(methods);

        methods.add_function("subpixel", |_, v: Value| match v {
            Value::Integer(i) => Ok(Value::Integer(i.clamp(0, 255))),
            Value::Number(i) => Ok(Value::Integer((i.clamp(0.0, 1.0) * 255.0) as i64)),
            _ => {
                return Err(mlua::Error::UserDataTypeMismatch);
            }
        });

        methods.add_function(
            "rgb",
            |_, (r, g, b): (ColorValue, ColorValue, ColorValue)| {
                Ok(color(r, g, b, ColorValue(0xff)))
            },
        );
        methods.add_function(
            "rgba",
            |_, (r, g, b, a): (ColorValue, ColorValue, ColorValue, ColorValue)| {
                Ok(color(r, g, b, a))
            },
        );
        methods.add_function("invert", |_, color: Color| Ok(color.invert()));

        impl_const!(methods; "white" => WHITE);
        impl_const!(methods; "black" => BLACK);
        impl_const!(methods; "grey" => GREY);
        impl_const!(methods; "red" => RED);
        impl_const!(methods; "green" => GREEN);
        impl_const!(methods; "blue" => BLUE);
        impl_const!(methods; "magenta" => MAGENTA);
        impl_const!(methods; "yellow" => YELLOW);
        impl_const!(methods; "cyan" => CYAN);
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
