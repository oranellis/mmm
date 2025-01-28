pub(crate) mod dir_entry;
pub(crate) mod filter;

use crate::error_type::MmmResult;
use crate::filesystem::filter::filter_hidden;
use dir_entry::get_dir_list;
use filter::{filter_and_score, filter_hidden_with_exception, FilterMatchEnum};
use std::{
    path::{Path, PathBuf},
    rc::Rc,
};

#[derive(Clone, Debug, PartialEq)]
pub enum MmmDirEntry {
    File {
        name: String,
        path: PathBuf,
        executable: bool,
    },
    Directory {
        name: String,
        path: PathBuf,
    },
    Link {
        name: String,
        path: PathBuf,
        linked_path: Option<PathBuf>,
    },
    Other {
        name: String,
        path: PathBuf,
    },
}

impl MmmDirEntry {
    #[allow(unused)]
    pub fn get_path(&self) -> &Path {
        match self {
            MmmDirEntry::File { path, .. } => path,
            MmmDirEntry::Directory { path, .. } => path,
            MmmDirEntry::Link { path, .. } => path,
            MmmDirEntry::Other { path, .. } => path,
        }
    }
    pub fn get_name(&self) -> &str {
        match self {
            MmmDirEntry::File { name, .. } => name.as_ref(),
            MmmDirEntry::Directory { name, .. } => name.as_ref(),
            MmmDirEntry::Link { name, .. } => name.as_ref(),
            MmmDirEntry::Other { name, .. } => name.as_ref(),
        }
    }
}

#[derive(Debug)]
pub struct MmmScoredDirEntry {
    pub entry: Rc<MmmDirEntry>,
    pub filter_match: Vec<FilterMatchEnum>,
    pub score: i32,
}

#[derive(Debug)]
pub struct MmmFilesys {
    filter: String,
    current_path: PathBuf,
    current_dir_list: Vec<Rc<MmmDirEntry>>,
    pub filtered_current_dir_list: Vec<MmmScoredDirEntry>,
    selected_entry: usize,
    pub filtered_parent_dir_list: Option<Vec<Rc<MmmDirEntry>>>,
    pub parent_current_entry: usize,
    pub show_hidden_files: bool,
}

impl MmmFilesys {
    pub fn from_path(current_path: PathBuf) -> MmmResult<MmmFilesys> {
        let mut current_dir_list = get_dir_list(&current_path)?;
        current_dir_list.sort_by_key(|entry| entry.get_name().to_string());
        let selected_entry = 0;
        let show_hidden_files = false;
        let filtered_current_dir_list = current_dir_list
            .iter()
            .filter_map(|entry| filter_hidden(entry.clone(), show_hidden_files))
            .filter_map(|entry| filter_and_score(entry.clone(), ""))
            .collect();
        let filtered_parent_dir_list = current_path
            .parent()
            .map(get_dir_list)
            .transpose()?
            .map(|mut list| {
                list.sort_by_key(|entry| entry.get_name().to_owned());
                list
            })
            .map(|list| {
                list.into_iter()
                    .filter_map(|entry| {
                        filter_hidden_with_exception(
                            entry.clone(),
                            show_hidden_files,
                            current_path.file_name().and_then(|name| name.to_str())?,
                        )
                    })
                    .collect::<Vec<Rc<MmmDirEntry>>>()
            });
        let parent_current_entry = if let Some(pdl) = &filtered_parent_dir_list {
            pdl.iter()
                .position(|entry| entry.get_path() == current_path)
                .expect("cannot find parent dir")
        } else {
            0
        };
        let filter = String::new();
        Ok(MmmFilesys {
            current_path,
            current_dir_list,
            selected_entry,
            filtered_current_dir_list,
            filtered_parent_dir_list,
            parent_current_entry,
            filter,
            show_hidden_files,
        })
    }

    pub fn change_directory(&mut self, path: PathBuf) -> MmmResult<()> {
        self.current_path = path;
        self.current_dir_list = get_dir_list(&self.current_path)?;
        self.current_dir_list
            .sort_by_key(|entry| entry.get_name().to_string());
        self.selected_entry = 0;
        self.filtered_current_dir_list = self
            .current_dir_list
            .iter()
            .filter_map(|entry| filter_hidden(entry.clone(), self.show_hidden_files))
            .filter_map(|entry| filter_and_score(entry.clone(), ""))
            .collect();
        self.filtered_parent_dir_list = self
            .current_path
            .parent()
            .map(get_dir_list)
            .transpose()?
            .map(|mut list| {
                list.sort_by_key(|entry| entry.get_name().to_owned());
                list
            })
            .map(|list| {
                list.into_iter()
                    .filter_map(|entry| {
                        filter_hidden_with_exception(
                            entry.clone(),
                            self.show_hidden_files,
                            self.current_path
                                .file_name()
                                .and_then(|name| name.to_str())?,
                        )
                    })
                    .collect()
            });
        self.parent_current_entry = if let Some(pdl) = &self.filtered_parent_dir_list {
            pdl.iter()
                .position(|entry| entry.get_path() == self.current_path)
                .expect("cannot find parent dir")
        } else {
            0
        };
        self.filter = String::new();
        Ok(())
    }

    pub fn get_current_path(&self) -> &Path {
        &self.current_path
    }

    pub fn increment_current_selected(&mut self) {
        // Disabled until implemented properly
        // self.selected_entry = max(self.selected_entry + 1, max(self.current_dir_list.len(), 0));
    }

    pub fn decrement_current_selected(&mut self) {
        // Disabled until implemented properly
        // self.selected_entry = min(self.selected_entry - 1, 0);
    }

    pub fn toggle_show_hidden_files(&mut self) -> MmmResult<()> {
        self.show_hidden_files = !self.show_hidden_files;
        self.change_directory(self.current_path.clone())
    }

    pub fn clear_filter(&mut self) {
        self.filter = String::new();
        self.populate_filtered_list();
    }

    pub fn filter_add_char(&mut self, c: char) {
        self.filter.push(c);
        self.populate_filtered_list();
    }

    pub fn get_selected_entry(&self) -> Option<Rc<MmmDirEntry>> {
        if !self.filtered_current_dir_list.is_empty() {
            self.filtered_current_dir_list
                .get(self.selected_entry)
                .map(|value| value.entry.clone())
        } else {
            None
        }
    }

    pub fn filter_is_empty(&self) -> bool {
        self.filter.is_empty()
    }

    pub fn get_filter(&self) -> &str {
        &self.filter
    }

    fn populate_filtered_list(&mut self) {
        if self.filter_is_empty() {
            self.filtered_current_dir_list = self
                .current_dir_list
                .iter()
                .filter_map(|entry| filter_hidden(entry.clone(), self.show_hidden_files))
                .map(|entry| MmmScoredDirEntry {
                    entry: entry.clone(),
                    score: 0,
                    filter_match: vec![FilterMatchEnum::NoMatch; entry.get_name().len()],
                })
                .collect();
        } else {
            let mut filtered_scored: Vec<MmmScoredDirEntry> = self
                .current_dir_list
                .iter()
                .filter_map(|entry| filter_hidden(entry.clone(), self.show_hidden_files))
                .filter_map(|entry| filter_and_score(entry.clone(), &self.filter))
                .collect();
            filtered_scored.sort_by_key(|entry| entry.score);
            filtered_scored.reverse();
            self.filtered_current_dir_list = filtered_scored;
        }
    }

    pub fn try_nav_into(&mut self) -> MmmResult<()> {
        if let Some(dir_entry) = self.get_selected_entry() {
            if let MmmDirEntry::Directory { name: _, path } = &*dir_entry {
                self.change_directory(path.to_path_buf())?;
            }
        }
        Ok(())
    }

    pub fn try_nav_back(&mut self) -> MmmResult<()> {
        if let Some(path) = self.current_path.parent() {
            self.change_directory(path.to_path_buf())?;
        }
        Ok(())
    }
}
