use crate::ui::constants::TEXT_HINT_NAVIGATE;
use ratatui::{prelude::*, widgets::*};

use super::utils::color_capital_letter;

pub fn render_footer(area: Rect, buf: &mut Buffer) {
    let block = Block::default().padding(Padding::top(1));
    let inner_area = block.inner(area);
    Widget::render(block, area, buf);

    let layout = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Max(13),
        Constraint::Max(9),
        Constraint::Max(9),
        Constraint::Max(6),
        Constraint::Max(13),
        Constraint::Max(4),
    ]);
    let [col_navigate, col_version, col_explore, col_refresh, col_sort, col_delete, col_quit] =
        layout.areas(inner_area);

    let version = env!("CARGO_PKG_VERSION");
    let text_version = format!("v:{}", version);
    let text_explore = color_capital_letter("Explore,".into(), None, None);
    let text_refresh = color_capital_letter("Refresh,".into(), None, None);
    let text_sort = color_capital_letter("Sort,".into(), None, None);
    let text_delete = color_capital_letter("Delete - 2x,".into(), None, None);
    let text_quit = color_capital_letter("Quit".into(), None, None);

    Paragraph::new(TEXT_HINT_NAVIGATE)
        .left_aligned()
        .render(col_navigate, buf);
    Paragraph::new(text_version)
        .left_aligned()
        .render(col_version, buf);
    Paragraph::new(text_explore)
        .left_aligned()
        .render(col_explore, buf);
    Paragraph::new(text_refresh)
        .left_aligned()
        .render(col_refresh, buf);
    Paragraph::new(text_sort)
        .left_aligned()
        .render(col_sort, buf);
    Paragraph::new(text_delete)
        .left_aligned()
        .render(col_delete, buf);
    Paragraph::new(text_quit)
        .left_aligned()
        .render(col_quit, buf);
}
