use crate::fs::SortBy;

pub struct InitConfig {
    pub file_path: Option<String>,
}

impl InitConfig {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<InitConfig, &'static str> {
        args.next();

        Ok(InitConfig {
            file_path: args.next(),
        })
    }
}

#[derive(Debug)]
pub struct UIConfig {
    pub colored: bool,
    pub confirming_deletion: bool,
    pub sort_by: SortBy,
    pub move_to_trash: bool,
    pub open_file: bool,
    pub debug_enabled: bool,
}

pub const EVENT_INTERVAL: u64 = 100;
