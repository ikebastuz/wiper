use crate::app::App;
use crate::fs::Folder;
use crate::fs::FolderEntryType;
use crate::fs::SortBy;
use ratatui::{prelude::*, style::palette::tailwind, widgets::*};

const NORMAL_ROW_COLOR: Color = tailwind::SLATE.c950;
const TEXT_COLOR: Color = tailwind::SLATE.c200;
const TABLE_HEADER_FG: Color = tailwind::SLATE.c200;
const TABLE_HEADER_BG: Color = tailwind::SLATE.c900;
const TEXT_SELECTED_BG: Color = tailwind::SLATE.c700;
const TEXT_PRE_DELETED_BG: Color = tailwind::RED.c600;
const TABLE_SPACE_WIDTH: usize = 40;

// Texts
pub const TEXT_UNKNOWN: &str = "N/A";
pub const TEXT_PARENT_DIR: &str = "..";
const TEXT_TITLE: &str = "Space inspector";
const TEXT_HINT_L1: &str = "↓↑ - move | \"Enter\" - select | \"Backspace\" - parent";
const TEXT_HINT_L2: &str =
    "\"d-d\" - delete | \"s\" - sort | \"c\" - color | \"t\" - trash | \"q\" - exit";

#[derive(Debug)]
pub struct UIConfig {
    pub colored: bool,
    pub confirming_deletion: bool,
    pub sort_by: SortBy,
    pub move_to_trash: bool,
    pub open_file: bool,
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(3),
        ]);
        let [header_area, rest_area, footer_area] = vertical.areas(area);

        let maybe_folder = self.get_current_folder();

        render_title(header_area, buf, maybe_folder, &self.ui_config);
        render_table(rest_area, buf, maybe_folder, &self.ui_config);
        render_footer(footer_area, buf);
    }
}

fn value_to_box(value: &bool) -> String {
    match value {
        true => "[x]".to_string(),
        false => "[ ]".to_string(),
    }
}

fn render_title(area: Rect, buf: &mut Buffer, maybe_folder: Option<&Folder>, config: &UIConfig) {
    let vertical_layout = Layout::horizontal([Constraint::Min(1), Constraint::Min(1)]);
    let [left, right] = vertical_layout.areas(area);

    if let Some(folder) = maybe_folder {
        Paragraph::new(format!(
            "{} | {} | {}",
            TEXT_TITLE,
            folder.title,
            format_file_size(folder.get_size()),
        ))
        .bold()
        .centered()
        .render(left, buf);
    }

    let config_text = Text::from(format!(
        "Colored: {} | Trash: {}",
        value_to_box(&config.colored),
        value_to_box(&config.move_to_trash)
    ));
    Paragraph::new(config_text)
        .right_aligned()
        .render(right, buf);
}

fn render_table(area: Rect, buf: &mut Buffer, maybe_folder: Option<&Folder>, config: &UIConfig) {
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
            area,
            buf,
            &mut TableState::default().with_selected(Some(folder.cursor_index)),
        );
    }
}

fn folder_to_rows<'a>(folder: &'a Folder, config: &'a UIConfig) -> Vec<Row<'a>> {
    let max_entry_size = folder.get_max_entry_size();

    folder
        .to_list()
        .iter()
        .map(|item| {
            let (item_size, bar, color) = match item.size {
                Some(size) => {
                    let percent = if max_entry_size == 0 {
                        0
                    } else {
                        (size * TABLE_SPACE_WIDTH as u64 / max_entry_size).div_euclid(1)
                    };
                    let mut b = String::new();
                    let color = calculate_color(percent, max_entry_size);
                    for _ in 0..percent {
                        b.push('█');
                    }
                    (Text::from(format_file_size(size)), Text::from(b), color)
                }
                None => (Text::from(TEXT_UNKNOWN), Text::from(" "), NORMAL_ROW_COLOR),
            };
            let prefix = match item.kind == FolderEntryType::Folder {
                true => Text::from("[ ]"),
                false => Text::from("   "),
            };

            let mut bar_style = Style::default();
            if config.colored {
                bar_style = bar_style.fg(color);
            }

            Row::new(vec![
                prefix,
                Text::from(item.title.clone()),
                item_size,
                bar.style(bar_style),
            ])
        })
        .collect()
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    let vertical_layout = Layout::vertical([Constraint::Min(1), Constraint::Min(1)]);
    let [first_line, second_line] = vertical_layout.areas(area);
    Paragraph::new(TEXT_HINT_L1)
        .centered()
        .render(first_line, buf);
    Paragraph::new(TEXT_HINT_L2)
        .centered()
        .render(second_line, buf);
}

fn format_file_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if size >= TB {
        format!("{:.2} TB", size as f64 / TB as f64)
    } else if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} bytes", size)
    }
}

fn calculate_color(percent: u64, _max_entry_size: u64) -> Color {
    let colors = [
        Color::Rgb(0, 128, 0),    // Green
        Color::Rgb(50, 205, 50),  // LimeGreen
        Color::Rgb(173, 255, 47), // GreenYellow
        Color::Rgb(255, 255, 0),  // Yellow
        Color::Rgb(255, 165, 0),  // Orange
        Color::Rgb(255, 0, 0),    // Red
    ];

    let index =
        ((percent as f64 / TABLE_SPACE_WIDTH as f64) * (colors.len() - 1) as f64).round() as usize;

    colors[index]
}
