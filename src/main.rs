use secret_sharing::{deconstruct, reconstruct};
use rand::prelude::*;

fn main() {
    let mut rng = thread_rng();

    let secret = [20, 1, 2, 3, 4, 5, 6, 7];
    let mut fragments = deconstruct(&mut rng, secret, 11, 20);

    for _ in 0..10 {
        fragments.shuffle(&mut rng);
        assert_eq!(secret, reconstruct(&fragments[..11]));
    }
}
