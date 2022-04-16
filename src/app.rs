use crate::config::Config;
use crate::events;
use crate::draw;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use anyhow::Result;
use std::{
    error::Error,
    io::{self, Stdout},
    sync::{Mutex, Arc},
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
}

impl App {
    fn new(config: &Config) -> Result<App> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;
        Ok(App { terminal })
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
    let mut app = Arc::new(Mutex::new(App::new(config)?));
    let event_recv = events::setup_key_handler();

    let moved_app = app.clone();

    events::handle_notify(moved_app);

    loop {
        if let Ok(events::HGEvent::UserEvent(key_event)) = event_recv.recv() {
            todo!()
        }
    }

    Ok(())
}
