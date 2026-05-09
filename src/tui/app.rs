use std::fs;
use std::sync::{Arc, Mutex};

use crossterm::event::{KeyCode, KeyModifiers};
use pyo3::prelude::*;
use pyo3::types::PyDictMethods;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState};
use ratatui_textarea::TextArea;

use crate::event::Event;
use crate::grid::Grid;
use crate::python::Screen;
use crate::tui::grid_view::GridView;

fn build_help_text(width: usize, height: usize) -> String {
    format!(
        r##"
=== pymodo Python API Reference ===

The screen object is available globally. No imports needed.
Canvas: {width} columns (x) x {height} rows (y), origin (0,0) = top-left.

--- Methods ---

screen.set(x, y, ch, fg, bg)
  Place a single character at (x, y).
  x: int (0-{wmax}), y: int (0-{hmax})
  ch: str (single character)
  fg, bg: str (color name, case-insensitive)

screen.print(text, x, y, fg, bg)
  Print a string at (x, y). Auto-wraps at col {width}.
  Scrolls up when reaching the bottom row.

screen.clear()
  Clear the entire grid back to empty cells.

screen.scroll(n)
  Scroll the screen UP by n rows. Bottom fills blank.
  If n >= {height}, clears entirely.

screen.refresh()
  Force an immediate redraw. Use in animation loops.

--- Colors ---

black, white, red, cyan, purple, green, blue,
yellow, orange, brown, lightred, darkgray, gray,
lightgreen, lightblue, lightgray

(Case-insensitive, C64-inspired RGB values)

--- Quick Examples ---

# Single character
screen.set(10, 5, "A", "yellow", "black")

# Print text
screen.print("Hello!", 5, 10, "white", "blue")

# Animation loop
import time
for i in range({width}):
    screen.clear()
    screen.set(i %% {width}, 12, "@", "green", "black")
    screen.refresh()
    time.sleep(0.1)

# Pattern
for y in range({height}):
    for x in range({width}):
        ch = "#" if (x + y) %% 2 == 0 else "."
        screen.set(x, y, ch, "yellow", "black")

--- Tips ---

- Out-of-bounds writes are silently ignored
- screen.print() auto-wraps and scrolls
- Use screen.refresh() in loops to see each frame
- Standard Python works (time.sleep, f-strings, etc.)
- Errors show on screen; press any key to return

--- Editor Shortcuts ---

F1 / Ctrl+/   Toggle this help panel
F5 / Ctrl+R Run the script
Ctrl+O      Open a file
Ctrl+C  Quit
Up/Down     Scroll this help panel (when visible)
PageUp/PageDn  Scroll help faster
"##,
        width = width,
        height = height,
        wmax = width - 1,
        hmax = height - 1,
    )
}

#[derive(PartialEq)]
pub enum Mode {
    Editor,
    Run,
    Open,
}

pub struct App<'a> {
    textarea: TextArea<'a>,
    grid: Arc<Mutex<Grid>>,
    mode: Mode,
    error_msg: Option<String>,
    open_input: String,
    open_error: Option<String>,
    quit: bool,
    help_visible: bool,
    help_scroll_offset: usize,
    fullscreen: bool,
}

impl<'a> App<'a> {
    pub fn new(grid: Arc<Mutex<Grid>>, initial_code: Option<&str>, fullscreen: bool) -> Self {
        let mut textarea = TextArea::default();
        if let Some(code) = initial_code {
            for (i, line) in code.lines().enumerate() {
                textarea.insert_str(line);
                if i < code.lines().count() - 1 {
                    textarea.input(crossterm::event::KeyEvent::new(
                        crossterm::event::KeyCode::Enter,
                        crossterm::event::KeyModifiers::NONE,
                    ));
                }
            }
        }
        App {
            textarea,
            grid,
            mode: Mode::Editor,
            error_msg: None,
            open_input: String::new(),
            open_error: None,
            quit: false,
            help_visible: false,
            help_scroll_offset: 0,
            fullscreen,
        }
    }

    pub fn should_quit(&self) -> bool {
        self.quit
    }

    /// Returns true if the caller should run the code (needs terminal access).
    pub fn handle_event(&mut self, event: &Event) -> bool {
        match event {
            Event::Key(key) => {
                match self.mode {
                    Mode::Editor => return self.handle_editor_key(key),
                    Mode::Run => self.handle_run_key(key),
                    Mode::Open => self.handle_open_key(key),
                }
            }
            Event::Resize(w, h) => {
                if self.fullscreen {
                    let new_grid = Grid::new(*w as usize, *h as usize);
                    *self.grid.lock().unwrap() = new_grid;
                }
            }
            Event::Tick => {}
        }
        false
    }

    fn handle_editor_key(&mut self, key: &crossterm::event::KeyEvent) -> bool {
        let is_ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

        match (key.code, is_ctrl) {
            // F1 or Ctrl+/ => Toggle help
            (KeyCode::F(1), _) | (KeyCode::Char('7'), true) => {
                self.help_visible = !self.help_visible;
                if self.help_visible {
                    self.help_scroll_offset = 0;
                }
            }
            // F5 or Ctrl+R => Run
            (KeyCode::F(5), _) | (KeyCode::Char('r'), true) => {
                return true;
            }
            // Ctrl+O => Open file
            (KeyCode::Char('o'), true) => {
                self.open_input.clear();
                self.open_error = None;
                self.mode = Mode::Open;
            }
            // Ctrl+C => Quit
            (KeyCode::Char('c'), true) => {
                self.quit = true;
            }
            // When help is visible, handle scrolling with arrow keys and page up/down
            _ if self.help_visible => {
                match key.code {
                    KeyCode::Up => {
                        self.help_scroll_offset = self.help_scroll_offset.saturating_sub(1);
                    }
                    KeyCode::Down => {
                        self.help_scroll_offset += 1;
                    }
                    KeyCode::PageUp => {
                        self.help_scroll_offset = self.help_scroll_offset.saturating_sub(10);
                    }
                    KeyCode::PageDown => {
                        self.help_scroll_offset += 10;
                    }
                    _ => {
                        self.textarea.input(*key);
                    }
                }
            }
            _ => {
                self.textarea.input(*key);
            }
        }
        false
    }

    fn handle_run_key(&mut self, _key: &crossterm::event::KeyEvent) {
        // Any key returns to editor mode.
        self.mode = Mode::Editor;
    }

    fn handle_open_key(&mut self, key: &crossterm::event::KeyEvent) {
        let is_ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

        match (key.code, is_ctrl) {
            // Enter => Load file
            (KeyCode::Enter, _) => {
                let path = self.open_input.trim().to_string();
                if path.is_empty() {
                    self.mode = Mode::Editor;
                    return;
                }
                match fs::read_to_string(&path) {
                    Ok(contents) => {
                        // Replace textarea contents
                        let mut new_textarea = TextArea::default();
                        for (i, line) in contents.lines().enumerate() {
                            new_textarea.insert_str(line);
                            if i < contents.lines().count() - 1 {
                                new_textarea.input(crossterm::event::KeyEvent::new(
                                    crossterm::event::KeyCode::Enter,
                                    crossterm::event::KeyModifiers::NONE,
                                ));
                            }
                        }
                        self.textarea = new_textarea;
                        self.mode = Mode::Editor;
                    }
                    Err(e) => {
                        self.open_error = Some(format!("Failed to open '{}': {}", path, e));
                    }
                }
            }
            // Esc => Cancel
            (KeyCode::Esc, _) => {
                self.mode = Mode::Editor;
            }
            // Backspace
            (KeyCode::Backspace, _) => {
                self.open_input.pop();
            }
            // Regular character input
            (KeyCode::Char(c), _) => {
                self.open_input.push(c);
            }
            _ => {}
        }
    }

    pub fn run_code(
        &mut self,
        terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
    ) {
        self.error_msg = None;
        self.grid.lock().unwrap().clear();

        let lines: Vec<_> = self.textarea.lines().iter().map(|l| l.as_str()).collect();
        let code: String = lines.join("\n");

        let grid_clone = self.grid.clone();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            pyo3::Python::attach(|py| -> PyResult<()> {
                let mut screen = Screen::from_grid(grid_clone);
                // SAFETY: terminal is valid for the duration of this call.
                // Python runs synchronously under the GIL, so no concurrent access occurs.
                unsafe { screen.set_terminal_ptr(terminal) };

                // Use a single dict as both globals and locals so closures can resolve `screen`.
                let namespace = pyo3::types::PyDict::new(py);
                namespace.set_item("screen", screen)?;

                // Replace time.sleep with a chunked version that redraws between sleeps,
                // preventing the TUI from freezing during sleep calls.
                let inject_cstr = std::ffi::CString::new(
                    concat!(
                        "import time, os\n",
                        "_sleep_real = time.sleep\n",
                        "def _sleep_chunk(seconds):\n",
                        "    chunk = 0.016\n",
                        "    if seconds <= chunk:\n",
                        "        _sleep_real(seconds)\n",
                        "        return\n",
                        "    n = int(seconds / chunk) + 1\n",
                        "    for _ in range(n):\n",
                        "        _sleep_real(chunk)\n",
                        "        screen.refresh()\n",
                        "time.sleep = _sleep_chunk\n",
                    ),
                )
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Invalid code: {}", e)))?;
                py.run(inject_cstr.as_c_str(), Some(&namespace), Some(&namespace))?;

                let code_cstr = std::ffi::CString::new(&*code).map_err(|e| {
                    pyo3::exceptions::PyValueError::new_err(format!("Invalid code: {}", e))
                })?;
                py.run(code_cstr.as_c_str(), Some(&namespace), Some(&namespace))
            })
        }));

        match result {
            Ok(Ok(())) => {
                self.mode = Mode::Run;
            }
            Ok(Err(e)) => {
                self.error_msg = Some(e.to_string());
                self.mode = Mode::Run;
            }
            Err(e) => {
                self.error_msg = Some(format!(
                    "{}",
                    e.downcast_ref::<String>()
                        .map(|s| s.as_str())
                        .or_else(|| e.downcast_ref::<&str>().copied())
                        .unwrap_or("Failed to initialize Python interpreter")
                ));
                self.mode = Mode::Run;
            }
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        match self.mode {
            Mode::Editor => self.render_editor(frame, area),
            Mode::Run => self.render_run(frame, area),
            Mode::Open => self.render_open(frame, area),
        }
    }

    fn render_editor(&mut self, frame: &mut Frame, area: Rect) {
        self.textarea.set_cursor_line_style(Style::default().bg(Color::Rgb(50, 50, 50)));

        // Read size info from grid (drop lock before further rendering)
        let size_label = if self.fullscreen {
            "fullscreen".to_string()
        } else {
            let g = self.grid.lock().unwrap();
            format!("{}x{}", g.width, g.height)
        };

        let title = Line::from(vec![
            Span::styled("pymodo", Style::new().fg(Color::Cyan).bold()),
            Span::raw(format!("  |  screen: {}  |  ", size_label)),
            Span::styled("F1 / Ctrl+/: Help", Style::new().fg(Color::DarkGray)),
            Span::raw("  |  "),
            Span::styled("F5 / Ctrl+R: Run", Style::new().fg(Color::DarkGray)),
            Span::raw("  |  "),
            Span::styled("Ctrl+O: Open", Style::new().fg(Color::DarkGray)),
            Span::raw("  |  "),
            Span::styled("Ctrl+C: Quit", Style::new().fg(Color::DarkGray)),
        ]);

        let block = Block::bordered().title(title);
        let inner = block.inner(area);

        frame.render_widget(block, area);

        if self.help_visible {
            // Split: editor on left (60%), help on right (40%)
            let chunks = Layout::horizontal([
                Constraint::Percentage(60),
                Constraint::Percentage(40),
            ]).split(inner);

            frame.render_widget(&self.textarea, chunks[0]);
            self.render_help_panel(frame, chunks[1]);
        } else {
            frame.render_widget(&self.textarea, inner);
        }
    }

    fn render_help_panel(&mut self, frame: &mut Frame, area: Rect) {
        let help_block = Block::bordered().title(Line::from(vec![
            Span::styled(" Help (Up/Down to scroll) ", Style::new().fg(Color::Cyan).bold()),
        ]));
        let inner = help_block.inner(area);

        frame.render_widget(help_block, area);

        let grid = self.grid.lock().unwrap();
        let help_content = build_help_text(grid.width, grid.height);

        // Build the help text lines
        let all_lines: Vec<Line> = help_content
            .lines()
            .map(|line| {
                let trimmed = line.trim_end();
                if trimmed.starts_with("===") {
                    Line::from(vec![Span::styled(
                        trimmed,
                        Style::new().fg(Color::Cyan).bold(),
                    )])
                } else if trimmed.starts_with("---") {
                    Line::from(vec![Span::styled(
                        trimmed,
                        Style::new().fg(Color::Yellow),
                    )])
                } else if trimmed.starts_with(' ') || trimmed.starts_with('#') {
                    Line::from(vec![Span::styled(
                        trimmed,
                        Style::new().fg(Color::Gray),
                    )])
                } else if trimmed.contains('(') && trimmed.ends_with(')') {
                    Line::from(vec![Span::styled(
                        trimmed,
                        Style::new().fg(Color::Green).bold(),
                    )])
                } else if trimmed.starts_with('-') {
                    Line::from(vec![Span::styled(
                        trimmed,
                        Style::new().fg(Color::Gray).bold(),
                    )])
                } else {
                    Line::from(vec![Span::raw(trimmed)])
                }
            })
            .collect();

        let total_lines = all_lines.len();
        let visible_height = inner.height as usize;

        // Clamp scroll offset
        if total_lines > visible_height {
            self.help_scroll_offset = self.help_scroll_offset.min(total_lines - visible_height);
        } else {
            self.help_scroll_offset = 0;
        }

        // Render visible lines
        let visible_lines: Vec<Line> = all_lines
            .iter()
            .skip(self.help_scroll_offset)
            .take(visible_height)
            .cloned()
            .collect();

        let help_text = Text::from(visible_lines);
        frame.render_widget(Paragraph::new(help_text), inner);

        // Scrollbar
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .thumb_style(Style::new().fg(Color::DarkGray));
        let mut scrollbar_state = ScrollbarState::default()
            .content_length(total_lines)
            .position(self.help_scroll_offset);
        frame.render_stateful_widget(scrollbar, area, &mut scrollbar_state);
    }

    fn render_run(&mut self, frame: &mut Frame, area: Rect) {
        let grid = self.grid.lock().unwrap();
        let grid_width = grid.width as u16;
        let grid_height = grid.height as u16;

        if let Some(ref msg) = self.error_msg {
            // Error mode: show grid on top, error bar at bottom
            let grid_area_height = area.height.saturating_sub(2);
            if grid_area_height > 0 {
                let (grid_area, status_area) = if self.fullscreen {
                    let grid_area = Rect::new(area.x, area.y, area.width, grid_area_height);
                    let status_area = Rect::new(area.x, area.y + grid_area_height, area.width, 2);
                    (grid_area, status_area)
                } else {
                    self.compute_centered_areas(area, grid_width, grid_height.saturating_sub(2))
                };

                frame.render_widget(GridView(&grid), grid_area);

                let error_text = Paragraph::new(Line::from(msg.as_str()))
                    .style(Style::new().fg(Color::Red).bg(Color::Black));
                frame.render_widget(error_text, status_area);
            } else {
                let error_text = Paragraph::new(Line::from(msg.as_str()))
                    .style(Style::new().fg(Color::Red).bg(Color::Black));
                frame.render_widget(error_text, area);
            }
        } else if !self.fullscreen && (grid_width < area.width || grid_height < area.height) {
            // Non-fullscreen with smaller grid: center it
            let centered_area = self.compute_centered_rect(area, grid_width, grid_height);
            frame.render_widget(GridView(&grid), centered_area);
        } else {
            // Fullscreen or grid fills terminal
            frame.render_widget(GridView(&grid), area);
        }
    }

    /// Compute a centered Rect within `area` for the given grid dimensions.
    fn compute_centered_rect(&self, area: Rect, grid_width: u16, grid_height: u16) -> Rect {
        let x_offset = if area.width > grid_width {
            (area.width - grid_width) / 2
        } else {
            0
        };
        let y_offset = if area.height > grid_height {
            (area.height - grid_height) / 2
        } else {
            0
        };
        Rect::new(
            area.x + x_offset,
            area.y + y_offset,
            grid_width.min(area.width),
            grid_height.min(area.height),
        )
    }

    /// Compute centered grid and status areas within `area`.
    fn compute_centered_areas(&self, area: Rect, grid_width: u16, grid_area_height: u16) -> (Rect, Rect) {
        let total_height = grid_area_height + 2; // grid + status bar
        let y_offset = if area.height > total_height {
            (area.height - total_height) / 2
        } else {
            0
        };
        let x_offset = if area.width > grid_width {
            (area.width - grid_width) / 2
        } else {
            0
        };

        let w = grid_width.min(area.width);
        let h = grid_area_height.min(area.height.saturating_sub(2));
        let grid_area = Rect::new(area.x + x_offset, area.y + y_offset, w, h);
        let status_area = Rect::new(
            area.x + x_offset,
            grid_area.y + grid_area.height,
            w,
            2.min(area.height - (grid_area.y + grid_area.height - area.y)),
        );

        (grid_area, status_area)
    }

    fn render_open(&mut self, frame: &mut Frame, area: Rect) {
        // Render the editor above the input bar
        let input_bar_height = 1;
        let editor_area = Rect::new(area.x, area.y, area.width, area.height - input_bar_height);
        let input_area = Rect::new(area.x, area.y + editor_area.height, area.width, input_bar_height);

        // Draw a prompt line at the bottom
        let prompt = if let Some(ref err) = self.open_error {
            Line::from(vec![
                Span::styled("Open file: ", Style::new().fg(Color::Cyan)),
                Span::raw(&self.open_input),
                Span::styled(format!("  {}", err), Style::new().fg(Color::Red)),
            ])
        } else {
            Line::from(vec![
                Span::styled("Open file: ", Style::new().fg(Color::Cyan)),
                Span::raw(&self.open_input),
            ])
        };
        let input_widget = Paragraph::new(prompt)
            .style(Style::new().bg(Color::Black));
        frame.render_widget(input_widget, input_area);

        // Render editor above
        self.render_editor(frame, editor_area);
    }
}
