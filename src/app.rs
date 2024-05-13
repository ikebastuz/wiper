use opener;
use std::error;

use crate::fps_counter::FPSCounter;
use crate::fs::{delete_file, delete_folder, DataStore, DataStoreKey, FolderEntryType, SortBy};
use crate::task_manager::TaskManager;
use std::path::PathBuf;

use crate::config::{InitConfig, UIConfig};
use std::env;

use crate::logger::{Logger, MessageLevel};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

enum DiffKind {
    Subtract,
}

/// Application.
#[derive(Debug)]
pub struct App<S: DataStore<DataStoreKey>> {
    /// Config to render UI
    pub ui_config: UIConfig,
    /// Is the application running?
    pub running: bool,
    /// Task manager for async jobs
    pub task_manager: TaskManager<S>,
    /// Store for filesystem data
    pub store: S,
    /// Debug logger
    pub logger: Logger,
    pub fps_counter: FPSCounter,
}

impl<S: DataStore<DataStoreKey>> App<S> {
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

        let mut app = App {
            running: true,
            ui_config: UIConfig {
                colored: true,
                confirming_deletion: false,
                sort_by: SortBy::Size,
                move_to_trash: true,
                open_file: true,
                debug_enabled: false,
            },
            task_manager: TaskManager::<S>::new(),
            store: S::new(),
            logger: Logger::new(),
            fps_counter: FPSCounter::new(),
        };

        app.store.set_current_path(&current_path);

        app
    }

    pub fn init(&mut self) {
        let path_buf = self.store.get_current_path().clone();
        self.logger
            .log(path_buf.to_string_lossy().to_string(), MessageLevel::Info);
        self.task_manager.maybe_add_task(&self.store, &path_buf);
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.task_manager.process_next_batch();
        self.task_manager.read_receiver_stack(&mut self.store);
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
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
    }

    fn sort_current_folder(&mut self) {
        self.store
            .sort_current_folder(self.ui_config.sort_by.clone());
    }

    // MIGRATE: DONE
    pub fn on_toggle_move_to_trash(&mut self) {
        self.ui_config.move_to_trash = !self.ui_config.move_to_trash;
    }

    pub fn on_cursor_up(&mut self) {
        if let Some(folder) = self.store.get_current_folder_mut() {
            if folder.cursor_index > 0 {
                folder.cursor_index -= 1;
            }
        }
        self.ui_config.confirming_deletion = false;
    }

    pub fn on_cursor_down(&mut self) {
        if let Some(folder) = self.store.get_current_folder_mut() {
            if folder.cursor_index < folder.entries.len() - 1 {
                folder.cursor_index += 1;
            }
        }
        self.ui_config.confirming_deletion = false;
    }

    // MIGRATE: DONE
    fn navigate_to_parent(&mut self) {
        let to_process_subfolders = self.store.move_to_parent();

        self.logger.log(
            self.store.get_current_path().to_string_lossy().to_string(),
            MessageLevel::Info,
        );
        for subfolder in to_process_subfolders {
            self.task_manager.maybe_add_task(&self.store, &subfolder);
        }
    }

    fn navigate_to_child(&mut self, title: &String) {
        let child_path = self.store.move_to_child(title);
        self.logger
            .log(child_path.to_string_lossy().to_string(), MessageLevel::Info);
        self.task_manager.maybe_add_task(&self.store, &child_path);
    }

    pub fn on_backspace(&mut self) {
        self.navigate_to_parent();
    }

    pub fn on_enter(&mut self) {
        if let Some(folder) = self.store.get_current_folder().cloned() {
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
                        let mut file_name = self.store.get_current_path().clone();
                        file_name.push(entry.title.clone());
                        let _ = opener::open(file_name);
                    }
                }
            }
        }
        self.ui_config.confirming_deletion = false;
    }

    pub fn on_delete(&mut self) {
        if let Some(mut folder) = self.store.get_current_folder().cloned() {
            let entry = folder.get_selected_entry();

            let mut to_delete_path = PathBuf::from(&self.store.get_current_path());
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
                            self.store.remove_path(&to_delete_path);
                            self.store.set_current_folder(folder.clone());
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
                                let parent_folder = PathBuf::from(to_delete_path.parent().unwrap());
                                self.propagate_size_update_upwards(
                                    &parent_folder,
                                    subfile_size,
                                    DiffKind::Subtract,
                                );
                            }
                            folder.remove_selected();
                            self.store.set_current_folder(folder.clone());
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
        // TODO: check if after changing parent size
        // we need to re-sort folder
        let mut parent_path = to_delete_path.clone();
        while let Some(parent) = parent_path.parent() {
            if let Some(parent_folder) = self.store.get_folder_mut(&parent_path) {
                if let Some(parent_folder_entry) =
                    parent_folder.entries.get_mut(parent_folder.cursor_index)
                {
                    if let Some(size) = parent_folder_entry.size.as_mut() {
                        match diff_kind {
                            DiffKind::Subtract => *size -= entry_diff,
                        }
                    }
                } else {
                }
                parent_folder.sorted_by = None;
                parent_path = parent.to_path_buf();
            } else {
                break;
            }
        }
    }

    pub fn toggle_debug(&mut self) {
        self.ui_config.debug_enabled = !self.ui_config.debug_enabled;
    }

    pub fn pre_render(&mut self) {
        self.sort_current_folder();
    }
}
