mod aliases;
pub mod error;
pub mod num;
mod vector;

pub use aliases::*;
pub use vector::*;

#[macro_export]
macro_rules! vector {
    () => {
        $crate::math::Vector::default()
    };
    ($($n:expr),*) => {
        $crate::math::Vector::from([$($n),*])
    };
}
