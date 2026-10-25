pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur during vector operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The provided index is out of bounds.
    ///
    /// Contains the invalid index and the current length of the vector.
    IndexOutOfBounds { index: usize, len: usize },
}
