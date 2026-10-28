use std::mem::transmute;

use sdl2::{
    pixels::Color,
    render::WindowCanvas,
    surface::SurfaceRef,
    video::{FullscreenType, WindowPos},
};

use crate::{
    error::{Error, Result},
    math::{
        Axis::{X, Y},
        Vector2,
    },
};

pub struct Window {
    pub inner: WindowCanvas,
    pub color: Color,
}

macro_rules! inner_fn {
    ($name:ident($($v:ident: $t:ty),*) $(-> $out:ty)?) => {
        pub fn $name(&self $(,$v: $t)*) $(-> $out)? {
            self.inner.$name($($v),*)
        }
    };
    (mut $name:ident($($v:ident: $t:ty),*) $(-> $out:ty)?) => {
        pub fn $name(&mut self $(,$v: $t)*) $(-> $out)? {
            self.inner.$name($($v),*)
        }
    };

    (window $name:ident($($v:ident: $t:ty),*) $(-> $out:ty)?) => {
        pub fn $name(&self $(,$v: $t)*) $(-> $out)? {
            self.inner.window().$name($($v),*)
        }
    };

    (mut window $name:ident($($v:ident: $t:ty),*) $(-> $out:ty)?) => {
        pub fn $name(&mut self $(,$v: $t)*) $(-> $out)? {
            self.inner.window_mut().$name($($v),*)
        }
    };

    (mut window:map_err $name:ident($($v:ident: $t:ty),*) -> $out:ty) => {
        pub fn $name(&mut self $(,$v: $t)*) -> Result<$out> {
            self.inner.window_mut().$name($($v),*).map_err(Error::from)
        }
    };
}

impl Window {
    pub fn clear(&mut self) {
        self.inner.set_draw_color(self.color);
        self.inner.clear();
    }
}

#[allow(dead_code)]
impl Window {
    inner_fn!(mut present());
    inner_fn!(window size() -> (u32, u32));
    inner_fn!(window is_always_on_top() -> bool);
    inner_fn!(window is_maximized() -> bool);
    inner_fn!(window is_minimized() -> bool);
    inner_fn!(mut window show());
    inner_fn!(mut window hide());
    inner_fn!(mut window raise());
    inner_fn!(mut window maximize());
    inner_fn!(mut window minimize());
    inner_fn!(mut window restore());

    inner_fn!(mut window:map_err set_size(w: u32, h:u32) -> ());
    inner_fn!(mut window set_bordered(bordered: bool));
    inner_fn!(mut window set_resizable(resizable: bool));
    inner_fn!(mut window:map_err set_title(title: &str) -> ());
    // #need impl userdata
    inner_fn!(mut window set_icon(icon: impl AsRef<SurfaceRef>));
    // #end
    pub fn set_fullscreen(&mut self, flag: bool) -> Result<()> {
        self.inner
            .window_mut()
            .set_fullscreen(unsafe { transmute::<i16, FullscreenType>(flag as i16) })
            .map_err(Error::from)
    }

    pub fn set_position(&mut self, position: Vector2<i64>) {
        self.inner.window_mut().set_position(
            WindowPos::Positioned(position[X] as _),
            WindowPos::Positioned(position[Y] as _),
        )
    }

    pub fn set_color(&mut self, color: impl Into<Color>) {
        self.color = color.into();
    }

    #[allow(dead_code, unused_must_use)]
    fn test(&mut self) {}
}

impl From<WindowCanvas> for Window {
    fn from(value: WindowCanvas) -> Self {
        Self {
            inner: value,
            color: Color::WHITE,
        }
    }
}
