use std::ops::{Add, Div, Mul, Neg, Sub};

pub trait FiniteField
where
    Self: Sized
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + Neg<Output = Self>
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Copy,
{
    fn zero() -> Self;
    fn one() -> Self;

    fn pow(self, exp: Self) -> Self;
}

impl FiniteField for i32 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn pow(self, exp: Self) -> Self {
        self.pow(exp as u32)
    }
}

pub struct EnumerateIter<F, I> {
    acc: F,
    inner: I,
}

impl<F: FiniteField, I: Iterator> EnumerateIter<F, I> {
    pub fn new(inner: I) -> Self {
        Self {
            acc: F::zero(),
            inner,
        }
    }
}

impl<F: FiniteField, I: Iterator> Iterator for EnumerateIter<F, I> {
    type Item = (F, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|i| {
            let out = (self.acc, i);
            self.acc = self.acc + F::one();
            out
        })
    }
}
