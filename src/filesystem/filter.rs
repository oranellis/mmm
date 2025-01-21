use super::{MmmDirEntry, MmmDirList};
use std::cmp::Ordering;

#[derive(Clone, Debug)]
struct ScoredDirEntry {
    pub dir_entry: MmmDirEntry,
    pub score: i32,
}

impl Ord for ScoredDirEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.score).cmp(&(other.score))
    }
}

impl PartialOrd for ScoredDirEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ScoredDirEntry {
    fn eq(&self, other: &Self) -> bool {
        (self.score) == (other.score)
    }
}

impl Eq for ScoredDirEntry {}

/// Filters with a filter and returns count elements, scored by how close to the root the filter
/// matches
///
/// * `dir_list`: The list of paths to filter
/// * `filter`: The filter string to match against
/// * `count`: The maximum number of elements returned
pub fn filter_files(dir_list: &MmmDirList, filter: &str, count: usize) -> MmmDirList {
    let mut filtered_list: Vec<ScoredDirEntry> = dir_list
        .entries
        .iter()
        .filter_map(|entry| {
            filter_match(&entry.get_name().to_string_lossy(), filter).map(|score| ScoredDirEntry {
                dir_entry: entry.clone(),
                score,
            })
        })
        .collect();
    filtered_list.sort_unstable_by_key(|dir_entry| {
        dir_entry
            .dir_entry
            .get_name()
            .to_str()
            .map(|s| s.to_string())
    });
    filtered_list.sort();
    MmmDirList {
        path: dir_list.path.to_owned(),
        entries: filtered_list
            .into_iter()
            .take(count)
            .map(|sde| sde.dir_entry)
            .collect(),
    }
}

fn filter_match(base: &str, filter: &str) -> Option<i32> {
    if filter.is_empty() {
        return Some(0);
    };
    let mut finished_filter = false;
    let mut score = 0;
    let mut filter_iter = filter.chars();
    let mut test_char = filter_iter
        .next()
        .expect("non empty string assertion failed");
    for (index, base_char) in base.char_indices() {
        if base_char.to_ascii_lowercase() == test_char.to_ascii_lowercase() {
            score += (index as i32) + 1;
            let next_char_option = filter_iter.next();
            match next_char_option {
                Some(next_char) => test_char = next_char,
                None => {
                    finished_filter = true;
                    break;
                }
            }
        }
    }
    if finished_filter {
        Some(score)
    } else {
        None
    }
}
