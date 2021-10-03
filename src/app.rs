use crate::{config::LytConfig, drpc::Drpc, renderer::REngine};
use std::{
    env,
    io::{self, Stdout},
    path::PathBuf,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event as CEvent, KeyCode},
};

enum Event<I> {
    Input(I),
    Tick,
}

pub struct App {
    pub work_dir: PathBuf, //Where the working directory is.
    pub quit: bool,        //Becomes true when the user presses <ESC>, causing the program to exit.
    pub config: LytConfig,  //The config that is used.
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
            config: LytConfig::get(),
            stdout: io::stdout(),
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut renderer = REngine::new(); //Fully refactored Rendering engine.

        tokio::spawn(async move{
            REngine::start(&mut renderer);
        });

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

        loop {
            //Hotkeys, This is prob gonna be refactored before I do networking but after I finish the SynEngine.
            match rx.recv()? {
                Event::Input(event) => match event.code {
                    KeyCode::Char('q') => {
                        self.quit = true;
                        break;
                    }
                    KeyCode::Char('-') => {}
                    KeyCode::Char('u') => {}
                    _ => {}
                },
                Event::Tick => {}
            }
        }
        Ok(())
    }
}
