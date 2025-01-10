use std::io::{stdout, Write};

use crossterm::{
    cursor::MoveTo,
    style::{Print, ResetColor},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, EnterAlternateScreen, LeaveAlternateScreen,
        SetTitle,
    },
    ExecutableCommand, QueueableCommand,
};

use crate::types::MmmResult;

use super::composer::{split_into_writes, TerminalBuffer};

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
    pub fn print_buffer(&self) -> MmmResult<()> {
        let mut stdout = stdout();
        stdout
            .queue(MoveTo(0, 0))?
            .queue(Print(&self.buffer))?
            .flush()?;
        Ok(())
    }

    pub fn print_buffer_diff(&mut self, new_buffer: TerminalBuffer) -> MmmResult<()> {
        let mut stdout = stdout();
        if new_buffer.terminal_size != self.terminal_size {
            new_buffer.print_buffer()?;
            *self = new_buffer;
            panic!();
        } else {
            if let Some(write_chunks) = split_into_writes(&self.buffer, &new_buffer.buffer, 0) {
                println!("{:?}", write_chunks);
                panic!();
                for write_chunk in write_chunks {
                    stdout
                        .queue(MoveTo(
                            (write_chunk.position % new_buffer.terminal_size.col as usize)
                                .try_into()
                                .unwrap(),
                            (write_chunk.position / new_buffer.terminal_size.col as usize)
                                .try_into()
                                .unwrap(),
                        ))?
                        .queue(Print(write_chunk.chunk))?;
                }
                stdout.flush()?;
                *self = new_buffer;
            }
        }
        Ok(())
    }
}
