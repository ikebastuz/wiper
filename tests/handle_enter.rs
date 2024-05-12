pub mod common;
use crate::common::*;

use wiper::app::App;
use wiper::fs::Store;
mod handle_enter {
    use super::*;

    #[tokio::test]
    async fn updates_current_tree_when_enters_subfolder() {
        let mut app: App<Store> = setup_app_view();
        await_for_tasks(&mut app).await;

        app.on_cursor_down();
        app.on_enter();
        await_for_tasks(&mut app).await;

        assert_cursor_index(&app, 0);
        assert_parent_folder_a_state(&app);
    }

    #[tokio::test]
    async fn navigates_back_to_parent_folder() {
        let mut app: App<Store> = setup_app_view();
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
        let mut app: App<Store> = setup_app_view();
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
