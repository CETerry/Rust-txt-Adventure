use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC`
        KeyCode::Esc => {
            app.quit();
        }
        KeyCode::Backspace => {
            app.delete_char();
        }
        KeyCode::Enter => {
            app.submit_command();
        }
        KeyCode::Left => {
            app.move_cursor_left();
        }
        KeyCode::Right => {
            app.move_cursor_right();
        }
        KeyCode::Char(c) => {
            app.enter_char(c);
        }
        KeyCode::Up => {
            app.move_inventory_up();
        }
        KeyCode::Down => {
            app.move_inventory_down();
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
