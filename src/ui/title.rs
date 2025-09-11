use crate::config::UIConfig;
use crate::fs::Folder;
use ratatui::{prelude::*, widgets::*};

use crate::ui::utils::{format_file_size, value_to_box};

use super::utils::color_capital_letter;

pub fn render_title(
    area: Rect,
    buf: &mut Buffer,
    maybe_folder: Option<&Folder>,
    ui_config: &UIConfig,
) {
    let horizontal_layout = Layout::horizontal([Constraint::Fill(1), Constraint::Max(37)]);
    let [left_col, right_col] = horizontal_layout.areas(area);

    // Folder data
    if let Some(folder) = maybe_folder {
        Paragraph::new(format!(
            "{} | {}",
            folder.title,
            format_file_size(folder.get_size()),
        ))
        .bold()
        .left_aligned()
        .render(left_col, buf);
    }

    // Settings
    let config_layout = Layout::horizontal([Constraint::Max(12), Constraint::Max(11), Constraint::Max(14)]);
    let [col_color, col_trash, col_walk] = config_layout.areas(right_col);
    let walk_status = if ui_config.walk_enabled { "DEEP".into() } else { "SHALLOW".into() };

    let text_color = color_capital_letter(
        "Colored: ".into(),
        None,
        Some(value_to_box(&ui_config.colored)),
    );
    let text_trash = color_capital_letter(
        "Trash: ".into(),
        None,
        Some(value_to_box(&ui_config.move_to_trash)),
    ); 
    let text_walk = color_capital_letter(
        "Walk: ".into(),
        Some(" ".into()), // Spacer to account for left-aligning of walk status display
        Some(walk_status));

    Paragraph::new(text_color)
        .right_aligned()
        .render(col_color, buf);
    Paragraph::new(text_trash)
        .right_aligned()
        .render(col_trash, buf);
    Paragraph::new(text_walk)
        .left_aligned()
        .render(col_walk, buf);
}
