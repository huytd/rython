mod event;
mod grid;
mod python;
mod tui;

use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::grid::Grid;
use crate::tui::App;

const DEFAULT_WIDTH: usize = 40;
const DEFAULT_HEIGHT: usize = 25;

struct CliArgs {
    file: Option<String>,
    width: Option<usize>,
    height: Option<usize>,
    fullscreen: bool,
}

fn parse_args() -> CliArgs {
    let args: Vec<String> = std::env::args().collect();
    let mut file = None;
    let mut width = None;
    let mut height = None;
    let mut fullscreen = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--size" => {
                if i + 2 >= args.len() {
                    eprintln!("Error: --size requires WIDTH HEIGHT arguments");
                    std::process::exit(1);
                }
                width = Some(args[i + 1].parse().unwrap_or_else(|_| {
                    eprintln!("Error: invalid width '{}'", args[i + 1]);
                    std::process::exit(1);
                }));
                height = Some(args[i + 2].parse().unwrap_or_else(|_| {
                    eprintln!("Error: invalid height '{}'", args[i + 2]);
                    std::process::exit(1);
                }));
                i += 3;
            }
            "--fullscreen" | "-f" => {
                fullscreen = true;
                i += 1;
            }
            "--help" | "-h" => {
                println!("Usage: rython [OPTIONS] [file.py]");
                println!();
                println!("Options:");
                println!("  --size WIDTH HEIGHT   Set custom screen size (default: {}x{})", DEFAULT_WIDTH, DEFAULT_HEIGHT);
                println!("  --fullscreen, -f      Fill the entire terminal");
                println!("  --help, -h            Show this help message");
                std::process::exit(0);
            }
            arg if arg.starts_with('-') => {
                eprintln!("Error: unknown option '{}'", arg);
                std::process::exit(1);
            }
            _ => {
                if file.is_some() {
                    eprintln!("Error: only one file argument allowed");
                    std::process::exit(1);
                }
                file = Some(args[i].clone());
                i += 1;
            }
        }
    }

    CliArgs {
        file,
        width,
        height,
        fullscreen,
    }
}

fn unsaved_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("Could not determine home directory");
    path.push(".rython");
    path.push("unsaved");
    path
}

fn main() {
    let cli = parse_args();

    if cli.fullscreen && (cli.width.is_some() || cli.height.is_some()) {
        eprintln!("Error: --fullscreen and --size are mutually exclusive");
        std::process::exit(1);
    }

    let initial_code = if let Some(path_str) = &cli.file {
        let path = PathBuf::from(path_str);
        match fs::read_to_string(&path) {
            Ok(contents) => Some(contents),
            Err(e) => {
                eprintln!("Error opening '{}': {}", path.display(), e);
                std::process::exit(1);
            }
        }
    } else {
        // Load unsaved buffer from previous session
        let path = unsaved_path();
        if path.exists() {
            match fs::read_to_string(&path) {
                Ok(contents) => Some(contents),
                Err(e) => {
                    eprintln!("Warning: could not load unsaved buffer: {}", e);
                    None
                }
            }
        } else {
            None
        }
    };

    let (width, height, fullscreen) = if cli.fullscreen {
        // Get terminal size for fullscreen mode
        match crossterm::terminal::size() {
            Ok((w, h)) => (w as usize, h.max(1) as usize, true),
            Err(_) => (DEFAULT_WIDTH, DEFAULT_HEIGHT, false),
        }
    } else {
        (
            cli.width.unwrap_or(DEFAULT_WIDTH),
            cli.height.unwrap_or(DEFAULT_HEIGHT),
            false,
        )
    };

    let grid = Arc::new(Mutex::new(Grid::new(width, height)));
    let initial_code_ref = initial_code.as_deref();
    let initial_path = cli.file.clone();
    let mut app = App::new(grid.clone(), initial_code_ref, initial_path, fullscreen);

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
        let should_run = app.handle_event(&event);

        if should_run {
            app.run_code(terminal);
        }

        if app.should_quit() {
            break;
        }
    }
    Ok(())
}
