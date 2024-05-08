use opener;
use std::collections::VecDeque;
use std::error;

use crate::fs::{delete_file, delete_folder, path_to_folder, Folder, FolderEntryType, SortBy};
use std::time::SystemTime;
use std::{collections::HashMap, path::PathBuf};
use tokio::sync::mpsc::Receiver;

use crate::config::{InitConfig, UIConfig};
use std::env;

use tokio::sync::mpsc;

const THREAD_LIMIT: usize = 1000;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

enum DiffKind {
    Subtract,
}

type FileTreeMap = HashMap<String, Folder>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Current file path buffer
    pub current_path: PathBuf,
    pub ui_config: UIConfig,
    pub file_tree_map: FileTreeMap,

    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: i8,

    pub time: u128,
    pub path_buf_stack: VecDeque<PathBuf>,
    pub receiver_stack: Vec<Receiver<(PathBuf, Folder)>>,
    pub attempts_to_read: u64,
}

impl Default for App {
    fn default() -> Self {
        Self {
            current_path: PathBuf::from("."),
            file_tree_map: HashMap::new(),
            running: true,
            counter: 0,
            time: 0,
            path_buf_stack: VecDeque::new(),
            receiver_stack: vec![],
            attempts_to_read: 0,
            ui_config: UIConfig {
                colored: false,
                confirming_deletion: false,
                sort_by: SortBy::Title,
                move_to_trash: true,
                open_file: true,
            },
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(config: InitConfig) -> Self {
        let current_path = match config.file_path {
            Some(path) => {
                let path_buf = PathBuf::from(&path);
                if path_buf.is_absolute() {
                    path_buf
                } else {
                    let current_dir = env::current_dir().unwrap();
                    let abs_path = current_dir.join(&path_buf);
                    abs_path
                }
            }
            None => env::current_dir().unwrap(),
        };

        let app = App {
            current_path,
            ..Self::default()
        };

        app
    }

    pub fn init(&mut self) {
        self.process_filepath(&self.current_path.clone());
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        let mut idx = 0;

        let free_threads = THREAD_LIMIT - self.receiver_stack.len();

        if free_threads > 0 {
            let new_tasks = free_threads.min(self.path_buf_stack.len());
            for _ in 0..new_tasks {
                match self.path_buf_stack.pop_back() {
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
        while idx < self.receiver_stack.len() {
            self.attempts_to_read += 1;
            match self.receiver_stack[idx].try_recv() {
                Ok(result) => {
                    let (path_buf, mut folder) = result;
                    for child_entry in folder.entries.iter_mut() {
                        if child_entry.kind == FolderEntryType::Folder {
                            let mut subfolder_path = path_buf.clone();
                            subfolder_path.push(&child_entry.title);

                            let subfolder_size = self.process_filepath(&subfolder_path);
                            child_entry.size = Some(subfolder_size);
                        }
                    }
                    self.file_tree_map
                        .insert(path_buf.to_string_lossy().to_string(), folder.clone());

                    let mut t = folder.clone();
                    let mut p = path_buf.clone();

                    while let Some(parent_buf) = p.parent() {
                        if parent_buf == p {
                            break;
                        }
                        if let Some(parent_folder) =
                            self.file_tree_map.get_mut(parent_buf.to_str().unwrap())
                        {
                            for entry in parent_folder.entries.iter_mut() {
                                if entry.title == t.title {
                                    entry.size = Some(t.get_size());

                                    match self.ui_config.sort_by {
                                        SortBy::Size => {
                                            parent_folder.sort_by_size();
                                        }
                                        SortBy::Title => {
                                            parent_folder.sort_by_title();
                                        }
                                    }

                                    break;
                                }
                            }
                            t = parent_folder.clone();
                        }
                        p = parent_buf.to_path_buf();
                    }

                    self.receiver_stack.remove(idx);
                }
                Err(_) => {
                    idx += 1;
                }
            }
        }

        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => self.time = duration.as_millis(),
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        };
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }

    fn get_current_path_string(&self) -> String {
        self.current_path.to_string_lossy().to_string()
    }

    pub fn get_current_folder(&self) -> Option<&Folder> {
        self.file_tree_map.get(&self.get_current_path_string())
    }

    fn process_filepath(&mut self, path_buf: &PathBuf) -> u64 {
        if !self
            .file_tree_map
            .contains_key(&path_buf.to_string_lossy().to_string())
        {
            let path_string = path_buf.to_string_lossy().into_owned();

            if let Some(folder) = self.file_tree_map.get(&path_string) {
                return folder.get_size();
            }

            self.path_buf_stack.push_back(path_buf.to_path_buf());
            //
            // let (sender, receiver) = mpsc::channel(1);
            // let path_buf_clone = path_buf.clone();
            //
            // tokio::spawn(async move {
            //     let path_buf = path_buf_clone;
            //     let folder = path_to_folder(path_buf.clone());
            //     let _ = sender.send((path_buf, folder)).await;
            // });
            //
            // self.receiver_stack.push(receiver);

            return 0;
        }

        0
    }

    pub fn on_toggle_coloring(&mut self) {
        self.ui_config.colored = !self.ui_config.colored;
    }

    pub fn on_toggle_sorting(&mut self) {
        match self.ui_config.sort_by {
            SortBy::Title => {
                self.ui_config.sort_by = SortBy::Size;
            }
            SortBy::Size => {
                self.ui_config.sort_by = SortBy::Title;
            }
        }

        self.sort_current_folder();
    }

    fn sort_current_folder(&mut self) {
        if let Some(mut folder) = self.get_current_folder().cloned() {
            match self.ui_config.sort_by {
                SortBy::Size => {
                    folder.sort_by_size();
                }
                SortBy::Title => {
                    folder.sort_by_title();
                }
            }
            self.set_current_folder(folder);
        }
    }
    fn set_current_folder(&mut self, folder: Folder) {
        self.file_tree_map
            .insert(self.get_current_path_string(), folder);
    }

    pub fn on_toggle_move_to_trash(&mut self) {
        self.ui_config.move_to_trash = !self.ui_config.move_to_trash;
    }

    fn get_current_folder_v2(&mut self) -> Option<&mut Folder> {
        self.file_tree_map.get_mut(&self.get_current_path_string())
    }

    pub fn on_cursor_up(&mut self) {
        if let Some(folder) = self.get_current_folder_v2() {
            if folder.cursor_index > 0 {
                folder.cursor_index -= 1;
            }
        }
        self.ui_config.confirming_deletion = false;
    }

    pub fn on_cursor_down(&mut self) {
        if let Some(folder) = self.get_current_folder_v2() {
            if folder.cursor_index < folder.entries.len() - 1 {
                folder.cursor_index += 1;
            }
        }
        self.ui_config.confirming_deletion = false;
    }

    fn navigate_to_parent(&mut self) {
        if let Some(parent) = PathBuf::from(&self.current_path).parent() {
            let parent_buf = parent.to_path_buf();
            self.current_path = parent_buf.clone();
            self.process_filepath(&parent_buf);
            self.sort_current_folder();
        }
    }

    fn navigate_to_child(&mut self, title: &String) {
        let mut new_path = PathBuf::from(&self.current_path);
        new_path.push(title);
        self.current_path = new_path;
        self.process_filepath(&PathBuf::from(&self.current_path));
        self.sort_current_folder();
    }

    pub fn on_backspace(&mut self) {
        self.navigate_to_parent();
    }

    pub fn on_enter(&mut self) {
        if let Some(folder) = self.get_current_folder().cloned() {
            let entry = folder.get_selected_entry();

            match entry.kind {
                FolderEntryType::Parent => {
                    self.navigate_to_parent();
                }
                FolderEntryType::Folder => {
                    self.navigate_to_child(&entry.title);
                }
                FolderEntryType::File => {
                    if self.ui_config.open_file {
                        let mut file_name = PathBuf::from(&self.current_path.clone());
                        file_name.push(entry.title.clone());
                        let _ = opener::open(file_name);
                    }
                }
            }
        }
        self.ui_config.confirming_deletion = false;
    }

    pub fn on_delete(&mut self) {
        if let Some(mut folder) = self.get_current_folder().cloned() {
            let entry = folder.get_selected_entry();

            let mut to_delete_path = PathBuf::from(&self.current_path);
            to_delete_path.push(&entry.title);

            match entry.kind {
                FolderEntryType::Parent => {}
                FolderEntryType::Folder => {
                    if !self.ui_config.confirming_deletion {
                        self.ui_config.confirming_deletion = true;
                    } else {
                        if let Ok(_) = delete_folder(&to_delete_path, &self.ui_config) {
                            if let Some(subfolder_size) = entry.size {
                                self.propagate_size_update_upwards(
                                    &to_delete_path,
                                    subfolder_size,
                                    DiffKind::Subtract,
                                );
                            }
                            folder.remove_selected();
                            let path_string = to_delete_path.to_string_lossy().into_owned();
                            self.file_tree_map.remove(&path_string);
                            self.set_current_folder(folder);
                            self.ui_config.confirming_deletion = false;
                        }
                    }
                }
                FolderEntryType::File => {
                    if !self.ui_config.confirming_deletion {
                        self.ui_config.confirming_deletion = true;
                    } else {
                        if let Ok(_) = delete_file(&to_delete_path, &self.ui_config) {
                            if let Some(subfile_size) = entry.size {
                                self.propagate_size_update_upwards(
                                    &to_delete_path,
                                    subfile_size,
                                    DiffKind::Subtract,
                                );
                            }
                            folder.remove_selected();
                            self.set_current_folder(folder);
                            self.ui_config.confirming_deletion = false;
                        }
                    }
                }
            }
        }
    }

    fn propagate_size_update_upwards(
        &mut self,
        to_delete_path: &PathBuf,
        entry_diff: u64,
        diff_kind: DiffKind,
    ) {
        let mut parent_path = to_delete_path.clone();
        while let Some(parent) = parent_path.parent() {
            if let Some(parent_folder) = self.file_tree_map.get_mut(parent.to_str().unwrap()) {
                if let Some(parent_folder_entry) =
                    parent_folder.entries.get_mut(parent_folder.cursor_index)
                {
                    if let Some(size) = parent_folder_entry.size.as_mut() {
                        match diff_kind {
                            DiffKind::Subtract => *size -= entry_diff,
                        }
                    }
                }
                parent_path = parent.to_path_buf();
            } else {
                break;
            }
        }
    }
}

#[path = "tests.rs"]
mod tests;
