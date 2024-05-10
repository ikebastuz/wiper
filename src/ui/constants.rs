use ratatui::{prelude::*, style::palette::tailwind};

pub const NORMAL_ROW_COLOR: Color = tailwind::SLATE.c950;
pub const TEXT_COLOR: Color = tailwind::SLATE.c200;
pub const TABLE_HEADER_FG: Color = tailwind::SLATE.c200;
pub const TABLE_HEADER_BG: Color = tailwind::SLATE.c900;
pub const TEXT_SELECTED_BG: Color = tailwind::SLATE.c700;
pub const TEXT_PRE_DELETED_BG: Color = tailwind::RED.c600;
pub const TABLE_SPACE_WIDTH: usize = 40;

// Texts
pub const TEXT_UNKNOWN: &str = "N/A";
pub const TEXT_PARENT_DIR: &str = "..";
pub const TEXT_TITLE: &str = "Wiper";
pub const TEXT_HINT_L1: &str = "↓↑ - move | \"Enter\" - select | \"Backspace\" - parent";
pub const TEXT_HINT_L2: &str =
    "\"d-d\" - delete | \"s\" - sort | \"c\" - color | \"t\" - trash | \"q\" - exit";
