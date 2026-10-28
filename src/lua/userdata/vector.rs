use std::sync::Arc;

use mlua::{FromLua, IntoLua, Lua, MultiValue, UserData, Value};

use crate::math::{Axis, Vector, num::Number};

impl<T: Number, const N: usize> UserData for Vector<T, N> {
    fn add_fields<F: mlua::prelude::LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("x", |lua, this| get(lua, this, Axis::X));
        fields.add_field_method_get("y", |lua, this| get(lua, this, Axis::Y));
        fields.add_field_method_get("z", |lua, this| get(lua, this, Axis::Z));
        fields.add_field_method_get("w", |lua, this| get(lua, this, Axis::W));

        fields.add_field_method_set("x", |_, this, v: Value| set(this, Axis::X, v));
        fields.add_field_method_set("y", |_, this, v: Value| set(this, Axis::Y, v));
        fields.add_field_method_set("z", |_, this, v: Value| set(this, Axis::Z, v));
        fields.add_field_method_set("w", |_, this, v: Value| set(this, Axis::W, v));
    }

    fn add_methods<M: mlua::prelude::LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new2d", |_, values: MultiValue| new::<f64, 2>(values));
        methods.add_function("new3d", |_, values: MultiValue| new::<f64, 3>(values));
        methods.add_function("new4d", |_, values: MultiValue| new::<f64, 4>(values));
    }

    fn register(registry: &mut mlua::prelude::LuaUserDataRegistry<Self>) {
        Self::add_fields(registry);
        Self::add_methods(registry);
    }
}

impl<T: Number, const N: usize> FromLua for Vector<T, N> {
    fn from_lua(value: Value, _: &Lua) -> mlua::prelude::LuaResult<Self> {
        let table = match value {
            Value::Table(table) => table,
            _ => return Err(mlua::Error::UserDataTypeMismatch),
        };

        let len = table.raw_len();

        if N > len {
            return Err(mlua::Error::UserDataTypeMismatch);
        }

        let mut data = [T::default(); N];
        (0..len)
            .try_for_each(|i| match table.pop::<Value>() {
                Ok(value) => T::try_from(value).map(|v| {
                    data[i] = v;
                }),
                Err(err) => Err(err),
            })
            .map(|_| Vector::from(data))
    }
}

fn new<T: Number, const N: usize>(values: MultiValue) -> mlua::Result<Vector<T, N>> {
    let mut data = [T::default(); N];
    let result = values.into_iter().enumerate().try_for_each(|(i, v)| {
        T::try_from(v).map(|v| {
            data[i] = v;
        })
    });

    result.map(|_| Vector::from(data))
}

fn set<T: Number, const N: usize>(
    vec: &mut Vector<T, N>,
    idx: impl Into<usize>,
    val: Value,
) -> mlua::Result<()> {
    let value = T::try_from(val)?;

    vec.try_set(idx.into(), value)
        .map_err(|err| mlua::Error::ExternalError(Arc::new(err)))
}

fn get<T: Number, const N: usize>(
    lua: &Lua,
    vec: &Vector<T, N>,
    idx: impl Into<usize>,
) -> mlua::Result<Value> {
    match vec.get(idx.into()) {
        Some(number) => number.into_lua(lua),
        None => Value::Nil.into_lua(lua),
    }
}
