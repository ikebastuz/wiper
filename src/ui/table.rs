use crate::config::UIConfig;
use crate::fs::Folder;
use crate::fs::SortBy;
use crate::logger::Logger;
use crate::logger::MessageLevel;
use ratatui::{prelude::*, widgets::*};

use crate::ui::constants::{
    NORMAL_ROW_COLOR, TABLE_HEADER_BG, TABLE_HEADER_FG, TABLE_SPACE_WIDTH, TEXT_COLOR,
    TEXT_PRE_DELETED_BG, TEXT_SELECTED_BG,
};
use crate::ui::utils::folder_to_rows;

pub fn render_table(
    area: Rect,
    buf: &mut Buffer,
    maybe_folder: Option<&Folder>,
    config: &UIConfig,
    logger: &Logger,
) {
    let horizontal_layout_schema = match config.debug_enabled {
        true => [Constraint::Min(1), Constraint::Min(1)],
        false => [Constraint::Min(1), Constraint::Max(0)],
    };

    let horizontal_layout = Layout::horizontal(horizontal_layout_schema);
    let [left_col, right_col] = horizontal_layout.areas(area);

    if let Some(folder) = maybe_folder {
        let block = Block::default()
            .borders(Borders::ALL)
            .fg(TEXT_COLOR)
            .bg(NORMAL_ROW_COLOR);

        let header_style = Style::default().fg(TABLE_HEADER_FG).bg(TABLE_HEADER_BG);
        let selected_style = if config.confirming_deletion {
            Style::default().bg(TEXT_PRE_DELETED_BG)
        } else {
            Style::default().bg(TEXT_SELECTED_BG)
        };

        let header_titles = match config.sort_by {
            SortBy::Title => ["", "Name ↓", "Size", "Space"],
            SortBy::Size => ["", "Name", "Size ↓", "Space"],
        };

        let header = header_titles
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1);

        let rows = folder_to_rows(&folder, &config);

        let table = Table::new(
            rows,
            [
                Constraint::Length(3),
                Constraint::Length(40),
                Constraint::Length(20),
                Constraint::Length(TABLE_SPACE_WIDTH as u16),
            ],
        )
        .block(block)
        .header(header)
        .highlight_symbol(">>> ")
        .highlight_style(selected_style)
        .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(
            table,
            left_col,
            buf,
            &mut TableState::default().with_selected(Some(folder.cursor_index)),
        );
    }

    if config.debug_enabled {
        let items: Vec<ListItem> = logger
            .messages
            .iter()
            .enumerate()
            .map(|(_i, (level, message))| {
                let style = Style::default();
                let style = match level {
                    MessageLevel::Info => style.fg(TEXT_COLOR),
                    MessageLevel::Error => style.fg(TEXT_PRE_DELETED_BG),
                };
                ListItem::from(message.clone()).style(style)
            })
            .collect();

        let items = List::new(items);
        StatefulWidget::render(items, right_col, buf, &mut ListState::default());
    }
}
