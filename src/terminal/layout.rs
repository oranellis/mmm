use std::cmp::min;

use doubuff::helpers::stop_display;
use terminal_vec2::{vec2, Vec2};

use crate::error_type::MmmResult;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MmmLayout {
    pub term_size: Vec2,
    pub app_pos: Vec2,
    pub app_size: Vec2,
    pub parent_pos: Vec2,
    pub parent_size: Vec2,
    pub vert_sep_pos: Vec2,
    pub vert_sep_size: Vec2,
    pub horiz_sep_pos: Vec2,
    pub horiz_sep_size: Vec2,
    pub current_pos: Vec2,
    pub current_size: Vec2,
    pub path_disp_pos: Vec2,
    pub path_disp_width: usize,
    pub search_width: usize,
}

impl MmmLayout {
    pub fn new() -> MmmResult<Self> {
        let (col, row) = crossterm::terminal::size().expect("Unable to determine terminal size");
        #[cfg(not(target_os = "windows"))]
        let terminal_size: Vec2 = (col, row).into();
        #[cfg(target_os = "windows")]
        let terminal_size: Vec2 = (col + 1, row + 1).into();
        Self::from_size(terminal_size)
    }

    pub fn from_size(term_size: Vec2) -> MmmResult<Self> {
        if term_size.col < 10 || term_size.row < 4 {
            stop_display()?;
            Err("display too small")?;
        }
        let app_size = vec2!(
            min(term_size.col, 98),
            term_size.row - min(term_size.row.saturating_sub(32), 10)
        )?;
        let app_pos = vec2!(
            (term_size.col - app_size.col) / 2,
            (term_size.row - app_size.row) / 2
        )?;
        let parent_pos = app_pos + vec2!(1, 3)?;
        let parent_size = vec2!(min((app_size.col * 31) / 98, 31), app_size.row - 4)?;
        let vert_sep_pos = app_pos + vec2!(parent_size.col + 1, 2)?;
        let vert_sep_size = vec2!(1, app_size.row - 2)?;
        let current_pos = app_pos + vec2!(parent_size.col + 2, 3)?;
        let current_size = vec2!(
            app_size
                .col
                .saturating_sub(parent_size.col)
                .saturating_sub(3),
            parent_size.row
        )?;
        let horiz_sep_pos = app_pos + vec2!(0, 2)?;
        let horiz_sep_size = vec2!(app_size.col, 1)?;
        let search_width = min(app_size.col as usize - 2, 20);
        let path_disp_width = app_size.col as usize - 2 - search_width;
        let path_disp_pos = app_pos + vec2!(1, 1)?;

        Ok(MmmLayout {
            term_size,
            app_pos,
            app_size,
            parent_pos,
            parent_size,
            vert_sep_pos,
            vert_sep_size,
            current_pos,
            current_size,
            horiz_sep_pos,
            horiz_sep_size,
            search_width,
            path_disp_width,
            path_disp_pos,
        })
    }
}
