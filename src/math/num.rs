//! Number types and such
use std::ops::{Add, Mul, Sub};

pub trait Zero: Sized + Add<Self, Output = Self> {
    /// Returns the zero element of the structure
    fn zero() -> Self;

    /// Zero is always an additive identity, that is, given any element `x`, one
    /// should have `x + 0 = x` and `0 + x = x`.
    fn add_zero<T: Sized + Add<T, Output = T>>(x: T) -> T {
        x
    }
}

/// Used to implement boiler plate code for typical numerical types
#[macro_export]
macro_rules! zero_impl {
    ($t: ty, $zero: expr) => {
        impl Zero for $t {
            fn zero() -> $t {
                $zero
            }

            fn add_zero<T: Sized + Add<T, Output = T>>(x: T) -> T {
                x
            }
        }
    };
}

zero_impl!(usize, 0);
zero_impl!(u8, 0);
zero_impl!(u16, 0);
zero_impl!(u32, 0);
zero_impl!(u64, 0);
zero_impl!(u128, 0);

zero_impl!(isize, 0);
zero_impl!(i8, 0);
zero_impl!(i16, 0);
zero_impl!(i32, 0);
zero_impl!(i64, 0);
zero_impl!(i128, 0);

zero_impl!(f32, 0.0);
zero_impl!(f64, 0.0);

/// Multiplicative identity, given `x`, we have `x * 1 = x` and `1 * x = x`
pub trait One: Sized + Mul<Self, Output = Self> {
    /// Returns the one element of the structure
    fn one() -> Self;

    fn multiply_one<T: Sized + Mul<Self, Output = Self>>(x: T) -> T {
        x
    }
}

/// Used to implement boiler plate code for typical numerical types
#[macro_export]
macro_rules! one_impl {
    ($t: ty, $one: expr) => {
        impl One for $t {
            fn one() -> $t {
                $one
            }

            fn multiply_one<T: Sized + Mul<Self, Output = Self>>(x: T) -> T {
                x
            }
        }
    };
}

one_impl!(usize, 1);
one_impl!(u8, 1);
one_impl!(u16, 1);
one_impl!(u32, 1);
one_impl!(u64, 1);
one_impl!(u128, 1);

one_impl!(isize, 1);
one_impl!(i8, 1);
one_impl!(i16, 1);
one_impl!(i32, 1);
one_impl!(i64, 1);
one_impl!(i128, 1);

one_impl!(f32, 1.0);
one_impl!(f64, 1.0);

/// Typical numerical trait assemblying the most important traits
pub trait Num:
    PartialEq
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Zero
    + One
{
    // Empty trait
}

macro_rules! impl_num {
    ($($t: ty)*) => ($(
        impl Num for $t {}
    )*)
}

impl_num!(usize u8 u16 u32 u64 u128);
impl_num!(isize i8 i16 i32 i64 i128);
impl_num!(f32 f64);
