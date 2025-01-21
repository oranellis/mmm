mod debug;
mod filesystem;
mod nvim;
mod style;
mod terminal;
mod types;

use std::{fs::File, io::Write, path::PathBuf};

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
use nvim::{command_setup, command_teardown};
use terminal::{draw::draw_dir, events::MmmEventType};
use tokio::time::{sleep, Duration};

async fn mmm() -> MmmResult<PathBuf> {
    let mut shared_state = MmmState::new();
    let mut event_stream = EventStream::new();
    let mut old_buffer = TerminalBuffer::new(
        String::with_capacity(shared_state.get_display_string_capacity()),
        &shared_state.terminal_size,
    );
    let mut one_time_trigger = Box::pin(async {}.fuse());

    loop {
        // Wait for a year before updating, just a placeholder for interrupting logic
        let mut timer = Box::pin(sleep(Duration::from_secs(31536000))).fuse();
        let mut terminal_event_future = event_stream.next().fuse();
        let mut terminal_event = None;
        let mut redraw = false;

        // Wait for an event, the only async section, is async for the event stream to work
        select! {
            terminal_event_local = terminal_event_future => {
                if let Some(te) = terminal_event_local {
                    terminal_event = Some(te?);
                }
            },
            _ = one_time_trigger => {},
            _ = timer => {},
        }

        // Process events
        let mut key_event_option: Option<MmmEventType> = None;
        if let Some(event) = terminal_event {
            match event {
                Event::Key(key_press) => {
                    key_event_option = shared_state.process_key_press(key_press)
                }
                Event::Resize(col, row) => shared_state.process_resize_event((col, row).into()),
                _ => {}
            }
        };
        if shared_state.quit {
            break;
        }

        // Loop getting filesystem information
        let mut current_dir_display_list;
        let mut parent_dir_display_list;
        let mut should_loop;
        loop {
            should_loop = false;
            shared_state.current_dir_list = Some(get_dir_list(&shared_state.current_path)?);
            shared_state.parent_dir_list = shared_state
                .current_path
                .parent()
                .map(get_dir_list)
                .transpose()?;

            // Filter and sort display lists
            current_dir_display_list = shared_state.current_dir_list.as_ref().map(|cdl| {
                filter_files(
                    cdl,
                    &shared_state.search_text,
                    shared_state.layout.currentdir_size.row.into(),
                )
            });
            parent_dir_display_list = shared_state
                .parent_dir_list
                .as_ref()
                .map(|pdl| filter_files(pdl, "", shared_state.layout.parentdir_size.row.into()));

            // Process file selection
            if let Some(key_event) = &key_event_option {
                match key_event {
                    MmmEventType::Space => {
                        if let Some(current_dir_display_list) = &current_dir_display_list {
                            if let Some(entry) = current_dir_display_list.entries.get(0) {
                                match entry {
                                    filesystem::MmmDirEntry::Directory { name: _, path } => {
                                        key_event_option = None;
                                        shared_state.current_path = path.to_path_buf();
                                        shared_state.search_text = String::new();
                                        should_loop = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    MmmEventType::Enter => {
                        if let Some(current_dir_display_list) = &current_dir_display_list {
                            if let Some(entry) = current_dir_display_list.entries.get(0) {
                                match entry {
                                    filesystem::MmmDirEntry::File {
                                        name: _,
                                        path,
                                        executable: _,
                                    } => {
                                        key_event_option = None;
                                        should_loop = true;
                                        shared_state.search_text = String::new();
                                        command_setup()?;
                                        let _ =
                                            std::process::Command::new("nvim").arg(path).status();
                                        command_teardown()?;
                                        redraw = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            if !should_loop {
                break;
            }
        }

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
        if redraw {
            old_buffer = new_buffer.queue_print_buffer()?;
        } else {
            old_buffer = new_buffer.queue_print_buffer_diff(old_buffer)?;
        }
        move_cursor(shared_state.search_cursor_pos(0))?;
        flush()?;
    }

    Ok(shared_state.current_path)
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
