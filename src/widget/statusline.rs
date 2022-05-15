use chrono::prelude::*;
use tui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
};

use crate::{
    app::SearchMode,
    app_global::{HG_INFO, THEME},
};

/// 状态栏
pub struct StatusLine {}

#[derive(Debug)]
pub struct StatusLineState {
    pub mode: SearchMode,
    page_no: usize,
}

impl Default for StatusLineState {
    fn default() -> StatusLineState {
        StatusLineState {
            mode: SearchMode::Normal,
            page_no: 1,
        }
    }
}

impl StatusLineState {
    pub fn page_no(&self) -> usize {
        self.page_no
    }

    pub fn get_page_no(&self, wait_remove: String, search_mode: SearchMode) -> usize {
        match search_mode {
            SearchMode::Normal => 1,
            SearchMode::Volume => wait_remove[1..].parse::<usize>().unwrap(),
            SearchMode::Category => self.page_no,
        }
    }

    pub fn set_page_no(&mut self, page_no: usize) {
        if page_no < 1 {
            self.page_no = 1;
        } else if page_no > HG_INFO.max_volume {
            self.page_no = HG_INFO.max_volume;
        } else {
            self.page_no = page_no;
        }
    }

    pub fn set_mode(&mut self, mode: SearchMode) {
        if self.mode != mode {
            // 有改变
            self.mode = mode;
            self.page_no = 1;
        }
    }
}

impl StatefulWidget for StatusLine {
    type State = StatusLineState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let theme_style = THEME.get().unwrap();

        let layout = Layout::default()
            .direction(tui::layout::Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(40),
                    Constraint::Percentage(20),
                    Constraint::Percentage(40),
                ]
                .as_ref(),
            )
            .split(area);

        // clock layout[2]
        let now = Local::now();
        Paragraph::new(format!(
            " ⏰ {} 🌟 {} 📚项目数 {} 个",
            now.format("%Y-%m-%d %H:%M:%S"),
            HG_INFO.star,
            HG_INFO.project_count
        ))
        .style(theme_style.tips)
        .block(
            Block::default()
                .borders(Borders::LEFT)
                .border_type(tui::widgets::BorderType::Double),
        )
        .render(layout[2], buf);

        // info layout[1]
        //
        //
        let text = match state.mode {
            SearchMode::Normal => "搜索模式".into(),
            SearchMode::Volume => format!("⇦ h   第 {} 期   l ⇨", state.page_no),
            SearchMode::Category => format!("⇦ h   第 {} 页   l ⇨", state.page_no),
        };

        Paragraph::new(text)
            .block(Block::default().borders(Borders::NONE))
            .style(theme_style.title)
            .alignment(tui::layout::Alignment::Center)
            .render(layout[1], buf);

        // time layout[0]
        // "输入:help 或按 ctrl h 查看帮助"
        Paragraph::new(Spans::from(vec![
            Span::styled(" 按", theme_style.tips),
            Span::styled(" ctrl h", theme_style.key),
            Span::styled(" 查看帮助 按", theme_style.tips),
            Span::styled(" q", theme_style.key),
            Span::styled(" 键退出", theme_style.tips),
        ]))
        .block(
            Block::default()
                .borders(Borders::RIGHT)
                .border_type(tui::widgets::BorderType::Double),
        )
        .render(layout[0], buf);
    }
}
