use crate::config::Config;
use crate::events;
use crate::widget::{InputState, ContentState};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use anyhow::Result;
use std::{
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
    /// 终端
    pub terminal: Terminal<CrosstermBackend<Stdout>>,

    /// 用户输入框
    pub input: InputState,

    /// 内容展示
    pub content: ContentState,
}

impl App {
    fn new(config: &Config) -> Result<App> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;
        Ok(App {
            terminal,
            input: InputState::default(),
            content: ContentState::default(),
        })
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


    let moved_app = app.clone();
    events::handle_notify(moved_app);

    let moved_app = app.clone();
    let event_recv = events::handle_key_event(moved_app);

    Ok(())
}
