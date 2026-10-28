use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur during vector operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The provided index is out of bounds.
    ///
    /// Contains the invalid index and the current length of the vector.
    IndexOutOfBounds { index: usize, len: usize },
}

impl AsRef<str> for Error {
    fn as_ref(&self) -> &str {
        match self {
            Error::IndexOutOfBounds { .. } => "IndexOutOfBounds",
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IndexOutOfBounds { index, len } => f
                .debug_struct(self.as_ref())
                .field("index", index)
                .field("len", len)
                .finish(),
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
