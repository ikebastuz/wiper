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

impl DSHashmap {
    fn get_sorted_file_types_by_size(&self) -> Vec<(String, u64)> {
        let mut file_types: Vec<(String, u64)> = self
            .file_type_map
            .iter()
            .map(|(k, &v)| (k.clone(), v))
            .collect();
        file_types.sort_by(|a, b| b.1.cmp(&a.1));
        file_types
    }
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

    fn move_to_parent(&mut self) {
        if let Some(parent) = &self.current_path.parent() {
            let parent_buf = parent.to_path_buf();
            self.current_path = parent_buf.clone();
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

    fn append_file_type_size(&mut self, file_type: String, size: u64) {
        let total_size = self.file_type_map.entry(file_type).or_insert(0);
        *total_size += size;
    }

    fn get_chart_data(&self, threshold: f64) -> Vec<(String, u64)> {
        let sorted_file_types = self.get_sorted_file_types_by_size();
        let total_size: u64 = sorted_file_types.iter().map(|(_, size)| *size).sum();
        let mut accumulated_size: u64 = 0;
        let mut chart_data: Vec<(String, u64)> = Vec::new();
        let mut rest_size: u64 = 0;

        for (file_type, size) in sorted_file_types {
            if accumulated_size as f64 / total_size as f64 <= threshold {
                chart_data.push((file_type, size));
                accumulated_size += size;
            } else {
                rest_size += size;
            }
        }

        if rest_size > 0 {
            chart_data.push(("rest".to_string(), rest_size));
        }

        chart_data
    }
}
