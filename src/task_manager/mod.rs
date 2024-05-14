use crate::fs::{path_to_folder, DataStore, DataStoreKey, Folder, FolderEntryType};
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::SystemTime;

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
    pub running_tasks: Arc<Mutex<usize>>,
    _store: PhantomData<S>,
}

fn _heavy_computation() {
    let mut _sum = 0.0;
    for i in 0..10_000_000 {
        _sum += (i as f64).sqrt();
    }
}
impl<S: DataStore<DataStoreKey>> TaskManager<S> {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        let path_buf_stack = Arc::new(Mutex::new(VecDeque::<PathBuf>::new()));
        let running_tasks = Arc::new(Mutex::new(0));

        let worker_stack = Arc::clone(&path_buf_stack);
        let worker_sender = sender.clone();
        let running_tasks_clone = Arc::clone(&running_tasks);
        thread::spawn(move || loop {
            let task = {
                let mut stack = worker_stack.lock().unwrap();
                stack.pop_front()
            };

            if let Some(path_buf) = task {
                let mut tasks = running_tasks_clone.lock().unwrap();
                *tasks += 1;
                drop(tasks);

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
            running_tasks,
            _store: PhantomData,
        }
    }

    pub fn add_task(&mut self, path_buf: &PathBuf) {
        {
            let mut stack = self.path_buf_stack.lock().unwrap();
            stack.push_back(path_buf.to_path_buf());
        } // Lock is released here

        self.maybe_start_timer();
    }

    pub fn is_done(&self) -> bool {
        let stack = self.path_buf_stack.lock().unwrap();
        let running_tasks = self.running_tasks.lock().unwrap();
        stack.is_empty() && *running_tasks == 0
    }

    pub fn maybe_add_task(&mut self, store: &S, path_buf: &PathBuf) {
        if !store.has_path(&path_buf) {
            self.add_task(path_buf);
        }
    }

    pub fn handle_results(&mut self, store: &mut S) {
        let mut tasks_finished = 0;
        loop {
            match self.receiver.try_recv() {
                Ok((path_buf, folder)) => {
                    tasks_finished += 1;
                    self.process_entry(store, &path_buf, folder);
                }
                _ => {
                    break;
                }
            }
        }

        self.maybe_stop_timer();

        let mut running_tasks = self.running_tasks.lock().unwrap();
        *running_tasks -= tasks_finished;
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
    fn maybe_start_timer(&mut self) {
        if let Ok(duration) = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            if self.task_timer.start.is_none() {
                // Start is None - record start
                self.task_timer.start = Some(duration.as_millis());
            } else {
                // Start is not None
                if self.task_timer.finish.is_some() {
                    // Finish is not None - restart
                    self.task_timer.start = Some(duration.as_millis());
                    self.task_timer.finish = None;
                }
            };
        };
    }

    fn maybe_stop_timer(&mut self) {
        if let Ok(duration) = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            if self.path_buf_stack.lock().unwrap().is_empty()
                && *self.running_tasks.lock().unwrap() == 0
            {
                if self.task_timer.start.is_some() && self.task_timer.finish.is_none() {
                    self.task_timer.finish = Some(duration.as_millis());
                }
            }
        };
    }
}
