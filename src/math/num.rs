use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Neg, Sub},
};

use mlua::{IntoLua, Value};

pub trait Number: Add + Sub + Div + Mul + Sized + Copy + Neg + IntoLua + Default + Debug {
    fn type_name() -> &'static str;
    fn try_from(value: Value) -> Result<Self, mlua::Error>;
}

macro_rules! impl_number {
    ($t:tt) => {
        impl Number for $t {
            fn type_name() -> &'static str {
                stringify!($t)
            }

            fn try_from(value: Value) -> Result<Self, mlua::Error> {
                match value {
                    Value::Integer(n) => Ok(n as $t),
                    Value::Number(n) => Ok(n as $t),
                    _ => Err(mlua::Error::FromLuaConversionError {
                        from: value.type_name(),
                        to: $t::type_name().to_string(),
                        message: None,
                    }),
                }
            }
        }
    };
}

impl_number!(f64);
impl_number!(i64);
