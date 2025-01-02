use std::{
    io::{stdout, ErrorKind, Write},
    sync::mpsc::{channel, Receiver, Sender},
};

use crossterm::{
    cursor::{position, MoveLeft, MoveTo},
    event::{read, Event, KeyCode},
    style::{Print, ResetColor},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, EnterAlternateScreen, LeaveAlternateScreen,
        SetTitle,
    },
    ExecutableCommand, QueueableCommand,
};

enum UpdateType {
    String(String),
    Backspace,
    Clear,
}

struct ScreenUpdate {
    update_type: UpdateType,
    screen_pos: (u16, u16),
}

fn terminal_loop(rx: &Receiver<ScreenUpdate>) -> Result<(), std::io::Error> {
    let mut stdout = stdout();
    stdout.execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    stdout
        .queue(ResetColor)?
        .queue(Clear(crossterm::terminal::ClearType::All))?
        .queue(MoveTo(0, 10))?
        .queue(SetTitle("mmm"))?
        .flush()?;
    loop {
        match rx.recv() {
            Ok(update) => {
                let (mut x, y) = update.screen_pos;
                match update.update_type {
                    UpdateType::String(recv_string) => {
                        stdout.execute(Print(recv_string))?;
                        x += 1;
                    }
                    UpdateType::Backspace => {
                        stdout
                            .queue(MoveLeft(1))?
                            .queue(Print(" "))?
                            .queue(MoveLeft(1))?
                            .flush()?;
                        x = if x == 0 { 0 } else { x - 1 };
                    }
                    UpdateType::Clear => {
                        stdout
                            .queue(Clear(crossterm::terminal::ClearType::All))?
                            .queue(MoveTo(0, 10))?
                            .flush()?;
                    }
                };
                stdout
                    .queue(MoveTo(0, 5))?
                    .queue(Print(format!("({}, {})", x, y)))?
                    .queue(MoveTo(x, y))?
                    .flush()?;
            }
            Err(_) => break,
        }
    }
    stdout.execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    return Err(std::io::Error::new(ErrorKind::Other, "Keyboard interrupt"));
}

fn key_input_loop(term_tx: Sender<ScreenUpdate>) {
    loop {
        if let Event::Key(key_event) = read().expect("Error reading key") {
            term_tx
                .send(ScreenUpdate {
                    update_type: match key_event.code {
                        KeyCode::Char(c) => UpdateType::String(c.to_string()),
                        KeyCode::Backspace => UpdateType::Backspace,
                        KeyCode::Tab => UpdateType::Clear,
                        KeyCode::Esc => break, // Cheeky loop control flow
                        _ => continue,         // Even cheekier loop control flow
                    },
                    screen_pos: position().unwrap(),
                })
                .expect("Error sending data to display");
        }
    }
}

fn main() {
    let (terminal_tx, terminal_rx): (Sender<ScreenUpdate>, Receiver<ScreenUpdate>) = channel();
    let terminal_thread = std::thread::spawn(move || {
        terminal_loop(&terminal_rx).expect_err("Terminal error");
    });
    let key_input_tx = terminal_tx.clone();
    let key_input_thread = std::thread::spawn(move || {
        key_input_loop(key_input_tx);
    });
    key_input_thread
        .join()
        .expect("Error waiting for thread to join");
    drop(terminal_tx);
    terminal_thread
        .join()
        .expect("Error waiting for thread to join");
    println!("Quitting mmm...");
}
