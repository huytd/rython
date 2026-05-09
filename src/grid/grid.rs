use super::cell::Cell;
use super::color::Color;

#[derive(Clone)]
pub struct Grid {
    cells: Vec<Cell>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Grid {
            cells: vec![Cell::default(); width * height],
            width,
            height,
        }
    }

    fn idx(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(y * self.width + x)
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, ch: char, fg: Color, bg: Color) {
        if let Some(i) = self.idx(x, y) {
            self.cells[i] = Cell::new(ch, fg, bg);
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        self.idx(x, y).and_then(|i| self.cells.get(i))
    }

    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            *cell = Cell::default();
        }
    }

    pub fn scroll(&mut self, n: usize) {
        let w = self.width;
        let h = self.height;
        if n >= h {
            self.clear();
            return;
        }
        let mut new_cells = vec![Cell::default(); w * h];
        for y in n..h {
            for x in 0..w {
                let src = y * w + x;
                let dst = (y - n) * w + x;
                new_cells[dst] = self.cells[src];
            }
        }
        self.cells = new_cells;
    }

    /// Print text at (x, y), advancing x and wrapping to next row when hitting the right edge.
    pub fn print(&mut self, text: &str, mut x: usize, mut y: usize, fg: Color, bg: Color) {
        let w = self.width;
        let h = self.height;
        for ch in text.chars() {
            if x >= w {
                x = 0;
                y += 1;
            }
            if y >= h {
                self.scroll(1);
                y = h - 1;
            }
            self.set(x, y, ch, fg, bg);
            x += 1;
        }
    }

    pub fn rows(&self) -> &[Cell] {
        &self.cells
    }
}
