use rand::Rng;

const GRID_SIZE: usize = 15;
const REGION_LENGTH: usize = 8;

const MARKOV_MATRIX: [f64; 25] = [
    0.1, 0.9, 0.0, 0.0, 0.0,
    0.2, 0.3, 0.5, 0.0, 0.0,
    0.0, 0.35, 0.3, 0.35, 0.0,
    0.0, 0.0, 0.5, 0.3, 0.2,
    0.0, 0.0, 0.0, 0.9, 0.1,
];

fn r(lower: i32, upper: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(lower..=upper)
}

fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}

fn grid_index(x: i32, y: i32) -> usize {
    (x as usize) + (y as usize) * GRID_SIZE
}

fn update_markov_vector(state_vector: &mut [f64; 5], mut state: i32) -> i32 {
    let a = state_vector[0];
    let b = state_vector[1];
    let c = state_vector[2];
    let d = state_vector[3];
    let e = state_vector[4];

    for i in 0..5 {
        state_vector[i] =
            a * MARKOV_MATRIX[i] +
            b * MARKOV_MATRIX[i + 5] +
            c * MARKOV_MATRIX[i + 10] +
            d * MARKOV_MATRIX[i + 15] +
            e * MARKOV_MATRIX[i + 20];
    }

    let new_state = random_double();

    if new_state < state_vector[0] {
        state = 0;
        *state_vector = [1.0, 0.0, 0.0, 0.0, 0.0];
    } else if new_state < state_vector[0] + state_vector[1] {
        state = 1;
        *state_vector = [0.0, 1.0, 0.0, 0.0, 0.0];
    } else if new_state < state_vector[0] + state_vector[1] + state_vector[2] {
        state = 2;
        *state_vector = [0.0, 0.0, 1.0, 0.0, 0.0];
    } else if new_state < state_vector[0] + state_vector[1] + state_vector[2] + state_vector[3] {
        state = 3;
        *state_vector = [0.0, 0.0, 0.0, 1.0, 0.0];
    } else {
        state = 4;
        *state_vector = [0.0, 0.0, 0.0, 0.0, 1.0];
    }

    state
}

fn markov_step(state_vector: &mut [f64; 5], state: &mut i32, idx: &mut usize, x: &mut i32, y: &mut i32) {
    match *state {
        0 => *y -= 1,
        1 => {
            *y -= 1;
            *x += 1;
        }
        2 => *x += 1,
        3 => {
            *y += 1;
            *x += 1;
        }
        4 => *y += 1,
        _ => {}
    }

    *idx = grid_index(*x, *y);
    *state = update_markov_vector(state_vector, *state);
}

fn print_area(area: &[i32; GRID_SIZE * GRID_SIZE]) {
    for y in (0..GRID_SIZE).rev() {
        for x in 0..GRID_SIZE {
            let ch = if area[grid_index(x as i32, y as i32)] != 0 {
                "# "
            } else {
                ". "
            };
            print!("{}", ch);
        }
        println!();
    }
}

fn create_markov_curve(area: &mut [i32; GRID_SIZE * GRID_SIZE]) {
    let mut y: i32 = (GRID_SIZE / 2) as i32;
    let mut x: i32 = y - (REGION_LENGTH as i32 / 2);
    let mut idx = grid_index(x, y);

    let mut state_vector = [0.0, 0.0, 0.0, 0.0, 0.0];
    let mut state = r(0, 4);
    state_vector[state as usize] = 1.0;

    for _ in 0..REGION_LENGTH {
        if x < 0 || x >= GRID_SIZE as i32 || y < 0 || y >= GRID_SIZE as i32 {
            break;
        }

        area[idx] = 1;
        markov_step(&mut state_vector, &mut state, &mut idx, &mut x, &mut y);
    }
}

fn main() {
    let mut area = [0; GRID_SIZE * GRID_SIZE];
    create_markov_curve(&mut area);
    print_area(&area);
}