use super::boxes::TerminalBoxes;
use crate::types::{MmmResult, MmmState};

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

    pub fn draw_search_str(&self) -> MmmResult<String> {
        let layout = &self.layout;
        let cols = &self.terminal_size.col;
        let rows = &self.terminal_size.row;
        let pos = (layout.search_box_position.row * cols) + (layout.search_box_position.col);
        let mut ret_string = String::new();
        ret_string.push_str(&"�".repeat(pos.into()));
        ret_string.push_str(&self.search_text);
        Ok(ret_string)
    }
}
