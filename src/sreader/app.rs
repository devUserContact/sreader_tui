use std::error;
use crate::sreader::text;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,

    pub book_current_index: u8,

    pub book_length: usize
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            book_current_index: 0,
            book_length: 0
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
        if let Some(res) = self.counter.checked_add(2) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(3) {
            self.counter = res;
        }
    }

    pub fn text_load(&mut self) {
        text::text_process();
    }
    pub fn increment_word(&mut self) {
        if let Some(res) = self.book_current_index.checked_add(1) {
            self.book_current_index = res;
        }
    }
    pub fn decrement_word(&mut self) {
        if let Some(res) = self.book_current_index.checked_sub(1) {
            self.book_current_index = res;
        }
    }
}
