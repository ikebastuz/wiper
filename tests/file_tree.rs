pub mod common;

use crate::common::*;
use wiper::app::App;
use wiper::fs::DSHashmap;
use wiper::fs::FolderEntryType;

mod file_tree {

    use wiper::fs::DataStore;

    use super::*;
    #[tokio::test]
    async fn test_ordering_by_kind() {
        let mut app: App<DSHashmap> = setup_app_view();
        await_for_tasks(&mut app).await;

        assert_item_at_index_is(&app, 0, FolderEntryType::Parent);
        assert_item_at_index_is(&app, 1, FolderEntryType::Folder);
        assert_item_at_index_is(&app, 2, FolderEntryType::Folder);
        assert_item_at_index_is(&app, 3, FolderEntryType::Folder);
        assert_item_at_index_is(&app, 4, FolderEntryType::File);
        assert_item_at_index_is(&app, 5, FolderEntryType::File);
        assert_item_at_index_is(&app, 6, FolderEntryType::File);
    }

    #[tokio::test]
    async fn test_ordering_by_title() {
        let mut app: App<DSHashmap> = setup_app_view();
        await_for_tasks(&mut app).await;

        assert_root_view_folder_sorted_by_title(&app);
    }

    #[tokio::test]
    async fn test_switching_ordering_to_size() {
        let mut app: App<DSHashmap> = setup_app_view();
        await_for_tasks(&mut app).await;

        app.on_toggle_sorting();
        await_for_tasks(&mut app).await;

        assert_root_view_folder_sorted_by_size(&app);
    }

    #[tokio::test]
    async fn test_ordering_persists_after_navigating_into_folder() {
        let mut app: App<DSHashmap> = setup_app_view();
        await_for_tasks(&mut app).await;

        app.on_toggle_sorting();
        await_for_tasks(&mut app).await;

        app.on_cursor_down();
        app.on_enter();
        await_for_tasks(&mut app).await;

        assert_item_at_index_title(&app, 0, "..".to_string());
        assert_item_at_index_title(&app, 1, "folder2_file3.txt".to_string());
        assert_item_at_index_title(&app, 2, "folder2_file2.txt".to_string());
        assert_item_at_index_title(&app, 3, "folder2_file1.txt".to_string());
    }

    #[tokio::test]
    async fn test_ordering_persists_after_navigating_to_parent() {
        let mut app: App<DSHashmap> = setup_app_view();
        await_for_tasks(&mut app).await;

        app.on_cursor_down();
        app.on_enter();
        app.on_toggle_sorting();
        await_for_tasks(&mut app).await;

        app.on_enter();
        await_for_tasks(&mut app).await;

        assert_root_view_folder_sorted_by_size(&app);
    }

    #[tokio::test]
    async fn test_switching_ordering_back_to_title() {
        let mut app: App<DSHashmap> = setup_app_view();
        await_for_tasks(&mut app).await;

        app.on_toggle_sorting();
        app.on_toggle_sorting();
        await_for_tasks(&mut app).await;

        assert_root_view_folder_sorted_by_title(&app);
    }

    #[tokio::test]
    async fn has_correct_amount_file_tree_keys() {
        let mut app: App<DSHashmap> = setup_app_view();
        await_for_tasks(&mut app).await;

        assert_eq!(app.store.get_nodes_len(), 4);
    }
}
