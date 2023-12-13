use std::error;
use crate::game::Backend;
use ratatui::widgets::ListState;

#[derive(Debug)]
pub enum InputMode {
    Normal,
    Editing,
}

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,
    /// input
    pub input: String,
    /// output
    pub output: String,
    /// input mode
    pub input_mode: InputMode,
    /// cursor position
    pub cursor_position: usize,
    /// history
    pub history: Vec<String>,
    /// backend
    pub backend: Backend,
    /// Selected Index in Inventory
    pub inventory_state: ListState,
    /// Whether or not the game is ended
    pub game_ended: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            input: String::new(),
            output: String::new(),
            input_mode: InputMode::Editing,
            cursor_position: 0,
            history: Vec::new(),
            backend: Backend::new(),
            inventory_state: ListState::default(),
            game_ended: false,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn move_cursor_right(&mut self) {
        let new_pos = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(new_pos);
    }

    pub fn move_cursor_left(&mut self) {
        let new_pos = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(new_pos);
    }

    pub fn enter_char(&mut self, new_char: char) {
        self.input.insert(self.cursor_position, new_char);

        self.move_cursor_right();
    }

    pub fn delete_char(&mut self) {
        if self.cursor_position == 0 {
            return;
        }
        let before = self.input.chars().take(self.cursor_position - 1);
        let after = self.input.chars().skip(self.cursor_position);
        self.input = before.chain(after).collect();
        self.move_cursor_left();
    }

    pub fn submit_command(&mut self) {
        let response = self.backend.send_command(self.input.as_str());
		if response.status == crate::game::Status::Exit {
			self.game_ended = true;
		}
		self.output = response.response;
        self.input = String::new();
        self.cursor_position = 0;
    }

    pub fn move_inventory_up(&mut self) {
        if self.backend.get_inventory().len() == 0 {
            self.inventory_state.select(None);
            return;
        }
        let new_pos = self.inventory_state.selected().unwrap_or(0).saturating_sub(1);
        self.inventory_state.select(Some(new_pos));
    }

    pub fn move_inventory_down(&mut self) {
        if self.backend.get_inventory().len() == 0 {
            self.inventory_state.select(None);
            return;
        }
        let new_pos = self.inventory_state.selected().unwrap_or(0).saturating_add(1);
        let new_pos = new_pos.clamp(0, self.backend.get_inventory().len() - 1);
        self.inventory_state.select(Some(new_pos));
    }

    pub fn clamp_inventory(&mut self) {
        if self.backend.get_inventory().len() == 0 {
            self.inventory_state.select(None);
            return;
        }
        let new_pos = self.inventory_state.selected().unwrap_or(0);
        let new_pos = new_pos.clamp(0, self.backend.get_inventory().len() - 1);
        self.inventory_state.select(Some(new_pos));
    }


    fn clamp_cursor(&mut self, new_pos: usize) -> usize {
        new_pos.clamp(0, self.input.len())
    }
}
