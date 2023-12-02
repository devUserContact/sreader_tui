use crate::sreader::app::{App, AppResult};
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
        // Counter handlers
        KeyCode::Right => {
            app.increment_word();
        }
        KeyCode::Left => {
            app.decrement_word();
        }
        // Other handlers you could add here.
        KeyCode::Char('l') => {
            app.text_load();
        }
        KeyCode::Char(' ') => {
            app.sread_text();
        }
        _ => {}
    }
    Ok(())
}
