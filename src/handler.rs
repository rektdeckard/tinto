use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        KeyCode::Tab => {
            if key_event.modifiers == KeyModifiers::SHIFT {
                app.prev_view();
            } else {
                app.next_view();
            }
        }
        KeyCode::Right => {
            if key_event.modifiers != KeyModifiers::SHIFT {
                app.next_view();
            }
        }
        KeyCode::Left => {
            if key_event.modifiers != KeyModifiers::SHIFT {
                app.prev_view();
            }
        }
        KeyCode::Up | KeyCode::Char('k') => app.prev_list_item(),
        KeyCode::Down | KeyCode::Char('j') => app.next_list_item(),
        KeyCode::Enter | KeyCode::Char(' ') => {
            app.activate_current();
        }
        _ => {}
    }
    Ok(())
}
