pub mod field;
pub mod polynomial;

use field::FiniteField;
use isochronous_finite_fields::GF;
use polynomial::Polynomial;
use rand_core::RngCore;

pub type Secret<const N: usize> = [u8; N];

pub fn deconstruct<R: RngCore, const N: usize>(
    rng: &mut R,
    secret: Secret<N>,
    threshold: u8,
    count: u8,
) -> Vec<(u8, Secret<N>)> {
    let mut fragments = vec![(0, [0; N]); count as usize];

    for (i, (j, _)) in fragments.iter_mut().enumerate() {
        *j = i as u8;
    }

    for (i, part) in secret.into_iter().enumerate() {
        let generator: Polynomial<_> = [GF(part)]
            .into_iter()
            .chain(
                (0..)
                    .map(|_| GF::random(rng))
                    .filter(|&f| f != GF::zero())
                    .take(threshold.checked_sub(1).unwrap() as usize),
            )
            .collect();

        for (j, frag) in fragments.iter_mut() {
            frag[i] = generator.get(GF(*j)).0;
        }
    }

    fragments
}

pub fn reconstruct<const N: usize>(fragments: &[(u8, Secret<N>)]) -> Secret<N> {
    let mut secret = [0; N];

    for i in 0..N {
        secret[i] = Polynomial::lagrange_zero(fragments.iter().map(|&(x, y)| (GF(x), GF(y[i])))).0;
    }

    secret
}
