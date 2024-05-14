use crate::ui::constants::{TEXT_HINT_L1, TEXT_HINT_L2};
use ratatui::{prelude::*, widgets::*};

pub fn render_footer(area: Rect, buf: &mut Buffer) {
    let block = Block::default().padding(Padding::vertical(1));

    let inner_area = block.inner(area);
    Widget::render(block, area, buf);

    let vertical_layout = Layout::vertical([Constraint::Max(1), Constraint::Max(1)]);
    let [first_line, second_line] = vertical_layout.areas(inner_area);
    Paragraph::new(TEXT_HINT_L1)
        .left_aligned()
        .render(first_line, buf);
    Paragraph::new(TEXT_HINT_L2)
        .left_aligned()
        .render(second_line, buf);
}
