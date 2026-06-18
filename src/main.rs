use rand::{RngExt, SeedableRng};
use rand::rngs::StdRng;
use std::collections::HashMap;

struct Player {
    position_x: f64;
    position_y: f64;
};

struct Chunk {
    height: f32;
};

fn deterministic_heightmap(seed: i32, x: i32, y: i32) -> f32 {
    let mut h = seed;

    h ^= (x as u32).wrapping_mul(374761393);
    h ^= (y as u32).wrapping_mul(668265263);

    h = (h ^ (h >> 13)).wrapping_mul(1274126177);
    h ^= h >> 16;

    // Convert to [0.0, 1.0)
    h as f32 / u32::MAX as f32
}

fn initial_chunks(seed: i32, x: i32, y: i32) {
    for n in -50..50 {
        for k in -50..50 {
            let mut chunk = Chunk {
                height: deterministic_heightmap(seed, x+n, y+k);
            }
            chunks.insert((x+n, y+k), chunk);
        }
    }
}

fn main() {

    let seed = 123456;
    let mut rng = StdRng::seed_from_u64(seed);
    let x: i32 = rng.random_range(1..=10);

    let mut chunks: HashMap<(i32, i32), Chunk> = HashMap::new();

    let mut player = Player {
        position_x: 0.0;
        position_y: 0.0;
    }

    initial_chunks(player.position_x.floor() as i32, player.position_y.floor() as i32);

    println!("Hello World!");
    println!("Random number is: {}", x);
}