use crate::{
    style::thick_border_from_directions,
    types::{MmmError, MmmResult, Vec2d},
};

pub struct TerminalBoxes {
    terminal_size: Vec2d,
    border_bits: Vec<Vec<bool>>,
}

impl TerminalBoxes {
    pub fn new(terminal_size: Vec2d) -> TerminalBoxes {
        TerminalBoxes {
            terminal_size,
            border_bits: vec![vec![false; terminal_size.col as usize]; terminal_size.row as usize],
        }
    }

    pub fn add_box(&mut self, top_left: Vec2d, bottom_right: Vec2d) -> MmmResult<&mut Self> {
        if top_left.col > bottom_right.col || top_left.row > bottom_right.row {
            return Err(MmmError::Layout);
        }
        for column in top_left.col..bottom_right.col + 1 {
            let row_top = top_left.row;
            let row_bottom = bottom_right.row;
            self.set_bit(column, row_top);
            self.set_bit(column, row_bottom);
        }
        for row in top_left.row..bottom_right.row + 1 {
            let column_left = top_left.col;
            let column_right = bottom_right.col;
            self.set_bit(column_left, row);
            self.set_bit(column_right, row);
        }
        Ok(self)
    }

    fn set_bit(&mut self, column: u16, row: u16) {
        if column < self.terminal_size.col && row < self.terminal_size.row {
            self.border_bits[row as usize][column as usize] = true;
        }
    }

    fn check_bit(&self, column: u16, row: u16) -> bool {
        if column < self.terminal_size.col && row < self.terminal_size.row {
            self.border_bits[row as usize][column as usize]
        } else {
            false
        }
    }

    fn has_north(&self, column: u16, row: u16) -> bool {
        if row == 0 {
            return false;
        }
        self.border_bits[(row - 1) as usize][column as usize]
    }

    fn has_east(&self, column: u16, row: u16) -> bool {
        if column == self.terminal_size.col - 1 {
            return false;
        }
        self.border_bits[row as usize][(column + 1) as usize]
    }

    fn has_south(&self, column: u16, row: u16) -> bool {
        if row == self.terminal_size.row - 1 {
            return false;
        }
        self.border_bits[(row + 1) as usize][column as usize]
    }

    fn has_west(&self, column: u16, row: u16) -> bool {
        if column == 0 {
            return false;
        }
        self.border_bits[row as usize][(column - 1) as usize]
    }
}

impl std::fmt::Display for TerminalBoxes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display_string = String::with_capacity(self.border_bits.len());
        for row in 0..self.border_bits.len() as u16 {
            for column in 0..self.border_bits[0].len() as u16 {
                if self.check_bit(column, row) {
                    display_string.push(thick_border_from_directions(
                        self.has_north(column, row),
                        self.has_east(column, row),
                        self.has_south(column, row),
                        self.has_west(column, row),
                    ));
                } else {
                    display_string.push(' ');
                }
            }
        }
        write!(f, "{}", display_string)
    }
}
