use super::cell::Cell;
use super::color::Color;

pub const WIDTH: usize = 40;
pub const HEIGHT: usize = 25;

#[derive(Clone)]
pub struct Grid {
    cells: Vec<Cell>,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            cells: vec![Cell::default(); WIDTH * HEIGHT],
        }
    }

    fn idx(x: usize, y: usize) -> Option<usize> {
        if x < WIDTH && y < HEIGHT {
            Some(y * WIDTH + x)
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, ch: char, fg: Color, bg: Color) {
        if let Some(i) = Self::idx(x, y) {
            self.cells[i] = Cell::new(ch, fg, bg);
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        Self::idx(x, y).and_then(|i| self.cells.get(i))
    }

    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            *cell = Cell::default();
        }
    }

    pub fn scroll(&mut self, n: usize) {
        if n >= HEIGHT {
            self.clear();
            return;
        }
        let mut new_cells = vec![Cell::default(); WIDTH * HEIGHT];
        for y in n..HEIGHT {
            for x in 0..WIDTH {
                let src = y * WIDTH + x;
                let dst = (y - n) * WIDTH + x;
                new_cells[dst] = self.cells[src];
            }
        }
        self.cells = new_cells;
    }

    /// Print text at (x, y), advancing x and wrapping to next row when hitting the right edge.
    pub fn print(&mut self, text: &str, mut x: usize, mut y: usize, fg: Color, bg: Color) {
        for ch in text.chars() {
            if x >= WIDTH {
                x = 0;
                y += 1;
            }
            if y >= HEIGHT {
                self.scroll(1);
                y = HEIGHT - 1;
            }
            self.set(x, y, ch, fg, bg);
            x += 1;
        }
    }

    pub fn rows(&self) -> &[Cell] {
        &self.cells
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}
