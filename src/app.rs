use crate::{config::MuConfig, drpc::Drpc, playtree::RustMUTree};
use std::{
    env,
    io::{self, Stdout},
    path::PathBuf,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event as CEvent, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use directories::UserDirs;
use rodio::{source::Source, Decoder, OutputStream, OutputStreamHandle, Sink};
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
    pub work_dir: PathBuf, //Where the working directory is.
    pub quit: bool,        //Becomes true when the user presses <ESC>, causing the program to exit.
    pub config: MuConfig,  //The config that is used.
    pub stdout: Stdout,    //The standard output stream.
}

impl App {
    pub fn new() -> Self {
        Self {
            work_dir: match env::current_exe() {
                Ok(exe_path) => exe_path,
                Err(_e) => PathBuf::new(),
            },
            quit: false,
            config: MuConfig::get(),
            stdout: io::stdout(),
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let backend = CrosstermBackend::new(self.stdout.lock()); // Crossterm backend for the terminal
        let mut terminal = Terminal::new(backend)?; // Terminal Backend.
        enable_raw_mode().expect("can run in raw mode"); //Sets the terminal to run in raw mode.
        let (tx, rx) = mpsc::channel();

        if self.config.settings.discord {
            thread::spawn(move || {
                let disc = Drpc::default();
                Drpc::run(&disc);
            });
        } // Discord Rich Presence.

        thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                let timeout = Duration::from_millis(200)
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));

                if event::poll(timeout).expect("poll works") {
                    if let CEvent::Key(key) = event::read().expect("can read events") {
                        tx.send(Event::Input(key)).expect("can send events");
                    }
                }

                if last_tick.elapsed() >= Duration::from_millis(200) && tx.send(Event::Tick).is_ok()
                {
                    last_tick = Instant::now();
                }
            }
        }); //Ticking and polling.

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
                    .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
                    .split(horizontal_chunks[1]); //Main Horizontal Chunks

                let mutree = RustMUTree::parse(false, None);

                //renderer
                // Play tree
                let playtree = RustMUTree::display(mutree)
                    .block(
                        Block::default()
                            .title("│ Playlist │")
                            .borders(Borders::ALL)
                            .style(Style::default().fg(Color::Rgb(
                                self.config.theme.borders[0],
                                self.config.theme.borders[1],
                                self.config.theme.borders[2],
                            ))),
                    )
                    .style(Style::default().fg(Color::Rgb(
                        self.config.theme.minor_text[0],
                        self.config.theme.minor_text[1],
                        self.config.theme.minor_text[2],
                    )))
                    .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                    .highlight_symbol(&self.config.theme.selectsymbol);

                rect.render_widget(playtree, horizontal_chunks[0]);

                let queue = Block::default()
                    .title("│ Queue │")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Rgb(
                        self.config.theme.minor_text[0],
                        self.config.theme.minor_text[1],
                        self.config.theme.minor_text[2],
                    )));

                rect.render_widget(queue, vertical_chunks[0]);

                let player = Block::default()
                    .title("│ Player │")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Rgb(
                        self.config.theme.minor_text[0],
                        self.config.theme.minor_text[1],
                        self.config.theme.minor_text[2],
                    )));
                rect.render_widget(player, vertical_chunks[1]);
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
