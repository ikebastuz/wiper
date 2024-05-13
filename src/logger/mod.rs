use std::collections::VecDeque;

#[derive(Debug)]
pub enum MessageLevel {
    Info,
    Error,
}

#[derive(Debug)]
pub struct Logger {
    pub messages: VecDeque<(MessageLevel, String)>,
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            messages: VecDeque::new(),
        }
    }

    pub fn log(&mut self, message: String, level: MessageLevel) {
        self.messages.push_front((level, message));
    }
}
