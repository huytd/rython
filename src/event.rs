use crossterm::event::{self, Event as CEvent, KeyEvent, KeyEventKind};
use std::time::Duration;

pub enum Event {
    Key(KeyEvent),
    Tick,
}

pub fn poll() -> Event {
    if event::poll(Duration::from_millis(100)).unwrap_or(false) {
        if let CEvent::Key(key) = event::read().unwrap_or_else(|_| CEvent::Key(KeyEvent::new(crossterm::event::KeyCode::Enter, crossterm::event::KeyModifiers::NONE))) {
            if key.kind == KeyEventKind::Press {
                return Event::Key(key);
            }
        }
    }
    Event::Tick
}
