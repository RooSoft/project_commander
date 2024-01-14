use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::terminal_ui::app::App;

pub fn update(app: &mut App, key_event: KeyEvent) {
    if app.searching {
        update_search_mode(app, key_event)
    } else {
        update_normal_mode(app, key_event)
    }
}

fn update_normal_mode(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            }
        }
        KeyCode::Up | KeyCode::Char('k') => app.select_up(),
        KeyCode::Down | KeyCode::Char('j') => app.select_down(),

        KeyCode::Char('g') => app.select_first(),
        KeyCode::Char('G') => app.select_last(),

        KeyCode::Char('/') => app.search(),

        KeyCode::Enter => app.apply(),
        _ => {}
    };
}

fn update_search_mode(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc => app.stop_search(),

        KeyCode::Up | KeyCode::Char('k') => app.select_up(),
        KeyCode::Down | KeyCode::Char('j') => app.select_down(),

        KeyCode::Enter => app.apply(),

        KeyCode::Char(c) => app.add_to_search(c),

        _ => app.apply()
    };
}
