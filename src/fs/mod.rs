use crate::config::UIConfig;
use crate::ui::constants::TEXT_UNKNOWN;
use std::fs::{read_dir, remove_dir_all, remove_file};
use std::path::PathBuf;
use trash;

mod folder;
mod folder_entry;
mod store;
pub use folder::Folder;
pub use folder_entry::{FolderEntry, FolderEntryType};
pub use store::{DSHashmap, DataStore, DataStoreKey, DataStoreType};

#[derive(Debug, Clone, PartialEq)]
pub enum SortBy {
    Title,
    Size,
}
/// Returns new unsorted folder
pub fn path_to_folder(path: PathBuf) -> Folder {
    let folder_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(TEXT_UNKNOWN);
    let mut folder = Folder::new(folder_name.to_string());

    match read_dir(path.clone()) {
        Ok(path) => {
            for entry in path.into_iter().flatten() {
                let file_name = entry.file_name();
                if let Some(file_name) = file_name.to_str() {
                    let mut folder_entry = FolderEntry {
                        kind: FolderEntryType::File,
                        title: file_name.to_owned(),
                        size: None,
                        is_loaded: true,
                    };
                    if entry.path().is_dir() {
                        folder_entry.kind = FolderEntryType::Folder;
                    } else {
                        match entry.metadata() {
                            Ok(metadata) => {
                                folder_entry.size = Some(metadata.len());
                            }
                            Err(_) => {
                                folder.has_error = true;
                            }
                        }
                    }
                    folder.entries.push(folder_entry);
                }
            }
        }
        Err(_) => {
            folder.has_error = true;
        }
    }

    folder
}

pub fn delete_folder(path: &PathBuf, config: &UIConfig) -> std::io::Result<()> {
    if config.move_to_trash {
        match trash::delete(path) {
            Ok(_) => Ok(()),
            Err(err) => Err(std::io::Error::new(std::io::ErrorKind::Other, err)),
        }
    } else {
        remove_dir_all(path)?;
        Ok(())
    }
}

pub fn delete_file(path: &PathBuf, config: &UIConfig) -> std::io::Result<()> {
    if config.move_to_trash {
        match trash::delete(path) {
            Ok(_) => Ok(()),
            Err(err) => Err(std::io::Error::new(std::io::ErrorKind::Other, err)),
        }
    } else {
        remove_file(path)?;
        Ok(())
    }
}
