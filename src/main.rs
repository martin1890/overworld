use rand::{RngExt, SeedableRng};
use rand::rngs::StdRng;

fn main() {
    let seed = 123456;
    let mut rng = StdRng::seed_from_u64(seed);

    let x: i32 = rng.random_range(1..=10);

    println!("Hello World!");
    println!("Random number is: {}", x);
}