pub mod common;
use crate::common::*;

use wiper::app::App;
use wiper::fs::DataStoreType;
mod handle_enter {

    use super::*;

    #[test]
    fn updates_current_tree_when_enters_subfolder() {
        let mut app: App<DataStoreType> = setup_app_view();
        handle_tasks_synchronously(&mut app);

        app.on_cursor_down();
        app.on_enter();
        handle_tasks_synchronously(&mut app);

        assert_cursor_index(&app, 0);
        assert_parent_folder_a_state(&app);
    }

    #[test]
    fn navigates_back_to_parent_folder() {
        let mut app: App<DataStoreType> = setup_app_view();
        handle_tasks_synchronously(&mut app);

        app.on_cursor_down();
        app.on_enter();
        handle_tasks_synchronously(&mut app);

        assert_parent_folder_a_state(&app);

        app.on_enter();
        handle_tasks_synchronously(&mut app);

        assert_parent_folder_state(&app);
        assert_cursor_index(&app, 1);
    }

    #[test]
    fn does_nothing_when_tries_to_enter_file() {
        let mut app: App<DataStoreType> = setup_app_view();
        handle_tasks_synchronously(&mut app);

        app.on_cursor_down();
        app.on_cursor_down();
        app.on_cursor_down();
        app.on_cursor_down();
        app.on_cursor_down();
        assert_cursor_index(&app, 5);

        app.on_enter();
        handle_tasks_synchronously(&mut app);

        assert_cursor_index(&app, 5);
        assert_parent_folder_state(&app);
    }
}
