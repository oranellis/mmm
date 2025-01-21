use crate::{filesystem::MmmDirList, terminal::layout::MmmLayout};
use std::{
    cmp::min,
    ops::{Add, Sub},
    path::PathBuf,
};

// MmmState

#[derive(Debug, Default, PartialEq)]
pub struct MmmState {
    pub current_path: PathBuf,
    pub current_dir_list: Option<MmmDirList>,
    pub io_loading_state: Option<u8>,
    pub layout: MmmLayout,
    pub parent_dir_list: Option<MmmDirList>,
    pub quit: bool,
    pub search_text: String,
    pub selected_entry: u8,
    pub terminal_size: Vec2d,
}

impl MmmState {
    pub fn new() -> Self {
        let (col, row) = crossterm::terminal::size().expect("Unable to determine terminal size");
        #[cfg(not(target_os = "windows"))]
        let terminal_size: Vec2d = (col, row).into();
        #[cfg(windows)]
        let terminal_size: Vec2d = (col + 1, row + 1).into();
        let current_path = std::env::current_dir().expect("Error getting filesystem path");
        let layout = MmmLayout::from_size(terminal_size.col, terminal_size.row);
        Self {
            current_path,
            current_dir_list: None,
            io_loading_state: None,
            layout,
            parent_dir_list: None,
            quit: false,
            search_text: String::from(""),
            selected_entry: 0,
            terminal_size,
        }
    }

    pub fn search_cursor_pos(&self, search_box_padding: u16) -> Vec2d {
        let cursor_right_dist = min(
            search_box_padding + self.search_text.len() as u16,
            self.layout.search_box_width,
        );
        self.layout.search_box_position + (cursor_right_dist, 0).into()
    }

    pub fn get_display_string_capacity(&self) -> usize {
        self.terminal_size.col as usize + self.terminal_size.row as usize
    }
}

// Vec2d

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec2d {
    pub col: u16,
    pub row: u16,
}

impl Add for Vec2d {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2d {
            col: self.col + rhs.col,
            row: self.row + rhs.row,
        }
    }
}

impl Sub for Vec2d {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2d {
            col: self.col - rhs.col,
            row: self.row - rhs.row,
        }
    }
}

impl From<(u16, u16)> for Vec2d {
    fn from(value: (u16, u16)) -> Self {
        Vec2d {
            col: value.0,
            row: value.1,
        }
    }
}

// Error and Result type

#[derive(Debug)]
pub enum MmmError {
    Layout,
    Io(std::io::Error),
}

impl std::fmt::Display for MmmError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MmmError::Layout => write!(f, "layout error"),
            MmmError::Io(err) => write!(f, "i/o error: {}", err),
        }
    }
}

impl std::error::Error for MmmError {}

impl From<std::io::Error> for MmmError {
    fn from(error: std::io::Error) -> Self {
        MmmError::Io(error)
    }
}

pub type MmmResult<T> = Result<T, MmmError>;
