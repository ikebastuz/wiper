use ratatui::{prelude::*, widgets::*};

use crate::ui::utils::format_file_size;

pub fn render_chart(area: Rect, buf: &mut Buffer, chart_data: Vec<(String, u64)>) {
    let block = Block::default().padding(Padding::top(1));
    let inner_area = block.inner(area);
    Widget::render(block, area, buf);

    let total_size: u64 = chart_data.iter().map(|(_, size)| *size).sum();
    let percentages: Vec<u16> = chart_data
        .iter()
        .map(|(_, size)| (size * 100 / total_size) as u16)
        .collect();

    let mut constraints = Constraint::from_percentages(percentages.clone());
    constraints.pop();
    constraints.push(Constraint::Fill(1));
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(inner_area);

    for (i, (file_type, size)) in chart_data.iter().enumerate() {
        let mut text = format!("{}: {}", file_type, format_file_size(*size));
        // Hide size from "short" filytypes
        if percentages[i] < 10 {
            text = file_type.to_string();
        }
        let paragraph = Paragraph::new(text)
            .centered()
            .block(Block::default().borders(Borders::ALL));
        paragraph.render(layout[i], buf);
    }
}
