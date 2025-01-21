use super::layout::MmmLayout;
use crate::types::{MmmState, Vec2d};
use crossterm::event::{KeyCode, KeyEvent};

impl MmmState {
    pub fn process_resize_event(&mut self, new_size: Vec2d) {
        self.terminal_size = new_size;
        self.layout = MmmLayout::from_size(new_size.col, new_size.row);
    }

    pub fn process_key_press(&mut self, key_event: KeyEvent) {
        if key_event.code == KeyCode::Esc {
            self.quit = true;
        }
        match key_event.code {
            KeyCode::Esc => self.quit = true,
            KeyCode::Char(c) => {
                self.search_text.push(c);
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
            _ => {}
        }
    }
}
