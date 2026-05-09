use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Widget;

use crate::grid::{Cell, Grid};

pub struct GridView<'a>(pub &'a Grid);

impl Widget for GridView<'_> {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let grid = self.0;
        let width = grid.width;
        let height = grid.height;
        let default_cell = Cell::default();
        let mut lines = Vec::with_capacity(height);
        for y in 0..height {
            let mut spans = Vec::with_capacity(width);
            for x in 0..width {
                let cell = grid.get(x, y).unwrap_or(&default_cell);
                spans.push(Span::styled(
                    cell.ch.to_string(),
                    Style::new()
                        .fg(cell.fg.to_ratatui())
                        .bg(cell.bg.to_ratatui()),
                ));
            }
            lines.push(Line::from(spans));
        }
        let text = ratatui::text::Text::from(lines);
        text.render(area, buffer);
    }
}
