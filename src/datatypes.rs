use std::path::PathBuf;

#[derive(PartialEq, Clone)]
pub struct MmmState {
    pub current_path: PathBuf,
    pub io_loading_state: Option<u8>,
    pub quit: bool,
    pub search_text: String,
    pub terminal_size: (u16, u16),
}

impl MmmState {
    pub fn new() -> Self {
        Self {
            current_path: std::env::current_dir().expect("Error getting filesystem path"),
            io_loading_state: None,
            quit: false,
            search_text: String::from(""),
            terminal_size: crossterm::terminal::size().unwrap(),
        }
    }
}
