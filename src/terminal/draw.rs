use std::{
    cmp::{max, min},
    rc::Rc,
};

use crossterm::style::{Color, Colors};
use doubuff::{
    boxes::TerminalBoxes,
    buffer::TerminalBuffer,
    styled_char::{add_style_to_string, StyledChar},
};
use terminal_vec2::{vec2, Vec2};

use crate::{
    error_type::MmmResult,
    filesystem::{filter::FilterMatchEnum, MmmDirEntry, MmmFilesys, MmmScoredDirEntry},
};

use super::layout::MmmLayout;

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

    pub fn draw(
        self,
        term_buffer: &mut TerminalBuffer,
        filesys: &MmmFilesys,
        layout: &MmmLayout,
    ) -> MmmResult<()> {
        if self.is_any() {
            if self.background {
                *term_buffer = TerminalBuffer::new(layout.term_size);
            }
            draw_background(term_buffer, layout)?;
            draw_current_dir(
                term_buffer,
                &filesys.filtered_current_dir_list,
                layout.current_pos,
                layout.current_size,
            )?;
            draw_search_str(
                term_buffer,
                layout.path_disp_pos,
                layout.path_disp_width,
                filesys
                    .get_current_path()
                    .to_str()
                    .ok_or("cannot get current path")?,
                layout.search_width,
                filesys.get_filter(),
            )?;
            if let Some(pdl) = &filesys.filtered_parent_dir_list {
                draw_parent_dir(
                    term_buffer,
                    pdl,
                    filesys.parent_current_entry,
                    layout.parent_pos,
                    layout.parent_size,
                    filesys.show_hidden_files,
                )?;
            }
            term_buffer
                .move_cursor(layout.app_pos + vec2!(2, 0)?)?
                .styled_print(add_style_to_string(
                    " mmm ",
                    Color::Reset,
                    Color::Reset,
                    false,
                ))?;
        }
        Ok(())
    }

    fn is_any(&self) -> bool {
        self.background || self.current_dir || self.search_box
    }
}

pub fn draw_background(term_buffer: &mut TerminalBuffer, layout: &MmmLayout) -> MmmResult<()> {
    let term_size = crossterm::terminal::size()?.into();
    let mut boxes = TerminalBoxes::new_thin(term_size);
    boxes
        .add_box(layout.app_pos, layout.app_size)
        .add_box(layout.vert_sep_pos, layout.vert_sep_size)
        .add_box(layout.horiz_sep_pos, layout.horiz_sep_size);
    term_buffer.draw_box(boxes)?;
    Ok(())
}

pub fn draw_current_dir(
    term_buffer: &mut TerminalBuffer,
    filtered_list: &[MmmScoredDirEntry],
    pos: Vec2,
    size: Vec2,
) -> MmmResult<()> {
    for i in 0..size.row as usize {
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
            let mut styled_str = add_style_to_string(formatted_name, fg_colour, bg_colour, bold);
            for (i, c) in styled_str.iter_mut().enumerate() {
                if *entry.filter_match.get(i).ok_or("out of bounds of filter")?
                    == FilterMatchEnum::Match
                {
                    *c = StyledChar {
                        character: c.character,
                        bold: true,
                        colour: Colors::new(Color::Red, Color::Reset),
                    }
                }
            }
            let print_pos = pos + vec2!(0, i)?;
            term_buffer
                .move_cursor(print_pos)?
                .styled_print(styled_str)?;
        } else {
            break;
        }
    }
    Ok(())
}

pub fn draw_parent_dir(
    term_buffer: &mut TerminalBuffer,
    dir_list: &[Rc<MmmDirEntry>],
    selected: usize,
    pos: Vec2,
    size: Vec2,
    allow_hidden: bool,
) -> MmmResult<()> {
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
    for i in 0..size.row as usize {
        if let Some(entry) = dir_list.get(i + top_row) {
            let is_selected = i + top_row == selected;
            let fg_colour;
            let bg_colour;
            let bold;
            if is_selected {
                if !allow_hidden
                    && entry
                        .get_name()
                        .chars()
                        .next()
                        .ok_or("cannot index empty name")?
                        == '.'
                {
                    fg_colour = Color::DarkGrey;
                } else {
                    fg_colour = Color::Red;
                }
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
            let styled_str = add_style_to_string(formatted_name, fg_colour, bg_colour, bold);
            term_buffer
                .move_cursor(pos + vec2!(0, i)?)?
                .styled_print(styled_str)?;
            if is_selected {
                let extra_str = if formatted_name.len() == size.col as usize {
                    "┤".to_string()
                } else {
                    ' '.to_string()
                        + &"─".repeat(size.col as usize - formatted_name.len() - 1)
                        + "┤"
                };
                let styled_extra_string =
                    add_style_to_string(&extra_str, Color::Reset, Color::Reset, false);
                term_buffer.styled_print(styled_extra_string)?;
            }
        }
    }
    Ok(())
}

pub fn draw_search_str(
    term_buffer: &mut TerminalBuffer,
    position: Vec2,
    path_width: usize,
    path_str: &str,
    search_width: usize,
    search_str: &str,
) -> MmmResult<()> {
    let trunc_path_str = if path_str == "/" {
        "".to_owned()
    } else {
        path_str
            .chars()
            .rev()
            .take(path_width.saturating_sub(1))
            .collect::<Vec<char>>()
            .into_iter()
            .rev()
            .collect::<String>()
    };
    let styled_path_str =
        add_style_to_string(&trunc_path_str, Color::DarkGrey, Color::Reset, false);
    let styled_seperator_string = if path_width == 0 {
        add_style_to_string("", Color::Reset, Color::Reset, false)
    } else {
        add_style_to_string("/", Color::White, Color::Reset, false)
    };
    let trunc_search_str = search_str
        .chars()
        .rev()
        .take(search_width)
        .collect::<Vec<char>>()
        .into_iter()
        .rev()
        .collect::<String>();
    let styled_search_str = add_style_to_string(&trunc_search_str, Color::Red, Color::Reset, true);
    term_buffer
        .move_cursor(position)?
        .styled_print(styled_path_str)?
        .styled_print(styled_seperator_string)?
        .styled_print(styled_search_str)?;
    Ok(())
}

fn clamp_string(input: &str, max_len: usize) -> &str {
    input
        .char_indices()
        .nth(max_len)
        .map_or(input, |(idx, _)| &input[..idx])
}
