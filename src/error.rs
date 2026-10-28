use std::{
    ffi::NulError,
    fmt::{Display, Pointer},
    io,
    sync::Arc,
};

use sdl2::IntegerOrSdlError;

use crate::context::error::BuilderError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    Builder(BuilderError),
    Lua(mlua::Error),
    IO {
        kind: io::ErrorKind,
        message: String,
    },
    Other(String),
}

impl From<io::Error> for Error {
    fn from(val: io::Error) -> Self {
        Error::IO {
            kind: val.kind(),
            message: val.to_string(),
        }
    }
}

impl From<mlua::Error> for Error {
    fn from(val: mlua::Error) -> Self {
        Error::Lua(val)
    }
}

impl From<BuilderError> for Error {
    fn from(val: BuilderError) -> Self {
        Error::Builder(val)
    }
}

impl From<NulError> for Error {
    fn from(_: NulError) -> Self {
        Error::Builder(BuilderError::NulError)
    }
}

impl From<IntegerOrSdlError> for Error {
    fn from(val: IntegerOrSdlError) -> Self {
        Error::Builder(BuilderError::from(val))
    }
}

impl From<String> for Error {
    fn from(val: String) -> Self {
        Error::Other(val)
    }
}

impl Error {
    pub fn bad_argument<'a>(
        self,
        to: Option<&'a str>,
        pos: usize,
        name: Option<&'a str>,
    ) -> mlua::Error {
        mlua::Error::BadArgument {
            to: to.map(|v| v.to_string()),
            pos,
            name: name.map(|v| v.to_string()),
            cause: self.arcarc(),
        }
    }

    pub fn external(self) -> mlua::Error {
        mlua::Error::ExternalError(Arc::new(self))
    }

    #[allow(clippy::arc_with_non_send_sync)]
    fn arcarc(self) -> Arc<mlua::Error> {
        Arc::new(self.external())
    }
}

unsafe impl Sync for Error {}
unsafe impl Send for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Builder(builder_error) => builder_error.fmt(f),
            Error::Lua(error) => error.fmt(f),
            Error::IO { kind, message } => f.write_str(&format!("kind: {kind}, msg: {message}")),
            Error::Other(msg) => f.write_str(msg),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}
