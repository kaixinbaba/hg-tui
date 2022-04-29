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

        // project name
        let project_name_layout = Layout::default()
            .direction(tui::layout::Direction::Horizontal)
            .constraints([Constraint::Percentage(33), Constraint::Percentage(67)].as_ref())
            .split(layout[0]);

        Paragraph::new("🐝 项目名称：")
            .block(Block::default().borders(Borders::ALL))
            .render(project_name_layout[0], buf);

        Paragraph::new("🏁 项目地址：")
            .block(Block::default().borders(Borders::ALL))
            .render(project_name_layout[1], buf);

        // project stars

        let project_stars_layout = Layout::default()
            .direction(tui::layout::Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                    Constraint::Percentage(34),
                ]
                .as_ref(),
            )
            .split(layout[1]);

        Paragraph::new("🌟 Star:")
            .block(Block::default().borders(Borders::ALL))
            .render(project_stars_layout[0], buf);
        Paragraph::new("👀 Watch:")
            .block(Block::default().borders(Borders::ALL))
            .render(project_stars_layout[1], buf);
        Paragraph::new("🌸 Fork:")
            .block(Block::default().borders(Borders::ALL))
            .render(project_stars_layout[2], buf);

        // project desc
        let project_desc_layout = Layout::default()
            .direction(tui::layout::Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(layout[2]);

        Paragraph::new("🍗 简介:")
            .block(Block::default().borders(Borders::ALL))
            .render(project_desc_layout[0], buf);
        Paragraph::new("🎮 图片:")
            .block(Block::default().borders(Borders::ALL))
            .render(project_desc_layout[1], buf);
    }
}
