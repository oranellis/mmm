use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// Returns the size on the disk of a file or folder in bytes
///
/// * `path`: The file or folder to get the size of
pub fn get_path_size(path: &Path) -> io::Result<u64> {
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

pub fn get_dir_list(path: &Path) -> io::Result<Vec<PathBuf>> {
    let dir_list: Vec<PathBuf> = fs::read_dir(path)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .collect();
    Ok(dir_list)
}

pub fn parent_path_from(path: &Path) -> Option<PathBuf> {
    path.parent().map(|p| p.to_path_buf())
}
