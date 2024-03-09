use core::{hash::Hash, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}};
use crate::To;

pub trait Zero {
    const ZERO: Self;
}

pub trait One {
    const ONE: Self;
}

macro_rules! impl_zero_one {
    ($($ty:ty,)*) => {$(
        impl Zero for $ty {
            const ZERO: Self = 0;
        }
        impl One for $ty {
            const ONE: Self = 1;
        }
    )*};
}
impl_zero_one!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize,);

pub trait PositiveInteger:
    Add<Self, Output = Self>
    + Send
    + Sync
    + AddAssign
    + Sub<Self, Output = Self>
    + SubAssign
    + Copy
    + PartialEq
    + PartialOrd
    + Ord
    + Eq
    + Default
    + Mul<Self, Output = Self>
    + MulAssign
    + Div<Self, Output = Self>
    + DivAssign
    + Zero
    + One
    + Hash
    + To<usize>
{
}

impl<T> PositiveInteger for T where
    T: Add<Self, Output = Self>
        + Send
        + Sync
        + AddAssign
        + Sub<Self, Output = Self>
        + SubAssign
        + Copy
        + PartialEq
        + PartialOrd
        + Ord
        + Eq
        + Default
        + Mul<Self, Output = Self>
        + MulAssign
        + Div<Self, Output = Self>
        + DivAssign
        + Zero
        + One
        + Hash
        + To<usize>
{
}
