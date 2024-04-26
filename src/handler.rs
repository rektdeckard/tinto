use crate::app::{App, AppResult, RoomView, Tab};
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
        KeyCode::Right | KeyCode::Char('l') => {
            if key_event.modifiers != KeyModifiers::SHIFT {
                app.next_view();
            }
        }
        KeyCode::Left | KeyCode::Char('h') => {
            if key_event.modifiers != KeyModifiers::SHIFT {
                app.prev_view();
            }
        }
        KeyCode::Up | KeyCode::Char('k') => app.prev_list_item(),
        KeyCode::Down | KeyCode::Char('j') => app.next_list_item(),
        KeyCode::Enter | KeyCode::Char(' ') => {
            app.activate_current();
        }
        KeyCode::Char('a') | KeyCode::Char('A') => {
            app.view.active_tab = Tab::Areas;
        }
        KeyCode::Char('g') | KeyCode::Char('G') => {
            app.view.active_tab = Tab::Lights;
        }
        KeyCode::Char('r') | KeyCode::Char('R') => match app.view.active_tab {
            Tab::Areas => {
                app.view.room_active_view = RoomView::RoomList;
            }
            _ => {}
        },
        KeyCode::Char('s') | KeyCode::Char('S') => match app.view.active_tab {
            Tab::Areas => {
                app.view.room_active_view = RoomView::SceneList;
            }
            _ => {}
        },
        KeyCode::Char('x') | KeyCode::Char('X') => match app.view.active_tab {
            Tab::Areas => {
                app.view.room_active_view = RoomView::LightPanel;
            }
            _ => {}
        },
        KeyCode::Char('z') | KeyCode::Char('Z') => match app.view.active_tab {
            Tab::Areas => {
                app.view.room_active_view = RoomView::ZoneList;
            }
            _ => {}
        },
        _ => {}
    }
    Ok(())
}
