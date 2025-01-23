use std::io::{stdout, Write};

use crossterm::{
    cursor::MoveTo,
    style::{Color, Colors, Print, SetColors},
    QueueableCommand,
};

use crate::types::{MmmError, MmmResult, Vec2d};

#[derive(Clone, Debug, PartialEq)]
pub struct WriteChunk {
    pub position: usize,
    pub chunk: String,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StyledChar {
    pub character: char,
    pub colour: Colors,
}

impl From<char> for StyledChar {
    fn from(value: char) -> Self {
        Self {
            character: value,
            colour: Colors::new(Color::Reset, Color::Reset),
        }
    }
}

pub fn to_styled_string(value: &str, fg_colour: Color, bg_colour: Color) -> Vec<StyledChar> {
    value
        .chars()
        .map(|c| StyledChar {
            character: c,
            colour: Colors::new(fg_colour, bg_colour),
        })
        .collect()
}

#[derive(Debug, PartialEq)]
pub struct TerminalBuffer {
    pub buffer: Vec<StyledChar>,
    pub scratch_buffer: Vec<StyledChar>,
    pub terminal_size: Vec2d,
    pub cursor_pos: usize,
}

impl TerminalBuffer {
    pub fn new(terminal_size: &Vec2d) -> Self {
        Self {
            buffer: vec!['�'.into(); terminal_size.col as usize * terminal_size.row as usize],
            scratch_buffer: vec![
                ' '.into();
                terminal_size.col as usize * terminal_size.row as usize
            ],
            terminal_size: terminal_size.to_owned(),
            cursor_pos: 0,
        }
    }

    pub fn move_cursor(&mut self, pos: Vec2d) -> MmmResult<&mut Self> {
        if pos.col > self.terminal_size.col || pos.row > self.terminal_size.row {
            return Err(MmmError::TerminalBuffer);
        }
        self.cursor_pos = (pos.row as usize * self.terminal_size.col as usize) + pos.col as usize;
        Ok(self)
    }

    pub fn styled_print<T, I>(&mut self, iterable: T) -> MmmResult<&mut Self>
    where
        T: IntoIterator<Item = I>,
        I: Into<StyledChar>,
    {
        for sc in iterable {
            if let Some(mc) = self.scratch_buffer.get_mut(self.cursor_pos) {
                *mc = sc.into();
                self.cursor_pos += 1;
            } else {
                break;
            }
        }
        Ok(self)
    }

    pub fn flush(&mut self) -> MmmResult<()> {
        let mut stdout = stdout();
        stdout.queue(MoveTo(0, 0))?;
        for c in &self.scratch_buffer {
            stdout.queue(SetColors(c.colour))?;
            stdout.queue(Print(c.character))?;
        }
        stdout.flush()?;
        self.buffer = std::mem::replace(
            &mut self.scratch_buffer,
            vec![' '.into(); self.terminal_size.col as usize * self.terminal_size.row as usize],
        );
        Ok(())
    }
}
