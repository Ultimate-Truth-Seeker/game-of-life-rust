use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
//use rand::{rng, Rng};
/// Conway's Game of Life
pub struct GameOfLife {
    cols: usize,
    rows: usize,
    cell_size: u32,
    grid: Vec<Vec<bool>>,
    buffer: Vec<Vec<bool>>,
}

impl GameOfLife {
    /// Create a new Game of Life with random initial state
    pub fn new(width: u32, height: u32, cell_size: u32) -> Self {
        let cols = (width / cell_size) as usize;
        let rows = (height / cell_size) as usize;
        //let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        
        // initialize grid randomly
        let grid:Vec<Vec<bool>> = (0..rows)
            .map(|_| (0..cols).map(|_| false).collect())
            .collect();
        let buffer = grid.clone();
        
        GameOfLife { cols, rows, cell_size, grid, buffer }
    }

    /// Count alive neighbors for a given cell
    fn count_neighbors(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;
        for dy in [-1isize, 0, 1] {
            for dx in [-1isize, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nr = row as isize + dy;
                let nc = col as isize + dx;
                if nr >= 0 && nr < self.rows as isize && nc >= 0 && nc < self.cols as isize {
                    if self.grid[nr as usize][nc as usize] {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    /// Advance the simulation by one step
    pub fn update(&mut self) {
        for r in 0..self.rows {
            for c in 0..self.cols {
                let alive = self.grid[r][c];
                let neighbors = self.count_neighbors(r, c);
                self.buffer[r][c] = match (alive, neighbors) {
                    (true, x) if x < 2 => false,       // underpopulation
                    (true, 2) | (true, 3) => true,      // survival
                    (true, x) if x > 3 => false,       // overpopulation
                    (false, 3) => true,                 // reproduction
                    (otherwise, _) => otherwise,
                };
            }
        }
        // swap buffers
        std::mem::swap(&mut self.grid, &mut self.buffer);
    }

    /// Draw the current state into the framebuffer
    pub fn draw(&self, fb: &mut Framebuffer) {
        fb.set_current_color(Color::WHITE);
        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.grid[r][c] {
                    let x0 = c as u32 * self.cell_size;
                    let y0 = r as u32 * self.cell_size;
                    // fill the cell block
                    for dy in 0..self.cell_size {
                        for dx in 0..self.cell_size {
                            fb.set_pixel(x0 + dx, y0 + dy);
                        }
                    }
                }
            }
        }
    }

    pub fn set_pattern(
        &mut self,
        coords: &[(usize, usize)],
        row_offset: usize,
        col_offset: usize,
    ) {
        for &(r, c) in coords {
            let rr = row_offset + r;
            let cc = col_offset + c;
            if rr < self.rows && cc < self.cols {
                self.grid[rr][cc] = true;
            }
        }
    }

    /// Parse a standard Run-Length Encoded (RLE) string (e.g. OTCA_metapixel.rle)
    /// and set cells accordingly at the given offset.
    pub fn load_rle(
        &mut self,
        rle_data: &str,
        row_offset: usize,
        col_offset: usize,
    ) {
        let mut x = 0;
        let mut y = 0;
        for line in rle_data.lines() {
            // Skip comments and header
            if line.starts_with('#') || line.starts_with('x') {
                continue;
            }
            let mut count = 0;
            for ch in line.chars() {
                match ch {
                    '0'..='9' => {
                        count = count * 10 + (ch as usize - '0' as usize);
                    }
                    'b' => {
                        let n = count.max(1);
                        x += n;
                        count = 0;
                    }
                    'o' => {
                        let n = count.max(1);
                        for i in 0..n {
                            let rr = row_offset + y;
                            let cc = col_offset + x + i;
                            if rr < self.rows && cc < self.cols {
                                self.grid[rr][cc] = true;
                            }
                        }
                        x += n;
                        count = 0;
                    }
                    '$' => {
                        let n = count.max(1);
                        y += n;
                        x = 0;
                        count = 0;
                    }
                    '!' => return,
                    _ => {}
                }
            }
        }
    }
}