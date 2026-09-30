use crate::fs::{
    path_to_folder, propagate_folder_change, DataStore, DataStoreType, Folder, FolderEntry,
    FolderEntryType,
};
use crate::logger::Logger;
use crossbeam::channel::{Receiver, Sender};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

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
    /// A single-level directory listing for a path the user just navigated
    /// to, produced off the UI thread by `start_parent_scan`.
    ParentScan(PathBuf, Folder),
    Finished(u64),
}

#[derive(Debug)]
pub struct TaskManager {
    pub event_tx: Sender<TraversalEvent>,
    pub event_rx: Receiver<TraversalEvent>,
    pub is_working: bool,
}

impl TaskManager {
    pub fn new() -> Self {
        let (entry_tx, entry_rx) = crossbeam::channel::bounded(100);
        Self {
            event_rx: entry_rx,
            event_tx: entry_tx,
            is_working: false,
        }
    }

    pub fn is_done(&self) -> bool {
        !self.is_working
    }

    pub fn start(&mut self, input: Vec<PathBuf>, logger: &mut Logger) {
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

    /// Lists `path` off the UI thread and reports back via `ParentScan`,
    /// instead of blocking the caller with a synchronous `read_dir`.
    pub fn start_parent_scan(&mut self, path: PathBuf, logger: &mut Logger) {
        logger.start_timer("Traversal");
        self.is_working = true;
        let entry_tx = self.event_tx.clone();
        let _ = std::thread::Builder::new()
            .name("wiper-parent-scan".to_string())
            .spawn(move || {
                let folder = path_to_folder(path.clone());
                let _ = entry_tx.send(TraversalEvent::ParentScan(path, folder));
            });
    }

    pub fn process_results(&mut self, store: &mut DataStoreType, logger: &mut Logger) {
        while let Ok(event) = self.event_rx.try_recv() {
            match event {
                TraversalEvent::Entry(entry) => Self::process_entry(store, entry, logger),
                TraversalEvent::ParentScan(path, folder_new) => {
                    let to_process = Self::merge_parent_scan(store, &path, folder_new);
                    self.start(to_process, logger);
                }
                TraversalEvent::Finished(_) => {
                    self.is_working = false;
                    logger.stop_timer("Traversal");
                }
            }
        }
    }

    fn process_entry(store: &mut DataStoreType, entry: TraversalEntry, logger: &mut Logger) {
        match entry {
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
                        store.set_folder(&e.path(), default_folder);

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
                match store.get_folder_mut(&belongs_to) {
                    Some(folder) => {
                        if !extension.is_empty() {
                            folder.append_file_type_size(&extension, size);
                        }
                        folder.push_entry(folder_entry);
                    }
                    None => {
                        if let Some(belongs_to_name) = belongs_to.file_name() {
                            let mut folder =
                                Folder::new(belongs_to_name.to_string_lossy().to_string());
                            folder.push_entry(folder_entry);
                            store.set_folder(&belongs_to, folder);
                        }
                    }
                };

                // Traverse tree up - update parent folder sizes
                if let Some(title_traverse_os) = belongs_to.file_name() {
                    let title_traverse = title_traverse_os.to_string_lossy().to_string();

                    propagate_folder_change(store, &belongs_to, &title_traverse, |entry| {
                        entry.increment_size(size);
                    });

                    // Update parent's folder file_type_map too, following the
                    // same ancestor chain.
                    if !extension.is_empty() {
                        let mut path_traverse = belongs_to.to_path_buf();
                        while let Some(parent) = path_traverse.parent() {
                            let parent_buf = parent.to_path_buf();
                            let Some(parent_folder) = store.get_folder_mut(&parent_buf) else {
                                break;
                            };
                            parent_folder.append_file_type_size(&extension, size);
                            path_traverse = parent_buf;
                        }
                    }
                }
            }
            Err(_) => {
                logger.log("Done".into());
            }
        }
    }

    /// Diffs a freshly-listed directory against what the store already
    /// knows about `path`, merging in any newly-discovered files directly
    /// and returning the subfolders that still need a recursive scan.
    fn merge_parent_scan(store: &mut DataStoreType, path: &Path, folder_new: Folder) -> Vec<PathBuf> {
        let mut paths_to_process: Vec<PathBuf> = vec![];

        match store.get_folder_mut(path) {
            Some(folder_stored) => {
                // Folder already exists
                for child in folder_new.entries() {
                    if !folder_stored.has_entry_titled(&child.title) {
                        // No entry
                        if child.kind == FolderEntryType::Folder {
                            // Folder -> process
                            paths_to_process.push(path.join(&child.title));
                        } else {
                            // File -> simply push
                            folder_stored.push_entry(child.clone());
                        }
                    }
                }
            }
            None => {
                // Folder does not exist
                let mut folder_new = folder_new;
                let mut entries_to_keep: Vec<FolderEntry> = vec![];
                for child in folder_new.entries() {
                    if child.kind == FolderEntryType::Folder {
                        // Folder -> process
                        paths_to_process.push(path.join(&child.title));
                    } else {
                        // File -> simply push
                        entries_to_keep.push(child.clone());
                    }
                }
                folder_new.set_entries(entries_to_keep);
                store.set_folder(path, folder_new);
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

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}
