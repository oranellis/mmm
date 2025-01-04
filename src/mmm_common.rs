use std::path::PathBuf;

#[derive(PartialEq)]
pub enum UpdateType {
    None,
    Quit,
    Char(char),
    Backspace,
    Clear,
}

#[allow(dead_code)]
pub struct MmmState {
    pub reading_input: bool,
    pub waiting_for_io: bool,
    pub current_path: PathBuf,
    pub loading_state: Option<u8>,
}

impl MmmState {
    pub fn new() -> Self {
        Self {
            reading_input: false,
            waiting_for_io: false,
            current_path: std::env::current_dir().expect("Error getting filesystem path"),
            loading_state: None,
        }
    }
}
