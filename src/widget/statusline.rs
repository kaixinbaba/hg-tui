use std::{sync::atomic::AtomicUsize, time::Duration};

use chrono::prelude::*;
use tui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
};

use crate::app::SearchMode;

/// 状态栏
pub struct StatusLine {}

#[derive(Debug)]
pub struct StatusLineState {
    page_no: usize,
}

impl Default for StatusLineState {
    fn default() -> StatusLineState {
        StatusLineState { page_no: 1 }
    }
}

impl StatusLineState {
    pub fn page_no(&self) -> usize {
        self.page_no
    }

    pub fn get_page_no(&self, wait_remove: String, search_mode: SearchMode) -> usize {
        let page_no = match search_mode {
            SearchMode::Normal => 1,
            SearchMode::Volume => wait_remove[1..].parse::<usize>().unwrap(),
            SearchMode::Category => self.page_no,
        };
        page_no
    }

    pub fn set_page_no(&mut self, page_no: usize) {
        if page_no < 1 {
            self.page_no = 1;
        } else {
            self.page_no = page_no;
        }
    }
}

impl StatefulWidget for StatusLine {
    type State = StatusLineState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let layout = Layout::default()
            .direction(tui::layout::Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(25),
                    Constraint::Percentage(50),
                    Constraint::Percentage(25),
                ]
                .as_ref(),
            )
            .split(area);

        // clock layout[0]
        let now = Local::now();
        Paragraph::new(format!(
            "  ⏰ {}",
            now.format("%Y-%m-%d %H:%M:%S").to_string()
        ))
        .style(Style::default().fg(Color::LightYellow))
        .block(Block::default().borders(Borders::RIGHT))
        .render(layout[0], buf);

        // info layout[1]
        Paragraph::new(format!("第 {} 页", state.page_no))
            .block(Block::default().borders(Borders::NONE))
            .alignment(tui::layout::Alignment::Center)
            .render(layout[1], buf);

        // time layout[2]
        // "输入:help 或按 ctrl h 查看帮助"
        Paragraph::new(Spans::from(vec![
            Span::raw("输入"),
            Span::styled(":help", Style::default().fg(Color::Green)),
            Span::raw(" 或 "),
            Span::styled("ctrl h", Style::default().fg(Color::Green)),
            Span::raw(" 查看帮助"),
        ]))
        .block(Block::default().borders(Borders::LEFT))
        .render(layout[2], buf);
    }
}
