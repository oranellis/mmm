use crate::types::Vec2d;

#[derive(Clone, Debug, PartialEq)]
pub struct WriteChunk {
    pub position: usize,
    pub chunk: String,
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
            } else if let Some(new_char) = new_chars.next() {
                if new_char != '�' {
                    new_buf.push(new_char)
                } else {
                    new_buf.push(' ')
                }
            } else {
                break;
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
    let mut gap_distance = chunk_distance + 1;
    let mut staging_chunk_option: Option<WriteChunk> = None;
    let mut gap_string = String::new();
    let mut index: usize = 0;

    loop {
        let base_char_option = base_chars.next();
        let new_char_option = new_chars.next();
        if base_char_option.is_none() && new_char_option.is_none() {
            break;
        }

        let working_char;
        let changed;
        if base_char_option.is_none() {
            working_char = new_char_option.unwrap();
            changed = true;
        } else if new_char_option.is_none() {
            working_char = base_char_option.unwrap();
            changed = true;
        } else {
            working_char = new_char_option.unwrap();
            changed = new_char_option.unwrap() != base_char_option.unwrap();
        };

        if changed {
            gap_distance = 0;
            if !gap_string.is_empty() {
                match &mut staging_chunk_option {
                    Some(staging_chunk) => staging_chunk.chunk.push_str(&gap_string),
                    None => {
                        staging_chunk_option = Some(WriteChunk {
                            chunk: gap_string.to_string(),
                            position: index,
                        });
                    }
                }
                gap_string = String::new();
            }
            match &mut staging_chunk_option {
                Some(staging_chunk) => staging_chunk.chunk.push(working_char),
                None => {
                    staging_chunk_option = Some(WriteChunk {
                        chunk: working_char.to_string(),
                        position: index,
                    });
                }
            }
        } else {
            gap_distance += 1;
            if gap_distance <= chunk_distance {
                gap_string.push(working_char);
            } else {
                gap_string = String::new();
                if let Some(staging_chunk) = staging_chunk_option {
                    chunks.push(staging_chunk);
                    staging_chunk_option = None;
                }
            }
        }
        index += 1;
    }

    if let Some(staging_chunk) = staging_chunk_option {
        chunks.push(staging_chunk);
    }
    if !chunks.is_empty() {
        Some(chunks)
    } else {
        None
    }
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
        if let Some(some_res) = res {
            let mut res_iter = some_res.iter();
            assert!(res_iter.next().unwrap().chunk == " go zoom");
            assert!(res_iter.next().is_none());
        }
    }

    #[test]
    fn write_splitting_excess_base_length() {
        let str1 = "plane go zoom".to_string();
        let cmp_str = "plane".to_string();
        let res = split_into_writes(&str1, &cmp_str, 0);
        println!("{:?}", res);
        assert!(res.is_some());
        if let Some(some_res) = res {
            let mut res_iter = some_res.iter();
            assert!(res_iter.next().unwrap().chunk == " go zoom");
            assert!(res_iter.next().is_none());
        }
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
