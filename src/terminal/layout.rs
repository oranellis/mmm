use crate::datatypes::MmmState;

use super::Vec2d;

pub const PARENT_PERCENTAGE: f32 = 0.2; // As a fraction
pub const CENTER_PERCENTAGE: f32 = 0.5; // As a fraction

#[derive(PartialEq, Clone)]
enum MmmLayoutType {
    None,
    Default,
    Small,
}

#[derive(PartialEq, Clone)]
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
        if columns < 20 || rows < 5 {
            let layout_type = MmmLayoutType::None;
            let empty_vec = Vec2d { column: 0, row: 0 };
            MmmLayout {
                layout_type,
                childdir_border_position: empty_vec,
                childdir_border_size: empty_vec,
                childdir_position: empty_vec,
                childdir_size: empty_vec,
                currentdir_border_position: empty_vec,
                currentdir_border_size: empty_vec,
                currentdir_position: empty_vec,
                currentdir_size: empty_vec,
                parentdir_border_position: empty_vec,
                parentdir_border_size: empty_vec,
                parentdir_position: empty_vec,
                parentdir_size: empty_vec,
                search_box_border_position: empty_vec,
                search_box_border_size: empty_vec,
                search_box_position: empty_vec,
                search_box_width: 0,
            }
        } else if columns < 42 || rows < 8 {
            let layout_type = MmmLayoutType::Small;
            let empty_vec = Vec2d { column: 0, row: 0 };
            MmmLayout {
                layout_type,
                childdir_border_position: empty_vec,
                childdir_border_size: empty_vec,
                childdir_position: empty_vec,
                childdir_size: empty_vec,
                currentdir_border_position: Vec2d { column: 0, row: 0 },
                currentdir_border_size: Vec2d {
                    column: columns,
                    row: 3,
                },
                currentdir_position: empty_vec,
                currentdir_size: empty_vec,
                parentdir_border_position: empty_vec,
                parentdir_border_size: empty_vec,
                parentdir_position: empty_vec,
                parentdir_size: empty_vec,
                search_box_border_position: empty_vec,
                search_box_border_size: empty_vec,
                search_box_position: empty_vec,
                search_box_width: 0,
            }
        } else {
            let layout_type = MmmLayoutType::None;
            let empty_vec = Vec2d { column: 0, row: 0 };
            MmmLayout {
                layout_type,
                childdir_border_position: empty_vec,
                childdir_border_size: empty_vec,
                childdir_position: empty_vec,
                childdir_size: empty_vec,
                currentdir_border_position: empty_vec,
                currentdir_border_size: empty_vec,
                currentdir_position: empty_vec,
                currentdir_size: empty_vec,
                parentdir_border_position: empty_vec,
                parentdir_border_size: empty_vec,
                parentdir_position: empty_vec,
                parentdir_size: empty_vec,
                search_box_border_position: empty_vec,
                search_box_border_size: empty_vec,
                search_box_position: empty_vec,
                search_box_width: 0,
            }
        }
    }
}
