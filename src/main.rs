mod layout;
mod messages;
mod nodes;
use crate::{
    layout::ElementGeometry,
    messages::InteruptMessage,
    nodes::Node,
};
use std::{
    io,
    sync::mpsc,
    thread, env
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, KeyEvent, KeyCode, Event},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use nodes::get_node_list;
use tui::{
    backend::{Backend, CrosstermBackend},
    widgets::{Block, Borders, ListItem, List, BorderType},
    layout::{Constraint, Direction, Layout},
    Terminal, style::{Style, Color},
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
    let cur_dir_nodes: Vec<Node> = get_node_list(env::current_dir().unwrap()).expect("cannot get node list");

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
                    Constraint::Length(geometry.footer_height),
                ].as_ref())
                .split(size);

            let header = layout::generate_header::<B>();
            f.render_widget(header, chunks[0]);

            let body_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(geometry.col_1_width),
                    Constraint::Length(geometry.col_2_width),
                    Constraint::Length(geometry.col_3_width),
                ].as_ref())
                .split(chunks[1]);

            let column_1 = Block::default()
                .title("Column 1")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);
            f.render_widget(column_1, body_chunks[0]);

            let nodes_items: Vec<ListItem> = cur_dir_nodes.iter().map(|node| {
                let display_text = node.file_name.to_string_lossy();
                ListItem::new(display_text.to_string())
            }).collect();

            let nodes_list = List::new(nodes_items)
                .block(
                    Block::default()
                    .title("Current Directory")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                )
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::LightGreen))
                .highlight_symbol(">>");

            f.render_widget(nodes_list, body_chunks[1]); // Render in the second column

            let column_3 = Block::default()
                .title("Column 3")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);
            f.render_widget(column_3, body_chunks[2]);

            let footer = layout::generate_footer::<B>();
            f.render_widget(footer, chunks[2]);
        })?;

        match rx.recv().unwrap() {
            InteruptMessage::KeyCode(code) => {
                if code == KeyCode::Char('q') {
                    break;
                }
            },
            InteruptMessage::Resize => {
                geometry.recalculate(); // Assuming `recalculate` is a method that exists
            },
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
