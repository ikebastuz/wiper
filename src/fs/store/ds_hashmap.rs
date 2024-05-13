use crate::fs::{path_to_folder, DataStore, Folder, FolderEntryType, SortBy};
use std::collections::HashMap;
use std::path::PathBuf;

use super::DataStoreKey;

pub type FileTreeMap = HashMap<PathBuf, Folder>;

pub struct DSHashmap {
    /// Current file path buffer
    pub current_path: PathBuf,
    /// Map for all file paths
    pub store: FileTreeMap,
}

impl DataStore<DataStoreKey> for DSHashmap {
    fn new() -> DSHashmap {
        DSHashmap {
            current_path: PathBuf::from("."),
            store: HashMap::new(),
        }
    }

    fn get_current_path(&mut self) -> &PathBuf {
        &self.current_path
    }

    fn set_current_path(&mut self, path: &PathBuf) {
        self.current_path = path.clone();
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

    fn move_to_parent(&mut self) -> Vec<PathBuf> {
        if let Some(parent) = &self.current_path.parent() {
            let parent_buf = parent.to_path_buf();
            self.current_path = parent_buf.clone();

            self.process_path(&self.current_path.clone())
        } else {
            vec![]
        }
    }

    // TODO: Returns string that should be processed
    fn move_to_child(&mut self, title: &String) -> PathBuf {
        let mut new_path = PathBuf::from(&self.current_path);
        new_path.push(title);
        self.current_path = new_path.clone();

        new_path
    }

    fn get_entry_size(&mut self, path: &PathBuf) -> Option<u64> {
        if let Some(entry) = self.store.get(path) {
            Some(entry.get_size())
        } else {
            None
        }
    }

    fn remove_path(&mut self, path: &PathBuf) {
        self.store.remove(path);
    }

    fn get_nodes_len(&self) -> usize {
        self.store.keys().len()
    }

    fn process_path(&mut self, path_buf: &DataStoreKey) -> Vec<PathBuf> {
        let mut to_process_subfolders: Vec<PathBuf> = vec![];

        if !self.has_path(path_buf) {
            let mut folder = path_to_folder(path_buf.clone());
            for child_entry in folder.entries.iter_mut() {
                if child_entry.kind == FolderEntryType::Folder {
                    let mut subfolder_path = path_buf.clone();
                    subfolder_path.push(&child_entry.title);
                    child_entry.size = self.get_entry_size(&subfolder_path);
                    folder.sorted_by = None;

                    to_process_subfolders.push(subfolder_path);
                }
            }
            self.set_folder(&path_buf, folder.clone());

            let mut t = folder.clone();
            let mut p = path_buf.clone();

            while let Some(parent_buf) = p.parent() {
                if parent_buf == p {
                    break;
                }
                if let Some(parent_folder) = self.get_folder_mut(&PathBuf::from(parent_buf)) {
                    for entry in parent_folder.entries.iter_mut() {
                        if entry.title == t.title {
                            entry.size = Some(t.get_size());
                            parent_folder.sorted_by = None;

                            break;
                        }
                    }
                    t = parent_folder.clone();
                    p = parent_buf.to_path_buf();
                } else {
                    break;
                }
            }

            to_process_subfolders
        } else {
            to_process_subfolders
        }
    }
}
