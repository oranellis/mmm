mod debug;
mod filesystem;
mod style;
mod terminal;
mod types;

use crate::{
    filesystem::{dirlist::get_dir_list, filter::filter_files},
    terminal::{
        buffer::TerminalBuffer,
        crossterm_wrapper::{flush, move_cursor, start_display, stop_display},
    },
    types::{MmmResult, MmmState},
};
use crossterm::event::{Event, EventStream};
use futures::{select, FutureExt, StreamExt};
use terminal::draw::draw_dir;

async fn mmm() -> MmmResult<()> {
    let mut shared_state = MmmState::new();
    let mut event_stream = EventStream::new();
    let mut old_buffer = TerminalBuffer::new(
        String::with_capacity(shared_state.get_display_string_capacity()),
        &shared_state.terminal_size,
    );
    let mut one_time_trigger = Box::pin(async {}.fuse());

    loop {
        let mut timer = Box::pin(tokio::time::sleep(tokio::time::Duration::from_secs(10))).fuse();
        let mut terminal_event_future = event_stream.next().fuse();
        let mut terminal_event = None;

        // Wait for an event
        select! {
            terminal_event_local = terminal_event_future => {
                if let Some(te) = terminal_event_local {
                    terminal_event = Some(te?);
                }
            },
            _ = one_time_trigger => {},
            _ = timer => {},
        }
        if let Some(event) = terminal_event {
            match event {
                Event::Key(key_event) => shared_state.process_key_press(key_event),
                Event::Resize(col, row) => shared_state.process_resize_event((col, row).into()),
                _ => {}
            }
        };
        if shared_state.quit {
            break;
        }

        // Get filesystem information
        shared_state.current_dir_list = Some(get_dir_list(&shared_state.current_path)?);
        shared_state.parent_dir_list = shared_state
            .current_path
            .parent()
            .map(get_dir_list)
            .transpose()?;

        // Filter and sort display lists
        let current_dir_display_list = shared_state.current_dir_list.as_ref().map(|cdl| {
            filter_files(
                cdl,
                &shared_state.search_text,
                shared_state.layout.currentdir_size.row.into(),
            )
        });
        let parent_dir_display_list = shared_state
            .parent_dir_list
            .as_ref()
            .map(|pdl| filter_files(pdl, "", shared_state.layout.parentdir_size.row.into()));

        // Create and display full terminal buffer
        let mut new_buffer = TerminalBuffer::new(
            String::with_capacity(shared_state.get_display_string_capacity()),
            &shared_state.terminal_size,
        );
        new_buffer.add_layer(&shared_state.draw_outline()?);
        new_buffer.add_layer(&shared_state.draw_search_str());
        new_buffer.add_layer(&draw_dir(
            current_dir_display_list,
            &shared_state.layout.currentdir_position,
            &shared_state.layout.currentdir_size,
            &shared_state.terminal_size.col,
        ));
        new_buffer.add_layer(&draw_dir(
            parent_dir_display_list,
            &shared_state.layout.parentdir_position,
            &shared_state.layout.parentdir_size,
            &shared_state.terminal_size.col,
        ));
        old_buffer = new_buffer.queue_print_buffer_diff(old_buffer)?;
        move_cursor(shared_state.search_cursor_pos(0))?;
        flush()?;
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    start_display().expect("error starting display");
    let mmm_result = mmm().await;
    stop_display().expect("error stopping display");
    match mmm_result {
        Ok(_) => {
            println!("Quitting mmm...");
            std::process::exit(0)
        }
        Err(err) => {
            eprintln!("An error ocurred, {}", err);
            std::process::exit(1)
        }
    }
}
