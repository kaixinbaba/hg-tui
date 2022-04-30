use anyhow::bail;
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
use crate::parse::CategoryParser;
use crate::theme::{CATEGORY_STYLE, TITLE_STYLE};

use super::projectdetail;

const TABLE_TITLE: &'static str = " 搜索结果 ";

const SELECT_ARROW: &'static str = "►";

lazy_static! {
    static ref HEADERS: Vec<&'static str> = vec!["№", "名称", "期数", "分类", "介绍"];
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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
    Kotlin,
    MachineLearning,
    Book,
    Other,
}

impl Default for Category {
    fn default() -> Self {
        Self::Java
    }
}

impl TryFrom<String> for Category {
    type Error = anyhow::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let lower = s.to_lowercase();
        let category = match lower.as_ref() {
            "java" => Category::Java,
            "python" | "py" => Category::Python,
            "javascript" | "js" => Category::Javascript,
            "rust" => Category::Rust,
            "c" => Category::C,
            "c++" | "cpp" => Category::Cpp,
            "php" => Category::PHP,
            "objectc" | "oc" => Category::ObjectC,
            "go" => Category::Go,
            "css" => Category::Css,
            "c#" | "cs" => Category::Csharp,
            "kotlin" => Category::Kotlin,
            "swift" => Category::Swift,
            "ml" | "ai" => Category::MachineLearning,
            "ruby" => Category::Ruby,
            "book" => Category::Book,
            "other" => Category::Other,
            _ => bail!("请输入有效的类别名称，如：java, py, js, go 等"),
        };
        Ok(category)
    }
}

// impl From<String> for Category {
//     fn from(content: String) -> Self {
//         match content {
//             "Java" => Category::Java,
//             "Python" => Category::Python,
//             "Javascript" => Category::Javascript,
//             "Rust" => Category::Rust,
//             "C" => Category::C,
//             "C++" => Category::Cpp,
//             "PHP" => Category::PHP,
//             "Object-C" => Category::ObjectC,
//             "Go" => Category::Go,
//             "Css" => Category::Css,
//             "C#" => Category::Csharp,
//             "Koltin" => Category::Koltin,
//             "Swift" => Category::Swift,
//             "机器学习" => Category::MachineLearning,
//             "Ruby" => Category::Ruby,
//             "开源书籍" => Category::Book,
//             _ => Category::Other,
//         }
//     }
// }

impl From<Category> for String {
    fn from(category: Category) -> String {
        match category {
            Category::Java => "Java".into(),
            Category::Python => "Python".into(),
            Category::Javascript => "Javascript".into(),
            Category::Rust => "Rust".into(),
            Category::C => "C".into(),
            Category::Cpp => "C++".into(),
            Category::PHP => "PHP".into(),
            Category::ObjectC => "Object-C".into(),
            Category::Go => "Go".into(),
            Category::Css => "Css".into(),
            Category::Csharp => "C#".into(),
            Category::Kotlin => "Kotlin".into(),
            Category::Swift => "Swift".into(),
            Category::MachineLearning => "机器学习".into(),
            Category::Ruby => "Ruby".into(),
            Category::Book => "开源书籍".into(),
            Category::Other => "其他".into(),
        }
    }
}

impl Category {
    pub fn to_zh(&self) -> String {
        match self {
            Category::Java => "Java 项目".into(),
            Category::Python => "Python 项目".into(),
            Category::Javascript => "Javascript 项目".into(),
            Category::Rust => "Rust 项目".into(),
            Category::C => "C 项目".into(),
            Category::Cpp => "C++ 项目".into(),
            Category::PHP => "PHP 项目".into(),
            Category::ObjectC => "Object-C 项目".into(),
            Category::Go => "Go 项目".into(),
            Category::Css => "Css 项目".into(),
            Category::Csharp => "C# 项目".into(),
            Category::Kotlin => "Kotlin 项目".into(),
            Category::Swift => "Swift 项目".into(),
            Category::MachineLearning => "机器学习".into(),
            Category::Ruby => "Ruby 项目".into(),
            Category::Book => "开源书籍".into(),
            Category::Other => "其他".into(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Project {
    /// 项目名称
    pub name: String,

    /// 期数
    pub volume: String,

    /// 种类
    pub category: String,

    /// github http url
    pub url: String,

    /// 描述
    pub desc: String,

    /// star 数
    pub star: String,

    /// watch 数
    pub watch: String,

    /// fork 数
    pub fork: String,
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
            category: category.into().replace(" 项目", ""),
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
    active: bool,
    pub tstate: TableState,
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

    pub fn next(&mut self, incr: usize) {
        let cur = match self.tstate.selected() {
            Some(index) => index,
            None => 0,
        };
        let next = if cur + incr >= self.cur.len() - 1 {
            self.cur.len() - 1
        } else {
            cur + incr
        };
        self.tstate.select(Some(next));
    }

    pub fn prev(&mut self, incr: usize) {
        let cur = match self.tstate.selected() {
            Some(index) => index,
            None => 0,
        };

        let next = if cur < incr { 0 } else { cur - incr };
        self.tstate.select(Some(next));
    }

    pub fn first(&mut self) {
        self.tstate.select(Some(0));
    }

    pub fn last(&mut self) {
        self.tstate.select(Some(self.cur.len() - 1));
    }

    pub fn get_selected(&self) -> Project {
        self.cur
            .get(self.tstate.selected().unwrap())
            .unwrap()
            .clone()
    }
}

impl Default for ContentState {
    fn default() -> ContentState {
        ContentState {
            cur: Vec::default(),
            active: false,
            tstate: TableState::default(),
        }
    }
}

fn new_cell<'a>(symbol: impl ToString, style: Style) -> Cell<'a> {
    Cell::from(symbol.to_string()).style(style)
}

impl StatefulWidget for Content {
    type State = ContentState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let header_cells = HEADERS.iter().map(|h| Cell::from(*h).style(*TITLE_STYLE));
        let header = Row::new(header_cells)
            // .style(normal_style)
            .height(1)
            .bottom_margin(1);

        let rows = state.cur.iter().enumerate().map(|(i, project)| {
            let mut cells: Vec<Cell> = Vec::with_capacity(5);

            cells.push(new_cell(i + 1, Style::default()));
            cells.push(new_cell(project.name.clone(), Style::default()));
            cells.push(new_cell(project.volume.clone(), Style::default()));

            let category = if let Ok(category) = Category::try_from(project.category.clone()) {
                category
            } else {
                Category::Other
            };

            let color_style = if let Some(color_style) = CATEGORY_STYLE.get(&category) {
                *color_style
            } else {
                Style::default().fg(Color::White)
            };

            cells.push(new_cell(project.category.clone(), Style::default()));
            cells.push(new_cell(project.desc.clone(), Style::default()));

            let style = match state.tstate.selected() {
                Some(index) if index == i => {
                    Style::default().bg(Color::Cyan).fg(Color::Rgb(255, 116, 0))
                }
                _ => color_style,
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
