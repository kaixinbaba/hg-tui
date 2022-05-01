use tui::buffer::Buffer;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Style};

use tui::text::Span;
use tui::widgets::{Block, Borders, Clear, Paragraph, StatefulWidget, Widget};

use crate::events::Message;

///
///
/// 提示弹窗
pub struct Popup {}

#[derive(Debug, Default)]
pub struct PopupState {
    pub msg: Message,
    pub size: Rect,
}

impl PopupState {
    pub fn new(size: Rect) -> PopupState {
        PopupState {
            msg: Message::default(),
            size,
        }
    }
}

impl StatefulWidget for Popup {
    type State = PopupState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        Clear.render(area, buf);

        let (title, style, msg) = match &state.msg {
            Message::Error(msg) => {
                let style = Style::default().fg(Color::Red);
                let title = Span::styled(" ✖ 报错啦 ✖ ", style);
                (title, style, msg)
            }
            Message::Warn(msg) => {
                let style = Style::default().fg(Color::Yellow);
                let title = Span::styled(" ⚠️ 警告 ", style);
                (title, style, msg)
            }
            Message::Tips(msg) => {
                let style = Style::default().fg(Color::White);
                let title = Span::styled(" ✧ 提示 ✧ ", style);
                (title, style, msg)
            }
        };

        let block = Block::default()
            .style(style.bg(Color::DarkGray))
            .title_alignment(Alignment::Center)
            .title(title)
            .borders(Borders::ALL);

        Paragraph::new(format!("\n{}\n\n\n☟ 按任何键关闭窗口...", msg))
            .alignment(Alignment::Center)
            .style(style)
            .block(block)
            .render(area, buf);
    }
}
