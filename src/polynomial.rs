use std::{fmt::Display, ops::Add};

use crate::field::{FiniteField, EnumerateIter};

#[derive(Debug)]
pub struct Polynomial<F> {
    coeffs: Vec<(F, F)>,
}

impl<F: Display> Display for Polynomial<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut c = self.coeffs.iter();
        if let Some((i, a)) = c.next() {
            write!(f, "{a}x^{i}")?;
        }

        for (i, a) in c {
            write!(f, " + {a}x^{i}")?;
        }

        Ok(())
    }
}

impl<F> FromIterator<(F, F)> for Polynomial<F> {
    fn from_iter<T: IntoIterator<Item = (F, F)>>(iter: T) -> Self {
        Self {
            coeffs: iter.into_iter().collect(),
        }
    }
}

impl<F: FiniteField + Display> Polynomial<F> {
    pub fn zero() -> Self {
        Self {
            coeffs: vec![(F::zero(), F::zero())],
        }
    }
    pub fn one() -> Self {
        Self {
            coeffs: vec![(F::zero(), F::one())],
        }
    }

    pub fn new_ordered(coeffs: impl IntoIterator<Item = F>) -> Self {
        EnumerateIter::new(coeffs.into_iter()).collect()
    }

    pub fn add(&self, rhs: &Self) -> Self {
        let mut sum = Polynomial::new_ordered([]);

        let mut c1 = self.coeffs.iter().peekable();
        let mut c2 = rhs.coeffs.iter().peekable();

        while let (Some(&&(i, a)), Some(&&(j, b))) = (c1.peek(), c2.peek()) {
            if i == j {
                sum.coeffs.push((i, a + b));
                c1.next();
                c2.next();
            } else if i < j {
                sum.coeffs.push((i, a));
                c1.next();
            } else {
                sum.coeffs.push((j, b));
                c2.next();
            }
        }

        sum.coeffs.extend(c1);
        sum.coeffs.extend(c2);

        sum
    }
    pub fn mul(&self, rhs: &Self) -> Self {
        self.coeffs
            .iter()
            .map(|&(i, a)| {
                let mut m: Polynomial<_> =
                    rhs.coeffs.iter().map(|&(j, b)| (i + j, a * b)).collect();
                m.coeffs.sort_by(|(i, _), (j, _)| i.cmp(j));
                m
            })
            .fold(Self::zero(), |acc, p| acc.add(&p))
    }
    pub fn scale(&self, rhs: F) -> Self {
        self.coeffs.iter().map(|&(i, a)| (i, a * rhs)).collect()
    }
    pub fn scale_div(&self, rhs: F) -> Self {
        self.coeffs.iter().map(|&(i, a)| (i, a / rhs)).collect()
    }

    pub fn get(&self, x: F) -> F {
        self.coeffs
            .iter()
            .map(|&(i, a)| a * x.pow(i))
            .fold(F::zero(), Add::add)
    }

    pub fn lagrange_new(points: impl Iterator<Item = (F, F)> + Clone) -> Self {
        let mut out = Polynomial::new_ordered([]);
        let mut out_denom = F::one();

        for (xi, y) in points.clone() {
            let mut numer = Self::one();
            let mut denom = F::one();
            for (xj, _) in points.clone() {
                if xi != xj {
                    numer = numer.mul(&Self::new_ordered([-xj, F::one()]));
                    denom = denom * (xi - xj);
                }
            }

            out = out.scale(denom).add(&numer.scale(y * out_denom));
            out_denom = out_denom * denom;
        }

        out.scale_div(out_denom)
    }
}
