pub mod common;

use crate::common::*;

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
