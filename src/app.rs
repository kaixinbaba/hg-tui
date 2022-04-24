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

#[derive(Debug, Clone, Copy)]
pub enum AppMode {
    /// 搜索模式
    Search,

    /// 浏览模式
    View,
}

pub struct App {
    /// 终端
    pub terminal: Terminal<CrosstermBackend<Stdout>>,

    /// 用户输入框
    pub input: InputState,

    /// 内容展示
    pub content: ContentState,

    /// 重绘
    redraw_tx: Sender<HGEvent>,

    /// 模式
    pub mode: AppMode,

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
            mode: AppMode::Search,
        })
    }
}

impl App {

    pub fn search(&mut self) -> Result<()> {
        if self.input.is_empty() {
            // 输入框为空直接返回
            return Ok(());
        }
        let wait_search = self.input.clear();
        let text = fetch::search(wait_search)?;
        let projects = parse::parse_search(text)?;
        self.content.add_projects(projects);

        Ok(())
    }


    pub fn switch_to_view(&mut self) -> Result<()> {
        self.input.deactive();
        self.content.active();
        self.mode = AppMode::View;

        Ok(())
    }

    pub fn switch_to_search(&mut self) -> Result<()> {
        self.content.deactive();
        self.input.active();
        self.mode = AppMode::Search;

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
    let event_recv = events::handle_key_event(moved_app);

    let moved_app = app.clone();
    events::handle_notify(moved_app);

    Ok(())
}
