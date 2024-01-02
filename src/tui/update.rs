use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::tui::app::App;

pub fn update(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            }
        }
        KeyCode::Right | KeyCode::Char('j') => app.increment_counter(),
        KeyCode::Left | KeyCode::Char('k') => app.decrement_counter(),
        KeyCode::Up | KeyCode::Char('h') => app.select_up(),
        KeyCode::Down | KeyCode::Char(';') => app.select_down(),

        KeyCode::Char('g') => app.select_first(),
        KeyCode::Char('G') => app.select_last(),

        KeyCode::Enter => app.apply(),
        _ => {}
    };
}
