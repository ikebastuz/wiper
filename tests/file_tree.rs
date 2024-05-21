pub mod common;

use crate::common::*;
use wiper::app::App;
use wiper::fs::FolderEntryType;

mod file_tree {

    use wiper::fs::{DataStore, DataStoreType};

    use super::*;
    #[test]
    fn test_ordering_by_kind() {
        let mut app: App<DataStoreType> = setup_app_view();
        handle_tasks_synchronously(&mut app);

        assert_item_at_index_is(&app, 0, FolderEntryType::Parent);
        assert_item_at_index_is(&app, 1, FolderEntryType::Folder);
        assert_item_at_index_is(&app, 2, FolderEntryType::Folder);
        assert_item_at_index_is(&app, 3, FolderEntryType::Folder);
        assert_item_at_index_is(&app, 4, FolderEntryType::File);
        assert_item_at_index_is(&app, 5, FolderEntryType::File);
        assert_item_at_index_is(&app, 6, FolderEntryType::File);
    }

    #[test]
    fn test_ordering_by_title() {
        let mut app: App<DataStoreType> = setup_app_view();
        handle_tasks_synchronously(&mut app);

        assert_root_view_folder_sorted_by_title(&app);
    }

    #[test]
    fn test_switching_ordering_to_size() {
        let mut app: App<DataStoreType> = setup_app_view();
        handle_tasks_synchronously(&mut app);

        app.on_toggle_sorting();
        handle_tasks_synchronously(&mut app);

        assert_root_view_folder_sorted_by_size(&app);
    }

    #[test]
    fn test_ordering_persists_after_navigating_into_folder() {
        let mut app: App<DataStoreType> = setup_app_view();
        handle_tasks_synchronously(&mut app);

        app.on_toggle_sorting();
        handle_tasks_synchronously(&mut app);

        app.on_cursor_down();
        app.on_enter();
        handle_tasks_synchronously(&mut app);

        assert_item_at_index_title(&app, 0, "..".to_string());
        assert_item_at_index_title(&app, 1, "folder2_file3.txt".to_string());
        assert_item_at_index_title(&app, 2, "folder2_file2.txt".to_string());
        assert_item_at_index_title(&app, 3, "folder2_file1.txt".to_string());
    }

    #[test]
    fn test_ordering_persists_after_navigating_to_parent() {
        let mut app: App<DataStoreType> = setup_app_view();
        handle_tasks_synchronously(&mut app);

        app.on_cursor_down();
        app.on_enter();
        app.on_toggle_sorting();
        handle_tasks_synchronously(&mut app);

        app.on_enter();
        handle_tasks_synchronously(&mut app);

        assert_root_view_folder_sorted_by_size(&app);
    }

    #[test]
    fn test_switching_ordering_back_to_title() {
        let mut app: App<DataStoreType> = setup_app_view();
        handle_tasks_synchronously(&mut app);

        app.on_toggle_sorting();
        app.on_toggle_sorting();
        handle_tasks_synchronously(&mut app);

        assert_root_view_folder_sorted_by_title(&app);
    }

    #[test]
    fn has_correct_amount_file_tree_keys() {
        let mut app: App<DataStoreType> = setup_app_view();
        handle_tasks_synchronously(&mut app);

        assert_eq!(app.store.get_nodes_len(), 5);
    }
}
