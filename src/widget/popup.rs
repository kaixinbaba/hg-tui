use tui::buffer::Buffer;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Style};

use tui::widgets::{Block, Borders, Clear, Paragraph, StatefulWidget, Widget};

///
///
/// 提示弹窗
pub struct Popup {}

#[derive(Debug, Default)]
pub struct PopupState {
    pub msg: String,
}

impl StatefulWidget for Popup {
    type State = PopupState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        Clear.render(area, buf);
        Paragraph::new(format!(
            "\n{}\n\n\n☟ 按任何键关闭窗口...",
            state.msg.clone()
        ))
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .style(Style::default().bg(Color::DarkGray))
                .title(" ✖ 报错啦 ✖ ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL),
        )
        .render(area, buf);
    }
}
