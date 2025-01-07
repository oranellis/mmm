use std::{io::Stdout, time::Duration};

use crossterm::event::{poll, read, Event};

use crate::mmm_common::MmmState;

fn process_terminal_event(event: Event) {}

pub fn terminal_interaction_loop(stdout: &Stdout, update: &MmmState) {
    loop {
        let size = terminal::size().unwrap();
        if poll(Duration::from_millis(500)).unwrap() {
            let event = read().unwrap();
            process_terminal_event(event);
        }
    }
}
