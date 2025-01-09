use std::io;

use crate::style::ThickBorders;

pub struct TerminalBoxes {
    columns: u16,
    rows: u16,
    border_bits: Vec<Vec<bool>>,
}

impl TerminalBoxes {
    pub fn new(columns: u16, rows: u16) -> TerminalBoxes {
        TerminalBoxes {
            columns,
            rows,
            border_bits: vec![vec![false; columns as usize]; rows as usize],
        }
    }

    pub fn add_box(
        &mut self,
        top_left: (u16, u16),
        bottom_right: (u16, u16),
    ) -> std::io::Result<()> {
        if top_left.0 > bottom_right.0 || top_left.1 > bottom_right.1 {
            return Err(std::io::Error::new(
                io::ErrorKind::Other,
                "Incorrect box coords",
            ));
        }
        for column in top_left.0..bottom_right.0 + 1 {
            let row_top = top_left.1;
            let row_bottom = bottom_right.1;
            self.set_bit(column, row_top);
            self.set_bit(column, row_bottom);
        }
        for row in top_left.1..bottom_right.1 + 1 {
            let column_left = top_left.0;
            let column_right = bottom_right.0;
            self.set_bit(column_left, row);
            self.set_bit(column_right, row);
        }
        Ok(())
    }

    fn set_bit(&mut self, column: u16, row: u16) {
        if column < self.columns && row < self.rows {
            self.border_bits[row as usize][column as usize] = true;
        }
    }

    fn check_bit(&self, column: u16, row: u16) -> bool {
        if column < self.columns && row < self.rows {
            return self.border_bits[row as usize][column as usize];
        } else {
            false
        }
    }

    fn has_north(&self, column: u16, row: u16) -> bool {
        if row == 0 {
            return false;
        }
        return self.border_bits[(row - 1) as usize][column as usize];
    }

    fn has_east(&self, column: u16, row: u16) -> bool {
        if column == self.columns - 1 {
            return false;
        }
        return self.border_bits[row as usize][(column + 1) as usize];
    }

    fn has_south(&self, column: u16, row: u16) -> bool {
        if row == self.rows - 1 {
            return false;
        }
        return self.border_bits[(row + 1) as usize][column as usize];
    }

    fn has_west(&self, column: u16, row: u16) -> bool {
        if column == 0 {
            return false;
        }
        return self.border_bits[row as usize][(column - 1) as usize];
    }
}

impl std::fmt::Display for TerminalBoxes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display_string = String::with_capacity(self.border_bits.len());
        for row in 0..self.border_bits.len() as u16 {
            for column in 0..self.border_bits[0].len() as u16 {
                if self.check_bit(column, row) {
                    display_string.push(ThickBorders::from_directions(
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
