use crate::fs::{path_to_folder, DataStore, DataStoreKey, Folder, FolderEntryType};
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::time::SystemTime;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;

// TODO: Explore avoiding hardcoding amount of worker threads
const THREAD_LIMIT: usize = 1000;

#[derive(Debug)]
pub struct TaskTimer {
    pub start: Option<u128>,
    pub finish: Option<u128>,
}

#[derive(Debug)]
pub struct TaskManager<S: DataStore<DataStoreKey>> {
    /// Stack of file paths to process
    pub path_buf_stack: VecDeque<PathBuf>,
    /// Stack of receivers to accept processed path
    pub receiver_stack: Vec<Receiver<(PathBuf, Folder)>>,
    /// Job execution timer
    pub task_timer: TaskTimer,
    _store: PhantomData<S>,
}

impl<S: DataStore<DataStoreKey>> TaskManager<S> {
    pub fn new() -> Self {
        TaskManager::<S> {
            path_buf_stack: VecDeque::new(),
            receiver_stack: Vec::new(),
            task_timer: TaskTimer {
                start: None,
                finish: None,
            },
            _store: PhantomData,
        }
    }

    pub fn is_done(&mut self) -> bool {
        self.receiver_stack.len() == 0 && self.path_buf_stack.len() == 0
    }

    pub fn process_next_batch(&mut self) {
        let free_threads = THREAD_LIMIT - self.receiver_stack.len();

        if free_threads > 0 {
            let new_tasks = free_threads.min(self.path_buf_stack.len());
            for _ in 0..new_tasks {
                match self.path_buf_stack.pop_front() {
                    Some(pb) => {
                        let (sender, receiver) = mpsc::channel(1);
                        let path_buf_clone = pb.clone();

                        tokio::spawn(async move {
                            let path_buf = path_buf_clone;
                            let folder = path_to_folder(path_buf.clone());
                            let _ = sender.send((path_buf, folder)).await;
                        });

                        self.receiver_stack.push(receiver);
                    }
                    None => {}
                }
            }
        }
    }

    pub fn read_receiver_stack(&mut self, store: &mut S) {
        let mut idx = 0;
        while idx < self.receiver_stack.len() {
            match self.receiver_stack[idx].try_recv() {
                Ok(result) => {
                    let (path_buf, folder) = result;
                    self.process_entry(store, &path_buf, folder);
                    // TODO: probably unsafe
                    self.receiver_stack.remove(idx);
                }
                Err(_) => {
                    idx += 1;
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

    pub fn add_task(&mut self, path_buf: &PathBuf) {
        self.maybe_start_timer();
        self.path_buf_stack.push_back(path_buf.clone());
    }

    pub fn maybe_add_task(&mut self, store: &S, path_buf: &PathBuf) {
        if !store.has_path(&path_buf) {
            self.add_task(path_buf);
        }
    }

    fn maybe_start_timer(&mut self) {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => {
                if self.path_buf_stack.len() == 0 && self.task_timer.start.is_none() {
                    // Start is None - record start
                    self.task_timer.start = Some(duration.as_millis());
                } else {
                    // Start is not None
                    if self.task_timer.finish.is_some() {
                        // Finish is not None - restart
                        self.task_timer.start = Some(duration.as_millis());
                        self.task_timer.finish = None;
                    }
                }
            }
            _ => {}
        };
    }

    fn maybe_stop_timer(&mut self) {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => {
                if self.path_buf_stack.len() == 0 && self.receiver_stack.len() == 0 {
                    if self.task_timer.start.is_some() && self.task_timer.finish.is_none() {
                        self.task_timer.finish = Some(duration.as_millis());
                    }
                }
            }
            _ => {}
        };
    }
}
