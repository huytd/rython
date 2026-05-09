use super::color::Color;

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
}

impl Cell {
    pub fn new(ch: char, fg: Color, bg: Color) -> Self {
        Cell { ch, fg, bg }
    }

    pub fn blank(fg: Color, bg: Color) -> Self {
        Cell::new(' ', fg, bg)
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::blank(Color::Darkgray, Color::Black)
    }
}
