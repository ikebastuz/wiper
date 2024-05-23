use crate::fs::{path_to_folder, DataStore, DataStoreKey, Folder, FolderEntry, FolderEntryType};
use crate::logger::Logger;
use crossbeam::channel::{Receiver, Sender};
use std::ffi::OsStr;
use std::marker::PhantomData;
use std::path::PathBuf;

#[derive(Debug)]
pub struct EntryState {
    size: u64,
}

type WalkDir = jwalk::WalkDirGeneric<((), Option<Result<EntryState, jwalk::Error>>)>;

pub type TraversalEntry =
    Result<jwalk::DirEntry<((), Option<Result<EntryState, jwalk::Error>>)>, jwalk::Error>;

#[derive(Debug)]
pub enum TraversalEvent {
    Entry(TraversalEntry),
    Finished(u64),
}

#[derive(Debug)]
pub struct TaskManager<S: DataStore<DataStoreKey>> {
    pub event_tx: Sender<TraversalEvent>,
    pub event_rx: Receiver<TraversalEvent>,
    pub is_working: bool,
    _store: PhantomData<S>,
}

impl<S: DataStore<DataStoreKey>> TaskManager<S> {
    pub fn new() -> Self {
        let (entry_tx, entry_rx) = crossbeam::channel::bounded(100);
        Self {
            event_rx: entry_rx,
            event_tx: entry_tx,
            is_working: false,
            _store: PhantomData,
        }
    }

    pub fn is_done(&self) -> bool {
        !self.is_working
    }

    pub fn start(&mut self, input: Vec<DataStoreKey>, logger: &mut Logger) {
        logger.start_timer("Traversal");
        self.is_working = true;
        let entry_tx = self.event_tx.clone();
        let _ = std::thread::Builder::new()
            .name("wiper-walk-dispatcher".to_string())
            .spawn({
                move || {
                    for root_path in input.into_iter() {
                        for entry in Self::iter_from_path(&root_path).into_iter() {
                            if entry_tx.send(TraversalEvent::Entry(entry)).is_err() {
                                println!("Send err: channel closed");
                                return;
                            }
                        }
                    }
                    let _ = entry_tx.send(TraversalEvent::Finished(0));
                }
            });
    }

    pub fn process_results(&mut self, store: &mut S, logger: &mut Logger) {
        while let Ok(event) = self.event_rx.try_recv() {
            match event {
                TraversalEvent::Entry(entry) => match entry {
                    Ok(e) => {
                        // Construct entry
                        let belongs_to = e.parent_path.to_path_buf();
                        let title = e.file_name.to_string_lossy().to_string();
                        let extension = e
                            .path()
                            .extension()
                            .and_then(OsStr::to_str)
                            .unwrap_or("")
                            .to_string();

                        let kind = match e.file_type().is_dir() {
                            true => {
                                // Create store record for folder (edge-case for last-leaf-empty
                                // folders)
                                let default_folder = Folder::new(title.clone());
                                store.set_folder(&e.path().clone(), default_folder);

                                FolderEntryType::Folder
                            }
                            false => FolderEntryType::File,
                        };
                        let size = match e.client_state.as_ref() {
                            Some(Ok(my_entry)) => {
                                if kind == FolderEntryType::Folder {
                                    // Ignore folder metadata size
                                    0
                                } else {
                                    my_entry.size
                                }
                            }
                            _ => 0,
                        };

                        let folder_entry = FolderEntry {
                            title: title.clone(),
                            size: Some(size),
                            is_loaded: true,
                            kind,
                        };

                        // Add entry to parent folder
                        let parent_folder = store.get_folder_mut(&belongs_to.to_path_buf());
                        match parent_folder {
                            Some(folder) => {
                                if !extension.is_empty() {
                                    folder.append_file_type_size(&extension, size);
                                }
                                folder.entries.push(folder_entry);
                            }
                            None => {
                                if let Some(belongs_to_name) = belongs_to.file_name() {
                                    let mut folder =
                                        Folder::new(belongs_to_name.to_string_lossy().to_string());
                                    folder.entries.push(folder_entry);
                                    store.set_folder(&belongs_to.to_path_buf(), folder);
                                }
                            }
                        };

                        // Traverse tree up - update parent folder sizes
                        if let Some(title_traverse_os) = belongs_to.file_name() {
                            let mut title_traverse =
                                title_traverse_os.to_string_lossy().to_string();
                            let mut path_traverse = belongs_to.to_path_buf();

                            while let Some(parent_buf) = path_traverse.parent() {
                                if parent_buf == path_traverse {
                                    break;
                                }
                                if let Some(parent_folder) =
                                    store.get_folder_mut(&PathBuf::from(parent_buf))
                                {
                                    // Increment parent's entry size
                                    for child in parent_folder.entries.iter_mut() {
                                        if child.title == title_traverse
                                            && child.kind == FolderEntryType::Folder
                                        {
                                            child.increment_size(size);
                                            parent_folder.sorted_by = None;
                                            break;
                                        }
                                    }
                                    // Update parent's folder file_type_map
                                    if !extension.is_empty() {
                                        parent_folder.append_file_type_size(&extension, size);
                                    }
                                    title_traverse.clone_from(&parent_folder.title);
                                    path_traverse = parent_buf.to_path_buf();
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                    Err(_) => {
                        logger.log("Done".into());
                    }
                },
                TraversalEvent::Finished(_) => {
                    self.is_working = false;
                    logger.stop_timer("Traversal");
                }
            }
        }
    }

    pub fn process_path_sync(&self, store: &mut S, path: &DataStoreKey) -> Vec<DataStoreKey> {
        let mut folder_new = path_to_folder(path.clone());
        let mut paths_to_process: Vec<DataStoreKey> = vec![];
        let mut entries_to_keep: Vec<FolderEntry> = vec![];

        match store.get_folder_mut(path) {
            Some(folder_stored) => {
                // Folder already exists
                for child in folder_new.entries.iter_mut() {
                    if !folder_stored.entries.iter().any(|e| e.title == child.title) {
                        // No entry
                        if child.kind == FolderEntryType::Folder {
                            // Folder -> process
                            let mut child_path = path.clone();
                            child_path.push(child.title.clone());
                            paths_to_process.push(child_path);
                        } else {
                            // File -> simply push
                            folder_stored.entries.push(child.clone());
                        }
                    }
                }
            }
            None => {
                // Folder does not exist
                for child in folder_new.entries.iter_mut() {
                    if child.kind == FolderEntryType::Folder {
                        // Folder -> process
                        let mut child_path = path.clone();
                        child_path.push(child.title.clone());
                        paths_to_process.push(child_path);
                    } else {
                        // File -> simply push
                        entries_to_keep.push(child.clone());
                    }
                }
                folder_new.entries = entries_to_keep;
                store.set_folder(path, folder_new.clone());
            }
        }

        paths_to_process
    }

    pub fn iter_from_path(root_path: &PathBuf) -> WalkDir {
        let threads = num_cpus::get();

        let ignore_dirs = [];

        WalkDir::new(root_path)
            .follow_links(false)
            .skip_hidden(false)
            .process_read_dir({
                move |_, _, _, dir_entry_results| {
                    dir_entry_results.iter_mut().for_each(|dir_entry_result| {
                        if let Ok(dir_entry) = dir_entry_result {
                            let metadata = dir_entry.metadata();

                            if let Ok(metadata) = metadata {
                                dir_entry.client_state = Some(Ok(EntryState {
                                    size: metadata.len(),
                                }));
                            } else {
                                dir_entry.client_state = Some(Err(metadata.unwrap_err()));
                            }

                            if ignore_dirs.contains(&dir_entry.path()) {
                                dir_entry.read_children_path = None;
                            }
                        }
                    })
                }
            })
            .parallelism(match threads {
                0 => jwalk::Parallelism::RayonDefaultPool {
                    busy_timeout: std::time::Duration::from_secs(1),
                },
                1 => jwalk::Parallelism::Serial,
                _ => jwalk::Parallelism::RayonExistingPool {
                    pool: jwalk::rayon::ThreadPoolBuilder::new()
                        .stack_size(128 * 1024)
                        .num_threads(threads)
                        .thread_name(|idx| format!("wiper-walk-{idx}"))
                        .build()
                        .expect("fields we set cannot fail")
                        .into(),
                    busy_timeout: None,
                },
            })
    }
}

impl<S: DataStore<DataStoreKey>> Default for TaskManager<S> {
    fn default() -> Self {
        Self::new()
    }
}
