//
//
//

use crossterm::event::{KeyCode, KeyEvent};

use crate::app::App;

pub fn update(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc => app.quit(),
        KeyCode::Down => {},
        KeyCode::Up => {}
        _ => {}
    };
}
