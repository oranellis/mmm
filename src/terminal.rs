mod areas;
mod boxes;
mod draw;
mod events;
pub mod layout;

use crate::{
    datatypes::MmmState,
    filesystem::{get_dir_list, parent_path_from},
};
use crossterm::{
    cursor::MoveTo,
    event::{poll, read, Event},
    style::{Print, ResetColor},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, EnterAlternateScreen, LeaveAlternateScreen,
        SetTitle,
    },
    ExecutableCommand, QueueableCommand,
};
use draw::{draw_files, draw_outline};
use events::{process_key_press, process_resize_event};
use std::{
    io::{self, stdout, Write},
    time::Duration,
};

#[derive(PartialEq, Clone, Copy)]
pub struct Vec2d {
    column: u16,
    row: u16,
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

pub fn terminal_interaction(state: &mut MmmState) -> io::Result<()> {
    let old_state = state.clone();
    state.initialised = true;
    if poll(Duration::from_millis(500)).unwrap() {
        let event = read().unwrap();
        match event {
            Event::Key(keyevent) => process_key_press(state, keyevent),
            Event::Resize(columns, rows) => process_resize_event(state, columns, rows),
            _ => {}
        }
    }
    if let Some(layout) = layout::generate_layout(state) {
        if *state != old_state {
            draw_outline(state, &layout)?;
            state.current_dir_list = Some(get_dir_list(&state.current_path).unwrap());
            state.parent_dir_list = parent_path_from(&state.current_path)
                .map(|pathbuf| get_dir_list(&pathbuf).unwrap());
            draw_files(state, &layout)?;
            stdout().flush().unwrap();
        }
    } else {
        stdout()
            .queue(Clear(crossterm::terminal::ClearType::All))?
            .queue(MoveTo(0, 0))?
            .queue(Print("smol"))?
            .flush()?;
    }
    Ok(())
}
