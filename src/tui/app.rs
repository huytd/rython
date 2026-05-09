use std::fs;
use std::sync::{Arc, Mutex};

use crossterm::event::{KeyCode, KeyModifiers};
use pyo3::prelude::*;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};
use ratatui_textarea::TextArea;

use crate::event::Event;
use crate::grid::Grid;
use crate::python::Screen;
use crate::tui::grid_view::GridView;

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
}

impl<'a> App<'a> {
    pub fn new(grid: Arc<Mutex<Grid>>, initial_code: Option<&str>) -> Self {
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
        }
    }

    pub fn should_quit(&self) -> bool {
        self.quit
    }

    pub fn handle_event(&mut self, event: &Event) {
        if let Event::Key(key) = event {
            match self.mode {
                Mode::Editor => self.handle_editor_key(key),
                Mode::Run => self.handle_run_key(key),
                Mode::Open => self.handle_open_key(key),
            }
        }
    }

    fn handle_editor_key(&mut self, key: &crossterm::event::KeyEvent) {
        let is_ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

        match (key.code, is_ctrl) {
            // F5 or Ctrl+R => Run
            (KeyCode::F(5), _) | (KeyCode::Char('r'), true) => {
                self.run_code();
            }
            // Ctrl+O => Open file
            (KeyCode::Char('o'), true) => {
                self.open_input.clear();
                self.open_error = None;
                self.mode = Mode::Open;
            }
            // Esc or Ctrl+C => Quit
            (KeyCode::Esc, _) | (KeyCode::Char('c'), true) => {
                self.quit = true;
            }
            _ => {
                self.textarea.input(*key);
            }
        }
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

    fn run_code(&mut self) {
        self.error_msg = None;
        self.grid.lock().unwrap().clear();

        let code: String = self.textarea.lines().iter().map(|l| l.as_str()).collect::<Vec<_>>().join("\n");

        let grid_clone = self.grid.clone();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            pyo3::Python::attach(|py| -> PyResult<()> {
                let screen = Screen::from_grid(grid_clone);
                let locals = pyo3::types::PyDict::new(py);
                locals.set_item("screen", screen)?;
                let code_cstr = std::ffi::CString::new(&*code).map_err(|e| {
                    pyo3::exceptions::PyValueError::new_err(format!("Invalid code: {}", e))
                })?;
                py.run(code_cstr.as_c_str(), None, Some(&locals))
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
        let title = Line::from(vec![
            Span::styled("pymodo", Style::new().fg(Color::Cyan).bold()),
            Span::raw("  |  "),
            Span::styled("F5 / Ctrl+R: Run", Style::new().fg(Color::DarkGray)),
            Span::raw("  |  "),
            Span::styled("Ctrl+O: Open", Style::new().fg(Color::DarkGray)),
            Span::raw("  |  "),
            Span::styled("Esc / Ctrl+C: Quit", Style::new().fg(Color::DarkGray)),
        ]);

        let block = Block::bordered().title(title);
        let inner = block.inner(area);

        frame.render_widget(block, area);
        frame.render_widget(&self.textarea, inner);
    }

    fn render_run(&mut self, frame: &mut Frame, area: Rect) {
        if let Some(ref msg) = self.error_msg {
            let grid_area_height = area.height.saturating_sub(2);
            if grid_area_height > 0 {
                let grid_area = Rect::new(area.x, area.y, area.width, grid_area_height);
                let status_area = Rect::new(area.x, area.y + grid_area_height, area.width, area.height - grid_area_height);

                let grid = self.grid.lock().unwrap();
                frame.render_widget(GridView(&grid), grid_area);

                let error_text = Paragraph::new(Line::from(msg.as_str()))
                    .style(Style::new().fg(Color::Red).bg(Color::Black));
                frame.render_widget(error_text, status_area);
            } else {
                let error_text = Paragraph::new(Line::from(msg.as_str()))
                    .style(Style::new().fg(Color::Red).bg(Color::Black));
                frame.render_widget(error_text, area);
            }
        } else {
            let grid = self.grid.lock().unwrap();
            frame.render_widget(GridView(&grid), area);
        }
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
