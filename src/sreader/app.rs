use std::error;
use std::fs;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,

    pub text_array: Vec<String>,

    pub text_current_word: String,

    pub text_current_index: usize,

    pub text_length: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            text_array: Vec::new(),
            text_current_word: "".to_string(),
            text_current_index: 0,
            text_length: 0,
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
        let book: String =
            fs::read_to_string("./assets/lewisCarroll_alicesAdventuresInWonderland.txt")
                .expect("failed to read file");
        self.text_array = book.split(" ").map(|s| s.to_string()).collect();
        self.text_current_word = self.text_array[self.text_current_index.clone()].clone();
        self.text_length = self.text_array.len();
    }
    pub fn increment_word(&mut self) {
        if let Some(res) = self.text_current_index.checked_add(1) {
            self.text_current_index = res;
            self.text_current_word = self.text_array[self.text_current_index.clone()].clone();
        }
    }
    pub fn decrement_word(&mut self) {
        if let Some(res) = self.text_current_index.checked_sub(1) {
            self.text_current_index = res;
            self.text_current_word = self.text_array[self.text_current_index.clone()].clone();
        }
    }
}
