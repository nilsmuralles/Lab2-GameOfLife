pub struct Grid {
    pub width: i32,
    pub height: i32,
    pub cells: Vec<Vec<bool>>
}

impl Grid {
    pub fn new(width: i32, height: i32) -> Self {
        let cells = vec![vec![false; width as usize]; height as usize];
        Grid { width, height, cells }
    }

    pub fn add_cell(&mut self, x: i32, y: i32, is_alive: bool) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
           self.cells[x as usize][y as usize] = is_alive; 
        }
    }

    pub fn is_cell_alive(&mut self, x: i32, y: i32) -> bool {
        return self.cells[x as usize][y as usize];
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
}
