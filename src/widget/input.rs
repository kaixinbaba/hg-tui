use tui::buffer::Buffer;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::{Spans, Text};
use tui::widgets::{Block, Borders, Paragraph, StatefulWidget, Widget};

use unicode_width::UnicodeWidthStr;

use crate::app::SearchMode;

/// 用户输入框组件
pub struct Input {}

#[derive(Debug)]
pub struct InputState {
    input: String,
    active: bool,
    pub mode: SearchMode,
}

impl Default for InputState {
    fn default() -> InputState {
        InputState {
            input: String::default(),
            active: true,
            mode: SearchMode::Normal,
        }
    }
}

impl InputState {
    pub fn active(&mut self) {
        self.active = true;
    }

    pub fn deactive(&mut self) {
        self.active = false;
    }

    pub fn width(&self) -> u16 {
        self.input.width() as u16
    }

    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }

    pub fn clear(&mut self) -> String {
        let content = self.input.clone();
        self.input.clear();
        content
    }

    pub fn push_str(&mut self, s: &str) {
        self.input.push_str(s);
    }

    pub fn handle_char(&mut self, char: char) {
        if self.input.is_empty() {
            // 说明当前 char 是第一个字符
            self.mode = match char {
                '#' => SearchMode::Volume,
                '$' => SearchMode::Category,
                _ => SearchMode::Normal,
            }
        }
        self.input.push(char);
    }

    pub fn handle_backspace(&mut self) {
        self.input.pop();
    }
}

impl StatefulWidget for Input {
    type State = InputState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let style = if state.active {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        Paragraph::new(state.input.as_ref())
            .block(Block::default().borders(Borders::ALL).style(style))
            .render(area, buf);
    }
}
