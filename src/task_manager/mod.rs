use crate::fs::{path_to_folder, DataStore, DataStoreKey, Folder, FolderEntryType};
use crate::logger::{Logger, MessageLevel};
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::time::SystemTime;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

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
    /// Single receiver to accept processed paths
    pub receiver: Receiver<(PathBuf, Folder)>,
    /// Sender associated with the single receiver
    pub sender: Sender<(PathBuf, Folder)>,
    /// Job execution timer
    pub task_timer: TaskTimer,
    /// Amount of actively running worker threads
    pub active_tasks: usize,
    _store: PhantomData<S>,
}

impl<S: DataStore<DataStoreKey>> TaskManager<S> {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(THREAD_LIMIT);
        TaskManager::<S> {
            path_buf_stack: VecDeque::new(),
            receiver,
            sender,
            task_timer: TaskTimer {
                start: None,
                finish: None,
            },
            active_tasks: 0,
            _store: PhantomData,
        }
    }

    pub fn is_done(&mut self) -> bool {
        self.path_buf_stack.is_empty() && self.active_tasks == 0
    }

    pub fn process_next_batch(&mut self, logger: &mut Logger) {
        let free_threads = THREAD_LIMIT - self.active_tasks;

        if free_threads > 0 {
            let new_tasks = free_threads.min(self.path_buf_stack.len());
            if new_tasks > 0 {
                // TODO: log only when debug is enabled
                logger.log(
                    format!("Spawning {} threads", new_tasks),
                    MessageLevel::Info,
                );
            }
            for _ in 0..new_tasks {
                if let Some(pb) = self.path_buf_stack.pop_front() {
                    let sender_clone = self.sender.clone();
                    let path_buf_clone = pb.clone();
                    self.active_tasks += 1;
                    tokio::spawn(async move {
                        let path_buf = path_buf_clone;
                        let folder = path_to_folder(path_buf.clone());
                        let _ = sender_clone.send((path_buf, folder)).await;
                    });
                }
            }
        }
    }

    pub fn read_receiver_stack(&mut self, store: &mut S, logger: &mut Logger) {
        if self.active_tasks > 0 {
            // TODO: log only when debug is enabled
            logger.log(
                format!("Processing {} tasks", self.active_tasks),
                MessageLevel::Info,
            );
        }
        while let Ok((path_buf, folder)) = self.receiver.try_recv() {
            self.active_tasks -= 1;
            self.process_entry(store, &path_buf, folder);
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
                if self.path_buf_stack.is_empty() && self.task_timer.start.is_none() {
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
                if self.path_buf_stack.is_empty() && self.active_tasks == 0 {
                    if self.task_timer.start.is_some() && self.task_timer.finish.is_none() {
                        self.task_timer.finish = Some(duration.as_millis());
                    }
                }
            }
            _ => {}
        };
    }
}
