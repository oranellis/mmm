use super::boxes::TerminalBoxes;
use crate::{
    filesystem::MmmDirList,
    types::{MmmResult, MmmState, Vec2d},
};

/// A chat gipity special
///
/// * `input`: The string to sorten
/// * `max_len`: The max length
fn clamp_string(input: &str, max_len: usize) -> &str {
    if input.chars().count() > max_len {
        input
            .char_indices()
            .nth(max_len)
            .map_or(input, |(idx, _)| &input[..idx])
    } else {
        input
    }
}

impl MmmState {
    pub fn draw_outline(&self) -> MmmResult<String> {
        let mut boxes = TerminalBoxes::new(self.terminal_size);
        let layout = &self.layout;
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
        Ok(boxes.to_string())
    }

    pub fn draw_search_str(&self) -> String {
        let layout = &self.layout;
        let cols = &self.terminal_size.col;
        let pos = (layout.search_box_position.row * cols) + layout.search_box_position.col;
        let mut ret_string = String::new();
        ret_string.push_str(&"�".repeat(pos.into()));
        ret_string.push_str(&self.search_text);
        ret_string
    }
}

pub fn draw_dir(
    dir_list: Option<MmmDirList>,
    position: &Vec2d,
    size: &Vec2d,
    terminal_cols: &u16,
) -> String {
    let mut line = 0;
    let mut ret_string = String::new();
    if let Some(dir_list) = dir_list {
        for entry in dir_list.entries {
            let cols = terminal_cols;
            let desired_len =
                ((position.row + line) as usize * *cols as usize) + position.col as usize;
            let entry_display_string = &entry.get_name().to_string_lossy();
            ret_string.push_str(&"�".repeat(desired_len - ret_string.chars().count()));
            ret_string.push_str(clamp_string(entry_display_string, size.col as usize));
            line += 1;
            if line > size.row {
                break;
            }
        }
    }
    ret_string
}
