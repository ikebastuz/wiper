use crate::fs::{DataStore, Folder, SortBy};
use std::collections::HashMap;
use std::path::PathBuf;

pub type FileTreeMap = HashMap<PathBuf, Folder>;

pub struct DSHashmap {
    /// Current file path buffer
    pub current_path: PathBuf,
    /// Map for all file paths
    pub store: FileTreeMap,
}

impl DataStore for DSHashmap {
    fn new() -> DSHashmap {
        DSHashmap {
            current_path: PathBuf::from("."),
            store: HashMap::new(),
        }
    }

    fn current_filepath(&self) -> String {
        self.current_path.to_string_lossy().to_string()
    }

    fn current_folder(&self) -> Option<&Folder> {
        self.store.get(&self.current_path)
    }

    fn current_folder_mut(&mut self) -> Option<&mut Folder> {
        self.store.get_mut(&self.current_path)
    }

    fn set_current_folder(&mut self, folder: Folder) {
        self.store.insert(self.current_path.clone(), folder);
    }

    // TODO: refactor
    fn sort_current_folder(&mut self, sort_by: SortBy) {
        if let Some(folder) = self.current_folder_mut() {
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

    // TODO: Returns string that should be processed (sync)
    fn move_to_parent(&mut self) -> Option<String> {
        if let Some(parent) = &self.current_path.parent() {
            let parent_buf = parent.to_path_buf();
            self.current_path = parent_buf;

            Some(self.current_filepath())
        } else {
            None
        }
    }

    // TODO: Returns string that should be processed
    fn move_to_child(&mut self, title: &String) -> String {
        let mut new_path = PathBuf::from(&self.current_path);
        new_path.push(title);
        self.current_path = new_path;

        self.current_filepath()
    }

    fn delete_current_entry(&mut self) {}
}
