use crate::ui::constants::TEXT_HINT_L1;
use ratatui::{prelude::*, widgets::*};

use super::utils::color_capital_letter;

pub fn render_footer(area: Rect, buf: &mut Buffer) {
    let block = Block::default().padding(Padding::vertical(1));

    let inner_area = block.inner(area);
    Widget::render(block, area, buf);

    let layout = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Max(13),
        Constraint::Max(6),
        Constraint::Max(6),
    ]);
    let [col_navigate, col_delete, col_sort, col_quit] = layout.areas(inner_area);

    let text_delete = color_capital_letter("Delete - 2x,".into(), None, None);
    let text_sort = color_capital_letter("Sort,".into(), None, None);
    let text_quit = color_capital_letter("Quit".into(), None, None);

    Paragraph::new(TEXT_HINT_L1)
        .left_aligned()
        .render(col_navigate, buf);
    Paragraph::new(text_delete)
        .left_aligned()
        .render(col_delete, buf);
    Paragraph::new(text_sort)
        .left_aligned()
        .render(col_sort, buf);
    Paragraph::new(text_quit)
        .left_aligned()
        .render(col_quit, buf);
}
