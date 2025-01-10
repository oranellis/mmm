use crate::types::MmmResult;
use std::{
    fs,
    path::{Path, PathBuf},
};

use super::{MmmDirEntry, MmmDirList};

/// Returns the size on the disk of a file or folder in bytes
///
/// * `path`: The file or folder to get the size of
pub fn get_path_size(path: &Path) -> MmmResult<u64> {
    let mut total_size = 0;
    if path.is_file() {
        total_size += fs::metadata(path)?.len();
    } else if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            total_size += get_path_size(&entry_path)?;
        }
    }
    Ok(total_size)
}

pub fn get_dir_list(path: &Path) -> MmmResult<Option<MmmDirList>> {
    let mut dir_list = MmmDirList::new(path.to_path_buf());
    let mut entry_iter = fs::read_dir(path)?
        .filter_map(|entry| entry.ok())
        .peekable();
    if entry_iter.peek().is_none() {
        return Ok(None);
    }
    for entry in entry_iter {
        if let Ok(file_type) = entry.file_type() {
            if file_type.is_file() {
                let mut executable = false;
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    if let Ok(metadata) = entry.metadata() {
                        executable = metadata.permissions().mode() & 0o111 != 0;
                    }
                }
                dir_list.entries.push(MmmDirEntry::File {
                    name: entry.file_name(),
                    path: entry.path(),
                    executable,
                });
            } else if file_type.is_dir() {
                dir_list.entries.push(MmmDirEntry::Directory {
                    name: entry.file_name(),
                    path: entry.path(),
                });
            } else if file_type.is_symlink() {
                dir_list.entries.push(MmmDirEntry::Link {
                    name: entry.file_name(),
                    path: entry.path(),
                    linked_path: fs::read_link(entry.path()).ok(),
                });
            } else {
                dir_list.entries.push(MmmDirEntry::Other {
                    name: entry.file_name(),
                    path: entry.path(),
                });
            }
        }
    }
    Ok(Some(dir_list))
}

pub fn parent_path_from(path: &Path) -> Option<PathBuf> {
    path.parent().map(|p| p.to_path_buf())
}
