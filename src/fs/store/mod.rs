mod ds_hashmap;
pub use ds_hashmap::DSHashmap;

use crate::fs::{Folder, SortBy};

pub trait DataStore {
    fn new() -> Self;

    fn current_filepath(&self) -> String;

    fn current_folder(&self) -> Option<&Folder>;

    fn current_folder_mut(&mut self) -> Option<&mut Folder>;

    fn set_current_folder(&mut self, folder: Folder);

    fn sort_current_folder(&mut self, sort_by: SortBy);

    fn move_to_parent(&mut self) -> Option<String>;

    fn move_to_child(&mut self, title: &String) -> String;

    fn delete_current_entry(&mut self);
}
