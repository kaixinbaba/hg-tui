use tui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, StatefulWidget, Widget},
};

/// 项目明细
pub struct ProjectDetail {}

#[derive(Debug)]
pub struct ProjectDetailState {}

impl Default for ProjectDetailState {
    fn default() -> ProjectDetailState {
        ProjectDetailState {}
    }
}

impl StatefulWidget for ProjectDetail {
    type State = ProjectDetailState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        Clear.render(area, buf);

        Block::default()
            .borders(Borders::ALL)
            .title("项目详情")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded)
            .render(area, buf);

        let layout = Layout::default()
            .margin(2)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Min(30),
                ]
                .as_ref(),
            )
            .split(area);

        Paragraph::new("URL").render(layout[0], buf);
        Paragraph::new("Star").render(layout[1], buf);
        Paragraph::new("Content").render(layout[2], buf);
    }
}
