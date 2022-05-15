use tui::buffer::Buffer;
use tui::layout::{Alignment, Rect};
use tui::style::Color;

use tui::text::Span;
use tui::widgets::{Block, Borders, Clear, Paragraph, StatefulWidget, Widget};

use crate::app_global::THEME;
use crate::events::Message;

///
///
/// 提示弹窗
pub struct Popup {}

#[derive(Debug, Default)]
pub struct PopupState {
    pub msg: Message,
}

impl StatefulWidget for Popup {
    type State = PopupState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        Clear.render(area, buf);

        let theme_style = THEME.get().unwrap();

        let (title, style, msg) = match &state.msg {
            Message::Error(msg) => {
                let style = theme_style.background.fg(Color::Red);
                let title = Span::styled(" ✖ 报错啦 ✖ ", style);
                (title, style, msg)
            }
            Message::Warn(msg) => {
                let style = theme_style.background.fg(Color::Yellow);
                let title = Span::styled(" ⚠️ 警告 ", style);
                (title, style, msg)
            }
            Message::Tips(msg) => {
                let style = theme_style.background.fg(Color::DarkGray);
                let title = Span::styled(" ✧ 提示 ✧ ", style);
                (title, style, msg)
            }
        };

        let block = Block::default()
            .style(style)
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
