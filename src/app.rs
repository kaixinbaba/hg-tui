use crate::config::Config;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use anyhow::Result;
use std::{
    error::Error,
    io::{self, Stdout},
};

use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    terminal,
    text::Span,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Widget},
    Frame, Terminal,
};

pub struct App {
    terminal: Terminal<CrosstermBackend<Stdout>>,

    quit: bool,
}

impl App {
    fn new(config: &Config) -> Result<App> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;
        Ok(App { terminal, quit: false })
    }
}


impl Drop for App {

    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        ).unwrap();
        self.terminal.show_cursor().unwrap();
    }

}


pub(crate) fn start(config: &Config) -> Result<()> {
    let mut app = App::new(config)?;

    while !app.quit {

    }

    Ok(())
}
