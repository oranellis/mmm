use std::{fs::File, io::Write, path::PathBuf, time::Duration};

use crossterm::event::EventStream;
use doubuff::{
    buffer::TerminalBuffer,
    helpers::{start_display, stop_display},
};
use error_type::MmmResult;
use filesystem::MmmFilesys;
use futures::{select, FutureExt, StreamExt};
use terminal::{
    events::{
        decode_crossterm_event, get_state_update_type, process_state_update, MmmStateUpdateType,
    },
    layout::MmmLayout,
};
use tokio::time::sleep;

mod debug;
mod error_type;
mod filesystem;
mod terminal;

async fn mmm() -> MmmResult<PathBuf> {
    let initial_path = std::env::current_dir()?;
    let mut layout = MmmLayout::new()?;
    let mut filesys = MmmFilesys::from_path(initial_path)?;
    let mut term_buffer = TerminalBuffer::new(layout.term_size);
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
                        layout.term_size.col,
                        layout.term_size.row
                    ))
                },
            _ = timer => {},
        }

        // State update logic
        let state_update_option = decode_crossterm_event(terminal_event)
            .and_then(|event| get_state_update_type(event, &filesys));
        if state_update_option.is_none() {
            continue;
        }
        let draw_ops = match state_update_option.expect("illegal state_update_option state") {
            MmmStateUpdateType::Exit => {
                break;
            }
            state_update => process_state_update(state_update, &mut layout, &mut filesys)?,
        };

        // Rendering logic
        draw_ops.draw(&mut term_buffer, &filesys, &layout)?;
        term_buffer.flush()?;
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
            std::process::exit(0)
        }
        Err(err) => {
            eprintln!("An error ocurred, {}", err);
            std::process::exit(1)
        }
    }
}
