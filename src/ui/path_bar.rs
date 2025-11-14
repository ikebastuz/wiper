use ratatui::{prelude::*, widgets::*};
use std::path::Path;

pub fn render_path_bar(area: Rect, buf: &mut Buffer, current_path: &Path) {
    let full_path = current_path.to_string_lossy().to_string();

    // Truncate if path is too long for the display area
    let max_width = area.width.saturating_sub(2) as usize;
    let display_path = if full_path.len() > max_width {
        format!("...{}", &full_path[full_path.len().saturating_sub(max_width - 3)..])
    } else {
        full_path
    };

    Paragraph::new(display_path)
        .style(Style::default().fg(Color::Cyan))
        .left_aligned()
        .render(area, buf);
}
