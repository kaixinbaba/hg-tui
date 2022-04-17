use tui::buffer::Buffer;
use tui::layout::{Constraint, Rect, Layout, Direction};
use tui::widgets::{Block, Borders, Paragraph, Widget};

/// 用户输入框组件
#[derive(Debug, Default)]
pub struct Input {
    input: String,
}

impl Widget for &Input {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let input_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(40),
                Constraint::Percentage(30),
            ].as_ref()).split(area);
        Paragraph::new(self.input.as_ref())
            .block(Block::default().borders(Borders::ALL))
            .render(input_layout[1], buf);
    }
}
