use crate::config::UIConfig;
use crate::ui::constants::TEXT_UNKNOWN;
use std::fs::{read_dir, remove_dir_all, remove_file};
use std::path::{Path, PathBuf};
use trash;

mod folder;
mod folder_entry;
mod store;
pub use folder::Folder;
pub use folder_entry::{FolderEntry, FolderEntryType};
pub use store::{DSHashmap, DataStore, DataStoreType};

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
                    folder.push_entry(folder_entry);
                }
            }
        }
        Err(_) => {
            folder.has_error = true;
        }
    }

    folder
}

/// Walks from `start_path`'s own containing folder up through every
/// ancestor, applying `apply` to whichever ancestor entry represents the
/// folder that was just left. The entry is found by matching on title
/// (never by cursor position, which reflects UI navigation state, not
/// identity), so this stays correct regardless of where the cursor happens
/// to be sitting in each ancestor. Mirrors how sizes are accumulated
/// upwards while scanning.
pub fn propagate_folder_change<S: DataStore>(
    store: &mut S,
    start_path: &Path,
    start_title: &str,
    mut apply: impl FnMut(&mut FolderEntry),
) {
    let mut child_title = start_title.to_string();
    let mut path_traverse = start_path.to_path_buf();

    while let Some(parent) = path_traverse.parent() {
        let parent_buf = parent.to_path_buf();
        let Some(parent_folder) = store.get_folder_mut(&parent_buf) else {
            break;
        };

        if let Some(entry) = parent_folder.entry_mut_by_title(&child_title) {
            if entry.kind == FolderEntryType::Folder {
                apply(entry);
                parent_folder.sorted_by = None;
            }
        }

        child_title = parent_folder.title.clone();
        path_traverse = parent_buf;
    }
}

pub fn delete_folder(path: &PathBuf, config: &UIConfig) -> std::io::Result<()> {
    if config.move_to_trash {
        match trash::delete(path) {
            Ok(_) => Ok(()),
            Err(err) => Err(std::io::Error::other(err)),
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
            Err(err) => Err(std::io::Error::other(err)),
        }
    } else {
        remove_file(path)?;
        Ok(())
    }
}
