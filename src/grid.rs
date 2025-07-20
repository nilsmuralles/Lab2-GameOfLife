use std::vec;

use crate::framebuffer::FrameBuffer;

pub struct Grid {
    pub width: i32,
    pub height: i32,
    pub cells: Vec<Vec<bool>>,
    pub scale: i32
}

impl Grid {
    pub fn new(width: i32, height: i32, scale: i32) -> Self {
        let cells = vec![vec![false; height as usize]; width as usize];
        Grid { width, height, cells, scale }
    }

    pub fn add_cell(&mut self, x: i32, y: i32) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.cells[x as usize][y as usize] = true; 
        }
    }

    pub fn count_alive_neighbors(&mut self, x: i32, y: i32) -> u8 {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 { continue; } // Skip itself
                let next_x = x + dx;
                let next_y = y + dy;
                if next_x >= 0 && next_x < self.width && next_y >= 0 && next_y < self.height {
                    if self.cells[next_x as usize][next_y as usize] {
                        count += 1;
                    }
                }
            }
        }
        return count;
    }

    pub fn update(&mut self) {
        let mut next = vec![vec![false; self.height as usize]; self.width as usize];

        for x in 0..self.width  {
            for y in 0..self.height {
                let neighbors = self.count_alive_neighbors(x, y);
                let mut current_cell = self.cells[x as usize][y as usize];
                if current_cell {
                    if neighbors < 2 || neighbors > 3 {
                        current_cell = false;
                    }
                } else {
                    if neighbors == 3 {
                        current_cell = true;
                    }
                }
                next[x as usize][y as usize] = current_cell;
            }
        }

        self.cells = next;
    }

    pub fn draw(&self, fb: &mut FrameBuffer) {
        for x in 0..self.width  {
            for y in 0..self.height {
                if self.cells[x as usize][y as usize] {
                    for dy in 0..self.scale {
                        for dx in 0..self.scale {
                            fb.set_pixel(x * self.scale + dx, y * self.scale + dy);
                        }
                    }
                }
            }
        }
    }
}
