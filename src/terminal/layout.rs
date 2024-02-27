use crate::datatypes::MmmState;

pub const PARENT_PERCENTAGE: f32 = 0.2; // As a fraction
pub const CENTER_PERCENTAGE: f32 = 0.5; // As a fraction

pub struct MmmLayout {
    pub parent_width: u16,
    pub center_width: u16,
    pub child_width: u16,
    pub center_height: u16,
}

pub fn generate_layout(state: &MmmState) -> Option<MmmLayout> {
    let (columns, rows) = state.terminal_size;
    if columns < 16 || rows < 3 {
        return None;
    }

    let available_columns = columns - 4;
    let parent_width = (available_columns as f32 * PARENT_PERCENTAGE).round() as u16;
    let center_width = (available_columns as f32 * CENTER_PERCENTAGE).round() as u16;
    let child_width = available_columns - parent_width - center_width;

    Some(MmmLayout {
        parent_width,
        center_width,
        child_width,
        center_height: (rows + 1) / 2,
    })
}
