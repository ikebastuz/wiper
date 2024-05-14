use crate::fs::{path_to_folder, DataStore, DataStoreKey, Folder, FolderEntryType};
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
pub struct TaskTimer {
    pub start: Option<u128>,
    pub finish: Option<u128>,
}

#[derive(Debug)]
pub struct TaskManager<S: DataStore<DataStoreKey>> {
    /// Stack of file paths to process
    pub path_buf_stack: Arc<Mutex<VecDeque<PathBuf>>>,
    /// Single receiver to accept processed paths
    pub receiver: Receiver<(PathBuf, Folder)>,
    /// Sender associated with the single receiver
    pub sender: Sender<(PathBuf, Folder)>,
    /// Job execution timer
    pub task_timer: TaskTimer,
    _store: PhantomData<S>,
}

impl<S: DataStore<DataStoreKey>> TaskManager<S> {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        let path_buf_stack = Arc::new(Mutex::new(VecDeque::<PathBuf>::new()));

        // Spawn a single background thread
        let worker_stack = Arc::clone(&path_buf_stack);
        let worker_sender = sender.clone();
        thread::spawn(move || loop {
            let task = {
                let mut stack = worker_stack.lock().unwrap();
                stack.pop_front()
            };
            if let Some(path_buf) = task {
                let folder = path_to_folder(path_buf.clone());
                let _ = worker_sender.send((path_buf, folder));
            } else {
                thread::sleep(std::time::Duration::from_millis(100));
            }
        });

        TaskManager {
            path_buf_stack,
            receiver,
            sender,
            task_timer: TaskTimer {
                start: None,
                finish: None,
            },
            _store: PhantomData,
        }
    }

    pub fn add_task(&self, path_buf: &PathBuf) {
        let mut stack = self.path_buf_stack.lock().unwrap();
        stack.push_back(path_buf.to_path_buf());
    }

    pub fn is_done(&self) -> bool {
        let stack = self.path_buf_stack.lock().unwrap();
        stack.is_empty()
    }

    pub fn maybe_add_task(&mut self, store: &S, path_buf: &PathBuf) {
        if !store.has_path(&path_buf) {
            self.add_task(path_buf);
        }
    }

    pub fn handle_results(&mut self, store: &mut S) {
        loop {
            match self.receiver.try_recv() {
                Ok((path_buf, folder)) => {
                    self.process_entry(store, &path_buf, folder);
                }
                _ => {
                    break;
                }
            }
        }
        self.maybe_stop_timer();
    }

    pub fn process_entry(&mut self, store: &mut S, path_buf: &PathBuf, mut folder: Folder) {
        for child_entry in folder.entries.iter_mut() {
            if child_entry.kind == FolderEntryType::Folder {
                let mut subfolder_path = path_buf.clone();
                subfolder_path.push(&child_entry.title);
                child_entry.size = store.get_entry_size(&subfolder_path);
                folder.sorted_by = None;

                self.maybe_add_task(&store, &subfolder_path)
            }
        }
        store.set_folder(&path_buf, folder.clone());

        let mut t = folder.clone();
        let mut p = path_buf.clone();

        while let Some(parent_buf) = p.parent() {
            if parent_buf == p {
                break;
            }
            if let Some(parent_folder) = store.get_folder_mut(&PathBuf::from(parent_buf)) {
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
    }
    fn maybe_stop_timer(&mut self) {}
}
