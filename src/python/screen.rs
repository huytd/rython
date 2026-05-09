use pyo3::prelude::*;
use std::sync::{Arc, Mutex};

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
}

impl Screen {
    pub fn from_grid(grid: Arc<Mutex<Grid>>) -> Self {
        Screen {
            grid,
            terminal_ptr: TerminalPtr::null(),
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
            let _ = term.try_draw(|frame| {
                use crate::tui::grid_view::GridView;
                frame.render_widget(GridView(&grid), frame.area());
                Ok::<(), std::io::Error>(())
            });
        }
    }
}
