use std::str::FromStr;

use crate::draw;
use crate::parse::CategoryParser;
use lazy_static::lazy_static;
use tui::buffer::Buffer;
use tui::layout::{Alignment, Constraint, Rect};
use tui::style::{Color, Style};
use tui::text::Span;
use tui::widgets::{
    Block, BorderType, Borders, Cell, Paragraph, Row, StatefulWidget, Table, TableState, Widget,
};

use crossbeam_channel::Sender;

use crate::events::{HGEvent, Notify, NOTIFY};

const TABLE_TITLE: &'static str = " 搜索结果 ";

const SELECT_ARROW: &'static str = "►";

lazy_static! {
    static ref HEADERS: Vec<&'static str> = vec!["№", "名称", "期数", "分类", "介绍"];
}

#[derive(Debug, Clone, Copy)]
pub enum Category {
    Java,
    Python,
    Javascript,
    Rust,
    C,
    Cpp,
    Csharp,
    ObjectC,
    Css,
    Go,
    PHP,
    Ruby,
    Swift,
    Koltin,
    MachineLearning,
    Book,
    Other,
}

impl Default for Category {
    fn default() -> Self {
        Self::Java
    }
}

impl From<Category> for String {
    fn from(category: Category) -> String {
        match category {
            Java => "Java".into(),
            Python => "Python".into(),
            Javascript => "Javascript".into(),
            Rust => "Rust".into(),
            C => "C".into(),
            Cpp => "C++".into(),
            PHP => "PHP".into(),
            ObjectC => "Object-C".into(),
            Go => "Go".into(),
            Css => "Css".into(),
            Csharp => "C#".into(),
            Koltin => "Koltin".into(),
            Swift => "Swift".into(),
            MachineLearning => "机器学习".into(),
            Ruby => "Ruby".into(),
            Book => "开源书籍".into(),
            Other => "其他".into(),
        }
    }
}

impl Category {
    pub fn to_zh(&self) -> String {
        match self {
            Java => "Java 项目".into(),
            Python => "Python 项目".into(),
            Javascript => "Javascript 项目".into(),
            Rust => "Rust 项目".into(),
            C => "C 项目".into(),
            Cpp => "C++ 项目".into(),
            PHP => "PHP 项目".into(),
            ObjectC => "Object-C 项目".into(),
            Go => "Go 项目".into(),
            Css => "Css 项目".into(),
            Csharp => "C# 项目".into(),
            Koltin => "Koltin 项目".into(),
            Swift => "Swift 项目".into(),
            MachineLearning => "机器学习".into(),
            Ruby => "Ruby 项目".into(),
            Book => "开源书籍".into(),
            Other => "其他".into(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Project {
    /// 项目名称
    name: String,

    /// 期数
    volume: String,

    /// 种类
    category: String,

    /// github http url
    url: String,

    /// 描述
    desc: String,

    /// star 数
    star: String,

    /// watch 数
    watch: String,

    /// fork 数
    fork: String,
}

impl Project {
    pub fn new<T>(
        name: T,
        volume: T,
        category: T,
        url: T,
        desc: T,
        star: T,
        watch: T,
        fork: T,
    ) -> Project
    where
        T: Into<String>,
    {
        Project {
            name: name.into(),
            volume: volume.into(),
            category: category.into(),
            url: url.into(),
            desc: desc.into(),
            star: star.into(),
            watch: watch.into(),
            fork: fork.into(),
        }
    }
}

/// 数据表格展示
pub struct Content {}

#[derive(Debug)]
pub struct ContentState {
    /// 当前页数据
    cur: Vec<Project>,
    /// 下一页数据
    next: Option<Vec<Project>>,
    page_num: usize,
    page_size: usize,
    active: bool,
    tstate: TableState,
}

impl ContentState {
    pub fn add_projects(&mut self, mut projects: Vec<Project>) {
        self.cur.clear();
        self.cur.append(&mut projects);
    }

    pub fn active(&mut self) {
        self.active = true;
        if let None = self.tstate.selected() {
            self.tstate.select(Some(0));
        }
    }

    pub fn deactive(&mut self) {
        self.active = false;
    }

    pub fn next(&mut self) {
        let cur = match self.tstate.selected() {
            Some(index) => index,
            None => 0,
        };
        let next = if cur == self.cur.len() - 1 {
            0
        } else {
            cur + 1
        };
        self.tstate.select(Some(next));
    }

    pub fn prev(&mut self) {
        let cur = match self.tstate.selected() {
            Some(index) => index,
            None => 0,
        };
        let next = if cur == 0 {
            self.cur.len() - 1
        } else {
            cur - 1
        };
        self.tstate.select(Some(next));
    }
}

impl Default for ContentState {
    fn default() -> ContentState {
        ContentState {
            cur: Vec::default(),
            next: None,
            page_num: 1,
            page_size: 10,
            active: false,
            tstate: TableState::default(),
        }
    }
}

impl StatefulWidget for Content {
    type State = ContentState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let header_cells = HEADERS
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(Color::Green)));
        let header = Row::new(header_cells)
            // .style(normal_style)
            .height(1)
            .bottom_margin(1);

        let rows = state.cur.iter().enumerate().map(|(i, project)| {
            let mut cells: Vec<String> = Vec::with_capacity(5);

            cells.push((i + 1).to_string());
            cells.push(project.name.clone());
            cells.push(project.volume.to_string());
            cells.push(project.category.clone());
            cells.push(project.desc.clone());

            let style = match state.tstate.selected() {
                Some(index) if index == i => {
                    Style::default().bg(Color::Cyan).fg(Color::Rgb(255, 116, 0))
                }
                _ => Style::default(),
            };

            Row::new(cells).height(1).bottom_margin(2).style(style)
        });

        let table_title = if state.active {
            Span::styled(TABLE_TITLE, Style::default().fg(Color::Yellow))
        } else {
            Span::raw(TABLE_TITLE)
        };

        let table_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title_alignment(Alignment::Center)
            .title(table_title);

        let t = Table::new(rows).header(header).block(table_block).widths(&[
            Constraint::Percentage(3),
            Constraint::Percentage(15),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(62),
        ]);
        <Table as StatefulWidget>::render(t, area, buf, &mut state.tstate)
    }
}
