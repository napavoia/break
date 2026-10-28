mod aliases;

pub mod error;
pub mod num;
pub mod transform;
mod vector;

pub use aliases::*;
pub use vector::{Axis, Vector};

#[macro_export]
macro_rules! vector {
    () => {
        $crate::math::Vector::default()
    };
    ($($n:expr),*) => {
        $crate::math::Vector::from([$($n),*])
    };
}
