mod handler;

pub use handler::handle_key_events;

use crossterm::event::{Event as CrosstermEvent, KeyEvent, MouseEvent};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

use crate::app::AppResult;

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

pub struct EventHandler {
    sender: Sender<Event>,
    receiver: Receiver<Event>,
    handler: thread::JoinHandle<()>,
}

impl std::fmt::Debug for EventHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventHandler")
            .field("sender", &"Sender<Event>")
            .field("receiver", &"Receiver<Event>")
            .field("handler", &"JoinHandle<()>")
            .finish()
    }
}

impl EventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();
        let sender_clone = sender.clone();

        let handler = thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                // Check if it's time to send a tick event
                if last_tick.elapsed() >= tick_rate {
                    if sender_clone.send(Event::Tick).is_err() {
                        break; // Exit if the receiver has dropped
                    }
                    last_tick = Instant::now();
                }
                // Non-blocking check for terminal events
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
                thread::sleep(Duration::from_millis(1)); // Small sleep to prevent high CPU usage
            }
        });

        Self {
            sender,
            receiver,
            handler,
        }
    }

    pub fn next(&self) -> Result<Event, mpsc::RecvError> {
        self.receiver.recv()
    }
}
