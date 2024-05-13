use crate::config::UIConfig;
use crate::fs::Folder;
use ratatui::{prelude::*, widgets::*};

use crate::ui::utils::{format_file_size, value_to_box};

pub fn render_title(
    area: Rect,
    buf: &mut Buffer,
    maybe_folder: Option<&Folder>,
    ui_config: &UIConfig,
) {
    let vertical_layout = Layout::vertical([Constraint::Max(1)]);
    let [top_row] = vertical_layout.areas(area);

    let horizontal_layout = Layout::horizontal([Constraint::Min(1), Constraint::Min(1)]);
    let [left_col, right_col] = horizontal_layout.areas(top_row);

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

    let config_text = Text::from(format!(
        "Colored: {} | Trash: {}",
        value_to_box(&ui_config.colored),
        value_to_box(&ui_config.move_to_trash)
    ));
    Paragraph::new(config_text)
        .right_aligned()
        .render(right_col, buf);
}
