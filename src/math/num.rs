use std::ops::{Add, Div, Mul, Neg, Sub};

pub trait Number: Add + Sub + Div + Mul + Sized {
    fn one() -> Self;
    fn zero() -> Self;
}

pub trait NegNumber: Number + Neg {}

macro_rules! impl_numbers {
    (tt: $t:tt, zero: $zero:expr, one:$one:expr) => {
        impl Number for $t {
            fn one() -> Self{
                $one
            }

            fn zero() -> Self{
                $zero
            }
        }
    };
    ($((tt: $($t:tt),+; zero: $zero:expr; one:$one:expr;)),+) => {
        $(
            $(
                impl_numbers!(tt: $t, zero: $zero, one:$one);
            )+
        )+
    };

    (neg $t:tt) => { impl NegNumber for $t {} };
    (neg $($t:tt),+) => {
        $(
            impl_numbers!(neg $t);
        )+
    };
}

impl_numbers!(
    (
        tt: u8, u16, u32, u64, u128, i8, i16, i32, i64, i128;
        zero: 0;
        one: 1;
    ),
    (
        tt: f32, f64;
        zero: 0.0;
        one: 1.0;
    )
);

impl_numbers!(neg i8, i16, i32, i64, i128);
impl_numbers!(neg f32, f64);
