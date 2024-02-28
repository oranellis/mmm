use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, KeyEvent, KeyCode, Event},
    terminal::{self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use std::{io, sync::mpsc, thread, process::exit};
use tui::{
    backend::{Backend, CrosstermBackend},
    widgets::{Block, Borders, BorderType},
    layout::{Constraint, Direction, Layout},
    Terminal,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Application main loop
    let res = run_app(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

#[derive(PartialEq, Eq)]
enum InteruptMessage {
    KeyCode(KeyCode),
    Resize,
}

fn get_terminal_width() -> u16 {
    let (terminal_width, _) = terminal::size().unwrap_or_else(|_| {
        eprintln!("Error getting terminal size, quitting");
        exit(1);
    });
    terminal_width
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    // Setup channel for event handling
    let (tx, rx) = mpsc::channel::<InteruptMessage>();
    let tx_clone = tx.clone();

    // Event listening thread
    thread::spawn(move || {
        loop {
            // This call blocks until an event is available
            let event = event::read().unwrap();
                match event {
                    Event::Key(KeyEvent { code, .. }) => {
                        if code == KeyCode::Char('q') {
                            tx_clone.send(InteruptMessage::KeyCode(code)).unwrap();
                            break;
                        }
                    },
                    Event::Resize(_, _) => {
                        tx_clone.send(InteruptMessage::Resize).unwrap();
                    },
                    _ => {}
                }
        }
    });

    let mut column_width = get_terminal_width() / 3;
    let mut small_column_width = get_terminal_width() - (2*column_width);

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)].as_ref())
                .split(size);

            let header = generate_header::<B>();
            f.render_widget(header, chunks[0]);


            let body_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Length(small_column_width), Constraint::Length(column_width), Constraint::Length(column_width)].as_ref())
                .split(chunks[1]);

            for (i, chunk) in body_chunks.iter().enumerate() {
                let column = Block::default()
                    .title(format!("Column {}", i + 1))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded);
                f.render_widget(column, *chunk);
            }

            let footer: Block<'_> = generate_footer::<B>();
            f.render_widget(footer, chunks[2]);
        })?;

        // Check if there's a message to quit
        let interupt_message = rx.recv().unwrap();

        match interupt_message {
            InteruptMessage::KeyCode(code) => {
                if code == KeyCode::Char('q') {
                    break;
                }
            }
            InteruptMessage::Resize => {
                column_width = get_terminal_width() / 3;
                small_column_width = get_terminal_width() - (2*column_width);
            }
        }
    }

    Ok(())
}

fn generate_header<B: Backend>() -> Block<'static> {
    Block::default()
        .title("Header")
        .borders(Borders::NONE)
}

fn generate_footer<B: Backend>() -> Block<'static> {
    Block::default()
        .title("Footer")
        .borders(Borders::NONE)
}
