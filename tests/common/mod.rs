use std::thread;
use std::time::Duration;
use wiper::app::App;
use wiper::config::InitConfig;
use wiper::fs::{DataStore, Folder, FolderEntry, FolderEntryType, SortBy};

pub const TEST_FILE_PATH_VIEW: &str = "./tests/test_files/view";
pub const TEST_FILE_PATH_EDIT: &str = "./tests/test_files/edit";
pub fn setup_app_view() -> App {
    let c = InitConfig {
        file_path: Some(TEST_FILE_PATH_VIEW.to_string()),
    };
    let mut app = App::new(c);
    app.ui_config.open_file = false;
    app.ui_config.sort_by = SortBy::Title;
    app.init();
    app
}

pub fn setup_app_edit(postfix: &str) -> App {
    let c = InitConfig {
        file_path: Some(format!("{TEST_FILE_PATH_EDIT}_{postfix}")),
    };
    let mut app = App::new(c);
    app.ui_config.open_file = false;
    app.ui_config.move_to_trash = false;
    app.ui_config.sort_by = SortBy::Title;
    app.init();
    app
}

pub fn handle_tasks_synchronously(app: &mut App) {
    while !app.task_manager.is_done() {
        app.tick();
        thread::sleep(Duration::from_millis(10));
    }
    app.pre_render();
}

pub fn assert_item_at_index_is(app: &App, index: usize, kind: FolderEntryType) {
    assert_eq!(
        app.store
            .get_current_folder()
            .unwrap()
            .entries()
            .get(index)
            .unwrap()
            .kind,
        kind
    );
}

pub fn assert_item_at_index_title(app: &App, index: usize, title: String) {
    assert_eq!(
        app.store
            .get_current_folder()
            .unwrap()
            .entries()
            .get(index)
            .unwrap()
            .title,
        title
    );
}

pub fn assert_item_at_index_loading_state(app: &App, index: usize, is_loaded: bool) {
    assert_eq!(
        app.store
            .get_current_folder()
            .unwrap()
            .entries()
            .get(index)
            .unwrap()
            .is_loaded,
        is_loaded
    );
}

pub fn get_entry_by_kind(app: &App, kind: FolderEntryType) -> Vec<FolderEntry> {
    app.store
        .get_current_folder()
        .unwrap()
        .entries()
        .iter()
        .filter(|e| e.kind == kind)
        .cloned()
        .collect()
}

pub fn assert_parent_folder_state(app: &App) {
    assert_eq!(get_entry_by_kind(app, FolderEntryType::File).len(), 3);
    assert_eq!(get_entry_by_kind(app, FolderEntryType::Folder).len(), 3);
}

pub fn assert_parent_folder_a_state(app: &App) {
    assert_eq!(get_entry_by_kind(app, FolderEntryType::File).len(), 2);
    assert_eq!(get_entry_by_kind(app, FolderEntryType::Folder).len(), 0);
}

pub fn assert_delete_folder_state(app: &App) {
    assert_eq!(get_entry_by_kind(app, FolderEntryType::File).len(), 3);
    assert_eq!(get_entry_by_kind(app, FolderEntryType::Folder).len(), 1);
}

pub fn assert_cursor_index(app: &App, index: usize) {
    assert_eq!(app.store.get_current_folder().unwrap().cursor_index, index);
}

pub fn assert_root_view_folder_sorted_by_title(app: &App) {
    assert_item_at_index_title(app, 0, "..".to_string());
    assert_item_at_index_title(app, 1, "a_folder".to_string());
    assert_item_at_index_title(app, 2, "b_folder".to_string());
    assert_item_at_index_title(app, 3, "c_folder".to_string());
    assert_item_at_index_title(app, 4, "a_root_file.txt".to_string());
    assert_item_at_index_title(app, 5, "d_root_file.txt".to_string());
    assert_item_at_index_title(app, 6, "z_root_file.txt".to_string());
}

pub fn assert_root_view_folder_sorted_by_size(app: &App) {
    assert_item_at_index_title(app, 0, "..".to_string());
    assert_item_at_index_title(app, 1, "b_folder".to_string());
    assert_item_at_index_title(app, 2, "c_folder".to_string());
    assert_item_at_index_title(app, 3, "a_folder".to_string());
    assert_item_at_index_title(app, 4, "d_root_file.txt".to_string());
    assert_item_at_index_title(app, 5, "a_root_file.txt".to_string());
    assert_item_at_index_title(app, 6, "z_root_file.txt".to_string());
}

pub fn get_current_folder(app: &App) -> Option<&Folder> {
    app.store.get_current_folder()
}
