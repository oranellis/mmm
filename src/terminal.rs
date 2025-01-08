mod boxes;

use std::{
    io::{self, stdout, Write},
    time::Duration,
};

use crossterm::{
    cursor::MoveTo,
    event::{poll, read, Event, KeyCode, KeyEvent},
    style::{Print, ResetColor},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen, SetTitle,
    },
    ExecutableCommand, QueueableCommand,
};

use crate::{datatypes::MmmState, style::ThickBorders};

const PARENT_PERCENTAGE: f32 = 0.2; // As a fraction
const CENTER_PERCENTAGE: f32 = 0.5; // As a fraction

struct MmmLayout {
    parent_width: u16,
    center_width: u16,
    child_width: u16,
    center_height: u16,
}

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

pub fn terminal_interaction(state: &mut MmmState) {
    if poll(Duration::from_millis(100)).unwrap() {
        let event = read().unwrap();
        match event {
            Event::Key(keyevent) => process_key_press(state, keyevent),
            Event::Resize(columns, rows) => process_resize_event(state, columns, rows),
            _ => {}
        }
    }
    update_display(state).unwrap();
}

fn generate_layout(state: &MmmState) -> Option<MmmLayout> {
    let (columns, rows) = state.terminal_size;
    if columns < 16 || rows < 3 {
        return None;
    }

    let available_columns = columns - 4;
    let parent_width = (available_columns as f32 * PARENT_PERCENTAGE).round() as u16;
    let center_width = (available_columns as f32 * CENTER_PERCENTAGE).round() as u16;
    let child_width = available_columns - parent_width - center_width;

    Some(MmmLayout {
        parent_width,
        center_width,
        child_width,
        center_height: (rows + 1) / 2,
    })
}

fn process_key_press(state: &mut MmmState, key_event: KeyEvent) {
    if key_event.code == KeyCode::Esc {
        state.quit = true;
    }
}

fn process_resize_event(state: &mut MmmState, columns: u16, rows: u16) {
    state.terminal_size = (columns, rows);
}

fn update_display(state: &MmmState) -> crossterm::Result<()> {
    let mut stdout = stdout();
    if let Some(layout) = generate_layout(state) {
        // Draw borders
        stdout
            .queue(Clear(ClearType::All))?
            .queue(MoveTo(0, 0))?
            .queue(Print(ThickBorders::ES))?;
        for _ in 0..layout.parent_width {
            stdout.queue(Print(ThickBorders::EW))?;
        }
        stdout.queue(Print(ThickBorders::ESW))?;
        for _ in 0..layout.center_width {
            stdout.queue(Print(ThickBorders::EW))?;
        }
        stdout.queue(Print(ThickBorders::ESW))?;
        for _ in 0..layout.child_width {
            stdout.queue(Print(ThickBorders::EW))?;
        }
        stdout
            .queue(Print(ThickBorders::SW))?
            .queue(MoveTo(0, layout.center_height))?;
        stdout.flush()?;
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Cannot generate_layout",
        ))
    }
}
