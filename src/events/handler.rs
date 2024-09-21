use crate::app::{App, AppResult};
use crate::fs::{DataStore, DataStoreKey};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

macro_rules! handle_key_event {
    ($app:expr, $key_event:expr, $($pattern:pat => $action:expr),+) => {
        match $key_event {
            $(
                KeyEvent { code: $pattern, kind: KeyEventKind::Press, .. } => {
                    $action;
                }
            )+
            _ => {}
        }
    };
}

pub fn handle_key_events<S: DataStore<DataStoreKey>>(
    key_event: KeyEvent,
    app: &mut App<S>,
) -> AppResult<()> {
    handle_key_event!(
        app,
        key_event,
        KeyCode::Enter     | KeyCode::Right | KeyCode::Char('l') => app.on_enter(),
        KeyCode::Backspace | KeyCode::Left  | KeyCode::Char('h') => app.on_backspace(),
        KeyCode::Up   | KeyCode::Char('k') => app.on_cursor_up(),
        KeyCode::Down | KeyCode::Char('j') => app.on_cursor_down(),
        KeyCode::Esc       => app.on_escape(),
        KeyCode::Char('t') => app.on_toggle_move_to_trash(),
        KeyCode::Char('q') => app.quit(),
        KeyCode::Char('s') => app.on_toggle_sorting(),
        KeyCode::Char('e') => app.on_open_file_explorer(),
        KeyCode::Char('r') => app.reset(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {app.quit()} 
            else { app.on_toggle_coloring()}
        },
        KeyCode::Char('d') => {
            if key_event.modifiers == KeyModifiers::CONTROL {app.toggle_debug()} 
            else {app.on_delete()}
        }
    );
    Ok(())
}