use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use std::thread;
use std::time::Duration;
use tokio::sync::mpsc;

enum ComputationMessage {
    Result(u8),
    // Add other message types as needed
}

pub async fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Counter handlers
        KeyCode::Right => {
            let (sender, receiver) = mpsc::channel(1);
            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_secs(1)).await;
                let _ = sender.send(1).await;
            });

            app.receiver_stack.push(receiver);
        }
        KeyCode::Left => {
            let (sender, receiver) = mpsc::channel(1);
            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_secs(1)).await;
                let _ = sender.send(-1).await;
            });

            app.receiver_stack.push(receiver);
        }
        _ => {}
    }
    Ok(())
}
