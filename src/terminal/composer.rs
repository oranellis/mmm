use std::cmp::max;

use crate::types::Vec2d;

#[derive(Clone, Debug, PartialEq)]
pub struct WriteChunk {
    pub position: usize,
    pub chunk: String,
}

impl WriteChunk {
    pub fn new(position: usize, first_char: char) -> Self {
        let mut init_string = String::new();
        init_string.push(first_char);
        WriteChunk {
            position,
            chunk: init_string,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct TerminalBuffer {
    pub buffer: String,
    pub terminal_size: Vec2d,
}

impl TerminalBuffer {
    pub fn new(buffer: String, terminal_size: &Vec2d) -> Self {
        Self {
            buffer,
            terminal_size: terminal_size.to_owned(),
        }
    }

    // Not worki fix plz
    pub fn add_layer(&mut self, layer: &str) -> &mut Self {
        let mut new_buf = String::new();
        let mut new_chars = layer.chars();
        let mut old_chars = self.buffer.chars();
        loop {
            if let Some(old_char) = old_chars.next() {
                if let Some(new_char) = new_chars.next() {
                    if new_char != '�' {
                        new_buf.push(new_char)
                    } else {
                        new_buf.push(old_char)
                    }
                } else {
                    new_buf.push(old_char)
                }
            } else {
                if let Some(new_char) = new_chars.next() {
                    new_buf.push(new_char)
                } else {
                    break;
                }
            }
        }
        self.buffer = new_buf;
        self
    }
}

/// Splits the difference between two strings into write operations, with optional grouping of
/// chunks seperated by a chosen distance.
///
/// * `base_string`: The root to compare against
/// * `new_string`: The comparison string
/// * `chunk_distance`: The distance between chunks to merge, set to 0 for no merging
pub fn split_into_writes(
    base_string: &str,
    compare_string: &str,
    chunk_distance: usize,
) -> Option<Vec<WriteChunk>> {
    let mut chunks: Vec<WriteChunk> = vec![];
    let mut base_chars = base_string.chars();
    let mut new_chars = compare_string.chars();
    let mut gap_counter = chunk_distance + 1;
    let mut staging_string_option: Option<String> = None;
    let mut staging_string_start: usize = 0;
    let mut gap_string_option: Option<String> = None;
    let mut index: usize = 0;

    loop {
        let base_char_option = base_chars.next();
        let new_char_option = new_chars.next();
        if base_char_option.is_none() && new_char_option.is_none() {
            break;
        }
        let (working_char, changed) = if base_char_option.is_none() {
            (new_char_option.unwrap(), true)
        } else if new_char_option.is_none() {
            (base_char_option.unwrap(), true)
        } else {
            (
                new_char_option.unwrap(),
                new_char_option.unwrap() != base_char_option.unwrap(),
            )
        };

        if changed {
            if let Some(gap_string) = &gap_string_option {
                match &mut staging_string_option {
                    Some(staging_string) => staging_string.push_str(&gap_string),
                    None => {
                        staging_string_option = Some(gap_string.to_string());
                        staging_string_start = index;
                    }
                }
                gap_string_option = None;
            }
            match &mut staging_string_option {
                Some(staging_string) => staging_string.push(working_char),
                None => {
                    staging_string_option = Some(working_char.to_string());
                    staging_string_start = index;
                }
            }
        }

        // What are the cases
        //
        // character is changed -> push gap string onto staging string if present, gap_counter to 0
        // character is the same -> if gap_counter < chunk dist then push char onto gap string else
        // drop gap_string and push WriteChunk to list, increment gap_counter
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_splitting_simple() {
        let str1 = "Hello world!".to_string();
        let cmp_str = "Hello marld!".to_string();
        let res = split_into_writes(&str1, &cmp_str, 0);
        println!("{:?}", res);
        assert!(res.is_some());
        assert_eq!(res.unwrap().len(), 1);
    }

    #[test]
    fn write_splitting_multiple_changes() {
        let str1 = "The green paper plane".to_string();
        let cmp_str = "The red   paper rhino".to_string();
        let res = split_into_writes(&str1, &cmp_str, 0);
        println!("{:?}", res);
        assert!(res.is_some());
        if let Some(some_res) = res {
            let mut res_iter = some_res.iter();
            assert!(res_iter.next().unwrap().chunk == "red  ");
            assert!(res_iter.next().unwrap().chunk == "rhi");
            assert!(res_iter.next().unwrap().chunk == "o");
            assert!(res_iter.next().is_none());
        }
    }

    #[test]
    fn write_splitting_excess_compare_length() {
        let str1 = "plane".to_string();
        let cmp_str = "plane go zoom".to_string();
        let res = split_into_writes(&str1, &cmp_str, 0);
        println!("{:?}", res);
        assert!(res.is_some());
    }

    #[test]
    fn write_splitting_excess_base_length() {
        let str1 = "plane go zoom".to_string();
        let cmp_str = "plane".to_string();
        let res = split_into_writes(&str1, &cmp_str, 0);
        println!("{:?}", res);
        assert!(res.is_none());
    }

    #[test]
    fn grouped_chunks() {
        let str1 = "The aaaaaaaaaa plane".to_string();
        let cmp_str = "The abababaaba plane".to_string();
        let res = split_into_writes(&str1, &cmp_str, 1);
        println!("{:?}", res);
        assert!(res.is_some());
        if let Some(some_res) = res {
            let mut res_iter = some_res.iter();
            let out1 = res_iter.next().unwrap();
            assert!(out1.position == 5);
            assert!(out1.chunk == "babab");
            let out2 = res_iter.next().unwrap();
            assert!(out2.position == 12);
            assert!(out2.chunk == "b");
            assert!(res_iter.next().is_none());
        }
    }
}
