use std::{
    cmp::{max, min},
    rc::Rc,
};

use crossterm::style::{Color, Colors};

use crate::{
    filesystem::{filter::FilterMatchEnum, MmmDirEntry, MmmScoredDirEntry},
    types::{MmmResult, Vec2d},
};

use super::{
    boxes::TerminalBoxes,
    buffer::{to_styled_string, StyledChar, TerminalBuffer},
    layout::MmmLayout,
};

pub struct DrawOps {
    pub background: bool,
    pub current_dir: bool,
    pub search_box: bool,
}

impl DrawOps {
    pub fn new(background: bool, current_dir: bool, search_box: bool) -> Self {
        DrawOps {
            background,
            current_dir,
            search_box,
        }
    }

    pub fn is_any(&self) -> bool {
        self.background || self.current_dir || self.search_box
    }
}

fn clamp_string(input: &str, max_len: usize) -> &str {
    input
        .char_indices()
        .nth(max_len)
        .map_or(input, |(idx, _)| &input[..idx])
}

fn pad_string(input: &str, len: usize) -> String {
    format!("{: <width$}", input, width = len + 1)
}

impl TerminalBuffer {
    pub fn draw_background(&mut self, layout: &MmmLayout) -> MmmResult<&mut Self> {
        let mut boxes = TerminalBoxes::new(layout.terminal_size);
        boxes
            .add_box(
                layout.parentdir_border_position,
                layout.parentdir_border_position + layout.parentdir_border_size,
            )?
            .add_box(
                layout.currentdir_border_position,
                layout.currentdir_border_position + layout.currentdir_border_size,
            )?
            .add_box(
                layout.childdir_border_position,
                layout.childdir_border_position + layout.childdir_border_size,
            )?
            .add_box(
                layout.search_box_border_position,
                layout.search_box_border_position + layout.search_box_border_size,
            )?;
        let print_str = to_styled_string(&boxes.to_string(), Color::Reset, Color::Reset, false);
        self.move_cursor((0, 0).into())?.styled_print(print_str)?;
        Ok(self)
    }

    pub fn draw_search_str(
        &mut self,
        position: Vec2d,
        width: u16,
        filter: &str,
    ) -> MmmResult<&mut Self> {
        let print_str: String = filter
            .chars()
            .rev()
            .take(width as usize)
            .collect::<Vec<char>>()
            .into_iter()
            .rev()
            .collect();
        let padded_str = pad_string(&print_str, width as usize);
        let styled_str = to_styled_string(&padded_str, Color::Red, Color::Reset, true);
        self.move_cursor(position)?.styled_print(styled_str)?;
        Ok(self)
    }

    pub fn draw_current_dir(
        &mut self,
        filtered_list: &[MmmScoredDirEntry],
        position: Vec2d,
        size: Vec2d,
    ) -> MmmResult<&mut Self> {
        for i in 0..=size.row as usize {
            if let Some(entry) = filtered_list.get(i) {
                let fg_colour;
                let bg_colour;
                let bold;
                match entry.entry.as_ref() {
                    MmmDirEntry::Directory { name: _, path: _ } => {
                        fg_colour = Color::Blue;
                        bg_colour = Color::Reset;
                        bold = false;
                    }
                    _ => {
                        fg_colour = Color::White;
                        bg_colour = Color::Reset;
                        bold = false;
                    }
                };
                let formatted_name = clamp_string(entry.entry.get_name(), size.col as usize);
                let mut styled_str = to_styled_string(formatted_name, fg_colour, bg_colour, bold);
                for (i, c) in styled_str.iter_mut().enumerate() {
                    if *entry.filter_match.get(i).expect("out of filer bounds")
                        == FilterMatchEnum::Match
                    {
                        *c = StyledChar {
                            character: c.character,
                            bold: true,
                            colour: Colors::new(Color::Red, Color::Reset),
                        }
                    }
                }
                self.move_cursor((position.col, position.row + i as u16).into())?
                    .styled_print(styled_str)?;
            }
        }
        Ok(self)
    }

    pub fn draw_parent_dir(
        &mut self,
        dir_list: &[Rc<MmmDirEntry>],
        selected: usize,
        position: Vec2d,
        size: Vec2d,
    ) -> MmmResult<&mut Self> {
        let len = dir_list.len();
        // Calculate the top row using clamped centering formula
        let top_row: usize = max(
            min(
                selected as i32 - ((size.row as i32 - 1) / 2),
                (len as i32) - (1 + (size.row as i32)),
            ),
            0,
        )
        .try_into()
        .expect("unable to convert top_row to usize");
        for i in 0..=size.row as usize {
            if let Some(entry) = dir_list.get(i + top_row) {
                let fg_colour;
                let bg_colour;
                let bold;
                if i + top_row == selected {
                    fg_colour = Color::Red;
                    bg_colour = Color::Reset;
                    bold = true;
                } else {
                    match entry.as_ref() {
                        MmmDirEntry::Directory { name: _, path: _ } => {
                            fg_colour = Color::Blue;
                            bg_colour = Color::Reset;
                            bold = false;
                        }
                        _ => {
                            fg_colour = Color::White;
                            bg_colour = Color::Reset;
                            bold = false;
                        }
                    };
                }
                let formatted_name = clamp_string(entry.get_name(), size.col as usize);
                let styled_str = to_styled_string(formatted_name, fg_colour, bg_colour, bold);
                self.move_cursor((position.col, position.row + i as u16).into())?
                    .styled_print(styled_str)?;
                if i + top_row == selected {
                    let extra_str = ' '.to_string()
                        + &"─".repeat(size.col as usize - formatted_name.len())
                        + "┨";
                    let styled_extra_string =
                        to_styled_string(&extra_str, Color::Reset, Color::Reset, false);
                    self.styled_print(styled_extra_string)?;
                }
            }
        }
        Ok(self)
    }
}
