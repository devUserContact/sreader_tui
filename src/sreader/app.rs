use std::error;
use std::fs;
use std::thread::sleep;
use std::time::Duration;
use std::u64;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub counter: u8,
    pub text_array: Vec<String>,
    pub text_current_word: String,
    pub text_current_index: usize,
    pub text_length: usize,
    pub text_play_on: bool,
    pub text_read_rate: u64,
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
            text_play_on: false,
            text_read_rate: 10,
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

    pub fn text_load(&mut self) {
        let book: String =
            fs::read_to_string("./assets/lewisCarroll_alicesAdventuresInWonderland.txt")
                .expect("failed to read file");
        self.text_array = book.split_whitespace().map(|s| s.to_string()).collect();
        self.text_current_word = self.text_array[self.text_current_index.clone()].clone();
        self.text_length = self.text_array.len();
    }
    pub fn sread_text(&mut self) {
        self.text_play_on = true;
        if self.text_play_on == true {
            if let Some(res) = self.text_current_index.checked_add(1) {
                sleep(Duration::from_secs(1));
                self.text_current_index = res;
                self.text_current_word = self.text_array[self.text_current_index.clone()].clone();
            }
        } else {

        }
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
