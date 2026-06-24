use std::collections::HashMap;

use minifb::{Key, KeyRepeat, Window, WindowOptions};

use crate::Chunk;

const VIEW_RADIUS: i32 = 30;
const GRID_SIZE: usize = 60;
const CELL_SIZE: usize = 12;

const SCREEN_WIDTH: usize = GRID_SIZE * CELL_SIZE;
const SCREEN_HEIGHT: usize = GRID_SIZE * CELL_SIZE;

fn height_to_color(height: f32) -> u32 {
    if height < 0.25 {
        0x0033AA
    } else if height < 0.35 {
        0x2277DD
    } else if height < 0.50 {
        0x22AA22
    } else if height < 0.70 {
        0x228822
    } else if height < 0.85 {
        0x777777
    } else {
        0xFFFFFF
    }
}

fn get_or_create_chunk(
    chunks: &mut HashMap<(i32, i32), Chunk>,
    seed: u32,
    x: i32,
    y: i32,
    height_fn: fn(u32, i32, i32) -> f32,
) -> &Chunk {
    chunks.entry((x, y)).or_insert_with(|| Chunk {
        height: height_fn(seed, x, y),
    })
}

fn draw_cell(
    buffer: &mut [u32],
    grid_x: usize,
    grid_y: usize,
    color: u32,
) {
    let start_x = grid_x * CELL_SIZE;
    let start_y = grid_y * CELL_SIZE;

    for y in start_y..start_y + CELL_SIZE {
        for x in start_x..start_x + CELL_SIZE {
            buffer[y * SCREEN_WIDTH + x] = color;
        }
    }
}

fn draw_view(
    buffer: &mut [u32],
    chunks: &mut HashMap<(i32, i32), Chunk>,
    seed: u32,
    player_x: i32,
    player_y: i32,
    height_fn: fn(u32, i32, i32) -> f32,
) {
    buffer.fill(0x000000);

    let start_x = player_x - VIEW_RADIUS;
    let start_y = player_y - VIEW_RADIUS;

    for screen_y in 0..GRID_SIZE {
        for screen_x in 0..GRID_SIZE {
            let world_x = start_x + screen_x as i32;
            let world_y = start_y + screen_y as i32;

            let chunk = get_or_create_chunk(
                chunks,
                seed,
                world_x,
                world_y,
                height_fn,
            );

            let color = height_to_color(chunk.height);
            draw_cell(buffer, screen_x, screen_y, color);
        }
    }

    draw_cell(buffer, GRID_SIZE / 2, GRID_SIZE / 2, 0xFF0000);
}

pub fn run_heightmap_viewer(
    chunks: &mut HashMap<(i32, i32), Chunk>,
    seed: u32,
    start_x: i32,
    start_y: i32,
    height_fn: fn(u32, i32, i32) -> f32,
) -> Result<(), minifb::Error> {
    let mut window = Window::new(
        "Heightmap Viewer - WASD to move, ESC to exit",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions::default(),
    )?;

    let mut buffer = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT];

    let mut player_x = start_x;
    let mut player_y = start_y;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::W, KeyRepeat::Yes) {
            player_y -= 1;
        }

        if window.is_key_pressed(Key::S, KeyRepeat::Yes) {
            player_y += 1;
        }

        if window.is_key_pressed(Key::A, KeyRepeat::Yes) {
            player_x -= 1;
        }

        if window.is_key_pressed(Key::D, KeyRepeat::Yes) {
            player_x += 1;
        }

        draw_view(
            &mut buffer,
            chunks,
            seed,
            player_x,
            player_y,
            height_fn,
        );

        window.update_with_buffer(&buffer, SCREEN_WIDTH, SCREEN_HEIGHT)?;
    }

    Ok(())
}