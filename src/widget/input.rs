use tui::buffer::Buffer;
use tui::layout::{Constraint, Rect, Layout, Direction};
use tui::widgets::{Block, Borders, Paragraph, StatefulWidget, Widget};

/// 用户输入框组件
pub struct Input {}

#[derive(Debug, Default)]
pub struct InputState {
    input: String,
}



impl StatefulWidget for Input {
    type State = InputState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let input_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(40),
                Constraint::Percentage(30),
            ].as_ref()).split(area);
        Paragraph::new(state.input.as_ref())
            .block(Block::default().borders(Borders::ALL))
            .render(input_layout[1], buf);

    }
}
