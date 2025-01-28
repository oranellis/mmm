use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use terminal_vec2::{vec2, Vec2};

use crate::{error_type::MmmResult, filesystem::MmmFilesys};

use super::{draw::DrawOps, layout::MmmLayout};

pub enum MmmEventType {
    Key(char),
    Space,
    Enter,
    NextEntry,
    PrevEntry,
    Backspace,
    Escape,
    Resize(u16, u16),
    ToggleHidden,
}

pub enum MmmStateUpdateType {
    NavBack,
    NavInto,
    NextEntry,
    PrevEntry,
    AddChar(char),
    ClearSearch,
    Resize(u16, u16),
    ToggleHidden,
    Exit,
}

impl MmmLayout {
    pub fn process_resize_event(&mut self, new_size: Vec2) -> MmmResult<()> {
        #[cfg(not(target_os = "windows"))]
        let new_terminal_size = new_size;
        #[cfg(windows)]
        let new_terminal_size = Vec2 {
            col: new_size.col + 1,
            row: new_size.row + 1,
        };
        *self = Self::from_size(new_terminal_size)?;
        Ok(())
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
            } else if c == 'h' && key_event.modifiers.contains(KeyModifiers::CONTROL) {
                Some(MmmEventType::ToggleHidden)
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

pub fn get_state_update_type(
    event: MmmEventType,
    filesys_state: &MmmFilesys,
) -> Option<MmmStateUpdateType> {
    match event {
        MmmEventType::Enter => None, // Disabled for now
        MmmEventType::Key(c) => Some(MmmStateUpdateType::AddChar(c)),
        MmmEventType::Escape => Some(MmmStateUpdateType::Exit),
        MmmEventType::NextEntry => Some(MmmStateUpdateType::NextEntry),
        MmmEventType::PrevEntry => Some(MmmStateUpdateType::PrevEntry),
        MmmEventType::Resize(col, row) => Some(MmmStateUpdateType::Resize(col, row)),
        MmmEventType::Space => Some(MmmStateUpdateType::NavInto),
        MmmEventType::ToggleHidden => Some(MmmStateUpdateType::ToggleHidden),
        MmmEventType::Backspace => {
            if filesys_state.filter_is_empty() {
                Some(MmmStateUpdateType::NavBack)
            } else {
                Some(MmmStateUpdateType::ClearSearch)
            }
        }
    }
}

pub fn process_state_update(
    state_update: MmmStateUpdateType,
    layout: &mut MmmLayout,
    filesys: &mut MmmFilesys,
) -> MmmResult<DrawOps> {
    match state_update {
        MmmStateUpdateType::Exit => Err("unexpected exit state".into()),
        MmmStateUpdateType::Resize(col, row) => {
            layout.process_resize_event(vec2!(col, row)?)?;
            Ok(DrawOps::new(true, true, true))
        }
        MmmStateUpdateType::NavInto => {
            filesys.try_nav_into()?;
            Ok(DrawOps::new(false, true, true))
        }
        MmmStateUpdateType::NavBack => {
            filesys.try_nav_back()?;
            Ok(DrawOps::new(false, true, true))
        }
        MmmStateUpdateType::NextEntry => {
            filesys.increment_current_selected();
            Ok(DrawOps::new(false, true, false))
        }
        MmmStateUpdateType::PrevEntry => {
            filesys.decrement_current_selected();
            Ok(DrawOps::new(false, true, false))
        }
        MmmStateUpdateType::AddChar(c) => {
            filesys.filter_add_char(c);
            Ok(DrawOps::new(false, true, true))
        }
        MmmStateUpdateType::ClearSearch => {
            filesys.clear_filter();
            Ok(DrawOps::new(false, true, true))
        }
        MmmStateUpdateType::ToggleHidden => {
            filesys.toggle_show_hidden_files()?;
            Ok(DrawOps::new(false, true, false))
        }
    }
}
