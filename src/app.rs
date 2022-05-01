use crate::config::Config;
use crate::events::{self, show_help, warn, Message};
use crate::fetch;
use crate::parse::PARSER;
use crate::widget::content::Category;
use crate::widget::projectdetail::ProjectDetailState;
use crate::widget::{ContentState, InputState, PopupState, StatusLineState};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use anyhow::Result;
use std::{
    io::{self, Stdout},
    sync::{Arc, Mutex},
};

use tui::{backend::CrosstermBackend, Terminal};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SearchMode {
    /// 普通文本搜索
    Normal,

    /// 搜期数
    Volume,

    /// 搜类别
    Category,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    /// 搜索模式
    Search,

    /// 浏览模式
    View,

    /// 弹窗提示
    Popup,

    /// 项目明细
    Detail,
}

pub struct App {
    /// 终端
    pub terminal: Terminal<CrosstermBackend<Stdout>>,

    /// 用户输入框
    pub input: InputState,

    /// 内容展示
    pub content: ContentState,

    /// 弹窗提示
    pub popup: PopupState,

    /// 状态栏
    pub statusline: StatusLineState,

    /// 模式
    pub mode: AppMode,

    /// 当前类别
    pub curr_category: Option<Category>,

    /// 项目明细子页面
    pub project_detail: ProjectDetailState,
}

impl App {
    fn new(_config: &Config) -> Result<App> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        let tsize = terminal.size()?;
        terminal.clear()?;
        Ok(App {
            terminal,
            input: InputState::default(),
            content: ContentState::default(),
            popup: PopupState::new(tsize),
            statusline: StatusLineState::default(),
            mode: AppMode::Search,
            curr_category: None,
            project_detail: ProjectDetailState::default(),
        })
    }
}

impl App {
    pub fn search(&mut self) -> Result<()> {
        if self.input.is_empty() {
            // 输入框为空直接返回
            return Ok(());
        }
        let search_mode = self.input.mode;
        let wait_search = self.input.clear();
        if wait_search == ":help" {
            show_help();
            return Ok(());
        }

        let wait_remove = wait_search.clone();

        let text = fetch::fetch(wait_search, search_mode)?;

        let projects = PARSER.get(&search_mode).unwrap().parse(text)?;
        if projects.is_empty() {
            warn("无结果返回，请确认搜索关键字".into());
            return Ok(());
        }
        let mut category_change = false;

        if search_mode == SearchMode::Category {
            let category = Category::try_from(wait_remove[1..].to_string()).unwrap();
            if let Some(prev_category) = self.curr_category {
                category_change = prev_category != category;
            }
            self.curr_category = Some(category);
        } else {
            self.curr_category = None;
        }

        if category_change {
            self.statusline.set_page_no(1);
        } else {
            self.statusline.set_page_no(
                self.statusline
                    .get_page_no(wait_remove.clone(), search_mode),
            );
        }

        self.content.add_projects(projects);

        // 搜索完自动切换到浏览模式
        self.switch_to_view();

        Ok(())
    }

    pub fn switch_to_view(&mut self) {
        self.input.deactive();
        self.content.active();
        self.mode = AppMode::View;
    }

    pub fn switch_to_search(&mut self) {
        self.content.deactive();
        self.input.active();
        self.mode = AppMode::Search;
    }
    pub fn popup(&mut self, msg: Message) {
        self.popup.msg = msg;
        self.mode = AppMode::Popup;
    }
    pub fn next_page(&mut self) -> Result<()> {
        self.page(self.statusline.page_no() + 1)?;

        Ok(())
    }

    pub fn prev_page(&mut self) -> Result<()> {
        self.page(self.statusline.page_no() - 1)?;

        Ok(())
    }

    fn page(&mut self, page_no: usize) -> Result<()> {
        let text = match self.input.mode {
            SearchMode::Volume => fetch::fetch_volume(page_no),
            SearchMode::Category => fetch::fetch_category(self.curr_category.unwrap(), page_no),
            _ => {
                return Ok(());
            }
        };
        let projects = PARSER.get(&self.input.mode).unwrap().parse(text)?;
        self.content.add_projects(projects);
        self.content.tstate.select(Some(0));
        self.statusline.set_page_no(page_no);
        // 搜索完自动切换到浏览模式
        // self.switch_to_view();

        Ok(())
    }

    pub fn display_detail(&mut self) -> Result<()> {
        self.mode = AppMode::Detail;
        let project = self.content.get_selected();
        self.project_detail = project.into();
        Ok(())
    }

    pub fn open_browser(&self) -> Result<()> {
        let project = self.content.get_selected();
        webbrowser::open(project.url.as_ref())?;
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
        )
        .unwrap();
        self.terminal.show_cursor().unwrap();
    }
}

pub(crate) fn start(config: &Config) -> Result<()> {
    let app = Arc::new(Mutex::new(App::new(config)?));

    let moved_app = app.clone();
    events::handle_key_event(moved_app);

    let moved_app = app.clone();
    events::handle_notify(moved_app);

    Ok(())
}
