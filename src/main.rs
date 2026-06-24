use rand::{RngExt, SeedableRng};
use rand::rngs::StdRng;
use std::collections::HashMap;

mod rendering;
use rendering::minifb_viewer::run_heightmap_viewer;

struct Player {
    position_x: f64,
    position_y: f64,
}

pub(crate)struct Chunk {
    pub(crate)height: f32,
}

fn deterministic_heightmap(seed: u32, x: i32, y: i32) -> f32 {
    let mut h = seed;

    h ^= (x as u32).wrapping_mul(374761393);
    h ^= (y as u32).wrapping_mul(668265263);

    h = (h ^ (h >> 13)).wrapping_mul(1274126177);
    h ^= h >> 16;

    // Convert to [0.0, 1.0)
    h as f32 / u32::MAX as f32
}

fn initial_chunks(chunks: &mut HashMap<(i32, i32), Chunk>, seed: u32, x: i32, y: i32) {
    for n in -50..50 {
        for k in -50..50 {
            let chunk_height = deterministic_heightmap(seed, x+n, y+k);
            
            let chunk = Chunk {
                height: chunk_height,
            };
            chunks.insert((x+n, y+k), chunk);
        }
    }
}

fn main() -> Result<(), minifb::Error> {

    let seed: u32 = 123456;
    let mut rng = StdRng::seed_from_u64(seed as u64);
    let x: i32 = rng.random_range(1..=10);

    let mut chunks: HashMap<(i32, i32), Chunk> = HashMap::new();

    let mut player = Player {
        position_x: 0.0,
        position_y: 0.0,
    };

    initial_chunks(&mut chunks, seed, player.position_x.floor() as i32, player.position_y.floor() as i32);

    println!("Hello World!");
    println!("Random number is: {}", x);

    run_heightmap_viewer(
        &mut chunks,
        seed,
        player.position_x.floor() as i32,
        player.position_y.floor() as i32,
        deterministic_heightmap,
    )?;

    Ok(())
}