use std::ops::{Div, Mul, Rem};

pub fn gcd<T>(mut a: T, mut b: T) -> T
where
    T: Copy + Clone + PartialEq + PartialOrd + Rem<Output = T> + From<usize>,
{
    if a == b {
        return a;
    }
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }
    while b > 0.into() {
        let temp = a;
        a = b;
        b = temp % b;
    }
    a
}

pub fn lcm<T>(a: T, b: T) -> T
where
    T: Copy
        + Clone
        + PartialEq
        + PartialOrd
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + From<usize>,
{
    // LCM = a*b / gcd
    a * (b / gcd(a, b))
}
