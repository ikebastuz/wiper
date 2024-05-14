use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub enum MessageLevel {
    Info,
    Error,
}

#[derive(Debug)]
pub struct Logger {
    pub messages: VecDeque<(u128, MessageLevel, String)>,
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

impl Logger {
    fn new() -> Self {
        Logger {
            messages: VecDeque::new(),
        }
    }

    pub fn log(&mut self, message: String, level: MessageLevel) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        if self.messages.len() >= 10 {
            self.messages.pop_back();
        }
        self.messages.push_front((timestamp, level, message));
    }
}
