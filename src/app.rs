use crate::config::Config;
use crate::events::{self, HGEvent, NOTIFY, Notify};
use crate::widget::{InputState, ContentState};
use crate::fetch;
use crate::parse;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use crossbeam_channel::Sender;

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

    /// 重绘
    redraw_tx: Sender<HGEvent>,
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
            redraw_tx: NOTIFY.0.clone(),
        })
    }

    fn update(&self) {
        self.redraw_tx.send(HGEvent::NotifyEvent(Notify::Redraw)).unwrap();
    }
}

impl App {
    pub fn handle_char(&mut self, char: char) {
        self.input.active();
        self.input.push_char(char);
        self.update();
    }

    pub fn remove_char(&mut self) {
        self.input.remove_char();
        self.update();
    }

    pub fn search(&mut self) -> Result<()> {
        let wait_search = self.input.clear();
        let text = fetch::search(wait_search)?;
        let projects = parse::parse_search(text)?;
        self.content.add_projects(projects);
        self.update();

        Ok(())
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
