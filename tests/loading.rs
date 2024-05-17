pub mod common;

use crate::common::*;
use wiper::app::App;
use wiper::fs::DataStoreType;

mod loading {
    use super::*;

    #[test]
    fn root_folders_are_loaded() {
        let mut app: App<DataStoreType> = setup_app_view();
        handle_tasks_synchronously(&mut app);
        assert_item_at_index_loading_state(&app, 1, true);
        assert_item_at_index_loading_state(&app, 2, true);
        assert_item_at_index_loading_state(&app, 3, true);
        assert_item_at_index_loading_state(&app, 4, true);
        assert_item_at_index_loading_state(&app, 5, true);
        assert_item_at_index_loading_state(&app, 6, true);
    }
}
