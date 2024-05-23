use std::thread;
use std::time::Duration;
use wiper::app::App;
use wiper::config::InitConfig;
use wiper::fs::{DataStore, DataStoreKey, Folder, FolderEntry, FolderEntryType, SortBy};

pub const TEST_FILE_PATH_VIEW: &str = "./tests/test_files/view";
pub const TEST_FILE_PATH_EDIT: &str = "./tests/test_files/edit";
pub fn setup_app_view<S: DataStore<DataStoreKey>>() -> App<S> {
    let c = InitConfig {
        file_path: Some(TEST_FILE_PATH_VIEW.to_string()),
    };
    let mut app: App<S> = App::new(c);
    app.ui_config.open_file = false;
    app.ui_config.sort_by = SortBy::Title;
    app.init();
    app
}

pub fn setup_app_edit<S: DataStore<DataStoreKey>>(postfix: &str) -> App<S> {
    let c = InitConfig {
        file_path: Some(format!("{}_{}", TEST_FILE_PATH_EDIT, postfix)),
    };
    let mut app: App<S> = App::new(c);
    app.ui_config.open_file = false;
    app.ui_config.move_to_trash = false;
    app.ui_config.sort_by = SortBy::Title;
    app.init();
    app
}

pub fn handle_tasks_synchronously<S: DataStore<DataStoreKey>>(app: &mut App<S>) {
    while !app.task_manager.is_done() {
        app.tick();
        thread::sleep(Duration::from_millis(10));
    }
    app.pre_render();
}

pub fn assert_item_at_index_is<S: DataStore<DataStoreKey>>(
    app: &App<S>,
    index: usize,
    kind: FolderEntryType,
) {
    assert_eq!(
        app.store
            .get_current_folder()
            .unwrap()
            .entries
            .get(index)
            .unwrap()
            .kind,
        kind
    );
}

pub fn assert_item_at_index_title<S: DataStore<DataStoreKey>>(
    app: &App<S>,
    index: usize,
    title: String,
) {
    assert_eq!(
        app.store
            .get_current_folder()
            .unwrap()
            .entries
            .get(index)
            .unwrap()
            .title,
        title
    );
}

pub fn assert_item_at_index_loading_state<S: DataStore<DataStoreKey>>(
    app: &App<S>,
    index: usize,
    is_loaded: bool,
) {
    assert_eq!(
        app.store
            .get_current_folder()
            .unwrap()
            .entries
            .get(index)
            .unwrap()
            .is_loaded,
        is_loaded
    );
}

pub fn get_entry_by_kind<S: DataStore<DataStoreKey>>(
    app: &App<S>,
    kind: FolderEntryType,
) -> Vec<FolderEntry> {
    app.store
        .get_current_folder()
        .unwrap()
        .entries
        .iter()
        .filter(|e| e.kind == kind)
        .cloned()
        .collect()
}

pub fn assert_parent_folder_state<S: DataStore<DataStoreKey>>(app: &App<S>) {
    assert_eq!(get_entry_by_kind(app, FolderEntryType::File).len(), 3);
    assert_eq!(get_entry_by_kind(app, FolderEntryType::Folder).len(), 3);
}

pub fn assert_parent_folder_a_state<S: DataStore<DataStoreKey>>(app: &App<S>) {
    assert_eq!(get_entry_by_kind(app, FolderEntryType::File).len(), 2);
    assert_eq!(get_entry_by_kind(app, FolderEntryType::Folder).len(), 0);
}

pub fn assert_delete_folder_state<S: DataStore<DataStoreKey>>(app: &App<S>) {
    assert_eq!(get_entry_by_kind(app, FolderEntryType::File).len(), 3);
    assert_eq!(get_entry_by_kind(app, FolderEntryType::Folder).len(), 1);
}

pub fn assert_cursor_index<S: DataStore<DataStoreKey>>(app: &App<S>, index: usize) {
    assert_eq!(app.store.get_current_folder().unwrap().cursor_index, index);
}

pub fn assert_root_view_folder_sorted_by_title<S: DataStore<DataStoreKey>>(app: &App<S>) {
    assert_item_at_index_title(app, 0, "..".to_string());
    assert_item_at_index_title(app, 1, "a_folder".to_string());
    assert_item_at_index_title(app, 2, "b_folder".to_string());
    assert_item_at_index_title(app, 3, "c_folder".to_string());
    assert_item_at_index_title(app, 4, "a_root_file.txt".to_string());
    assert_item_at_index_title(app, 5, "d_root_file.txt".to_string());
    assert_item_at_index_title(app, 6, "z_root_file.txt".to_string());
}

pub fn assert_root_view_folder_sorted_by_size<S: DataStore<DataStoreKey>>(app: &App<S>) {
    assert_item_at_index_title(app, 0, "..".to_string());
    assert_item_at_index_title(app, 1, "b_folder".to_string());
    assert_item_at_index_title(app, 2, "c_folder".to_string());
    assert_item_at_index_title(app, 3, "a_folder".to_string());
    assert_item_at_index_title(app, 4, "d_root_file.txt".to_string());
    assert_item_at_index_title(app, 5, "a_root_file.txt".to_string());
    assert_item_at_index_title(app, 6, "z_root_file.txt".to_string());
}

pub fn get_current_folder<S: DataStore<DataStoreKey>>(app: &App<S>) -> Option<&Folder> {
    app.store.get_current_folder()
}
