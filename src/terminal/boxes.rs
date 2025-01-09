use std::io;

use crate::style::ThickBorders;

pub struct TerminalBoxes {
    columns: u16,
    rows: u16,
    border_bits: Vec<u8>,
}

/// Returns the bit position and index of a position given the width and height of the terminal
/// (bitpos: u8, index: usize)
///
/// * `columns`: Number of terminal columns
/// * `position`: (column, row) position to retrieve
fn coord_to_index(columns: u16, column: u16, row: u16) -> (u8, usize) {
    let linear_pos = row as u32 * columns as u32 + column as u32;
    ((linear_pos % 8) as u8, (linear_pos / 8) as usize)
}

impl TerminalBoxes {
    pub fn new(columns: u16, rows: u16) -> TerminalBoxes {
        TerminalBoxes {
            columns,
            rows,
            border_bits: vec![0; (columns as usize * rows as usize + 7) / 8],
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
        for column in top_left.0..bottom_right.0 {
            let (bitpos_top, index_top) = coord_to_index(self.columns, column, top_left.1);
            let (bitpos_bottom, index_bottom) = coord_to_index(self.columns, column, top_left.1);
            self.set_bit(bitpos_top, index_top);
            self.set_bit(bitpos_bottom, index_bottom);
        }
        Ok(())
    }

    fn set_bit(&mut self, bitpos: u8, index: usize) {
        let old_bits = self.border_bits[index].clone();
        let add_bit: u8 = 1 << bitpos;
        self.border_bits[index] = old_bits & add_bit;
    }

    fn check_bit(&self, bitpos: u8, index: usize) -> bool {
        return (self.border_bits[index] & (1 << bitpos)) != 0;
    }

    fn has_north(&self, column: u16, row: u16) -> bool {
        if row == 0 {
            return false;
        }
        let (bitpos, index) = coord_to_index(self.columns, column, row - 1);
        return (self.border_bits[index] & (1 << bitpos)) != 0;
    }

    fn has_east(&self, column: u16, row: u16) -> bool {
        if column == self.columns - 1 {
            return false;
        }
        let (bitpos, index) = coord_to_index(self.columns, column + 1, row);
        return (self.border_bits[index] & (1 << bitpos)) != 0;
    }

    fn has_south(&self, column: u16, row: u16) -> bool {
        if row == self.rows - 1 {
            return false;
        }
        let (bitpos, index) = coord_to_index(self.columns, column, row + 1);
        return (self.border_bits[index] & (1 << bitpos)) != 0;
    }

    fn has_west(&self, column: u16, row: u16) -> bool {
        if column == 0 {
            return false;
        }
        let (bitpos, index) = coord_to_index(self.columns, column - 1, row);
        return (self.border_bits[index] & (1 << bitpos)) != 0;
    }
}

impl std::fmt::Display for TerminalBoxes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display_string = String::with_capacity(self.border_bits.len());
        for index in 0..self.border_bits.len() {
            for bitpos in 0..7 {
                let column = (bitpos + (8 * index)) as u16 % self.columns;
                let row = (bitpos + (8 * index)) as u16 / self.columns;
                if self.check_bit(bitpos as u8, index) {
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
