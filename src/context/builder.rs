use crate::context::Context;
use crate::error::Error;
use crate::math::Axis::*;
use crate::vector;
use crate::{context::error::BuilderError, math::Size};
use sdl2::VideoSubsystem;
use sdl2::video::Window;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ContextBuilder {
    window: WindowBuilder,
}

impl ContextBuilder {
    pub fn window(mut self, builder: WindowBuilder) -> Self {
        self.window = builder;
        self
    }

    pub fn build(self) -> Result<Context, Error> {
        let sdl = sdl2::init()?;

        let video = sdl.video()?;

        let window = self.window.build(&video)?;

        Context::init(sdl, window)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowBuilder {
    pub title: String,
    pub position: Option<Size<i32>>,
    pub size: Size<u32>,
    pub min_size: Option<Size<u32>>,
    pub max_size: Option<Size<u32>>,
    pub resizable: bool,
    pub visible: bool,
    pub decorations: bool,
    pub transparent: bool,
    pub fullscreen: bool,
}

impl WindowBuilder {
    fn build(self, video: &VideoSubsystem) -> Result<Window, BuilderError> {
        let mut window = video.window(&self.title, self.size[X], self.size[Y]);
        window.position_centered();

        if let Some(pos) = self.position {
            window.position(pos[X], pos[Y]);
        }

        self.fullscreen.then(|| {
            window.fullscreen();
            Some(())
        });

        (!self.visible).then(|| {
            window.hidden();
            Some(())
        });

        self.resizable.then(|| {
            window.resizable();
            Some(())
        });

        let mut window = window.build()?;

        self.max_size
            .map(|size| window.set_maximum_size(size[X], size[Y]))
            .transpose()?;

        self.min_size
            .map(|size| window.set_minimum_size(size[X], size[Y]))
            .transpose()?;

        window.set_bordered(self.decorations);

        Ok(window)
    }
}

impl Default for WindowBuilder {
    fn default() -> Self {
        Self {
            title: String::from("break window"),
            position: None,
            size: vector!(800, 600),
            min_size: None,
            max_size: None,
            resizable: true,
            visible: true,
            decorations: true,
            transparent: false,
            fullscreen: false,
        }
    }
}

macro_rules! impl_setters {
    ($builder:ty; $(
        $(#[$meta:meta])*
        $name:ident : $ty:ty => $field:ident
    ),+ $(,)?) => {
        impl $builder {
            $(
                $(#[$meta])*
                pub fn $name(mut self, value: $ty) -> Self {
                    self.$field = value.into();
                    self
                }
            )+
        }
    };
}

impl_setters!(WindowBuilder;
    /// Sets the window title.
    title: impl Into<String> => title,
    /// Sets the window size.
    size: impl Into<Size<u32>> => size,
    /// Sets the window size.
    min_size: impl Into<Option<Size<u32>>> => min_size,
    /// Sets the window size.
    max_size: impl Into<Option<Size<u32>>> => max_size,
    /// Enables or disables resizing.
    resizable: bool => resizable,
    /// Shows or hides the window.
    visible: bool => visible,
    /// Enables or disables decorations.
    decorations: bool => decorations,
    /// Enables or disables transparency.
    transparent: bool => transparent,
    /// Enables or disables fullscreen.
    fullscreen: bool => fullscreen,
);
