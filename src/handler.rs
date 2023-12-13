use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC`
        KeyCode::Esc => {
            if app.output == "Are you sure you want to quit? (Press ESC again to quit)" {
                app.quit();
            } else {
                app.output = String::from("Are you sure you want to quit? (Press ESC again to quit)");
            }
        }
        KeyCode::Backspace => {
            if !app.game_ended {
                app.delete_char();
            }
        }
        KeyCode::Enter => {
            if !app.game_ended {
                app.submit_command();
            }
        }
        KeyCode::Left => {
            app.move_cursor_left();
        }
        KeyCode::Right => {
            app.move_cursor_right();
        }
        KeyCode::Char(c) => {
            if !app.game_ended {
                app.enter_char(c);
            }
        }
        KeyCode::Up => {
            if !app.game_ended {
                app.move_inventory_up();
            }
        }
        KeyCode::Down => {
            if !app.game_ended {
                app.move_inventory_down();
            }
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
