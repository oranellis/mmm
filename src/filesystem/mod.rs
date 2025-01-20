pub mod dirlist;
pub mod filter;

use std::{ffi::OsString, path::PathBuf};

#[derive(Debug, PartialEq)]
pub enum MmmDirEntry {
    File {
        name: OsString,
        path: PathBuf,
        executable: bool,
    },
    Directory {
        name: OsString,
        path: PathBuf,
    },
    Link {
        name: OsString,
        path: PathBuf,
        linked_path: Option<PathBuf>,
    },
    Other {
        name: OsString,
        path: PathBuf,
    },
}

#[derive(Debug, PartialEq)]
pub struct MmmDirList {
    path: PathBuf,
    entries: Vec<MmmDirEntry>,
}

impl MmmDirList {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            entries: vec![],
        }
    }
}
