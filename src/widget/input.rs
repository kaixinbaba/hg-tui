use tui::buffer::Buffer;
use tui::layout::{Constraint, Rect, Layout, Direction};
use tui::widgets::{Block, Borders, Paragraph, StatefulWidget, Widget};
use tui::style::{Style, Color};
use crossbeam_channel::Sender;


use crate::events::{HGEvent, NOTIFY, Notify};

/// 用户输入框组件
pub struct Input {}

#[derive(Debug)]
pub struct InputState {
    input: String,
    active: bool,
}

impl Default for InputState {
    fn default() -> InputState {
        InputState {
            input: String::default(),
            active: false,
        }

    }
}

impl InputState {
    pub fn active(&mut self) {
        self.active = true;
    }

    pub fn clear(&mut self) -> String {
        let content = self.input.clone();
        self.input.clear();
        content
    }

    pub fn push_char(&mut self, char: char) {
        self.input.push(char);
    }

    pub fn remove_char(&mut self) {
        self.input.pop();
    }
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

        let style = if state.active {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        Paragraph::new(state.input.as_ref())
            .block(Block::default().borders(Borders::ALL).style(style))
            .render(input_layout[1], buf);

    }
}
