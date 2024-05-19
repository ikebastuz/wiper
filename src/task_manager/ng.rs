use crate::fs::{DataStore, DataStoreKey, Folder, FolderEntry, FolderEntryType};
use crate::logger::Logger;
use crossbeam::channel::{Receiver, Sender};
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
pub struct TaskManagerNg<S: DataStore<DataStoreKey>> {
    pub event_tx: Sender<TraversalEvent>,
    pub event_rx: Receiver<TraversalEvent>,
    pub temp_has_work: bool,
    _store: PhantomData<S>,
}

impl<S: DataStore<DataStoreKey>> TaskManagerNg<S> {
    pub fn new() -> Self {
        let (entry_tx, entry_rx) = crossbeam::channel::bounded(100);
        Self {
            event_rx: entry_rx,
            event_tx: entry_tx,
            temp_has_work: false,
            _store: PhantomData,
        }
    }

    pub fn start(&mut self, input: Vec<DataStoreKey>, logger: &mut Logger) {
        logger.start_timer("TM-NG-proc");
        self.temp_has_work = true;
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
                        let parent_path = e.parent_path.to_path_buf();
                        let title = e.file_name.to_string_lossy().to_string();
                        let kind: FolderEntryType = match e.file_type().is_dir() {
                            true => FolderEntryType::Folder,
                            false => FolderEntryType::File,
                        };
                        let size = match e.client_state.as_ref() {
                            Some(Ok(my_entry)) => my_entry.size,
                            _ => 0,
                        };

                        let folder_entry = FolderEntry {
                            title: title.clone(),
                            size: Some(size),
                            is_loaded: true,
                            kind,
                        };

                        let parent_folder = store.get_folder_mut(&parent_path.to_path_buf());

                        match parent_folder {
                            Some(folder) => {
                                folder.entries.push(folder_entry);
                            }
                            None => {
                                let mut folder = Folder::new(title);
                                folder.entries.push(folder_entry);
                                store.set_folder(&PathBuf::from(parent_path.to_path_buf()), folder);
                            }
                        };
                        // --------------
                        // Update parent's sizes
                        // let mut folder_traverse = parent_folder.clone();
                        // let mut path_traverse = parent_path.to_path_buf();
                        //
                        // while let Some(parent_buf) = path_traverse.parent() {
                        //     if parent_buf == path_traverse {
                        //         break;
                        //     }
                        //     if let Some(parent_folder) =
                        //         store.get_folder_mut(&PathBuf::from(parent_buf))
                        //     {
                        //         for entry in parent_folder.entries.iter_mut() {
                        //             if entry.title == folder_traverse.title {
                        //                 entry.size = Some(folder_traverse.get_size());
                        //                 parent_folder.sorted_by = None;
                        //
                        //                 break;
                        //             }
                        //         }
                        //         folder_traverse = parent_folder.clone();
                        //         path_traverse = parent_buf.to_path_buf();
                        //     } else {
                        //         break;
                        //     }
                        // }
                    }
                    Err(_) => {
                        logger.log(format!("Done?"), None);
                    }
                },
                TraversalEvent::Finished(_) => {
                    self.temp_has_work = false;
                    logger.stop_timer("TM-NG-proc");
                }
            }
        }
    }

    pub fn iter_from_path(root_path: &PathBuf) -> WalkDir {
        let threads = num_cpus::get();

        // let ignore_dirs = [PathBuf::from("/Users/alexk/work/personal/rust/temp/src/a2")];
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
