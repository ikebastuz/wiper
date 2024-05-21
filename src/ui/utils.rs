use crate::config::UIConfig;
use crate::fs::Folder;
use crate::fs::FolderEntryType;
use crate::ui::constants::{NORMAL_ROW_COLOR, TABLE_SPACE_WIDTH, TEXT_UNKNOWN};
use ratatui::{prelude::*, widgets::*};

use super::constants::{TEXT_HIGHLIGHTED, TEXT_ICON_FOLDER_ASCII};

pub fn format_file_size(size: u64) -> String {
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

pub fn calculate_color(percent: u64, _max_entry_size: u64) -> Color {
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

pub fn value_to_box(value: &bool) -> String {
    match value {
        true => "[x]".to_string(),
        false => "[ ]".to_string(),
    }
}

pub fn folder_to_rows<'a>(folder: &'a Folder, config: &'a UIConfig) -> Vec<Row<'a>> {
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
                true => Text::from(TEXT_ICON_FOLDER_ASCII),
                false => Text::from("  "),
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

pub fn color_capital_letter<'a>(
    text: String,
    prefix: Option<String>,
    postfix: Option<String>,
) -> Line<'a> {
    let mut spans = Vec::new();

    if let Some(pre) = prefix {
        spans.push(Span::raw(pre));
    }

    if let Some(first_char) = text.chars().next() {
        let first_char_upper = first_char.to_uppercase().to_string();
        spans.push(Span::styled(
            first_char_upper,
            Style::default()
                .fg(TEXT_HIGHLIGHTED)
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::UNDERLINED),
        ));
        spans.push(Span::raw(text[first_char.len_utf8()..].to_string()));
    }

    if let Some(post) = postfix {
        spans.push(Span::raw(post));
    }

    Line::from(spans)
}
