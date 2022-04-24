use secret_sharing::Polynomial;

fn main() {
    let p = Polynomial::new_ordered([10, 100, 1000, 10000]);
    let p_ = Polynomial::lagrange_new((0..4).map(|x| (x, p.get(x))));

    println!("{p} == {p_}");
}
