use pyo3::prelude::*;
use std::sync::{Arc, Mutex};

use crate::grid::{Color, Grid};

#[pyclass]
pub struct Screen {
    grid: Arc<Mutex<Grid>>,
}

impl Screen {
    pub fn from_grid(grid: Arc<Mutex<Grid>>) -> Self {
        Screen { grid }
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
}
