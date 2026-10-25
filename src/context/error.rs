use sdl2::{IntegerOrSdlError, video::WindowBuildError};

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Error {
    HeightOverflows(u32),
    WidthOverflows(u32),
    IntegerOverflows(&'static str, u32),
    InvalidTitle,
    /// Cannot initialize `Sdl` from more than one thread.
    MultipleInit,
    SdlError(String),
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Cannot initialize `Sdl` from more than one thread." => Error::MultipleInit,
            _ => Error::SdlError(value),
        }
    }
}

impl From<WindowBuildError> for Error {
    fn from(value: WindowBuildError) -> Self {
        match value {
            WindowBuildError::HeightOverflows(v) => Self::HeightOverflows(v),
            WindowBuildError::WidthOverflows(v) => Self::WidthOverflows(v),
            WindowBuildError::InvalidTitle(_) => Self::InvalidTitle,
            WindowBuildError::SdlError(v) => Self::SdlError(v),
        }
    }
}

impl From<IntegerOrSdlError> for Error {
    fn from(value: IntegerOrSdlError) -> Self {
        match value {
            IntegerOrSdlError::IntegerOverflows(v1, v2) => Self::IntegerOverflows(v1, v2),
            IntegerOrSdlError::SdlError(v) => Self::SdlError(v),
        }
    }
}
