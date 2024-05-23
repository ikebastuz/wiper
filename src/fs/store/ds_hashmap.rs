use crate::fs::{DataStore, Folder, SortBy};
use std::collections::HashMap;
use std::path::PathBuf;

use super::DataStoreKey;

pub type FileTreeMap = HashMap<PathBuf, Folder>;

pub struct DSHashmap {
    /// Current file path buffer
    pub current_path: PathBuf,
    /// Map for all file paths
    pub store: FileTreeMap,
    pub file_type_map: HashMap<String, u64>,
}

impl DataStore<DataStoreKey> for DSHashmap {
    fn new() -> DSHashmap {
        DSHashmap {
            current_path: PathBuf::from("."),
            store: HashMap::new(),
            file_type_map: HashMap::new(),
        }
    }

    fn get_current_path(&mut self) -> &PathBuf {
        &self.current_path
    }

    fn set_current_path(&mut self, path: &PathBuf) {
        self.current_path.clone_from(path);
    }

    fn has_path(&self, path: &PathBuf) -> bool {
        self.store.contains_key(path)
    }

    fn get_current_folder(&self) -> Option<&Folder> {
        self.store.get(&self.current_path)
    }

    fn get_current_folder_mut(&mut self) -> Option<&mut Folder> {
        self.store.get_mut(&self.current_path)
    }

    fn set_folder(&mut self, path: &PathBuf, folder: Folder) {
        self.store.insert(path.clone(), folder);
    }

    fn get_folder_mut(&mut self, path: &PathBuf) -> Option<&mut Folder> {
        self.store.get_mut(path)
    }

    fn set_current_folder(&mut self, folder: Folder) {
        self.set_folder(&self.current_path.clone(), folder);
    }

    // TODO: refactor
    fn sort_current_folder(&mut self, sort_by: SortBy) {
        if let Some(folder) = self.get_current_folder_mut() {
            match &folder.sorted_by {
                None => match sort_by {
                    SortBy::Title => folder.sort_by_title(),
                    SortBy::Size => folder.sort_by_size(),
                },
                Some(folder_sort_by) => {
                    if folder_sort_by.clone() != sort_by {
                        match sort_by {
                            SortBy::Title => folder.sort_by_title(),
                            SortBy::Size => folder.sort_by_size(),
                        };
                    };
                }
            }
            folder.sorted_by = Some(sort_by);
        }
    }

    fn move_to_parent(&mut self) {
        if let Some(parent) = &self.current_path.parent() {
            let parent_buf = parent.to_path_buf();
            self.current_path.clone_from(&parent_buf);
        }
    }

    fn move_to_child(&mut self, title: &str) {
        let mut new_path = PathBuf::from(&self.current_path);
        new_path.push(title);
        self.current_path = new_path;
    }

    fn get_entry_size(&mut self, path: &PathBuf) -> Option<u64> {
        self.store.get(path).map(|entry| entry.get_size())
    }

    fn remove_path(&mut self, path: &PathBuf) {
        self.store.remove(path);
    }

    fn get_nodes_len(&self) -> usize {
        self.store.keys().len()
    }

    fn get_keys(&mut self) -> Vec<PathBuf> {
        self.store.keys().cloned().collect()
    }
}
