use crate::ui::{UIConfig, TEXT_UNKNOWN};
use std::fs::{read_dir, remove_dir_all, remove_file};
use std::path::PathBuf;
use trash;

mod folder;
mod folder_entry;
pub use folder::Folder;
pub use folder_entry::{FolderEntry, FolderEntryType};

#[derive(Debug)]
pub enum SortBy {
    Title,
    Size,
}

pub fn path_to_folder(path: &PathBuf) -> Folder {
    let folder_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(TEXT_UNKNOWN);
    let mut folder = Folder::new(folder_name.to_string());

    let path = read_dir(path.clone()).expect("Failed to read directory");
    for entry in path {
        if let Ok(entry) = entry {
            let file_name = entry.file_name();
            if let Some(file_name) = file_name.to_str() {
                let mut folder_entry = FolderEntry {
                    kind: FolderEntryType::File,
                    title: file_name.to_owned(),
                    size: None,
                };
                if entry.path().is_dir() {
                    folder_entry.kind = FolderEntryType::Folder;
                    folder.entries.push(folder_entry);
                } else {
                    let metadata = entry.metadata().expect("Failed to get metadata");
                    let size = metadata.len();

                    folder_entry.size = Some(size);
                    folder.entries.push(folder_entry);
                }
            }
        }
    }
    folder.sort_by_title();

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
