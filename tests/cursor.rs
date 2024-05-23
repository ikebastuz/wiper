pub mod common;

use crate::common::*;
use wiper::app::App;

mod cursor {

    use wiper::fs::DataStoreType;

    use super::*;

    #[test]
    fn updates_cursor_position() {
        let mut app: App<DataStoreType> = setup_app_view();
        handle_tasks_synchronously(&mut app);

        assert_cursor_index(&app, 0);

        app.on_cursor_down();
        assert_cursor_index(&app, 1);

        app.on_cursor_up();
        assert_cursor_index(&app, 0);
    }

    #[test]
    fn stops_cursor_at_very_top() {
        let mut app: App<DataStoreType> = setup_app_view();
        handle_tasks_synchronously(&mut app);

        assert_cursor_index(&app, 0);

        for _ in 0..10 {
            app.on_cursor_up();
        }

        assert_cursor_index(&app, 0);
    }

    #[test]
    fn stops_cursor_at_very_bottom() {
        let mut app: App<DataStoreType> = setup_app_view();
        handle_tasks_synchronously(&mut app);

        for _ in 0..20 {
            app.on_cursor_down();
        }
        assert_cursor_index(&app, 6);
    }
}
