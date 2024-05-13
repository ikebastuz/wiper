use std::path::PathBuf;

use crate::app::App;
use crate::fs::DataStore;
use ratatui::prelude::*;

pub mod constants;
mod content;
mod footer;
mod title;
mod utils;
pub use content::{render_content, DebugData};
pub use footer::render_footer;
pub use title::render_title;

impl<S: DataStore<PathBuf>> Widget for &mut App<S> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.pre_render();

        let vertical = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(3),
        ]);
        let [header_area, rest_area, footer_area] = vertical.areas(area);

        let maybe_folder = self.store.get_current_folder();

        let debug = DebugData {
            path_stack: self.task_manager.path_buf_stack.len(),
            threads: self.task_manager.receiver_stack.len(),
            time: self.time,
        };

        render_title(header_area, buf, maybe_folder, &self.ui_config);
        render_content(
            rest_area,
            buf,
            maybe_folder,
            &self.ui_config,
            &self.logger,
            &debug,
        );
        render_footer(footer_area, buf);
    }
}
