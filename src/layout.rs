use std::process::exit;
use crossterm::terminal;
use tui::{widgets::{Block, Borders}, backend::Backend};

pub(super) fn get_columns_width() -> u16 {
    let (terminal_width, _) = terminal::size().unwrap_or_else(|_| {
        eprintln!("Error getting terminal size, quitting");
        exit(1);
    });
    // Subtract 2 for the margains
    terminal_width - 2
}

pub(super) fn calculate_column_widths() -> (u16, u16, u16) {
    let base_width: u16 = 34;
    let min_width: u16 = 18;
    let terminal_width = get_columns_width();

    if terminal_width / 3 < min_width {
        if terminal_width / 2 < min_width {
            (0, terminal_width, 0)
        } else {
            (terminal_width / 2, terminal_width - (terminal_width / 2), 0)
        }
    } else if terminal_width > base_width * 3 {
        (base_width, base_width, terminal_width - (base_width * 2))
    } else {
        (terminal_width / 3, terminal_width / 3, terminal_width - (2 * terminal_width / 3))
    }
}

pub(super) fn generate_header<B: Backend>() -> Block<'static> {
    Block::default()
        .title("Header")
        .borders(Borders::NONE)
}

pub(super) fn generate_footer<B: Backend>() -> Block<'static> {
    Block::default()
        .title("Footer")
        .borders(Borders::NONE)
}
