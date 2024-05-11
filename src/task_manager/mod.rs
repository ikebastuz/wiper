use crate::app::{get_entry_size, FileTreeMap};
use crate::fs::{path_buf_to_string, path_to_folder, Folder, FolderEntryType};
use std::collections::VecDeque;
use std::path::PathBuf;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;

// TODO: Explore avoiding hardcoding amount of worker threads
const THREAD_LIMIT: usize = 1000;

#[derive(Debug)]
pub struct TaskManager {
    /// Stack of file paths to process
    pub path_buf_stack: VecDeque<PathBuf>,
    /// Stack of receivers to accept processed path
    pub receiver_stack: Vec<Receiver<(PathBuf, Folder)>>,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager {
            path_buf_stack: VecDeque::new(),
            receiver_stack: Vec::new(),
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

    pub fn read_receiver_stack(&mut self, file_tree_map: &mut FileTreeMap) {
        let mut idx = 0;
        while idx < self.receiver_stack.len() {
            match self.receiver_stack[idx].try_recv() {
                Ok(result) => {
                    let (path_buf, mut folder) = result;
                    // Push child folders to stack
                    for child_entry in folder.entries.iter_mut() {
                        if child_entry.kind == FolderEntryType::Folder {
                            let mut subfolder_path = path_buf.clone();
                            subfolder_path.push(&child_entry.title);
                            child_entry.size = get_entry_size(file_tree_map, &subfolder_path);
                            folder.sorted_by = None;

                            self.maybe_add_task(file_tree_map, &subfolder_path);
                        }
                    }

                    file_tree_map.insert(path_buf_to_string(&path_buf), folder.clone());

                    let mut t = folder.clone();
                    let mut p = path_buf.clone();

                    while let Some(parent_buf) = p.parent() {
                        if parent_buf == p {
                            break;
                        }
                        if let Some(parent_folder) =
                            file_tree_map.get_mut(parent_buf.to_str().unwrap())
                        {
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
                    // TODO: probably unsafe
                    self.receiver_stack.remove(idx);
                }
                Err(_) => {
                    idx += 1;
                }
            }
        }
    }

    pub fn add_task(&mut self, path_buf: &PathBuf) {
        self.path_buf_stack
            .push_back(path_buf.to_path_buf().clone());
    }

    pub fn maybe_add_task(&mut self, file_tree: &FileTreeMap, path_buf: &PathBuf) {
        if !file_tree.contains_key(&path_buf.to_string_lossy().to_string()) {
            self.add_task(path_buf);
        }
    }
}
