use crate::types::MmmResult;
use crossterm::{
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    QueueableCommand,
};
use std::io::stdout;

pub fn command_setup() -> MmmResult<()> {
    stdout().queue(LeaveAlternateScreen)?;
    Ok(())
}

pub fn command_teardown() -> MmmResult<()> {
    stdout().queue(EnterAlternateScreen)?;
    Ok(())
}
