use crossterm::event::{self, Event as CEvent, KeyEvent, KeyEventKind};
use std::time::Duration;

pub enum Event {
    Key(KeyEvent),
    Resize(u16, u16),
    Tick,
}

pub fn poll() -> Event {
    if event::poll(Duration::from_millis(100)).unwrap_or(false) {
        match event::read().unwrap_or_else(|_| CEvent::Key(KeyEvent::new(crossterm::event::KeyCode::Enter, crossterm::event::KeyModifiers::NONE))) {
            CEvent::Key(key) if key.kind == KeyEventKind::Press => return Event::Key(key),
            CEvent::Resize(w, h) => return Event::Resize(w, h),
            _ => {}
        }
    }
    Event::Tick
}
