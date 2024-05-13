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

#[derive(Debug)]
pub struct DebugData {
    pub path_stack: usize,
    pub threads: usize,
    pub time: u128,
}

pub fn render_content(
    area: Rect,
    buf: &mut Buffer,
    maybe_folder: Option<&Folder>,
    config: &UIConfig,
    logger: &Logger,
    debug_data: &DebugData,
) {
    let horizontal_layout = Layout::horizontal(match config.debug_enabled {
        true => [Constraint::Min(1), Constraint::Min(1)],
        false => [Constraint::Min(1), Constraint::Max(0)],
    });

    let [left_col, right_col] = horizontal_layout.areas(area);

    if let Some(folder) = maybe_folder {
        render_table(left_col, buf, folder, config);
    }

    if config.debug_enabled {
        render_debug_panel(right_col, buf, logger, debug_data);
    }
}

pub fn render_table(area: Rect, buf: &mut Buffer, folder: &Folder, config: &UIConfig) {
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
        area,
        buf,
        &mut TableState::default().with_selected(Some(folder.cursor_index)),
    );
}

pub fn render_debug_panel(area: Rect, buf: &mut Buffer, logger: &Logger, debug_data: &DebugData) {
    let outer_block = Block::default()
        .borders(Borders::ALL)
        .border_set(symbols::border::PROPORTIONAL_TALL)
        .border_style(TEXT_PRE_DELETED_BG)
        .padding(Padding::uniform(1))
        .title("Debug")
        .inner(area);

    let [top, bottom] =
        Layout::vertical([Constraint::Max(5), Constraint::Fill(1)]).areas(outer_block);

    let debug_text = Text::from(format!(
        "Time: {} | Logs: {}\nStack -> {} <-> {} <- Threads",
        debug_data.time,
        logger.messages.len(),
        debug_data.path_stack,
        debug_data.threads
    ));

    Paragraph::new(debug_text).left_aligned().render(top, buf);

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
    StatefulWidget::render(items, bottom, buf, &mut ListState::default());
}
