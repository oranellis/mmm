mod filesystem;
mod style;
mod terminal;
mod types;

use crossterm::event::{Event, EventStream};
use filesystem::dirlist::get_dir_list;
use futures::{select, FutureExt, StreamExt};
use terminal::{
    composer::TerminalBuffer,
    interactor::{start_display, stop_display},
};
use types::MmmState;

#[tokio::main]
async fn main() {
    let mut shared_state = MmmState::new();
    let mut event_stream = EventStream::new();
    let mut old_buffer = TerminalBuffer::new(String::new(), &shared_state.terminal_size);
    start_display().expect("error starting display");

    loop {
        let mut timer = Box::pin(tokio::time::sleep(tokio::time::Duration::from_secs(10))).fuse();
        let mut terminal_event_future = event_stream.next().fuse();
        let mut terminal_event = None;

        select! {
            terminal_event_local = terminal_event_future => {
                if let Some(Ok(te)) = terminal_event_local {
                    terminal_event = Some(te);
                }
            }
            _ = timer => {}
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

        shared_state.current_dir_list = get_dir_list(&shared_state.current_path).unwrap();
        let mut new_buffer = TerminalBuffer::new(String::new(), &shared_state.terminal_size);
        new_buffer.add_layer(&shared_state.draw_outline().unwrap());
        new_buffer.add_layer(&shared_state.draw_search_str().unwrap());
        // new_buffer.print_buffer().unwrap();
        old_buffer.print_buffer_diff(new_buffer).unwrap();
    }

    stop_display().expect("error stopping display");
    println!("Quitting mmm...");
}
