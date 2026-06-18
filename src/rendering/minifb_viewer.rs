use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Window, WindowOptions};

pub struct GridViewer {
    window: Window,
    buffer: Vec<u32>,

    grid_width: usize,
    grid_height: usize,
    cell_size: usize,

    screen_width: usize,
    screen_height: usize,

    paused: bool,
}

impl GridViewer {
    pub fn new(
        title: &str,
        grid_width: usize,
        grid_height: usize,
        cell_size: usize,
    ) -> Result<Self, minifb::Error> {
        let screen_width = grid_width * cell_size;
        let screen_height = grid_height * cell_size;

        let window = Window::new(
            title,
            screen_width,
            screen_height,
            WindowOptions::default(),
        )?;

        let buffer = vec![0; screen_width * screen_height];

        Ok(Self {
            window,
            buffer,
            grid_width,
            grid_height,
            cell_size,
            screen_width,
            screen_height,
            paused: false,
        })
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn should_step(&self) -> bool {
        self.window.is_key_pressed(Key::Right, KeyRepeat::No)
    }

    pub fn update_input(&mut self) {
        if self.window.is_key_pressed(Key::Space, KeyRepeat::No) {
            self.paused = !self.paused;
        }
    }

    pub fn draw_grid<T, F>(
        &mut self,
        grid: &[Vec<T>],
        mut color_fn: F,
    ) -> Result<(), minifb::Error>
    where
        F: FnMut(&T) -> u32,
    {
        self.clear(0xFFFFFF);

        for y in 0..self.grid_height {
            for x in 0..self.grid_width {
                let color = color_fn(&grid[y][x]);
                self.draw_cell(x, y, color);
            }
        }

        self.window
            .update_with_buffer(&self.buffer, self.screen_width, self.screen_height)
    }

    pub fn get_mouse_grid_position(&self) -> Option<(usize, usize)> {
        let (mouse_x, mouse_y) = self.window.get_mouse_pos(MouseMode::Discard)?;

        let grid_x = mouse_x as usize / self.cell_size;
        let grid_y = mouse_y as usize / self.cell_size;

        if grid_x < self.grid_width && grid_y < self.grid_height {
            Some((grid_x, grid_y))
        } else {
            None
        }
    }

    pub fn is_left_mouse_down(&self) -> bool {
        self.window.get_mouse_down(MouseButton::Left)
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.window.is_key_down(key)
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.window.is_key_pressed(key, KeyRepeat::No)
    }

    fn clear(&mut self, color: u32) {
        self.buffer.fill(color);
    }

    fn draw_cell(&mut self, grid_x: usize, grid_y: usize, color: u32) {
        let start_x = grid_x * self.cell_size;
        let start_y = grid_y * self.cell_size;

        for y in start_y..start_y + self.cell_size {
            for x in start_x..start_x + self.cell_size {
                let index = y * self.screen_width + x;
                self.buffer[index] = color;
            }
        }
    }
}