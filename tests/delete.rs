pub mod common;
use crate::common::*;
use wiper::app::App;
use wiper::fs::FolderEntryType;

mod delete {

    use wiper::fs::DataStoreType;

    use super::*;
    use std::fs::{self, File};
    use std::io::Write;

    const TEST_FILE_SIZE: usize = 100;

    fn generate_lorem_ipsum() -> String {
        // String is exactly 100 bytes
        String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore")
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
    fn create_testing_files(postfix: &str) {
        let custom_folder = format!("{}_{}", TEST_FILE_PATH_EDIT, postfix);
        fs::create_dir_all(&custom_folder).expect("Failed to create test folder");

        let mut folder_path = custom_folder.to_string();

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

    fn cleanup_testing_files(postfix: &str) {
        let custom_folder = format!("{}_{}", TEST_FILE_PATH_EDIT, postfix);
        if let Err(err) = fs::remove_dir_all(custom_folder) {
            eprintln!("Failed to remove test folder: {}", err);
        }
    }

    #[test]
    fn has_correct_initial_state() {
        let postfix = "01";
        create_testing_files(postfix);
        let mut app: App<DataStoreType> = setup_app_edit(postfix);
        handle_tasks_synchronously(&mut app);

        assert_delete_folder_state(&app);
        cleanup_testing_files(postfix);
    }

    #[test]
    fn does_nothing_when_cursor_is_at_the_top() {
        let postfix = "02";
        create_testing_files(postfix);
        let mut app: App<DataStoreType> = setup_app_edit(postfix);
        handle_tasks_synchronously(&mut app);

        assert_cursor_index(&app, 0);
        assert_delete_folder_state(&app);
        app.on_delete();
        app.on_delete();
        handle_tasks_synchronously(&mut app);

        assert_delete_folder_state(&app);
        cleanup_testing_files(postfix);
    }

    #[test]
    fn does_nothing_when_delete_pressed_once() {
        let postfix = "03";
        create_testing_files(postfix);
        let mut app: App<DataStoreType> = setup_app_edit(postfix);
        handle_tasks_synchronously(&mut app);

        assert_delete_folder_state(&app);
        app.on_cursor_down();
        app.on_delete();
        handle_tasks_synchronously(&mut app);

        assert_eq!(get_entry_by_kind(&app, FolderEntryType::File).len(), 3);
        assert_eq!(get_entry_by_kind(&app, FolderEntryType::Folder).len(), 1);
        cleanup_testing_files(postfix);
    }

    #[test]
    fn resets_delete_confirmation_on_cursor_move() {
        let postfix = "04";
        create_testing_files(postfix);
        let mut app: App<DataStoreType> = setup_app_edit(postfix);
        handle_tasks_synchronously(&mut app);

        app.on_delete();
        app.on_cursor_down();
        assert!(!app.ui_config.confirming_deletion);
        app.on_delete();
        app.on_cursor_up();
        assert!(!app.ui_config.confirming_deletion);
        cleanup_testing_files(postfix);
    }

    #[test]
    fn resets_delete_confirmation_on_folder_enter() {
        let postfix = "05";
        create_testing_files(postfix);
        let mut app: App<DataStoreType> = setup_app_edit(postfix);
        handle_tasks_synchronously(&mut app);

        app.on_cursor_down();
        app.on_delete();
        app.on_enter();
        assert!(!app.ui_config.confirming_deletion);
        cleanup_testing_files(postfix);
    }

    #[test]
    fn resets_delete_confirmation_after_deleting_folder() {
        let postfix = "06";
        create_testing_files(postfix);
        let mut app: App<DataStoreType> = setup_app_edit(postfix);
        handle_tasks_synchronously(&mut app);

        app.on_cursor_down();
        app.on_delete();
        app.on_delete();
        assert!(!app.ui_config.confirming_deletion);
        cleanup_testing_files(postfix);
    }

    #[test]
    fn resets_delete_confirmation_after_deleting_file() {
        let postfix = "07";
        create_testing_files(postfix);
        let mut app: App<DataStoreType> = setup_app_edit(postfix);
        handle_tasks_synchronously(&mut app);

        app.on_cursor_down();
        app.on_cursor_down();
        app.on_delete();
        app.on_delete();
        assert!(!app.ui_config.confirming_deletion);
        cleanup_testing_files(postfix);
    }

    #[test]
    fn resets_delete_after_clicking_escape() {
        let postfix = "14";
        create_testing_files(postfix);
        let mut app: App<DataStoreType> = setup_app_edit(postfix);
        handle_tasks_synchronously(&mut app);

        app.on_cursor_down();
        app.on_delete();
        app.on_escape();
        assert!(!app.ui_config.confirming_deletion);
        cleanup_testing_files(postfix);
    }

    #[test]
    fn deletes_folder() {
        let postfix = "08";
        create_testing_files(postfix);
        let mut app: App<DataStoreType> = setup_app_edit(postfix);
        handle_tasks_synchronously(&mut app);

        assert_delete_folder_state(&app);
        app.on_cursor_down();
        app.on_delete();
        app.on_delete();
        handle_tasks_synchronously(&mut app);

        assert_eq!(get_entry_by_kind(&app, FolderEntryType::File).len(), 3);
        assert_eq!(get_entry_by_kind(&app, FolderEntryType::Folder).len(), 0);
        cleanup_testing_files(postfix);
    }

    #[test]
    fn deletes_file() {
        let postfix = "09";
        create_testing_files(postfix);
        let mut app: App<DataStoreType> = setup_app_edit(postfix);
        handle_tasks_synchronously(&mut app);
        assert_delete_folder_state(&app);
        app.on_cursor_down();
        app.on_cursor_down();
        app.on_delete();
        app.on_delete();
        handle_tasks_synchronously(&mut app);

        assert_eq!(get_entry_by_kind(&app, FolderEntryType::File).len(), 2);
        assert_eq!(get_entry_by_kind(&app, FolderEntryType::Folder).len(), 1);
        cleanup_testing_files(postfix);
    }

    #[test]
    fn updated_current_folder_size() {
        let postfix = "10";
        create_testing_files(postfix);
        let mut app: App<DataStoreType> = setup_app_edit(postfix);
        handle_tasks_synchronously(&mut app);

        let root_entry = get_current_folder(&app).unwrap();
        assert_eq!(root_entry.get_size(), (TEST_FILE_SIZE * 9) as u64);

        app.on_cursor_down();
        app.on_cursor_down();
        app.on_delete();
        app.on_delete();
        handle_tasks_synchronously(&mut app);

        let root_entry_updated = get_current_folder(&app).unwrap();
        assert_eq!(root_entry_updated.get_size(), (TEST_FILE_SIZE * 8) as u64);

        cleanup_testing_files(postfix);
    }

    #[test]
    fn deleting_file_updates_parent_folders_sizes() {
        let postfix = "11";
        create_testing_files(postfix);
        let mut app: App<DataStoreType> = setup_app_edit(postfix);
        handle_tasks_synchronously(&mut app);

        let root_entry = get_current_folder(&app).unwrap();
        assert_eq!(root_entry.get_size(), (TEST_FILE_SIZE * 9) as u64);

        app.on_cursor_down();
        app.on_enter();
        handle_tasks_synchronously(&mut app);

        let folder_1 = get_current_folder(&app).unwrap();
        assert_eq!(folder_1.get_size(), (TEST_FILE_SIZE * 6) as u64);

        app.on_cursor_down();
        app.on_enter();
        handle_tasks_synchronously(&mut app);

        let folder_2 = get_current_folder(&app).unwrap();
        assert_eq!(folder_2.get_size(), (TEST_FILE_SIZE * 3) as u64);

        app.on_cursor_down();
        app.on_cursor_down();
        app.on_delete();
        app.on_delete();
        handle_tasks_synchronously(&mut app);

        let folder_2_upd = get_current_folder(&app).unwrap();
        assert_eq!(folder_2_upd.get_size(), (TEST_FILE_SIZE * 2) as u64);

        app.on_cursor_up();
        app.on_cursor_up();
        app.on_enter();
        handle_tasks_synchronously(&mut app);

        let folder_1_upd = get_current_folder(&app).unwrap();
        assert_eq!(folder_1_upd.get_size(), (TEST_FILE_SIZE * 5) as u64);
        assert_eq!(
            folder_1_upd.get_selected_entry_size(),
            (TEST_FILE_SIZE * 2) as u64
        );

        app.on_cursor_up();
        app.on_enter();
        handle_tasks_synchronously(&mut app);

        let root_entry_upd = get_current_folder(&app).unwrap();
        assert_eq!(root_entry_upd.get_size(), (TEST_FILE_SIZE * 8) as u64);
        assert_eq!(
            root_entry_upd.get_selected_entry_size(),
            (TEST_FILE_SIZE * 5) as u64
        );

        cleanup_testing_files(postfix);
    }

    #[test]
    fn deleting_folder_updates_parent_folders_sizes() {
        let postfix = "12";
        create_testing_files(postfix);
        let mut app: App<DataStoreType> = setup_app_edit(postfix);
        handle_tasks_synchronously(&mut app);

        let root_entry = get_current_folder(&app).unwrap();
        assert_eq!(root_entry.get_size(), (TEST_FILE_SIZE * 9) as u64);

        app.on_cursor_down();
        app.on_enter();
        handle_tasks_synchronously(&mut app);

        let folder_1 = get_current_folder(&app).unwrap();
        assert_eq!(folder_1.get_size(), (TEST_FILE_SIZE * 6) as u64);

        app.on_cursor_down();
        app.on_delete();
        app.on_delete();
        handle_tasks_synchronously(&mut app);

        let folder_1_upd = get_current_folder(&app).unwrap();
        assert_eq!(folder_1_upd.get_size(), (TEST_FILE_SIZE * 3) as u64);

        app.on_cursor_up();
        app.on_enter();
        handle_tasks_synchronously(&mut app);

        let root_entry_upd = get_current_folder(&app).unwrap();
        assert_eq!(root_entry_upd.get_size(), (TEST_FILE_SIZE * 6) as u64);
        assert_eq!(
            root_entry_upd.get_selected_entry_size(),
            (TEST_FILE_SIZE * 3) as u64
        );

        cleanup_testing_files(postfix);
    }

    #[test]
    fn moves_cursor_one_step_up_after_deleting_bottom_entry() {
        let postfix = "13";
        create_testing_files(postfix);
        let mut app: App<DataStoreType> = setup_app_edit(postfix);
        handle_tasks_synchronously(&mut app);

        for _ in 1..20 {
            app.on_cursor_down();
        }

        assert_eq!(get_current_folder(&app).unwrap().cursor_index, 4);
        app.on_delete();
        app.on_delete();
        handle_tasks_synchronously(&mut app);
        assert_eq!(get_current_folder(&app).unwrap().cursor_index, 3);

        cleanup_testing_files(postfix);
    }
}
