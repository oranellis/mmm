use std::io::{stdout, Write};

use super::buffer::{split_into_writes, TerminalBuffer};
use crate::types::{MmmResult, Vec2d};
use crossterm::{
    cursor::MoveTo,
    style::{Print, ResetColor},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, EnterAlternateScreen, LeaveAlternateScreen,
        SetTitle,
    },
    ExecutableCommand, QueueableCommand,
};

pub fn start_display() -> crossterm::Result<()> {
    stdout()
        .queue(SetTitle("mmm"))?
        .queue(EnterAlternateScreen)?
        .queue(ResetColor)?
        .queue(Clear(crossterm::terminal::ClearType::All))?
        .flush()?;
    enable_raw_mode()?;
    Ok(())
}

pub fn stop_display() -> crossterm::Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

impl TerminalBuffer {
    /// Prints the buffer to the screen
    pub fn queue_print_buffer(&self) -> MmmResult<()> {
        let mut stdout = stdout();
        stdout.queue(MoveTo(0, 0))?.queue(Print(&self.buffer))?;
        Ok(())
    }

    /// Prints the chunked diff between an old and new screen buffer, then returns self. Printing the diff saves
    /// rewriting the entire contents of the screen every frame which should speed up displaying
    /// although no benchmarks have been done to validate this. TerminalBuffer::print_buffer() is a
    /// simpler alternative which writes the whole contents of the screen.
    ///
    /// * `old_buffer`: The old screen buffer to diff against
    pub fn queue_print_buffer_diff(self, old_buffer: TerminalBuffer) -> MmmResult<TerminalBuffer> {
        let mut stdout = stdout();
        if self.terminal_size != old_buffer.terminal_size {
            self.queue_print_buffer()?;
        } else if let Some(write_chunks) = split_into_writes(&old_buffer.buffer, &self.buffer, 0) {
            for write_chunk in write_chunks {
                stdout
                    .queue(MoveTo(
                        (write_chunk.position % self.terminal_size.col as usize)
                            .try_into()
                            .unwrap(),
                        (write_chunk.position / self.terminal_size.col as usize)
                            .try_into()
                            .unwrap(),
                    ))?
                    .queue(Print(write_chunk.chunk))?;
            }
        }
        Ok(self)
    }
}

pub fn move_cursor(position: Vec2d) -> MmmResult<()> {
    stdout().queue(MoveTo(position.col, position.row))?;
    Ok(())
}

pub fn flush() -> MmmResult<()> {
    stdout().flush()?;
    Ok(())
}
