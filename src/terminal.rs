mod boxes;
mod draw;
mod events;
mod layout;

use crate::{
    datatypes::MmmState,
    filesystem::{get_dir_list, parent_path_from},
};
use crossterm::{
    event::{poll, read, Event},
    style::ResetColor,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, EnterAlternateScreen, LeaveAlternateScreen,
        SetTitle,
    },
    ExecutableCommand, QueueableCommand,
};
use draw::{draw_files, draw_outline};
use events::{process_key_press, process_resize_event};
use std::{
    io::{stdout, Write},
    time::Duration,
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

pub fn terminal_interaction(state: &mut MmmState) {
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
    let layout = layout::generate_layout(state).unwrap();
    if *state != old_state {
        draw_outline(state, &layout).unwrap();
        state.current_dir_list = Some(get_dir_list(&state.current_path).unwrap());
        state.parent_dir_list =
            Some(get_dir_list(&parent_path_from(&state.current_path).unwrap()).unwrap());
        draw_files(state, &layout).unwrap();
        stdout().flush().unwrap();
    }
}
