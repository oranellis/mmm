pub mod dirlist;
pub mod filter;

use std::{ffi::OsString, path::PathBuf};

#[derive(Clone, Debug, PartialEq)]
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

impl MmmDirEntry {
    #[allow(unused)]
    pub fn get_path(&self) -> &PathBuf {
        match self {
            MmmDirEntry::File { path, .. } => path,
            MmmDirEntry::Directory { path, .. } => path,
            MmmDirEntry::Link { path, .. } => path,
            MmmDirEntry::Other { path, .. } => path,
        }
    }
    pub fn get_name(&self) -> &OsString {
        match self {
            MmmDirEntry::File { name, .. } => name,
            MmmDirEntry::Directory { name, .. } => name,
            MmmDirEntry::Link { name, .. } => name,
            MmmDirEntry::Other { name, .. } => name,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct MmmDirList {
    pub path: PathBuf,
    pub entries: Vec<MmmDirEntry>,
}

impl MmmDirList {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            entries: vec![],
        }
    }
}
