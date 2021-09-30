use crate::{config::MuConfig, playtree::RustMUTree};
use std::env;
use std::path::PathBuf;

use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use directories::UserDirs;
use rodio::{source::Source, Decoder, OutputStream, OutputStreamHandle, Sink};
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
    },
    Terminal,
};

enum Event<I> {
    Input(I),
    Tick,
}

pub struct App {
    pub work_dir: PathBuf,                 //Where the working directory is.
    pub quit: bool,                        // Becomes true when the user presses <ESC>
    pub stream_handle: OutputStreamHandle, //Sound Engine Handler
    pub _stream: OutputStream,             //Sound Engine
    pub sink: Sink,
}

impl App {
    pub fn new(streamhandle: OutputStreamHandle, stream: OutputStream) -> Self {
        Self {
            work_dir: match env::current_exe() {
                Ok(exe_path) => exe_path,
                Err(_e) => PathBuf::new(),
            },
            quit: false,
            _stream: stream,
            sink: Sink::try_new(&streamhandle).unwrap(),
            stream_handle: streamhandle,
        }
    }
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        enable_raw_mode().expect("can run in raw mode"); //not sure exactly but doesnt exactly work without this
        let (tx, rx) = mpsc::channel();
        let tick_rate = Duration::from_millis(200); //refresh rate bassically.

        thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));

                if event::poll(timeout).expect("poll works") {
                    if let CEvent::Key(key) = event::read().expect("can read events") {
                        tx.send(Event::Input(key)).expect("can send events");
                    }
                }

                if last_tick.elapsed() >= tick_rate && tx.send(Event::Tick).is_ok() {
                    last_tick = Instant::now();
                }
            }
        }); //ticking and polling.

        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        let config = MuConfig::get();
        terminal.clear()?;

        loop {
            terminal.draw(|rect| {
                let size = rect.size();

                let horizontal_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(1)
                    .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                    .split(size); //Main Vertical Chunks

                let vertical_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints([
                        Constraint::Percentage(20),
                        Constraint::Percentage(60),
                        Constraint::Percentage(20),
                    ])
                    .split(horizontal_chunks[1]); //Main Horizontal Chunks

                let mutree = RustMUTree::parse(false, None);

                //renderer
                // Play tree
                let playtree = RustMUTree::display(mutree)
                    .block(
                        Block::default()
                            .title("│ Playlist │")
                            .borders(Borders::ALL)
                            .style(Style::default().fg(Color::Rgb(config.theme.borders[0] ,config.theme.borders[1], config.theme.borders[2]))))
                    .style(Style::default().fg(Color::Rgb(
                        config.theme.minor_text[0],
                        config.theme.minor_text[1],
                        config.theme.minor_text[2],
                    )))
                    .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                    .highlight_symbol(&config.theme.selectsymbol);

                rect.render_widget(playtree, horizontal_chunks[0]);

                let rmubox = Paragraph::new(vec![
                    Spans::from(vec![Span::raw(r"   ♫ ___           __  __  _____  __  ♫")]),
                    Spans::from(vec![Span::raw(r"  ♫ / _ \__ _____ / /_/  |/  / / / / ♫")]),
                    Spans::from(vec![Span::raw(r" ♫ / , _/ // (_-</ __/ /|_/ / /_/ / ♫")]),
                    Spans::from(vec![Span::raw(r"♫ /_/|_|\_,_/___/\__/_/  /_/\____/ ♫")]),
                ])
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::Rgb(config.theme.borders[0], config.theme.borders[1], config.theme.borders[2])))
                        .title("RustMU")
                        .border_type(BorderType::Thick),
                );

                rect.render_widget(rmubox, vertical_chunks[0]);

                let queue = Block::default()
                    .title("│ Queue │")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Rgb(
                        config.theme.minor_text[0],
                        config.theme.minor_text[1],
                        config.theme.minor_text[2],
                    )));

                rect.render_widget(queue, vertical_chunks[1]);

                let player = Block::default()
                    .title("│ Player │")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Rgb(
                        config.theme.minor_text[0],
                        config.theme.minor_text[1],
                        config.theme.minor_text[2],
                    )));
                rect.render_widget(player, vertical_chunks[2]);
            })?;

            match rx.recv()? {
                Event::Input(event) => match event.code {
                    KeyCode::Char('q') => {
                        self.quit = true;
                    }
                    KeyCode::Char('-') => {}
                    KeyCode::Char('u') => {}
                    _ => {}
                },
                Event::Tick => {}
            }

            if self.quit {
                disable_raw_mode()?;
                terminal.show_cursor()?;
                break;
            }
        }
        Ok(())
    }
}
