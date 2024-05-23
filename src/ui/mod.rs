use crate::fs::DataStore;
use crate::{app::App, fs::DataStoreKey};
use ratatui::prelude::*;
use ratatui::widgets::*;

mod chart;
pub mod constants;
mod content;
mod footer;
mod title;
mod utils;
use constants::TEXT_TITLE;
pub use content::{render_content, DebugData};
pub use footer::render_footer;
pub use title::render_title;

use self::chart::render_chart;
use self::constants::{TEXT_COLOR, TEXT_PRE_DELETED_BG};

impl<S: DataStore<DataStoreKey>> Widget for &mut App<S> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.pre_render();
        let maybe_folder = self.store.get_current_folder();

        let mut chart_data = vec![];

        // Helper data
        let fps = self.fps_counter.update();
        let (spin_left, spin_right) = self.spinner.get_icons(!self.task_manager.is_working);
        let debug = DebugData {
            folders: self.store.get_nodes_len(),
            fps: format!("{:.1}", fps),
            skipped_frames: format!("{:.1}", self.fps_counter.skipped_frames),
            spin_symbol: (spin_left, spin_right),
        };

        // Main wrapper
        let mut title = TEXT_TITLE;
        let mut border_color = TEXT_COLOR;

        if let Some(folder) = maybe_folder {
            if folder.has_error {
                title = "Error";
                border_color = TEXT_PRE_DELETED_BG;
            }
            chart_data = folder.get_chart_data(0.8, 5);
        }

        let block = Block::default()
            .title(format!(" {} {} {} ", spin_left, title, spin_right))
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(border_color)
            .padding(Padding::horizontal(1))
            .border_set(symbols::border::DOUBLE);
        let inner_area = block.inner(area);
        Widget::render(block, area, buf);

        // Layout
        let vertical = Layout::vertical([
            Constraint::Length(2), // Header - 2 lines
            Constraint::Fill(1),   // Content - Fill the rest of the space
            Constraint::Length(4), // Chart - 4 lines
            Constraint::Length(2), // Footer - 2 lines
        ]);
        let [header_area, rest_area, chart_area, footer_area] = vertical.areas(inner_area);

        render_title(header_area, buf, maybe_folder, &self.ui_config);
        render_content(
            rest_area,
            buf,
            maybe_folder,
            &self.ui_config,
            &self.logger,
            &debug,
        );
        render_chart(chart_area, buf, chart_data);
        render_footer(footer_area, buf);
    }
}
