mod layout;
mod messages;
use layout::ElementGeometry;
use messages::InteruptMessage;
mod nodes;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, KeyEvent, KeyCode, Event},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use std::{io, sync::mpsc, thread};
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let (tx, rx) = mpsc::channel::<InteruptMessage>();
    let tx_clone = tx.clone();

    // Event listening thread
    spawn_events_thread(tx_clone);

    let mut geometry: ElementGeometry = ElementGeometry::new();

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .horizontal_margin(geometry.margain_horizontal)
                .vertical_margin(geometry.margain_vertical)
                .constraints([
                    Constraint::Length(geometry.header_height),
                    Constraint::Min(0),
                    Constraint::Length(geometry.footer_height)
                ].as_ref())
                .split(size);

            let header = layout::generate_header::<B>();
            f.render_widget(header, chunks[0]);

            let body_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(geometry.col_1_width),
                    Constraint::Length(geometry.col_2_width),
                    Constraint::Length(geometry.col_3_width)
                ].as_ref())
                .split(chunks[1]);

            for (i, chunk) in body_chunks.iter().enumerate() {
                let column = Block::default()
                    .title(format!("Column {}", i + 1))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded);
                f.render_widget(column, *chunk);
            }

            let footer: Block<'_> = layout::generate_footer::<B>();
            f.render_widget(footer, chunks[2]);
        })?;

        let interupt_message = rx.recv().unwrap();

        match interupt_message {
            InteruptMessage::KeyCode(code) => {
                if code == KeyCode::Char('q') {
                    break;
                }
            }
            InteruptMessage::Resize => {
                geometry.recalculate()
            }
        }
    }

    Ok(())
}

fn spawn_events_thread(tx: mpsc::Sender<InteruptMessage>) {
    thread::spawn(move || {
        loop {
            let event = event::read().unwrap();
            match event {
                Event::Key(KeyEvent { code, .. }) => {
                    if code == KeyCode::Char('q') {
                        tx.send(InteruptMessage::KeyCode(code)).unwrap();
                        break;
                    }
                },
                Event::Resize(_, _) => {
                    tx.send(InteruptMessage::Resize).unwrap();
                },
                _ => {}
            }
        }
    });
}
