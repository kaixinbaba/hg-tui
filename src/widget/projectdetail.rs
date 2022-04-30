use tui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, StatefulWidget, Widget},
};

use crate::utils;

use super::content::Project;

/// 项目明细
pub struct ProjectDetail {}

#[derive(Debug, Default)]
pub struct ProjectDetailState {
    name: String,
    url: String,
    star: String,
    watch: String,
    fork: String,
    desc: String,
}

impl From<Project> for ProjectDetailState {
    fn from(project: Project) -> Self {
        Self {
            name: project.name,
            url: project.url,
            star: project.star,
            watch: project.watch,
            fork: project.fork,
            desc: project.desc,
        }
    }
}

impl StatefulWidget for ProjectDetail {
    type State = ProjectDetailState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        Clear.render(area, buf);

        Block::default()
            .borders(Borders::ALL)
            .title(" 项目详情 ")
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

        Paragraph::new(format!("{}", state.name))
            .block(
                Block::default()
                    .title(" 🐝 项目名称 ")
                    .borders(Borders::ALL),
            )
            .render(project_name_layout[0], buf);

        Paragraph::new(format!("{}", state.url))
            .block(
                Block::default()
                    .title(" 🏁 项目地址 ")
                    .borders(Borders::ALL),
            )
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

        Paragraph::new(format!("🌟 Star: {}", state.star))
            .block(Block::default().borders(Borders::ALL))
            .render(project_stars_layout[0], buf);
        Paragraph::new(format!("👀 Watch: {}", state.watch))
            .block(Block::default().borders(Borders::ALL))
            .render(project_stars_layout[1], buf);
        Paragraph::new(format!("🌸 Fork: {}", state.fork))
            .block(Block::default().borders(Borders::ALL))
            .render(project_stars_layout[2], buf);

        let desc = state.desc.clone();
        let desc_wrap = utils::wrap_lines(desc, 114);

        Paragraph::new(desc_wrap)
            .block(
                Block::default()
                    .title(" 🍗 简介 ")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::TOP),
            )
            .render(layout[2], buf);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_open_image() {
        let url = "https://github.com/amodm/webbrowser-rs";
        webbrowser::open(url);
    }
}
