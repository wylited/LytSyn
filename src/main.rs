//imports
#![allow(unused_imports)]
mod app;
mod playtree;

extern crate tokio;

use app::App;
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use rodio::OutputStream;
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

//structs
#[allow(dead_code)]
enum Event<I> {
    Input(I),
    Tick,
}

//functions
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let mut _args: Vec<String> = std::env::args().collect();

    let mut rust_mu: App = App::new(
        OutputStream::try_default().unwrap().1,
        OutputStream::try_default().unwrap().0,
    );
    rust_mu.run()
}
