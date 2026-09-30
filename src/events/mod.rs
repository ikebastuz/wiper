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
                // Block on input for whatever's left of this tick, instead
                // of a short poll plus an extra fixed sleep every loop.
                let timeout = tick_rate.saturating_sub(last_tick.elapsed());
                if let Ok(true) = crossterm::event::poll(timeout) {
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

                if last_tick.elapsed() >= tick_rate {
                    if sender_clone.send(Event::Tick).is_err() {
                        break; // Exit if the receiver has dropped
                    }
                    last_tick = Instant::now();
                }
            }
        });

        Self { receiver }
    }

    pub fn next(&self) -> Result<Event, mpsc::RecvError> {
        self.receiver.recv()
    }
}
