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
        KeyCode::Up | KeyCode::Char('k') => {
            app.on_cursor_up();
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.on_cursor_down();
        }
        KeyCode::Enter => {
            app.on_enter();
        }
        KeyCode::Backspace => {
            app.on_backspace();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            } else {
                app.on_toggle_coloring();
            }
        }
        KeyCode::Char('s') => {
            app.on_toggle_sorting();
        }
        KeyCode::Char('d') => {
            app.on_delete();
        }
        KeyCode::Char('t') => {
            app.on_toggle_move_to_trash();
        }
        // Counter handlers
        KeyCode::Right => {}
        KeyCode::Left => {
            // let (sender, receiver) = mpsc::channel(1);
            // tokio::spawn(async move {
            //     tokio::time::sleep(Duration::from_secs(1)).await;
            //     let _ = sender.send(-1).await;
            // });
            //
            // app.receiver_stack.push(receiver);
        }
        _ => {}
    }
    Ok(())
}
