use crate::draw;
use lazy_static::lazy_static;
use tui::buffer::Buffer;
use tui::layout::{Alignment, Constraint, Rect};
use tui::style::{Color, Style};
use tui::text::Span;
use tui::widgets::{
    Block, BorderType, Borders, Cell, Paragraph, Row, StatefulWidget, Table, TableState, Widget,
};

const TABLE_TITLE: &'static str = " 搜索结果 ";

lazy_static! {
    static ref HEADERS: Vec<&'static str> = vec!["名称", "期数", "分类", "介绍"];
}

#[derive(Debug, Clone, Copy)]
enum Category {
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
    fn from(category: Category) -> Self {
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

#[derive(Debug, Default)]
struct Project {
    /// 项目名称
    name: String,

    /// 期数
    phase: u8,

    /// 种类
    category: Category,

    /// github http url
    url: String,

    /// 描述
    desc: String,
}

impl Project {
    fn new<T>(name: T, phase: u8, category: T, url: T, desc: T) -> Project
    where
        T: Into<String>,
    {
        Project {
            name: name.into(),
            phase,
            category: Category::Java,
            url: url.into(),
            desc: desc.into(),
        }
    }
}

/// 数据表格展示
pub struct Content {}

#[derive(Debug)]
pub struct ContentState {
    result: Vec<Project>,
    page_num: usize,
    page_size: usize,
    active: bool,
    tstate: TableState,
}

impl Default for ContentState {
    fn default() -> ContentState {
        let mut result = Vec::new();
        result.push(Project::new(
            "name1",
            1,
            "Java",
            "http://www.baidu.com",
            "ajdflkdasjfldaksjfljasdflajsdflsajflsajadslfjalsjflasjdfalj",
        ));
        result.push(Project::new(
            "name2",
            2,
            "Java",
            "http://www.baidu.com",
            "ajdflkdasjfldaksjfljasdflajsdflsajflsajadslfjalsjflasjdfalj",
        ));
        result.push(Project::new(
            "name3",
            3,
            "Java",
            "http://www.baidu.com",
            "ajdflkdasjfldaksjfljasdflajsdflsajflsajadslfjalsjflasjdfalj",
        ));
        result.push(Project::new(
            "name4",
            4,
            "Java",
            "http://www.baidu.com",
            "ajdflkdasjfldaksjfljasdflajsdflsajflsajadslfjalsjflasjdfalj",
        ));
        result.push(Project::new(
            "name5",
            5,
            "Java",
            "http://www.baidu.com",
            "ajdflkdasjfldaksjfljasdflajsdflsajflsajadslfjalsjflasjdfalj",
        ));
        result.push(Project::new(
            "name6",
            6,
            "Java",
            "http://www.baidu.com",
            "ajdflkdasjfldaksjfljasdflajsdflsajflsajadslfjalsjflasjdfalj",
        ));
        result.push(Project::new(
            "name7",
            7,
            "Java",
            "http://www.baidu.com",
            "ajdflkdasjfldaksjfljasdflajsdflsajflsajadslfjalsjflasjdfalj",
        ));
        ContentState {
            result,
            page_num: 1,
            page_size: 10,
            active: true,
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

        let rows = state.result.iter().map(|project| {
            let mut cells: Vec<String> = Vec::with_capacity(4);

            cells.push(project.name.clone());
            cells.push(project.phase.to_string());
            cells.push(project.category.into());
            cells.push(project.desc.clone());

            let style = if let Some(index) = state.tstate.selected() {
                Style::default().bg(Color::Cyan).fg(Color::Rgb(255, 116, 0))
            } else {
                Style::default()
            };

            Row::new(cells).bottom_margin(1).style(style)
        });

        let table_title = if state.active {
            Span::styled(TABLE_TITLE, Style::default().fg(Color::Yellow))
        } else {
            Span::raw(TABLE_TITLE)
        };

        let mut table_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title_alignment(Alignment::Center)
            .title(table_title);

        let t = Table::new(rows).header(header).block(table_block).widths(&[
            Constraint::Percentage(15),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(65),
        ]);
        <Table as StatefulWidget>::render(t, area, buf, &mut state.tstate)
    }
}
