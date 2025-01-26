use super::MmmDirEntry;
use crate::error_type::MmmResult;
use std::{fs, path::Path, rc::Rc};

#[allow(unused)]
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

pub fn get_dir_list(path: &Path) -> MmmResult<Vec<Rc<MmmDirEntry>>> {
    let mut dir_list = vec![];
    let entry_iter = fs::read_dir(path)?.filter_map(|entry| entry.ok());
    for entry in entry_iter {
        if let Ok(file_type) = entry.file_type() {
            if file_type.is_file() {
                #[cfg(unix)]
                let mut executable = false;
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    if let Ok(metadata) = entry.metadata() {
                        executable = metadata.permissions().mode() & 0o111 != 0;
                    }
                }
                #[cfg(not(unix))]
                let executable = false;
                dir_list.push(Rc::new(MmmDirEntry::File {
                    name: entry.file_name().to_string_lossy().to_string(),
                    path: entry.path(),
                    executable,
                }));
            } else if file_type.is_dir() {
                dir_list.push(Rc::new(MmmDirEntry::Directory {
                    name: entry.file_name().to_string_lossy().to_string(),
                    path: entry.path(),
                }));
            } else if file_type.is_symlink() {
                dir_list.push(Rc::new(MmmDirEntry::Link {
                    name: entry.file_name().to_string_lossy().to_string(),
                    path: entry.path(),
                    linked_path: fs::read_link(entry.path()).ok(),
                }));
            } else {
                dir_list.push(Rc::new(MmmDirEntry::Other {
                    name: entry.file_name().to_string_lossy().to_string(),
                    path: entry.path(),
                }));
            }
        }
    }
    Ok(dir_list)
}
