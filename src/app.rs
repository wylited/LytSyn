use crate::playtree::RustMUTree;
use std::env;
use std::path::PathBuf;

use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
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

                if last_tick.elapsed() >= tick_rate {
                    if tx.send(Event::Tick).is_ok() {
                        last_tick = Instant::now();
                    }
                }
            }
        }); //ticking and polling.

        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;

        let mut configinst: crate::playtree::RustMUInst = crate::playtree::RustMUInst::get();

        loop {
            terminal.draw(|rect| {
                let size = rect.size();

                /* let initbox = Paragraph::new(vec![
                    Spans::from(vec![Span::raw(r"♫  ___         _   __  __ _   _  ♫")]),
                    Spans::from(vec![Span::raw(r"♫ | _ \_  _ __| |_|  \/  | | | | ♫")]),
                    Spans::from(vec![Span::raw(r"♫ |   / || (_-<  _| |\/| | |_| | ♫")]),
                    Spans::from(vec![Span::raw(r"♫ |_|_\\_,_/__/\__|_|  |_|\___/  ♫")]),
                ])
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::LightYellow))
                        .title("RustMU")
                        .border_type(BorderType::Plain),
                );

                rect.render_widget(initbox, size); */

                let horizontal_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(1)
                    .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                    .split(size); //Main Vertical Chunks

                let vertical_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
                    .split(horizontal_chunks[1]); //Main Horizontal Chunks

                //renderer

                // Play tree
                let playtree = Block::default().title("Playtree").borders(Borders::ALL);
                rect.render_widget(playtree, horizontal_chunks[0]);

                let queue = Block::default().title("Queue").borders(Borders::ALL);
                rect.render_widget(queue, vertical_chunks[0]);

                let player = Block::default().title("Player").borders(Borders::ALL);
                rect.render_widget(player, vertical_chunks[1]);
            })?;

            match rx.recv()? {
                Event::Input(event) => match event.code {
                    KeyCode::Char('q') => {
                        self.quit = true;
                    }
                    KeyCode::Char('-') => {}
                    KeyCode::Char('u') => {
                        
                    }
                    _ => {}
                },
                Event::Tick => {}
            }

            if self.quit == true {
                disable_raw_mode()?;
                terminal.show_cursor()?;
                break;
            }
        }
        Ok(())
    }
}
