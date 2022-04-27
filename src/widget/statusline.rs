use std::sync::atomic::AtomicUsize;

use chrono::prelude::*;
use tui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
};

static GLOBAL_THREAD_COUNT: AtomicUsize = AtomicUsize::new(0);

/// 状态栏
pub struct StatusLine {}

#[derive(Debug)]
pub struct StatusLineState {}

impl Default for StatusLineState {
    fn default() -> StatusLineState {
        StatusLineState {}
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
        .block(Block::default().borders(Borders::RIGHT))
        .render(layout[0], buf);

        // info layout[1]
        Paragraph::new("我是信息")
            .block(Block::default().borders(Borders::NONE))
            .render(layout[1], buf);

        // time layout[2]
        Paragraph::new("我是时长")
            .block(Block::default().borders(Borders::LEFT))
            .render(layout[2], buf);
    }
}
