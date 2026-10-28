use mlua::{Lua, MultiValue, ObjectLike, Table, UserData, Value};

use crate::{
    Color,
    game::{Game, Index, InnerFunctionEvent},
};

impl UserData for Game {
    fn add_fields<F: mlua::prelude::LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("window", |_, this| Ok(this.canvas.clone()));
        fields.add_field_method_get("color", |_, _| Ok(Color::WHITE));
        fields.add_field_method_get("inner_events", |lua, _| inner_function(lua));
    }

    fn add_methods<M: mlua::prelude::LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("instantiate", |_, this, local: Table| {
            Ok(this.instantiate(local))
        });

        methods.add_method_mut("hook", |_, this, (obj, name): (Index, String)| {
            let _: () = this.hook(obj, name);
            Ok(())
        });

        methods.add_method_mut("call", |_, this, (name, args): (String, MultiValue)| {
            let _: () = this.call(&name, args);
            Ok(())
        });
    }

    fn register(registry: &mut mlua::prelude::LuaUserDataRegistry<Self>) {
        Self::add_fields(registry);
        Self::add_methods(registry);
    }
}

pub fn inner_function(lua: &Lua) -> mlua::prelude::LuaResult<Value> {
    let table = lua.create_table()?;

    for el in InnerFunctionEvent::array() {
        table.push(el.name())?;
    }

    Ok(table.to_value())
}
