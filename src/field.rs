pub use isochronous_finite_fields::GF;

use rand_core::RngCore;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

pub trait FiniteField
where
    Self: Sized
        + Add<Output = Self>
        + AddAssign
        + Sub<Output = Self>
        + SubAssign
        + Mul<Output = Self>
        + MulAssign
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Copy
        + std::fmt::Debug,
{
    fn zero() -> Self;
    fn one() -> Self;

    fn random<R: RngCore>(rng: &mut R) -> Self;

    fn minv(self) -> Self;

    fn ainv(self) -> Self {
        Self::zero() - self
    }

    fn pow(self, exp: usize) -> Self {
        let mut out = Self::one();
        let mut exp_mask = 1usize.rotate_right(1);

        while exp_mask != 0 {
            out *= out;

            if (exp_mask & exp) != 0 {
                out *= self;
            }

            exp_mask >>= 1;
        }

        out
    }
}

impl FiniteField for GF {
    fn zero() -> Self {
        GF(0)
    }

    fn one() -> Self {
        GF(1)
    }

    fn random<R: RngCore>(rng: &mut R) -> Self {
        let mut b = [0];
        rng.fill_bytes(&mut b);
        GF(b[0])
    }

    fn minv(self) -> Self {
        self.multiplicative_inverse()
    }
}
