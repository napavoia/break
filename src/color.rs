use sdl2::pixels::Color as SdlColor;

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Color(pub SdlColor);

impl Color {
    #[inline]
    #[allow(non_snake_case)]
    pub const fn RGB(r: u8, g: u8, b: u8) -> Color {
        Color(SdlColor::RGB(r, g, b))
    }

    #[inline]
    #[allow(non_snake_case)]
    pub const fn RGBA(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color(SdlColor::RGBA(r, g, b, a))
    }

    pub fn invert(self) -> Color {
        Color(self.0.invert())
    }

    #[inline]
    pub const fn rgb(self) -> (u8, u8, u8) {
        self.0.rgb()
    }

    #[inline]
    pub const fn rgba(self) -> (u8, u8, u8, u8) {
        self.0.rgba()
    }

    pub const WHITE: Color = Color::RGBA(255, 255, 255, 255);
    pub const BLACK: Color = Color::RGBA(0, 0, 0, 255);
    pub const GRAY: Color = Color::RGBA(128, 128, 128, 255);
    pub const GREY: Color = Color::GRAY;
    pub const RED: Color = Color::RGBA(255, 0, 0, 255);
    pub const GREEN: Color = Color::RGBA(0, 255, 0, 255);
    pub const BLUE: Color = Color::RGBA(0, 0, 255, 255);
    pub const MAGENTA: Color = Color::RGBA(255, 0, 255, 255);
    pub const YELLOW: Color = Color::RGBA(255, 255, 0, 255);
    pub const CYAN: Color = Color::RGBA(0, 255, 255, 255);
}

impl From<Color> for sdl2::pixels::Color {
    fn from(val: Color) -> Self {
        val.0
    }
}
