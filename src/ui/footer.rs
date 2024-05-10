use crate::ui::constants::{TEXT_HINT_L1, TEXT_HINT_L2};
use ratatui::{prelude::*, widgets::*};

pub fn render_footer(area: Rect, buf: &mut Buffer) {
    let vertical_layout = Layout::vertical([Constraint::Min(1), Constraint::Min(1)]);
    let [first_line, second_line] = vertical_layout.areas(area);
    Paragraph::new(TEXT_HINT_L1)
        .centered()
        .render(first_line, buf);
    Paragraph::new(TEXT_HINT_L2)
        .centered()
        .render(second_line, buf);
}
