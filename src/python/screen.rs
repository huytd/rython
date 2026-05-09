use crossterm::event::{self as cevent, Event as CEvent, KeyEvent, KeyCode, KeyEventKind};
use pyo3::prelude::*;
use ratatui::layout::Rect;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::grid::{Color, Grid};

/// Wrapper around a raw pointer to the terminal that asserts Send + Sync.
/// SAFETY: valid only during Python execution. The GIL ensures single-threaded access,
/// and no other code touches the terminal while run_code() is executing.
struct TerminalPtr(*mut ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>);

unsafe impl Send for TerminalPtr {}
unsafe impl Sync for TerminalPtr {}

impl TerminalPtr {
    fn null() -> Self {
        TerminalPtr(std::ptr::null_mut())
    }

    #[allow(clippy::mut_from_ref)]
    fn as_mut(&self) -> Option<&mut ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>> {
        if self.0.is_null() {
            None
        } else {
            // SAFETY: caller guarantees the pointer is valid for the duration of use.
            Some(unsafe { &mut *self.0 })
        }
    }
}

#[pyclass]
pub struct Screen {
    grid: Arc<Mutex<Grid>>,
    terminal_ptr: TerminalPtr,
    fullscreen: bool,
}

impl Screen {
    pub fn from_grid(grid: Arc<Mutex<Grid>>, fullscreen: bool) -> Self {
        Screen {
            grid,
            terminal_ptr: TerminalPtr::null(),
            fullscreen,
        }
    }

    /// SAFETY: the pointer must remain valid for the lifetime of this Screen.
    pub unsafe fn set_terminal_ptr(
        &mut self,
        ptr: *mut ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
    ) {
        self.terminal_ptr = TerminalPtr(ptr);
    }
}

#[pymethods]
impl Screen {
    fn set(&self, x: usize, y: usize, ch: &str, fg: &str, bg: &str) -> PyResult<()> {
        let c = ch.chars().next().unwrap_or(' ');
        let fg_color = Color::from_name(fg).map_err(|e| pyo3::exceptions::PyValueError::new_err(e))?;
        let bg_color = Color::from_name(bg).map_err(|e| pyo3::exceptions::PyValueError::new_err(e))?;
        self.grid.lock().unwrap().set(x, y, c, fg_color, bg_color);
        Ok(())
    }

    #[pyo3(name = "print")]
    fn print_(&self, text: &str, x: usize, y: usize, fg: &str, bg: &str) -> PyResult<()> {
        let fg_color = Color::from_name(fg).map_err(|e| pyo3::exceptions::PyValueError::new_err(e))?;
        let bg_color = Color::from_name(bg).map_err(|e| pyo3::exceptions::PyValueError::new_err(e))?;
        self.grid.lock().unwrap().print(text, x, y, fg_color, bg_color);
        Ok(())
    }

    fn clear(&self) {
        self.grid.lock().unwrap().clear();
    }

    fn scroll(&self, n: usize) {
        self.grid.lock().unwrap().scroll(n);
    }

    fn refresh(&self) {
        if let Some(term) = self.terminal_ptr.as_mut() {
            let grid = self.grid.lock().unwrap().clone();
            let fullscreen = self.fullscreen;
            let _ = term.try_draw(|frame| {
                use crate::tui::grid_view::GridView;
                let area = frame.area();
                let grid_width = grid.width as u16;
                let grid_height = grid.height as u16;

                let render_area = if fullscreen || (grid_width >= area.width && grid_height >= area.height) {
                    area
                } else {
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
                };

                frame.render_widget(GridView(&grid), render_area);
                Ok::<(), std::io::Error>(())
            });
        }
    }

    /// Poll for a keypress from crossterm's event queue (non-blocking).
    /// Returns the key name as a string, or "none" if no key was pressed.
    fn read_key(&self) -> PyResult<String> {
        if cevent::poll(Duration::from_millis(0)).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))? {
            match cevent::read().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))? {
                CEvent::Key(KeyEvent { code, kind: KeyEventKind::Press, .. }) => {
                    Ok(key_to_name(code))
                }
                _ => Ok("none".to_string()),
            }
        } else {
            Ok("none".to_string())
        }
    }
}

fn key_to_name(code: KeyCode) -> String {
    match code {
        KeyCode::Char(c) => c.to_string(),
        KeyCode::Up => "up".to_string(),
        KeyCode::Down => "down".to_string(),
        KeyCode::Left => "left".to_string(),
        KeyCode::Right => "right".to_string(),
        KeyCode::Enter => "enter".to_string(),
        KeyCode::Esc => "esc".to_string(),
        KeyCode::Tab => "tab".to_string(),
        KeyCode::Backspace => "backspace".to_string(),
        KeyCode::Delete => "delete".to_string(),
        KeyCode::Insert => "insert".to_string(),
        KeyCode::Home => "home".to_string(),
        KeyCode::End => "end".to_string(),
        KeyCode::PageUp => "pageup".to_string(),
        KeyCode::PageDown => "pagedown".to_string(),
        KeyCode::F(n) => format!("f{}", n),
        KeyCode::CapsLock => "capslock".to_string(),
        KeyCode::ScrollLock => "scrolllock".to_string(),
        KeyCode::NumLock => "numlock".to_string(),
        KeyCode::PrintScreen => "printscreen".to_string(),
        KeyCode::Pause => "pause".to_string(),
        KeyCode::Menu => "menu".to_string(),
        KeyCode::Null => "none".to_string(),
        _ => "none".to_string(),
    }
}
