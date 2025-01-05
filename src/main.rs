mod mmm_common;

use crossterm::{
    cursor::{MoveLeft, MoveTo},
    event::{poll, read, Event, KeyCode},
    style::{Print, ResetColor},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, EnterAlternateScreen, LeaveAlternateScreen,
        SetTitle,
    },
    ExecutableCommand, QueueableCommand,
};
use mmm_common::{MmmState, UpdateType};
use std::{
    fs::{self, DirEntry},
    io::{self, stdout, Stdout, Write},
    path::Path,
    sync::{Arc, Mutex},
    thread::sleep,
};

/// Starts the terminal display
///
/// * `stdout`: A thread safe reference to the crossterm terminal output
fn start_display(stdout: Arc<Mutex<Stdout>>) -> Result<(), std::io::Error> {
    let mut stdout = stdout.lock().unwrap();
    stdout
        .queue(SetTitle("mmm"))?
        .queue(EnterAlternateScreen)?
        .queue(ResetColor)?
        .queue(Clear(crossterm::terminal::ClearType::All))?
        .flush()?;
    enable_raw_mode()?;
    Ok(())
}

/// Stops the terminal display
///
/// * `stdout`: A thread safe reference to the crossterm terminal output
fn stop_display(stdout: Arc<Mutex<Stdout>>) -> Result<(), std::io::Error> {
    let mut stdout = stdout.lock().unwrap();
    stdout.execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

/// Updates the terminal display
///
/// * `stdout`: A thread safe reference to the crossterm terminal output
/// * `update`: The update to perform
fn update_display(stdout: Arc<Mutex<Stdout>>, update: &UpdateType) -> Result<(), std::io::Error> {
    let mut stdout = stdout.lock().unwrap();
    match update {
        UpdateType::Char(recv_char) => {
            stdout.execute(Print(recv_char))?;
        }
        UpdateType::Backspace => {
            stdout
                .queue(MoveLeft(1))?
                .queue(Print(" "))?
                .queue(MoveLeft(1))?
                .flush()?;
        }
        UpdateType::Clear => {
            stdout
                .queue(Clear(crossterm::terminal::ClearType::All))?
                .queue(MoveTo(0, 10))?
                .flush()?;
        }
        _ => {}
    };
    Ok(())
}

/// Converts from a keycode to the update type required
///
/// * `key_event`: The crossterm KeyEvent to convert
fn process_key_input(key_event: crossterm::event::KeyEvent) -> UpdateType {
    match key_event.code {
        KeyCode::Char(c) => UpdateType::Char(c),
        KeyCode::Backspace => UpdateType::Backspace,
        KeyCode::Tab => UpdateType::Clear,
        KeyCode::Esc => UpdateType::Quit,
        _ => UpdateType::None,
    }
}

/// Returns the size on the disk of a file or folder in bytes
///
/// * `path`: The file or folder to get the size of
fn get_path_size(path: &Path) -> io::Result<u64> {
    let mut total_size = 0;
    if path.is_file() {
        total_size += fs::metadata(path)?.len();
    } else if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            total_size += get_path_size(&entry_path)?;
        }
    }
    Ok(total_size)
}

fn get_dir_list(path: &Path) -> io::Result<Vec<DirEntry>> {
    let dir_list: Vec<DirEntry> = fs::read_dir(path)?.filter_map(|entry| entry.ok()).collect();
    Ok(dir_list)
}

/// The main program loop for processing inputs and updating the display
///
/// * `stdout`: A thread safe reference to the crossterm terminal output
/// * `_state`: Currently unused
fn display_loop(stdout: Arc<Mutex<Stdout>>, _state: Arc<Mutex<MmmState>>) {
    loop {
        if poll(std::time::Duration::from_millis(100)).is_err() {
            continue;
        }
        let event: Event = read().unwrap();
        if let Event::Key(key_event) = event {
            let update_type = process_key_input(key_event);
            if update_type == UpdateType::Quit {
                break;
            }
            update_display(stdout.clone(), &update_type).expect("Error updating display");
        }
    }
}

fn display_paths(
    stdout: Arc<Mutex<Stdout>>,
    _state: Arc<Mutex<MmmState>>,
) -> Result<(), std::io::Error> {
    let mut stdout = stdout.lock().unwrap();
    let cwd = std::env::current_dir().expect("Failed to get cwd");
    let dir_list = get_dir_list(&cwd).expect("Failed to get cwd dirlist");
    let mut y: u16 = 0;
    stdout.queue(MoveTo(12, 0))?.flush()?;
    for x in dir_list {
        let fileee = x.file_name().to_string_lossy().to_string();
        y += 1;
        stdout
            .queue(Print(format!("{}", fileee)))?
            .queue(MoveTo(12, y))?
            .flush()?;
    }
    Ok(())
}

fn main() {
    let stdout = Arc::new(Mutex::new(stdout()));
    let state = Arc::new(Mutex::new(MmmState::new()));
    start_display(stdout.clone()).expect("Error starting display");
    display_paths(stdout.clone(), state.clone()).expect("Error displaying paths");
    display_loop(stdout.clone(), state);
    stop_display(stdout).expect("Error stopping display");
    println!("Quitting mmm...");
}
