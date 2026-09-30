use crate::ui::constants::TEXT_PARENT_DIR;

use crate::fs::folder_entry::{FolderEntry, FolderEntryType};
use std::cmp::Ordering;
use std::collections::HashMap;

use super::SortBy;

#[derive(Debug, Clone)]
pub struct Folder {
    pub title: String,
    pub cursor_index: usize,
    pub sorted_by: Option<SortBy>,
    entries: Vec<FolderEntry>,
    /// Maps an entry's title to its index in `entries`, so ancestor size
    /// propagation can find a child by name in O(1) instead of scanning.
    index_by_title: HashMap<String, usize>,
    pub has_error: bool,
    pub file_type_map: HashMap<String, u64>,
}

impl Folder {
    pub fn new(title: String) -> Self {
        let entries = vec![FolderEntry {
            kind: FolderEntryType::Parent,
            title: String::from(TEXT_PARENT_DIR),
            size: None,
            is_loaded: true,
        }];
        let index_by_title = Self::build_index(&entries);

        Folder {
            title,
            cursor_index: 0,
            sorted_by: None,
            entries,
            index_by_title,
            has_error: false,
            file_type_map: HashMap::new(),
        }
    }

    fn build_index(entries: &[FolderEntry]) -> HashMap<String, usize> {
        entries
            .iter()
            .enumerate()
            .map(|(index, entry)| (entry.title.clone(), index))
            .collect()
    }

    pub fn entries(&self) -> &[FolderEntry] {
        &self.entries
    }

    /// Replaces all entries at once (e.g. from a fresh directory listing).
    pub fn set_entries(&mut self, entries: Vec<FolderEntry>) {
        self.index_by_title = Self::build_index(&entries);
        self.entries = entries;
    }

    /// Appends a single entry, keeping the title index in sync.
    pub fn push_entry(&mut self, entry: FolderEntry) {
        self.index_by_title
            .insert(entry.title.clone(), self.entries.len());
        self.entries.push(entry);
    }

    pub fn has_entry_titled(&self, title: &str) -> bool {
        self.index_by_title.contains_key(title)
    }

    /// O(1) lookup of a direct child entry by title.
    pub fn entry_mut_by_title(&mut self, title: &str) -> Option<&mut FolderEntry> {
        let index = *self.index_by_title.get(title)?;
        self.entries.get_mut(index)
    }

    pub fn get_size(&self) -> u64 {
        self.entries
            .iter()
            .fold(0, |acc, entry| acc + entry.size.unwrap_or(0))
    }

    pub fn get_selected_entry_size(&self) -> u64 {
        self.get_selected_entry().map(|e| e.size.unwrap_or(0)).unwrap_or(0)
    }

    pub fn remove_selected(&mut self) {
        self.entries.remove(self.cursor_index);
        self.index_by_title = Self::build_index(&self.entries);
        self.cursor_index = self.cursor_index.min(self.entries.len().saturating_sub(1));
    }

    pub fn get_selected_entry(&self) -> Option<&FolderEntry> {
        self.entries.get(self.cursor_index)
    }

    pub fn get_max_entry_size(&self) -> u64 {
        let mut max_entry_size = 0;

        for file in &self.entries {
            if let Some(size) = file.size {
                if size > max_entry_size {
                    max_entry_size = size
                }
            }
        }

        max_entry_size
    }

    pub fn sort_by_title(&mut self) {
        self.entries.sort();
        self.index_by_title = Self::build_index(&self.entries);
    }

    pub fn sort_by_size(&mut self) {
        self.entries.sort_by(|a, b| {
            if a.kind == FolderEntryType::Parent || b.kind == FolderEntryType::Parent {
                // If either entry is a Parent, it should come before
                if a.kind == FolderEntryType::Parent && b.kind != FolderEntryType::Parent {
                    Ordering::Less
                } else if a.kind != FolderEntryType::Parent && b.kind == FolderEntryType::Parent {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            } else if let (Some(size_a), Some(size_b)) = (a.size, b.size) {
                // Sort by size in descending order
                size_b.cmp(&size_a)
            } else if a.size.is_some() {
                // Entries with size come before those without
                Ordering::Greater
            } else if b.size.is_some() {
                // Entries without size come after those with
                Ordering::Less
            } else {
                // If both entries have no size, maintain their order
                Ordering::Equal
            }
        });
        self.index_by_title = Self::build_index(&self.entries);
    }

    pub fn append_file_type_size(&mut self, file_type: &String, size: u64) {
        let total_size = self.file_type_map.entry(file_type.to_owned()).or_insert(0);
        *total_size += size;
    }

    fn get_sorted_file_types_by_size(&self) -> Vec<(String, u64)> {
        let mut file_types: Vec<(String, u64)> = self
            .file_type_map
            .iter()
            .map(|(k, &v)| (k.clone(), v))
            .collect();
        file_types.sort_by_key(|&(_, size)| std::cmp::Reverse(size));
        file_types
    }

    pub fn get_chart_data(&self, threshold: f64, max_items: usize) -> Vec<(String, u64)> {
        if max_items == 0 || self.file_type_map.is_empty() {
            return Vec::new();
        }

        let sorted_file_types = self.get_sorted_file_types_by_size();
        let total_size: u64 = sorted_file_types.iter().map(|(_, size)| *size).sum();
        let mut accumulated_size: u64 = 0;
        let mut chart_data: Vec<(String, u64)> = Vec::new();
        let mut rest_size: u64 = 0;

        for (file_type, size) in sorted_file_types.into_iter() {
            if total_size > 0
                && accumulated_size as f64 / total_size as f64 <= threshold
                && chart_data.len() < max_items - 1
            {
                chart_data.push((file_type, size));
                accumulated_size += size;
            } else {
                rest_size += size;
            }
        }

        if rest_size > 0 {
            chart_data.push(("rest".to_string(), rest_size));
        }

        chart_data
    }
}
