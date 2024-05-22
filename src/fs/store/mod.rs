mod ds_hashmap;
pub use ds_hashmap::DSHashmap;

use crate::fs::{Folder, SortBy};
use std::path::PathBuf;

pub trait DataStore<T> {
    fn new() -> Self;

    /// Get current active path
    fn get_current_path(&mut self) -> &T;

    /// Set current active path
    fn set_current_path(&mut self, path: &T);

    /// Check if store has provided path entry
    fn has_path(&self, path: &T) -> bool;

    /// Get optional current active Folder
    fn get_current_folder(&self) -> Option<&Folder>;

    /// Get optional current active mutable Folder
    fn get_current_folder_mut(&mut self) -> Option<&mut Folder>;

    /// Get optional mutable Folder for provided path
    fn get_folder_mut(&mut self, path: &T) -> Option<&mut Folder>;

    /// Update folder for provided path
    fn set_folder(&mut self, path: &T, folder: Folder);

    /// Update current active folder
    fn set_current_folder(&mut self, folder: Folder);

    /// Sort current active folder by provided order
    fn sort_current_folder(&mut self, sort_by: SortBy);

    /// Update current active path to its parent
    fn move_to_parent(&mut self);

    /// Update current active path to child folder by provided title
    fn move_to_child(&mut self, title: &str);

    /// Remove provided path record from store
    fn remove_path(&mut self, path: &T);

    /// Get total known size for provided path
    fn get_entry_size(&mut self, path: &T) -> Option<u64>;

    /// Get amount of processed file paths
    fn get_nodes_len(&self) -> usize;

    fn get_keys(&mut self) -> Vec<T>;
}

pub type DataStoreKey = PathBuf;
pub type DataStoreType = DSHashmap;
