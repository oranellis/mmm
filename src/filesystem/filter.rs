use std::cmp::Ordering;

use super::{MmmDirEntry, MmmDirList};

fn filter_match(base: &str, filter: &str) -> (bool, i32) {
    if filter.is_empty() {
        return (true, 0);
    };
    let mut finished_filter = false;
    let mut filter_iter = filter.chars();
    let mut test_char = filter_iter
        .next()
        .expect("non empty string assertion failed");
    let mut score = 0;
    for (index, base_char) in base.char_indices() {
        if base_char == test_char {
            score += index as i32;
            if let Some(next_char) = filter_iter.next() {
                test_char = next_char;
            } else {
                finished_filter = true;
                break;
            }
        }
    }
    if finished_filter {
        (true, score)
    } else {
        (false, 0)
    }
}

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
pub fn filter_files(dir_list: &MmmDirList, filter: &str, count: usize) -> Vec<MmmDirEntry> {
    // ok how would I go about a filter, I think there needs to be two seperate steps, the
    // filtering and then the scoring. They are going to be linked by the filter string but
    // they need to be done seperately in that order.
    //
    // OK first the filter, probably going to make an O(n) solution by just looping through the
    // dirlist and checking if some string patterns match. I think I'll actually split this out
    // into another function to check if the filter matches.
    //
    // For the filtering this is way more advanced, there is probably some complicated
    // combinatorics or something that solves this in an elegant way but I think I'm just going to
    // bash it out and see what sticks.
    let mut filtered_list: Vec<ScoredDirEntry> = vec![];
    for dir_entry in &dir_list.entries {
        let (matches, score) = filter_match(&dir_entry.get_path().to_string_lossy(), filter);
        if matches {
            filtered_list.push(ScoredDirEntry {
                dir_entry: dir_entry.clone(),
                score,
            });
        }
    }
    filtered_list.sort();
    filtered_list
        .into_iter()
        .take(count)
        .map(|sde| sde.dir_entry)
        .collect()
}
