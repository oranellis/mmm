use super::{MmmDirEntry, MmmScoredDirEntry};
use std::rc::Rc;

pub fn filter_and_score(entry: Rc<MmmDirEntry>, filter: &str) -> Option<MmmScoredDirEntry> {
    filter_match(entry.get_name(), filter).map(|filter_match| {
        let mut score: i32 = 0;
        if let MmmDirEntry::Directory { name: _, path: _ } = *entry {
            score += 1000000
        }
        score += evaluate_score(&filter_match);
        MmmScoredDirEntry {
            filter_match,
            entry,
            score,
        }
    })
}

fn filter_match(base: &str, filter: &str) -> Option<Vec<FilterMatchEnum>> {
    if filter.is_empty() {
        return Some(vec![FilterMatchEnum::NoMatch; base.len()]);
    };
    let mut finished_filter = false;
    let mut filter_iter = filter.chars();
    let mut test_char = filter_iter
        .next()
        .expect("non empty string assertion failed");
    let mut match_list: Vec<FilterMatchEnum> = vec![];
    for base_char in base.chars() {
        if base_char.eq_ignore_ascii_case(&test_char) {
            match_list.push(FilterMatchEnum::Match);
            let next_char_option = filter_iter.next();
            match next_char_option {
                Some(next_char) => test_char = next_char,
                None => {
                    finished_filter = true;
                    test_char = '�'
                }
            }
        } else {
            match_list.push(FilterMatchEnum::NoMatch);
        }
    }
    if finished_filter {
        Some(match_list)
    } else {
        None
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum FilterMatchEnum {
    NoMatch,
    Match,
}

fn evaluate_score(filter_match: &[FilterMatchEnum]) -> i32 {
    let length_penalisation = filter_match.len();
    let mut start_match_weight_score: i32 = 0;
    for (i, c) in filter_match.iter().enumerate() {
        if *c == FilterMatchEnum::Match {
            start_match_weight_score += 500 - (5 * i) as i32;
        }
    }
    start_match_weight_score - length_penalisation as i32
}
