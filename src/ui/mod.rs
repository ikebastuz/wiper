use crate::fs::DataStore;
use crate::{app::App, fs::DataStoreKey};
use ratatui::prelude::*;
use ratatui::widgets::*;

pub mod constants;
mod content;
mod footer;
mod title;
mod utils;
pub use content::{render_content, DebugData};
pub use footer::render_footer;
pub use title::render_title;

impl<S: DataStore<DataStoreKey>> Widget for &mut App<S> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.pre_render();
        let fps = self.fps_counter.update();

        let vertical = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(4),
        ]);

        let block = Block::default()
            .title(" Wiper ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .padding(Padding::horizontal(1))
            .border_set(symbols::border::DOUBLE);

        let inner_area = block.inner(area);

        Widget::render(block, area, buf);

        let [header_area, rest_area, footer_area] = vertical.areas(inner_area);

        let maybe_folder = self.store.get_current_folder();

        let debug = DebugData {
            path_stack: self.task_manager.path_buf_stack.lock().unwrap().len(),
            threads: *self.task_manager.running_tasks.lock().unwrap(),
            task_timer: &self.task_manager.task_timer,
            fps: format!("{:.1}", fps),
            skipped_frames: format!("{:.1}", self.fps_counter.skipped_frames),
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
