use std::process::exit;
use crossterm::terminal;
use tui::{widgets::{Block, Borders}, backend::Backend};

pub(super) fn get_terminal_size() -> (u16, u16) {
    terminal::size().unwrap_or_else(|_| {
        eprintln!("Error getting terminal size, quitting");
        exit(1);
    })
}

pub(super) fn generate_header<B: Backend>() -> Block<'static> {
    Block::default()
        .title("Header")
        .borders(Borders::NONE)
}

pub(super) fn generate_footer<B: Backend>() -> Block<'static> {
    Block::default()
        .title("Footer")
        .borders(Borders::NONE)
}

pub(super) struct ElementGeometry {
    pub term_width: u16,
    pub term_height: u16,
    pub header_height: u16,
    pub footer_height: u16,
    pub margain_horizontal: u16,
    pub margain_vertical: u16,
    pub col_1_width: u16,
    pub col_2_width: u16,
    pub col_3_width: u16,
}

impl ElementGeometry {
    pub fn new() -> ElementGeometry {
        let (term_width, term_height) = get_terminal_size();
        let (margain_horizontal, margain_vertical) = (1, 0);
        let (header_height, footer_height) = (3, 3);
        let (col_1_width, col_2_width, col_3_width) = ElementGeometry::calculate_column_widths(term_width, margain_horizontal);
        ElementGeometry {
            term_width,
            term_height,
            header_height,
            footer_height,
            margain_horizontal,
            margain_vertical,
            col_1_width,
            col_2_width,
            col_3_width,
        }
    }

    pub fn recalculate(&mut self) {
        let (width, height) = get_terminal_size();
        self.term_width = width;
        self.term_height = height;
        self.update_column_widths();
    }

    fn update_column_widths(&mut self) {
        let (int1, int2, int3) = ElementGeometry::calculate_column_widths(self.term_width, self.margain_horizontal);
        (self.col_1_width, self.col_2_width, self.col_3_width) = (int1, int2, int3);
    }

    fn calculate_column_widths(width: u16, margain_horizontal: u16) -> (u16, u16, u16) {
        let base_width: u16 = 34;
        let min_width: u16 = 18;
        let inner_width = width - (margain_horizontal * 2);

        if inner_width / 3 < min_width {
            if inner_width / 2 < min_width {
                (0, inner_width, 0)
            } else {
                (inner_width / 2, inner_width - (inner_width / 2), 0)
            }
        } else if inner_width > base_width * 3 {
            (base_width, base_width, inner_width - (base_width * 2))
        } else {
            (inner_width / 3, inner_width / 3, inner_width - (2 * inner_width / 3))
        }
    }
}
