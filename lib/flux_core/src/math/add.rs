use num_traits::{CheckedAdd, SaturatingAdd, WrappingAdd};

use super::enums::MathWrapMode;

pub fn add<T>(a: T, b: T) -> Option<T>
where
    T: CheckedAdd
{
    a.checked_add(&b)
}

pub fn add_using_mode<T>(a: T, b: T, mode: MathWrapMode) -> T
where
    T: SaturatingAdd + WrappingAdd
{
    match mode {
        MathWrapMode::Saturating => a.saturating_add(&b),
        MathWrapMode::Wrapping => a.wrapping_add(&b),
    }
}