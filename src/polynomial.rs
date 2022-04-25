use std::{fmt::Debug, ops::Add};

use crate::field::FiniteField;

#[derive(Clone)]
pub struct Polynomial<F> {
    pub coeffs: Vec<F>,
}

impl<F: Debug> Debug for Polynomial<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut c = self.coeffs.iter().enumerate();
        if let Some((_, a)) = c.next() {
            write!(f, "{a:?}")?;
        }

        for (i, a) in c {
            write!(f, " + {a:?}x")?;
            if i > 1 {
                write!(f, "^{i}")?;
            }
        }

        Ok(())
    }
}

impl<F> FromIterator<F> for Polynomial<F> {
    fn from_iter<T: IntoIterator<Item = F>>(iter: T) -> Self {
        Self {
            coeffs: iter.into_iter().collect(),
        }
    }
}

impl<F: FiniteField> Polynomial<F> {
    pub fn zero() -> Self {
        Self {
            coeffs: vec![F::zero()],
        }
    }
    pub fn one() -> Self {
        Self {
            coeffs: vec![F::one()],
        }
    }

    pub fn new(coeffs: Vec<F>) -> Self {
        Self { coeffs }
    }

    pub fn truncate(&mut self) {
        while Some(&F::zero()) == self.coeffs.last() {
            self.coeffs.pop();
        }
    }

    pub fn add(&self, rhs: &Self) -> Self {
        let (mut a, b) = if self.coeffs.len() > rhs.coeffs.len() {
            (self.clone(), rhs)
        } else {
            (rhs.clone(), self)
        };

        for (i, &c) in b.coeffs.iter().enumerate() {
            a.coeffs[i] = a.coeffs[i] + c;
        }

        a
    }
    pub fn mul(&self, rhs: &Self) -> Self {
        let mut out_coeffs = vec![F::zero(); self.coeffs.len() * rhs.coeffs.len()];

        for (i, &a) in self.coeffs.iter().enumerate() {
            for (j, &b) in rhs.coeffs.iter().enumerate() {
                out_coeffs[i + j] = out_coeffs[i + j] + a * b;
            }
        }

        Polynomial::new(out_coeffs)
    }
    pub fn scale(&self, rhs: F) -> Self {
        self.coeffs.iter().map(|&a| a * rhs).collect()
    }

    pub fn get(&self, x: F) -> F {
        self.coeffs
            .iter()
            .enumerate()
            .map(|(i, &a)| a * x.pow(i))
            .fold(F::zero(), Add::add)
    }

    pub fn lagrange_zero(points: impl Iterator<Item = (F, F)> + Clone) -> F {
        let mut out = F::zero();
        let mut out_denom = F::one();

        for (xi, y) in points.clone() {
            let mut numer = F::one();
            let mut denom = F::one();
            for (xj, _) in points.clone() {
                if xi != xj {
                    numer = numer * xj.ainv();
                    denom = denom * (xi - xj);
                }
            }

            out = out * denom + numer * y * out_denom;
            out_denom = out_denom * denom;
        }

        out * out_denom.minv()
    }

    pub fn lagrange_new(points: impl Iterator<Item = (F, F)> + Clone) -> Self {
        let mut out = Polynomial::zero();
        let mut out_denom = F::one();

        for (xi, y) in points.clone() {
            let mut numer = Self::one();
            let mut denom = F::one();
            for (xj, _) in points.clone() {
                if xi != xj {
                    numer = numer.mul(&Self::new(vec![xj.ainv(), F::one()]));
                    denom = denom * (xi - xj);
                }
            }

            out = out.scale(denom).add(&numer.scale(y * out_denom));
            out_denom = out_denom * denom;
        }

        out = out.scale(out_denom.minv());
        out.truncate();

        out
    }
}
