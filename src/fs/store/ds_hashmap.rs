use crate::fs::{DataStore, Folder, SortBy};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub type FileTreeMap = HashMap<PathBuf, Folder>;

#[derive(Debug)]
pub struct DSHashmap {
    /// Current file path buffer
    pub current_path: PathBuf,
    /// Map for all file paths
    pub store: FileTreeMap,
    pub file_type_map: HashMap<String, u64>,
}

impl DataStore for DSHashmap {
    fn new() -> DSHashmap {
        DSHashmap {
            current_path: PathBuf::from("."),
            store: HashMap::new(),
            file_type_map: HashMap::new(),
        }
    }

    fn get_current_path(&self) -> &Path {
        &self.current_path
    }

    fn set_current_path(&mut self, path: &Path) {
        self.current_path = path.to_path_buf();
    }

    fn get_current_folder(&self) -> Option<&Folder> {
        self.store.get(&self.current_path)
    }

    fn get_current_folder_mut(&mut self) -> Option<&mut Folder> {
        self.store.get_mut(&self.current_path)
    }

    fn set_folder(&mut self, path: &Path, folder: Folder) {
        self.store.insert(path.to_path_buf(), folder);
    }

    fn get_folder_mut(&mut self, path: &Path) -> Option<&mut Folder> {
        self.store.get_mut(path)
    }

    fn set_current_folder(&mut self, folder: Folder) {
        self.set_folder(&self.current_path.clone(), folder);
    }

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
        if let Some(parent) = self.current_path.parent() {
            self.current_path = parent.to_path_buf();
        }
    }

    fn move_to_child(&mut self, title: &str) {
        self.current_path.push(title);
    }

    fn remove_path(&mut self, path: &Path) {
        self.store.remove(path);
    }

    fn get_nodes_len(&self) -> usize {
        self.store.keys().len()
    }
}
