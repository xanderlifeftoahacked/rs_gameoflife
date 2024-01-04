use super::constants;

pub struct Grid {
    width: usize,
    height: usize,
    grid: Vec<Vec<bool>>,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            width: constants::GRID_WIDTH,
            height: constants::GRID_HEIGHT,
            grid: vec![vec![false; constants::GRID_WIDTH]; constants::GRID_HEIGHT],
        }
    }

    pub fn update(&mut self) {
        let temp_grid = self.grid.clone();
        for x in 0..self.width {
            for y in 0..self.height {
                self.update_cell_condition(&temp_grid, x, y);
            }
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> bool {
        self.grid[y][x]
    }

    pub fn set_cell(&mut self, x: usize, y: usize, condition: bool) {
        self.grid[y][x] = condition;
    }

    fn update_cell_condition(&mut self, temp_grid: &Vec<Vec<bool>>, x: usize, y: usize) {
        let mut close_alive_count = 0;

        let x = x as i32;
        let y = y as i32;

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if x + dx < 0
                    || x + dx >= self.width as i32
                    || y + dy < 0
                    || y + dy >= self.height as i32
                {
                    continue;
                }

                if temp_grid[(y + dy) as usize][(x + dx) as usize] {
                    close_alive_count += 1;
                }
            }
        }

        let cell = &mut self.grid[y as usize][x as usize];
        match (cell.clone(), close_alive_count) {
            (true, x) if x > 3 || x < 2 => *cell = false,
            (false, x) if x == 3 => *cell = true,
            (_, _) => (),
        }
    }
}
