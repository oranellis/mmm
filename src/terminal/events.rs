use super::layout::MmmLayout;
use crate::types::{MmmState, Vec2d};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub enum MmmEventType {
    Space,
    Enter,
    NextEntry,
    PrevEntry,
}

impl MmmState {
    pub fn process_resize_event(&mut self, new_size: Vec2d) {
        self.terminal_size = new_size;
        #[cfg(windows)]
        {
            self.terminal_size.col = self.terminal_size.col + 1;
            self.terminal_size.row = self.terminal_size.row + 1;
        }
        self.layout = MmmLayout::from_size(new_size.col, new_size.row);
    }

    pub fn process_key_press(&mut self, key_event: KeyEvent) -> Option<MmmEventType> {
        if key_event.code == KeyCode::Esc {
            self.quit = true;
        }
        let shift = key_event.modifiers.contains(KeyModifiers::SHIFT);
        match key_event.code {
            KeyCode::Esc => self.quit = true,
            KeyCode::Char(c) => {
                if c == ' ' {
                    return Some(MmmEventType::Space);
                } else {
                    self.search_text.push(c);
                }
            }
            KeyCode::Backspace => {
                if !self.search_text.is_empty() {
                    self.search_text = String::new();
                } else {
                    self.current_path = self
                        .current_path
                        .parent()
                        .unwrap_or(&self.current_path)
                        .to_path_buf()
                }
            }
            KeyCode::Tab => {
                if shift {
                    return Some(MmmEventType::PrevEntry);
                } else {
                    return Some(MmmEventType::NextEntry);
                }
            }
            KeyCode::Enter => return Some(MmmEventType::Enter),
            _ => {}
        }
        None
    }
}
