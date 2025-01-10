use crate::types::Vec2d;

pub const PARENT_PERCENTAGE: f32 = 0.2; // As a fraction
pub const CENTER_PERCENTAGE: f32 = 0.5; // As a fraction

#[derive(PartialEq, Clone, Default, Debug)]
pub enum MmmLayoutType {
    #[default]
    None,
    Normal,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MmmLayout {
    pub layout_type: MmmLayoutType,
    pub childdir_border_position: Vec2d,
    pub childdir_border_size: Vec2d,
    pub childdir_position: Vec2d,
    pub childdir_size: Vec2d,
    pub currentdir_border_position: Vec2d,
    pub currentdir_border_size: Vec2d,
    pub currentdir_position: Vec2d,
    pub currentdir_size: Vec2d,
    pub parentdir_border_position: Vec2d,
    pub parentdir_border_size: Vec2d,
    pub parentdir_position: Vec2d,
    pub parentdir_size: Vec2d,
    pub search_box_border_position: Vec2d,
    pub search_box_border_size: Vec2d,
    pub search_box_position: Vec2d,
    pub search_box_width: u16,
}

impl MmmLayout {
    pub fn from_size(columns: u16, rows: u16) -> Self {
        if columns < 42 || rows < 10 {
            MmmLayout::default()
        } else {
            let parent_width = (columns as f32 * PARENT_PERCENTAGE).round() as u16;
            let current_width = (columns as f32 * CENTER_PERCENTAGE).round() as u16;
            let child_width = columns - parent_width - current_width - 1;

            let parentdir_border_position = (0, 0).into();
            let currentdir_border_position = parentdir_border_position + (parent_width, 0).into();
            let childdir_border_position = currentdir_border_position + (current_width, 0).into();

            let parentdir_border_size = (parent_width, rows - 3).into();
            let currentdir_border_size = (current_width, rows - 3).into();
            let childdir_border_size = (child_width, rows - 3).into();

            let parentdir_position = parentdir_border_position + (1, 1).into();
            let parentdir_size = parentdir_border_size - (2, 2).into();
            let currentdir_position = currentdir_border_position + (1, 1).into();
            let currentdir_size = currentdir_border_size - (2, 2).into();
            let childdir_position = childdir_border_position + (1, 1).into();
            let childdir_size = childdir_border_size - (2, 2).into();

            let search_box_border_position = currentdir_border_position + (0, rows - 3).into();
            let search_box_border_size = (current_width, 2).into();
            let search_box_width = current_width - 2;
            let search_box_position = search_box_border_position + (1, 1).into();
            MmmLayout {
                layout_type: MmmLayoutType::Normal,
                childdir_border_position,
                childdir_border_size,
                childdir_position,
                childdir_size,
                currentdir_border_position,
                currentdir_border_size,
                currentdir_position,
                currentdir_size,
                parentdir_border_position,
                parentdir_border_size,
                parentdir_position,
                parentdir_size,
                search_box_border_position,
                search_box_border_size,
                search_box_position,
                search_box_width,
            }
        }
    }
}
