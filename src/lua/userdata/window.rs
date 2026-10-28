use crate::{add_method, color::Color, context::Window, math::Vector2};
use mlua::{ErrorContext, MultiValue, UserData};

impl UserData for Window {
    fn add_fields<F: mlua::prelude::LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("is_always_on_top", |_, this| Ok(this.is_always_on_top()));
        fields.add_field_method_get("is_maximized", |_, this| Ok(this.is_maximized()));
        fields.add_field_method_get("is_minimized", |_, this| Ok(this.is_minimized()));
        fields.add_field_method_get("size", |lua, this| {
            let (w, h) = this.size();
            let table = lua.create_table()?;
            table.set("width", w)?;
            table.set("height", h)?;

            Ok(table)
        });
    }

    fn add_methods<M: mlua::prelude::LuaUserDataMethods<Self>>(methods: &mut M) {
        add_method!(methods;; set_fullscreen(flag: bool));
        add_method!(methods;; set_size(w: u32, h: u32));
        add_method!(methods;; set_bordered(bordered: bool) -> Ok);
        add_method!(methods;; set_resizable(resizable: bool) -> Ok);
        add_method!(methods;; set_position(position: Vector2<i64>) -> Ok);
        add_method!(methods;; set_color(color: Color) -> Ok);
        add_method!(methods; show);
        add_method!(methods; hide);
        add_method!(methods; raise);
        add_method!(methods; maximize);
        add_method!(methods; minimize);
        add_method!(methods; restore);

        // todo
        add_method!(methods; "set_icon");

        methods.add_method_mut("set_title", |_, this, (title,): (String,)| {
            this.set_title(&title)
                .map_err(|e| e.external().context("set_title"))
        });
    }

    fn register(registry: &mut mlua::prelude::LuaUserDataRegistry<Self>) {
        Self::add_fields(registry);
        Self::add_methods(registry);
    }
}
