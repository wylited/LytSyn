use crate::config::LytConfig;
use crate::playtree::RustMUTree;

use std::io;

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode},
};

use tui::{Terminal, backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, style::{Color, Modifier, Style}, widgets::{Block, Borders}};

pub struct REngine {
    pub config: LytConfig,
    pub quit: bool,
}

impl REngine {
    pub fn new() -> Self{
        Self {
            config: LytConfig::get(),
            quit: false,
        }
    }

    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout); // Crossterm backend for the terminal
        let mut terminal = Terminal::new(backend)?; // Terminal Backend.
        enable_raw_mode().expect("can run in raw mode");

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
            
            if self.quit {
                disable_raw_mode()?;
                terminal.show_cursor()?;
                break;
            }
        }
        Ok(())
    }
}
