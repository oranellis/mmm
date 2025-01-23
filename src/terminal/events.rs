use super::layout::MmmLayout;
use crate::{filesystem::MmmFilesys, types::Vec2d};
use crossterm::event::{Event, KeyCode, KeyEvent};

pub enum MmmEventType {
    Key(char),
    Space,
    Enter,
    NextEntry,
    PrevEntry,
    Backspace,
    Escape,
    Resize(u16, u16),
}

pub enum MmmStateUpdate {
    NavBack,
    NavInto,
    NextEntry,
    PrevEntry,
    AddChar(char),
    ClearSearch,
    Resize(u16, u16),
    Exit,
}

impl MmmLayout {
    pub fn process_resize_event(&mut self, new_size: Vec2d) {
        let new_terminal_size;
        #[cfg(not(target_os = "windows"))]
        {
            new_terminal_size = new_size;
        }
        #[cfg(windows)]
        {
            new_terminal_size = Vec2d {
                col: new_size.col + 1,
                row: new_size.row + 1,
            };
        }
        *self = Self::from_size(new_terminal_size);
    }
}

pub fn decode_crossterm_event(event: Option<Event>) -> Option<MmmEventType> {
    if let Some(event) = event {
        match event {
            Event::Key(c) => decode_key_event(c),
            Event::Resize(col, row) => Some(MmmEventType::Resize(col, row)),
            _ => None,
        }
    } else {
        None
    }
}

fn decode_key_event(key_event: KeyEvent) -> Option<MmmEventType> {
    match key_event.code {
        KeyCode::Char(c) => {
            if c == ' ' {
                Some(MmmEventType::Space)
            } else {
                Some(MmmEventType::Key(c))
            }
        }
        KeyCode::Enter => Some(MmmEventType::Enter),
        KeyCode::BackTab => Some(MmmEventType::PrevEntry),
        KeyCode::Tab => Some(MmmEventType::NextEntry),
        KeyCode::Backspace => Some(MmmEventType::Backspace),
        KeyCode::Esc => Some(MmmEventType::Escape),
        _ => None,
    }
}

pub fn get_state_update(event: MmmEventType, filesys_state: &MmmFilesys) -> Option<MmmStateUpdate> {
    match event {
        MmmEventType::Enter => None, // Disabled for now
        MmmEventType::Key(c) => Some(MmmStateUpdate::AddChar(c)),
        MmmEventType::Escape => Some(MmmStateUpdate::Exit),
        MmmEventType::NextEntry => Some(MmmStateUpdate::NextEntry),
        MmmEventType::PrevEntry => Some(MmmStateUpdate::PrevEntry),
        MmmEventType::Resize(col, row) => Some(MmmStateUpdate::Resize(col, row)),
        MmmEventType::Space => Some(MmmStateUpdate::NavInto),
        MmmEventType::Backspace => {
            if filesys_state.filter_is_empty() {
                Some(MmmStateUpdate::NavBack)
            } else {
                Some(MmmStateUpdate::ClearSearch)
            }
        }
    }
}
