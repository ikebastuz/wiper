use std::collections::{HashMap, VecDeque};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub enum MessageLevel {
    Info,
    Error,
}

#[derive(Debug)]
pub struct Logger {
    pub messages: VecDeque<(u128, MessageLevel, String)>,
    timers: HashMap<String, SystemTime>,
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
            timers: HashMap::new(),
        }
    }

    pub fn log(&mut self, message: String) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        if self.messages.len() >= 30 {
            self.messages.pop_back();
        }
        self.messages
            .push_front((timestamp, MessageLevel::Info, message));
    }

    pub fn start_timer(&mut self, name: &str) {
        if !self.timers.contains_key(name) {
            let timestamp = SystemTime::now();
            self.timers.insert(name.to_string(), timestamp);
        }
    }

    pub fn stop_timer(&mut self, name: &str) {
        if let Some(start_time) = self.timers.remove(name) {
            let diff = SystemTime::now()
                .duration_since(start_time)
                .expect("Time went backwards")
                .as_secs_f64();
            self.log(format!("[{}]: {:.1}s", name, diff));
        }
    }
}
