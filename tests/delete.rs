pub mod common;
use crate::common::*;
use wiper::app::App;
use wiper::fs::DSHashmap;
use wiper::fs::FolderEntryType;

mod delete {

    use super::*;
    use std::fs::{self, File};
    use std::io::Write;

    const TEST_FILE_SIZE: usize = 446;

    fn generate_lorem_ipsum() -> String {
        String::from(
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
    Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. \
    Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi \
    ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit \
    in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur \
    sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt \
    mollit anim id est laborum.",
        )
    }

    /// - folder_1
    ///     - folder_2
    ///         - folder_3
    ///         - file_1
    ///         - file_2
    ///         - file_3
    ///     - file_1
    ///     - file_2
    ///     - file_3
    /// - file_1
    /// - file_2
    /// - file_3
    fn create_testing_files() {
        fs::create_dir_all(TEST_FILE_PATH_EDIT).expect("Failed to create test folder");

        let mut folder_path = format!("{}", TEST_FILE_PATH_EDIT);

        for folder_index in 1..4 {
            for file_index in 1..4 {
                let file_name = format!("file_to_delete_{}.txt", file_index);
                let file_path = format!("{}/{}", folder_path, file_name);
                let mut file = File::create(&file_path).expect("Failed to create test file");
                writeln!(file, "{}", generate_lorem_ipsum()).expect("Failed to write to test file");
            }

            folder_path = format!("{}/folder_to_delete_{}", folder_path, folder_index);

            fs::create_dir_all(&folder_path).expect("Failed to create test folder");
        }
    }

    fn cleanup_testing_files() {
        if let Err(err) = fs::remove_dir_all(TEST_FILE_PATH_EDIT) {
            eprintln!("Failed to remove test folder: {}", err);
        }
    }

    #[tokio::test]
    async fn has_correct_initial_state() {
        create_testing_files();
        let mut app: App<DSHashmap> = setup_app_edit();
        await_for_tasks(&mut app).await;

        assert_delete_folder_state(&app);
        cleanup_testing_files();
    }

    #[tokio::test]
    async fn does_nothing_when_cursor_is_at_the_top() {
        create_testing_files();
        let mut app: App<DSHashmap> = setup_app_edit();
        await_for_tasks(&mut app).await;

        assert_cursor_index(&app, 0);
        assert_delete_folder_state(&app);
        app.on_delete();
        app.on_delete();
        await_for_tasks(&mut app).await;

        assert_delete_folder_state(&app);
        cleanup_testing_files();
    }

    #[tokio::test]
    async fn does_nothing_when_delete_pressed_once() {
        create_testing_files();
        let mut app: App<DSHashmap> = setup_app_edit();
        await_for_tasks(&mut app).await;

        assert_delete_folder_state(&app);
        app.on_cursor_down();
        app.on_delete();
        await_for_tasks(&mut app).await;

        assert_eq!(get_entry_by_kind(&app, FolderEntryType::File).len(), 3);
        assert_eq!(get_entry_by_kind(&app, FolderEntryType::Folder).len(), 1);
        cleanup_testing_files();
    }

    #[tokio::test]
    async fn resets_delete_confirmation_on_cursor_move() {
        create_testing_files();
        let mut app: App<DSHashmap> = setup_app_edit();
        await_for_tasks(&mut app).await;

        app.on_delete();
        app.on_cursor_down();
        assert_eq!(app.ui_config.confirming_deletion, false);
        app.on_delete();
        app.on_cursor_up();
        assert_eq!(app.ui_config.confirming_deletion, false);
        cleanup_testing_files();
    }

    #[tokio::test]
    async fn resets_delete_confirmation_on_folder_enter() {
        create_testing_files();
        let mut app: App<DSHashmap> = setup_app_edit();
        await_for_tasks(&mut app).await;

        app.on_cursor_down();
        app.on_delete();
        app.on_enter();
        assert_eq!(app.ui_config.confirming_deletion, false);
        cleanup_testing_files();
    }

    #[tokio::test]
    async fn resets_delete_confirmation_after_deleting_folder() {
        create_testing_files();
        let mut app: App<DSHashmap> = setup_app_edit();
        await_for_tasks(&mut app).await;

        app.on_cursor_down();
        app.on_delete();
        app.on_delete();
        assert_eq!(app.ui_config.confirming_deletion, false);
        cleanup_testing_files();
    }

    #[tokio::test]
    async fn resets_delete_confirmation_after_deleting_file() {
        create_testing_files();
        let mut app: App<DSHashmap> = setup_app_edit();
        await_for_tasks(&mut app).await;

        app.on_cursor_down();
        app.on_cursor_down();
        app.on_delete();
        app.on_delete();
        assert_eq!(app.ui_config.confirming_deletion, false);
        cleanup_testing_files();
    }
    //
    #[tokio::test]
    async fn deletes_folder() {
        create_testing_files();
        let mut app: App<DSHashmap> = setup_app_edit();
        await_for_tasks(&mut app).await;

        assert_delete_folder_state(&app);
        app.on_cursor_down();
        app.on_delete();
        app.on_delete();
        await_for_tasks(&mut app).await;

        assert_eq!(get_entry_by_kind(&app, FolderEntryType::File).len(), 3);
        assert_eq!(get_entry_by_kind(&app, FolderEntryType::Folder).len(), 0);
        cleanup_testing_files();
    }

    #[tokio::test]
    async fn deletes_file() {
        create_testing_files();
        let mut app: App<DSHashmap> = setup_app_edit();
        await_for_tasks(&mut app).await;
        assert_delete_folder_state(&app);
        app.on_cursor_down();
        app.on_cursor_down();
        app.on_delete();
        app.on_delete();
        await_for_tasks(&mut app).await;

        assert_eq!(get_entry_by_kind(&app, FolderEntryType::File).len(), 2);
        assert_eq!(get_entry_by_kind(&app, FolderEntryType::Folder).len(), 1);
        cleanup_testing_files();
    }

    #[tokio::test]
    async fn updated_current_folder_size() {
        create_testing_files();
        let mut app: App<DSHashmap> = setup_app_edit();
        await_for_tasks(&mut app).await;

        let root_entry = get_current_folder(&app).unwrap();
        assert_eq!(root_entry.get_size(), (TEST_FILE_SIZE * 9) as u64);

        app.on_cursor_down();
        app.on_cursor_down();
        app.on_delete();
        app.on_delete();
        await_for_tasks(&mut app).await;

        let root_entry_updated = get_current_folder(&app).unwrap();
        assert_eq!(root_entry_updated.get_size(), (TEST_FILE_SIZE * 8) as u64);

        cleanup_testing_files();
    }

    #[tokio::test]
    async fn deleting_file_updates_parent_folders_sizes() {
        create_testing_files();
        let mut app: App<DSHashmap> = setup_app_edit();
        await_for_tasks(&mut app).await;

        let root_entry = get_current_folder(&app).unwrap();
        assert_eq!(root_entry.get_size(), (TEST_FILE_SIZE * 9) as u64);

        app.on_cursor_down();
        app.on_enter();
        await_for_tasks(&mut app).await;

        let folder_1 = get_current_folder(&app).unwrap();
        assert_eq!(folder_1.get_size(), (TEST_FILE_SIZE * 6) as u64);

        app.on_cursor_down();
        app.on_enter();
        await_for_tasks(&mut app).await;

        let folder_2 = get_current_folder(&app).unwrap();
        assert_eq!(folder_2.get_size(), (TEST_FILE_SIZE * 3) as u64);

        app.on_cursor_down();
        app.on_cursor_down();
        app.on_delete();
        app.on_delete();
        await_for_tasks(&mut app).await;

        let folder_2_upd = get_current_folder(&app).unwrap();
        assert_eq!(folder_2_upd.get_size(), (TEST_FILE_SIZE * 2) as u64);

        app.on_cursor_up();
        app.on_cursor_up();
        app.on_enter();
        await_for_tasks(&mut app).await;

        let folder_1_upd = get_current_folder(&app).unwrap();
        assert_eq!(folder_1_upd.get_size(), (TEST_FILE_SIZE * 5) as u64);
        assert_eq!(
            folder_1_upd.get_selected_entry_size(),
            (TEST_FILE_SIZE * 2) as u64
        );

        app.on_cursor_up();
        app.on_enter();
        await_for_tasks(&mut app).await;

        let root_entry_upd = get_current_folder(&app).unwrap();
        assert_eq!(root_entry_upd.get_size(), (TEST_FILE_SIZE * 8) as u64);
        assert_eq!(
            root_entry_upd.get_selected_entry_size(),
            (TEST_FILE_SIZE * 5) as u64
        );

        cleanup_testing_files();
    }

    #[tokio::test]
    async fn deleting_folder_updates_parent_folders_sizes() {
        create_testing_files();
        let mut app: App<DSHashmap> = setup_app_edit();
        await_for_tasks(&mut app).await;

        let root_entry = get_current_folder(&app).unwrap();
        assert_eq!(root_entry.get_size(), (TEST_FILE_SIZE * 9) as u64);

        app.on_cursor_down();
        app.on_enter();
        await_for_tasks(&mut app).await;

        let folder_1 = get_current_folder(&app).unwrap();
        assert_eq!(folder_1.get_size(), (TEST_FILE_SIZE * 6) as u64);

        app.on_cursor_down();
        app.on_delete();
        app.on_delete();
        await_for_tasks(&mut app).await;

        let folder_1_upd = get_current_folder(&app).unwrap();
        assert_eq!(folder_1_upd.get_size(), (TEST_FILE_SIZE * 3) as u64);

        app.on_cursor_up();
        app.on_enter();
        await_for_tasks(&mut app).await;

        let root_entry_upd = get_current_folder(&app).unwrap();
        assert_eq!(root_entry_upd.get_size(), (TEST_FILE_SIZE * 6) as u64);
        assert_eq!(
            root_entry_upd.get_selected_entry_size(),
            (TEST_FILE_SIZE * 3) as u64
        );

        cleanup_testing_files();
    }

    #[tokio::test]
    async fn moves_cursor_one_step_up_after_deleting_bottom_entry() {
        create_testing_files();
        let mut app: App<DSHashmap> = setup_app_edit();
        await_for_tasks(&mut app).await;

        for _ in 1..20 {
            app.on_cursor_down();
        }

        assert_eq!(get_current_folder(&app).unwrap().cursor_index, 4);
        app.on_delete();
        app.on_delete();
        await_for_tasks(&mut app).await;
        assert_eq!(get_current_folder(&app).unwrap().cursor_index, 3);

        cleanup_testing_files();
    }
}
