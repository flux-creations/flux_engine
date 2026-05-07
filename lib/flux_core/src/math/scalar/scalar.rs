use std::ops::{Add, Div, Mul, Sub};

pub trait Scalar:
    Sized
    + Copy
    + Clone
    + PartialEq
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Default
{
    fn zero() -> Self;
    fn one() -> Self;
    fn sqrt(self) -> Self;
    fn from_f64(v: f64) -> Self;
    fn to_f64(self) -> f64;
    fn cast<U: Scalar>(self) -> U {
        Scalar::from_f64(self.to_f64())
    }
}

macro_rules! impl_scalar {
    ($($t:ty),*) => {
        $(
        impl Scalar for $t {
            #[inline(always)]
            fn zero() -> Self { 0 as $t }

            #[inline(always)]
            fn one() -> Self { 1 as $t }

            #[inline(always)]
            fn sqrt(self) -> Self {
                (self as f64).sqrt() as $t
            }

            #[inline(always)]
            fn from_f64(v: f64) -> Self {
                (v + 0.5) as $t
            }

            #[inline(always)]
            fn to_f64(self) -> f64 {
                self as f64
            }
        }
        )*
    };
}

impl_scalar!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64);
