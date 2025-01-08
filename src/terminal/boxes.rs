use std::io;

struct TerminalBoxes {
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

    fn has_north() {}
    fn has_east() {}
    fn has_south() {}
    fn has_west() {}
}

impl std::fmt::Display for TerminalBoxes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display_string = String::with_capacity(self.border_bits.len());
        // TODO Make string output for drawing borders
        write!(f, "{}", "")
    }
}
