use std::error;

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

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
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
        // self.output = SendCommand(self.input.as_str());
        self.output = String::from(self.input.as_str());
        self.input = String::new();
        self.cursor_position = 0;
    }


    fn clamp_cursor(&mut self, new_pos: usize) -> usize {
        new_pos.clamp(0, self.input.len())
    }
}
