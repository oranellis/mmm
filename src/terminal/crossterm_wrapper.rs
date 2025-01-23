use std::io::{stdout, Write};

use crate::types::MmmResult;
use crossterm::{
    cursor::{Hide, Show},
    style::ResetColor,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, EnterAlternateScreen, LeaveAlternateScreen,
        SetTitle,
    },
    ExecutableCommand, QueueableCommand,
};

pub fn start_display() -> MmmResult<()> {
    stdout()
        .queue(SetTitle("mmm"))?
        .queue(EnterAlternateScreen)?
        .queue(ResetColor)?
        .queue(Clear(crossterm::terminal::ClearType::All))?
        .flush()?;
    enable_raw_mode()?;
    hide_cursor()?;
    Ok(())
}

pub fn stop_display() -> MmmResult<()> {
    stdout().execute(LeaveAlternateScreen)?;
    show_cursor()?;
    disable_raw_mode()?;
    Ok(())
}

pub fn hide_cursor() -> MmmResult<()> {
    stdout().queue(Hide)?;
    Ok(())
}

pub fn show_cursor() -> MmmResult<()> {
    stdout().queue(Show)?;
    Ok(())
}
