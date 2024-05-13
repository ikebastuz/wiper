mod ds_hashmap;
pub use ds_hashmap::DSHashmap;

use crate::fs::{Folder, SortBy};
use std::path::PathBuf;

pub trait DataStore {
    fn new() -> Self;

    fn get_current_path(&mut self) -> &PathBuf;

    fn set_current_path(&mut self, path: &PathBuf);

    fn has_path(&self, path: &PathBuf) -> bool;

    fn get_current_folder(&self) -> Option<&Folder>;

    fn get_current_folder_mut(&mut self) -> Option<&mut Folder>;

    fn get_folder_mut(&mut self, path: &PathBuf) -> Option<&mut Folder>;

    fn set_folder(&mut self, path: &PathBuf, folder: Folder);

    fn set_current_folder(&mut self, folder: Folder);

    fn sort_current_folder(&mut self, sort_by: SortBy);

    fn move_to_parent(&mut self) -> Option<PathBuf>;

    fn move_to_child(&mut self, title: &String) -> PathBuf;

    fn delete_current_entry(&mut self);

    fn remove_path(&mut self, path: &PathBuf);

    fn get_entry_size(&mut self, path: &PathBuf) -> Option<u64>;

    fn get_nodes_len(&self) -> usize;
}
