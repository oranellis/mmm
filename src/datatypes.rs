use crate::terminal::layout::MmmLayout;
use std::path::PathBuf;

#[derive(PartialEq, Clone)]
pub struct MmmState {
    pub current_path: PathBuf,
    pub current_dir_list: Option<Vec<PathBuf>>,
    pub initialised: bool,
    pub io_loading_state: Option<u8>,
    pub layout: Option<MmmLayout>,
    pub parent_dir_list: Option<Vec<PathBuf>>,
    pub quit: bool,
    pub search_text: String,
    pub selected_entry: u8,
    pub terminal_size: (u16, u16),
}

impl MmmState {
    pub fn new() -> Self {
        let terminal_size = crossterm::terminal::size().unwrap();
        let current_path = std::env::current_dir().expect("Error getting filesystem path");
        let layout = MmmLayout::from_size(terminal_size.0, terminal_size.1);
        Self {
            current_path,
            current_dir_list: None,
            initialised: false,
            io_loading_state: None,
            layout: None,
            parent_dir_list: None,
            quit: false,
            search_text: String::from(""),
            selected_entry: 0,
            terminal_size,
        }
    }

    pub fn update_terminal_size(&mut self, columns: u16, rows: u16) {
        self.terminal_size = (columns, rows);
        self.layout = 
    }
}
