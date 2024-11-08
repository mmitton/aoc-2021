use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

pub trait Integer:
    Sized
    + Copy
    + Clone
    + Display
    + Debug
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + Rem<Self, Output = Self>
    + AddAssign<Self>
    + SubAssign<Self>
    + MulAssign<Self>
    + DivAssign<Self>
    + RemAssign<Self>
    + Ord
    + Eq
{
    const ZERO: Self;
    const ONE: Self;
}

macro_rules! impl_integer {
    ($ty:ty) => {
        impl Integer for $ty {
            const ZERO: Self = 0;
            const ONE: Self = 1;
        }
    };
}

impl_integer!(u8);
impl_integer!(u16);
impl_integer!(u32);
impl_integer!(u64);
impl_integer!(u128);
impl_integer!(usize);
impl_integer!(i8);
impl_integer!(i16);
impl_integer!(i32);
impl_integer!(i64);
impl_integer!(i128);
impl_integer!(isize);
