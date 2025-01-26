use std::{fs::File, io::Write, path::PathBuf, time::Duration};

use crossterm::event::EventStream;
use filesystem::MmmFilesys;
use futures::{select, FutureExt, StreamExt};
use terminal::{
    buffer::TerminalBuffer,
    crossterm_wrapper::{start_display, stop_display},
    draw::DrawOps,
    events::{decode_crossterm_event, get_state_update, MmmStateUpdate},
    layout::MmmLayout,
};
use tokio::time::sleep;
use types::{MmmResult, Vec2d};

mod debug;
mod filesystem;
mod style;
mod terminal;
mod types;

async fn mmm() -> MmmResult<PathBuf> {
    let mut layout = MmmLayout::new();
    let mut filesys =
        MmmFilesys::from_path(std::env::current_dir().expect("Error getting filesystem path"))?;
    let mut terminal_buffer = TerminalBuffer::new(&layout.terminal_size);
    let mut event_stream = EventStream::new();
    let mut one_time_trigger = Box::pin(async {}.fuse());

    loop {
        // Wait for a year before updating, just a placeholder for interrupting logic
        let mut timer = Box::pin(sleep(Duration::from_secs(31536000))).fuse();
        let mut terminal_event_future = event_stream.next().fuse();
        let mut terminal_event = None;

        // Wait for an event, the only async section, this needs to be async for the event stream to work
        select! {
            terminal_event_local = terminal_event_future => {
                if let Some(te) = terminal_event_local {
                    terminal_event = Some(te?);
                }
            },
            _ = one_time_trigger => {
                    terminal_event = Some(crossterm::event::Event::Resize(
                        layout.terminal_size.col,
                        layout.terminal_size.row
                    ))
                },
            _ = timer => {},
        }

        // State update logic
        let state_update_option = decode_crossterm_event(terminal_event)
            .and_then(|event| get_state_update(event, &filesys));
        if state_update_option.is_none() {
            continue;
        }
        let draw_ops = match state_update_option.unwrap() {
            MmmStateUpdate::Exit => break,
            MmmStateUpdate::Resize(col, row) => {
                layout.process_resize_event(Vec2d { col, row });
                DrawOps::new(true, true, true)
            }
            MmmStateUpdate::NavInto => {
                filesys.try_nav_into()?;
                DrawOps::new(false, true, true)
            }
            MmmStateUpdate::NavBack => {
                filesys.try_nav_back()?;
                DrawOps::new(false, true, true)
            }
            MmmStateUpdate::NextEntry => {
                filesys.increment_current_selected();
                DrawOps::new(false, true, false)
            }
            MmmStateUpdate::PrevEntry => {
                filesys.decrement_current_selected();
                DrawOps::new(false, true, false)
            }
            MmmStateUpdate::AddChar(c) => {
                filesys.filter_add_char(c);
                DrawOps::new(false, true, true)
            }
            MmmStateUpdate::ClearSearch => {
                filesys.clear_filter();
                DrawOps::new(false, true, true)
            }
        };

        // Rendering logic
        if draw_ops.is_any() {
            if draw_ops.background {
                terminal_buffer = TerminalBuffer::new(&layout.terminal_size);
            }
            terminal_buffer
                .draw_background(&layout)?
                .draw_current_dir(
                    &filesys.filtered_current_dir_list,
                    layout.currentdir_position,
                    layout.currentdir_size,
                )?
                .draw_search_str(
                    layout.search_box_position,
                    layout.search_box_width,
                    filesys.get_filter(),
                )?;
            if let Some(pdl) = &filesys.parent_dir_list {
                terminal_buffer.draw_parent_dir(
                    pdl,
                    filesys.parent_current_entry,
                    layout.parentdir_position,
                    layout.parentdir_size,
                )?;
            }
            terminal_buffer.flush()?;
        }
    }

    Ok(filesys.get_current_path().to_path_buf())
}

#[tokio::main]
async fn main() {
    start_display().expect("error starting display");
    let mmm_result = mmm().await;
    stop_display().expect("error stopping display");
    match mmm_result {
        Ok(path) => {
            let file_path = "/tmp/mmm.path";
            let mut file = File::create(file_path).expect("Failed to create or open the temp file");
            file.write_all(path.to_string_lossy().as_bytes())
                .expect("Failed to write to temp file");
            println!("Navigating to dir {:?}", path);
            std::process::exit(0)
        }
        Err(err) => {
            eprintln!("An error ocurred, {}", err);
            std::process::exit(1)
        }
    }
}
