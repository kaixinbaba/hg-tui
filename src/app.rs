use crate::app_global::HG_INFO;
use crate::config::Config;
use crate::events::{self, warn, Message};
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
use std::fs::File;
use std::path::Path;
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

    /// 是否要显示帮助
    pub show_help: bool,
}

impl App {
    fn new(config: &Config) -> Result<App> {
        let show_help = Self::init_config(config.config_path.clone())? || config.show_help;

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
            popup: PopupState::default(),
            statusline: StatusLineState::default(),
            mode: AppMode::Search,
            curr_category: None,
            project_detail: ProjectDetailState::default(),
            show_help,
        })
    }

    /// 初始化配置文件
    fn init_config(config_path: String) -> Result<bool> {
        let path = Path::new(&config_path).join(".hgtui.toml");
        if path.exists() {
            return Ok(false);
        }
        if File::create(&path).is_ok() {
            return Ok(true);
        }
        Ok(false)
    }
}

impl App {
    pub fn search(&mut self, wait_search: Option<String>) -> Result<()> {
        if self.input.is_empty() && wait_search.is_none() {
            // 输入框为空直接返回
            return Ok(());
        }
        let search_mode = self.input.mode;

        let wait_search = wait_search.unwrap_or_else(|| self.input.clear());

        let wait_remove = wait_search.clone();

        let text = fetch::fetch(wait_search, search_mode)?;

        let (projects, last_parse) = PARSER.get(&search_mode).unwrap().parse(text)?;
        if projects.is_empty() {
            warn("无结果返回，请确认搜索关键字".into());
            return Ok(());
        }

        let wait_remove = match last_parse {
            crate::parse::LastParse::Volume(v) => {
                let mut volume = v
                    .split(' ')
                    .into_iter()
                    .collect::<Vec<&str>>()
                    .get(1)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();

                if volume > HG_INFO.max_volume {
                    volume = HG_INFO.max_volume;
                }

                format!("#{}", volume)
            }
            _ => wait_remove,
        };

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
            self.statusline
                .set_page_no(self.statusline.get_page_no(wait_remove, search_mode));
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

    fn page(&mut self, mut page_no: usize) -> Result<()> {
        let text = match self.input.mode {
            SearchMode::Volume => {
                if page_no > HG_INFO.max_volume {
                    page_no = HG_INFO.max_volume;
                }
                fetch::fetch_volume(page_no)
            }
            SearchMode::Category => fetch::fetch_category(self.curr_category.unwrap(), page_no),
            _ => {
                return Ok(());
            }
        };
        let (projects, _) = PARSER.get(&self.input.mode).unwrap().parse(text)?;
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

    pub fn open_browser(&self, url: Option<&str>) -> Result<()> {
        let project = self.content.get_selected();
        let url = url.unwrap_or_else(|| project.url.as_ref());
        webbrowser::open(url)?;
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

    events::handle_notify(app);

    Ok(())
}
