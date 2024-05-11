#[cfg(test)]
mod tests {
    const TEST_FILE_PATH_VIEW: &str = "./src/tests/test_files/view";
    const TEST_FILE_PATH_EDIT: &str = "./src/tests/test_files/edit";
    use crate::app::App;
    use crate::config::InitConfig;
    use crate::fs::{FolderEntry, FolderEntryType};

    fn setup_app_view() -> App {
        let c = InitConfig {
            file_path: Some(TEST_FILE_PATH_VIEW.to_string()),
        };
        let mut app = App::new(c);
        app.init();
        app.ui_config.open_file = false;
        app
    }

    fn setup_app_edit() -> App {
        let c = InitConfig {
            file_path: Some(TEST_FILE_PATH_EDIT.to_string()),
        };
        let mut app = App::new(c);
        app.init();
        app.ui_config.open_file = false;
        app.ui_config.move_to_trash = false;
        app
    }

    async fn await_for_tasks(app: &mut App) {
        while !app.task_manager.is_done() {
            app.tick();

            tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
        }
        app.pre_render();
    }

    fn assert_item_at_index_is(app: &App, index: usize, kind: FolderEntryType) {
        assert_eq!(
            app.get_current_folder()
                .unwrap()
                .entries
                .get(index)
                .unwrap()
                .kind,
            kind
        );
    }

    fn assert_item_at_index_title(app: &App, index: usize, title: String) {
        assert_eq!(
            app.get_current_folder()
                .unwrap()
                .entries
                .get(index)
                .unwrap()
                .title,
            title
        );
    }

    fn get_entry_by_kind(app: &App, kind: FolderEntryType) -> Vec<FolderEntry> {
        app.get_current_folder()
            .unwrap()
            .entries
            .iter()
            .filter(|e| e.kind == kind)
            .cloned()
            .collect()
    }

    fn assert_parent_folder_state(app: &App) {
        assert_eq!(get_entry_by_kind(app, FolderEntryType::File).len(), 3);
        assert_eq!(get_entry_by_kind(app, FolderEntryType::Folder).len(), 3);
    }

    fn assert_parent_folder_a_state(app: &App) {
        assert_eq!(get_entry_by_kind(app, FolderEntryType::File).len(), 2);
        assert_eq!(get_entry_by_kind(app, FolderEntryType::Folder).len(), 0);
    }

    fn assert_delete_folder_state(app: &App) {
        assert_eq!(get_entry_by_kind(app, FolderEntryType::File).len(), 3);
        assert_eq!(get_entry_by_kind(app, FolderEntryType::Folder).len(), 1);
    }

    fn assert_cursor_index(app: &App, index: usize) {
        assert_eq!(app.get_current_folder().unwrap().cursor_index, index);
    }

    fn assert_root_view_folder_sorted_by_title(app: &App) {
        assert_item_at_index_title(&app, 0, "..".to_string());
        assert_item_at_index_title(&app, 1, "a_folder".to_string());
        assert_item_at_index_title(&app, 2, "b_folder".to_string());
        assert_item_at_index_title(&app, 3, "c_folder".to_string());
        assert_item_at_index_title(&app, 4, "a_root_file.txt".to_string());
        assert_item_at_index_title(&app, 5, "d_root_file.txt".to_string());
        assert_item_at_index_title(&app, 6, "z_root_file.txt".to_string());
    }

    fn assert_root_view_folder_sorted_by_size(app: &App) {
        assert_item_at_index_title(&app, 0, "..".to_string());
        assert_item_at_index_title(&app, 1, "b_folder".to_string());
        assert_item_at_index_title(&app, 2, "c_folder".to_string());
        assert_item_at_index_title(&app, 3, "a_folder".to_string());
        assert_item_at_index_title(&app, 4, "d_root_file.txt".to_string());
        assert_item_at_index_title(&app, 5, "a_root_file.txt".to_string());
        assert_item_at_index_title(&app, 6, "z_root_file.txt".to_string());
    }

    mod file_tree {
        use super::*;

        #[tokio::test]
        async fn test_ordering_by_kind() {
            let mut app = setup_app_view();
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
            let mut app = setup_app_view();
            await_for_tasks(&mut app).await;

            assert_root_view_folder_sorted_by_title(&app);
        }

        #[tokio::test]
        async fn test_switching_ordering_to_size() {
            let mut app = setup_app_view();
            await_for_tasks(&mut app).await;

            app.on_toggle_sorting();
            await_for_tasks(&mut app).await;

            assert_root_view_folder_sorted_by_size(&app);
        }

        #[tokio::test]
        async fn test_ordering_persists_after_navigating_into_folder() {
            let mut app = setup_app_view();
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
            let mut app = setup_app_view();
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
            let mut app = setup_app_view();
            await_for_tasks(&mut app).await;

            app.on_toggle_sorting();
            app.on_toggle_sorting();

            assert_root_view_folder_sorted_by_title(&app);
        }

        #[tokio::test]
        async fn has_correct_amount_file_tree_keys() {
            let mut app = setup_app_view();
            await_for_tasks(&mut app).await;

            let file_tree = app.file_tree_map;

            assert_eq!(file_tree.keys().len(), 4);
        }
    }
    //
    mod cursor {
        use super::*;

        #[tokio::test]
        async fn updates_cursor_position() {
            let mut app = setup_app_view();
            await_for_tasks(&mut app).await;

            assert_cursor_index(&mut app, 0);

            app.on_cursor_down();
            assert_cursor_index(&mut app, 1);

            app.on_cursor_up();
            assert_cursor_index(&mut app, 0);
        }

        #[tokio::test]
        async fn stops_cursor_at_very_top() {
            let mut app = setup_app_view();
            await_for_tasks(&mut app).await;

            assert_cursor_index(&mut app, 0);

            for _ in 0..10 {
                app.on_cursor_up();
            }

            assert_cursor_index(&mut app, 0);
        }

        #[tokio::test]
        async fn stops_cursor_at_very_bottom() {
            let mut app = setup_app_view();
            await_for_tasks(&mut app).await;

            for _ in 0..20 {
                app.on_cursor_down();
            }
            assert_cursor_index(&mut app, 6);
        }
    }

    mod handle_enter {
        use super::*;

        #[tokio::test]
        async fn updates_current_tree_when_enters_subfolder() {
            let mut app = setup_app_view();
            await_for_tasks(&mut app).await;

            app.on_cursor_down();
            app.on_enter();

            await_for_tasks(&mut app).await;

            assert_cursor_index(&app, 0);
            assert_parent_folder_a_state(&app);
        }

        #[tokio::test]
        async fn navigates_back_to_parent_folder() {
            let mut app = setup_app_view();
            await_for_tasks(&mut app).await;

            app.on_cursor_down();
            app.on_enter();
            await_for_tasks(&mut app).await;

            assert_parent_folder_a_state(&app);

            app.on_enter();
            await_for_tasks(&mut app).await;

            assert_parent_folder_state(&app);
            assert_cursor_index(&app, 1);
        }

        #[tokio::test]
        async fn does_nothing_when_tries_to_enter_file() {
            let mut app = setup_app_view();
            await_for_tasks(&mut app).await;

            app.on_cursor_down();
            app.on_cursor_down();
            app.on_cursor_down();
            app.on_cursor_down();
            app.on_cursor_down();
            assert_cursor_index(&app, 5);

            app.on_enter();
            await_for_tasks(&mut app).await;

            assert_cursor_index(&app, 5);
            assert_parent_folder_state(&app);
        }
    }

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
                    writeln!(file, "{}", generate_lorem_ipsum())
                        .expect("Failed to write to test file");
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
            let mut app = setup_app_edit();
            await_for_tasks(&mut app).await;

            assert_delete_folder_state(&app);
            cleanup_testing_files();
        }

        #[tokio::test]
        async fn does_nothing_when_cursor_is_at_the_top() {
            create_testing_files();
            let mut app = setup_app_edit();
            await_for_tasks(&mut app).await;

            assert_cursor_index(&app, 0);
            assert_delete_folder_state(&app);
            app.on_delete();
            app.on_delete();
            assert_delete_folder_state(&app);
            cleanup_testing_files();
        }

        #[tokio::test]
        async fn does_nothing_when_delete_pressed_once() {
            create_testing_files();
            let mut app = setup_app_edit();
            await_for_tasks(&mut app).await;

            assert_delete_folder_state(&app);
            app.on_cursor_down();
            app.on_delete();
            assert_eq!(get_entry_by_kind(&app, FolderEntryType::File).len(), 3);
            assert_eq!(get_entry_by_kind(&app, FolderEntryType::Folder).len(), 1);
            cleanup_testing_files();
        }

        #[tokio::test]
        async fn resets_delete_confirmation_on_cursor_move() {
            create_testing_files();
            let mut app = setup_app_edit();
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
            let mut app = setup_app_edit();
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
            let mut app = setup_app_edit();
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
            let mut app = setup_app_edit();
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
            let mut app = setup_app_edit();
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
            let mut app = setup_app_edit();
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
            let mut app = setup_app_edit();
            await_for_tasks(&mut app).await;

            let root_entry = app.get_current_folder().unwrap();
            assert_eq!(root_entry.get_size(), (TEST_FILE_SIZE * 9) as u64);

            app.on_cursor_down();
            app.on_cursor_down();
            app.on_delete();
            app.on_delete();

            await_for_tasks(&mut app).await;
            let root_entry_updated = app.get_current_folder().unwrap();
            assert_eq!(root_entry_updated.get_size(), (TEST_FILE_SIZE * 8) as u64);

            cleanup_testing_files();
        }

        #[tokio::test]
        async fn deleting_file_updates_parent_folders_sizes() {
            create_testing_files();
            let mut app = setup_app_edit();
            await_for_tasks(&mut app).await;

            let root_entry = app.get_current_folder().unwrap();
            assert_eq!(root_entry.get_size(), (TEST_FILE_SIZE * 9) as u64);

            app.on_cursor_down();
            app.on_enter();
            await_for_tasks(&mut app).await;

            let folder_1 = app.get_current_folder().unwrap();
            assert_eq!(folder_1.get_size(), (TEST_FILE_SIZE * 6) as u64);

            app.on_cursor_down();
            app.on_enter();
            await_for_tasks(&mut app).await;

            let folder_2 = app.get_current_folder().unwrap();
            assert_eq!(folder_2.get_size(), (TEST_FILE_SIZE * 3) as u64);

            app.on_cursor_down();
            app.on_cursor_down();
            app.on_delete();
            app.on_delete();
            await_for_tasks(&mut app).await;

            let folder_2_upd = app.get_current_folder().unwrap();
            assert_eq!(folder_2_upd.get_size(), (TEST_FILE_SIZE * 2) as u64);

            app.on_cursor_up();
            app.on_cursor_up();
            app.on_enter();
            await_for_tasks(&mut app).await;

            let folder_1_upd = app.get_current_folder().unwrap();
            assert_eq!(folder_1_upd.get_size(), (TEST_FILE_SIZE * 5) as u64);
            assert_eq!(
                folder_1_upd.get_selected_entry_size(),
                (TEST_FILE_SIZE * 2) as u64
            );

            app.on_cursor_up();
            app.on_enter();
            await_for_tasks(&mut app).await;

            let root_entry_upd = app.get_current_folder().unwrap();
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
            let mut app = setup_app_edit();
            await_for_tasks(&mut app).await;

            let root_entry = app.get_current_folder().unwrap();
            assert_eq!(root_entry.get_size(), (TEST_FILE_SIZE * 9) as u64);

            app.on_cursor_down();
            app.on_enter();
            await_for_tasks(&mut app).await;

            let folder_1 = app.get_current_folder().unwrap();
            assert_eq!(folder_1.get_size(), (TEST_FILE_SIZE * 6) as u64);

            app.on_cursor_down();
            app.on_delete();
            app.on_delete();
            await_for_tasks(&mut app).await;

            let folder_1_upd = app.get_current_folder().unwrap();
            assert_eq!(folder_1_upd.get_size(), (TEST_FILE_SIZE * 3) as u64);

            app.on_cursor_up();
            app.on_enter();
            await_for_tasks(&mut app).await;

            let root_entry_upd = app.get_current_folder().unwrap();
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
            let mut app = setup_app_edit();
            await_for_tasks(&mut app).await;

            for _ in 1..20 {
                app.on_cursor_down();
            }

            assert_eq!(app.get_current_folder().unwrap().cursor_index, 4);
            app.on_delete();
            app.on_delete();
            await_for_tasks(&mut app).await;
            assert_eq!(app.get_current_folder().unwrap().cursor_index, 3);

            cleanup_testing_files();
        }
    }
}
