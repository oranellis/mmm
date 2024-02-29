use crossterm::event::KeyCode;

#[derive(PartialEq, Eq)]
pub(crate) enum InteruptMessage {
    KeyCode(KeyCode),
    Resize,
}

