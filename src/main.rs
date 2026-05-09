mod event;
mod grid;
mod python;
mod tui;

use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::grid::Grid;
use crate::tui::App;

fn main() {
    let initial_code = if std::env::args().len() > 2 {
        eprintln!("Usage: pymodo [file.py]");
        std::process::exit(1);
    } else if std::env::args().len() == 2 {
        let path = PathBuf::from(std::env::args().nth(1).unwrap());
        match fs::read_to_string(&path) {
            Ok(contents) => Some(contents),
            Err(e) => {
                eprintln!("Error opening '{}': {}", path.display(), e);
                std::process::exit(1);
            }
        }
    } else {
        None
    };

    let grid = Arc::new(Mutex::new(Grid::new()));
    let initial_code_ref = initial_code.as_deref();
    let mut app = App::new(grid.clone(), initial_code_ref);

    let mut terminal = ratatui::init();

    let result = run(&mut terminal, &mut app);
    ratatui::restore();

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(
    terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| app.render(frame))?;

        let event = crate::event::poll();
        app.handle_event(&event);

        if app.should_quit() {
            break;
        }
    }
    Ok(())
}
