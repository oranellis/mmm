mod datatypes;
mod filesystem;
mod style;
mod terminal;

use datatypes::MmmState;
use terminal::{start_display, stop_display, terminal_interaction};

fn main() {
    let mut shared_state = MmmState::new();
    start_display().unwrap();
    loop {
        if let Err(_) = terminal_interaction(&mut shared_state) {
            break;
        }
        if shared_state.quit == true {
            break;
        }
    }
    stop_display().unwrap();
    println!("Quitting mmm...");
}
