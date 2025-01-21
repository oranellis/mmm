use crate::types::MmmResult;
use crossterm::QueueableCommand;
use std::io::{stdout, Write};

#[allow(unused)]
pub fn debug_print(print_str: &str) -> MmmResult<()> {
    stdout()
        .queue(crossterm::terminal::LeaveAlternateScreen)?
        .queue(crossterm::style::Print(print_str))?
        .queue(crossterm::style::Print("\n\r"))?
        .queue(crossterm::terminal::EnterAlternateScreen)?
        .flush()?;
    Ok(())
}
