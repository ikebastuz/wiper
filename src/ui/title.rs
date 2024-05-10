use std::fmt::Debug;

use crate::config::UIConfig;
use crate::fs::Folder;
use ratatui::{prelude::*, widgets::*};

use crate::ui::utils::{format_file_size, value_to_box};

const TEXT_TITLE: &str = "Wiper";

#[derive(Debug)]
pub struct DebugData {
    pub path_stack: usize,
    pub threads: usize,
    pub time: u128,
}

pub fn render_title(
    area: Rect,
    buf: &mut Buffer,
    maybe_folder: Option<&Folder>,
    config: &UIConfig,
    debug_data: &DebugData,
) {
    let horizontal_layout = Layout::vertical([Constraint::Min(1), Constraint::Min(1)]);
    let [top_row, debug_row] = horizontal_layout.areas(area);

    let vertical_layout = Layout::horizontal([Constraint::Min(1), Constraint::Min(1)]);
    let [left_col, right_col] = vertical_layout.areas(top_row);

    if let Some(folder) = maybe_folder {
        Paragraph::new(format!(
            "{} | {} | {}",
            TEXT_TITLE,
            folder.title,
            format_file_size(folder.get_size()),
        ))
        .bold()
        .left_aligned()
        .render(left_col, buf);
    }

    let config_text = Text::from(format!(
        "Colored: {} | Trash: {}",
        value_to_box(&config.colored),
        value_to_box(&config.move_to_trash)
    ));
    Paragraph::new(config_text)
        .right_aligned()
        .render(right_col, buf);

    // Debug
    let debug_text = Text::from(format!(
        "Debug | Stack -> {} <-> {} <- Threads | Time {}",
        debug_data.path_stack, debug_data.threads, debug_data.time
    ));
    Paragraph::new(debug_text)
        .left_aligned()
        .render(debug_row, buf);
}
