use std::error;

use std::time::SystemTime;
use tokio::sync::mpsc::Receiver;
/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: i8,

    pub time: SystemTime,
    pub receiver_stack: Vec<Receiver<i8>>,
    pub attempts_to_read: u64,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            time: SystemTime::now(),
            receiver_stack: vec![],
            attempts_to_read: 0,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        let mut idx = 0;
        while idx < self.receiver_stack.len() {
            self.attempts_to_read += 1;
            match self.receiver_stack[idx].try_recv() {
                Ok(result) => {
                    self.counter += result;
                    self.receiver_stack.remove(idx);
                }
                Err(_) => {
                    idx += 1;
                }
            }
        }
        self.time = SystemTime::now()
    }

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
}
