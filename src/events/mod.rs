mod handler;

pub use handler::handle_key_events;

use crossterm::event::{Event as CrosstermEvent, KeyEvent, MouseEvent};
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::{Duration, Instant};

/// Terminal events.
#[derive(Clone, Copy, Debug)]
pub enum Event {
    /// Terminal tick.
    Tick,
    /// Key press.
    Key(KeyEvent),
    /// Mouse click/scroll.
    Mouse(MouseEvent),
    /// Terminal resize.
    Resize(u16, u16),
}

#[derive(Debug)]
pub struct EventHandler {
    receiver: Receiver<Event>,
}

impl EventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();
        let sender_clone = sender.clone();

        thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                if last_tick.elapsed() >= tick_rate {
                    if sender_clone.send(Event::Tick).is_err() {
                        break; // Exit if the receiver has dropped
                    }
                    last_tick = Instant::now();
                }
                if crossterm::event::poll(Duration::from_millis(1)).unwrap() {
                    if let Ok(crossterm_event) = crossterm::event::read() {
                        match crossterm_event {
                            CrosstermEvent::Key(key) => {
                                let _ = sender_clone.send(Event::Key(key));
                            }
                            CrosstermEvent::Mouse(mouse) => {
                                let _ = sender_clone.send(Event::Mouse(mouse));
                            }
                            CrosstermEvent::Resize(x, y) => {
                                let _ = sender_clone.send(Event::Resize(x, y));
                            }
                            _ => {}
                        }
                    }
                }
                thread::sleep(Duration::from_millis(10));
            }
        });

        Self { receiver }
    }

    pub fn next(&self) -> Result<Event, mpsc::RecvError> {
        self.receiver.recv()
    }
}
