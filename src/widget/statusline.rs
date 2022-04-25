use tui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
};

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
        Paragraph::new("我是状态栏")
            .block(Block::default().borders(Borders::NONE))
            .render(area, buf);
    }
}
