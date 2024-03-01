use std::{fs::FileType, ffi::OsString};

pub(super) struct Node {
    file_name: OsString,
    file_type: std::fs::FileType,
}
