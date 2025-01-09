use super::{boxes::TerminalBoxes, layout::MmmLayout};
use crate::datatypes::MmmState;
use crossterm::{cursor::MoveTo, style::Print, QueueableCommand};
use std::io::stdout;

pub fn draw_outline(state: &MmmState, layout: &MmmLayout) -> crossterm::Result<()> {
    let mut boxes = TerminalBoxes::new(state.terminal_size.0, state.terminal_size.1);
    boxes
        .add_box((0, 0), (layout.parent_width, state.terminal_size.1 - 1))
        .unwrap();
    boxes
        .add_box(
            (layout.parent_width, 0),
            (
                layout.parent_width + layout.center_width,
                state.terminal_size.1 - 1,
            ),
        )
        .unwrap();
    boxes
        .add_box(
            (layout.parent_width + layout.center_width, 0),
            (state.terminal_size.0 - 1, state.terminal_size.1 - 1),
        )
        .unwrap();
    stdout().queue(MoveTo(0, 0))?.queue(Print(boxes))?;
    Ok(())
}

pub fn draw_files(state: &MmmState, layout: &MmmLayout) -> crossterm::Result<()> {
    let mut stdout = stdout();
    let mut ypos = 2;
    let column_start = layout.parent_width + 1;
    let max_width = (layout.center_width - 1) as usize;
    stdout.queue(MoveTo(column_start, 1))?;
    let paths = state.current_dir_list.clone();
    if let Some(paths) = paths {
        for path in paths {
            stdout
                .queue(Print(
                    path.file_name()
                        .and_then(|name| name.to_str())
                        .map(|name| name.chars().take(max_width).collect())
                        .unwrap_or_else(|| String::from("")),
                ))?
                .queue(MoveTo(column_start, ypos))?;
            ypos += 1;
        }
    }
    Ok(())
}
