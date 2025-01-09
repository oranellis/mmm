use crossterm::event::{KeyCode, KeyEvent};

use crate::datatypes::MmmState;

pub fn process_key_press(state: &mut MmmState, key_event: KeyEvent) {
    if key_event.code == KeyCode::Esc {
        state.quit = true;
    }
}

pub fn process_resize_event(state: &mut MmmState, columns: u16, rows: u16) {
    state.terminal_size = (columns, rows);
}
