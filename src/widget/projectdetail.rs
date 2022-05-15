use tui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, StatefulWidget, Widget},
};

use crate::{app_global::THEME, theme::choose_font_style, utils};

use super::content::{Category, Project};

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
    category: Category,
}

impl From<Project> for ProjectDetailState {
    fn from(project: Project) -> Self {
        let category = if let Ok(category) = Category::try_from(project.category) {
            category
        } else {
            Category::Other
        };

        ProjectDetailState {
            name: project.name,
            url: project.url,
            star: project.star,
            watch: project.watch,
            fork: project.fork,
            desc: project.desc,
            category,
        }
    }
}

impl StatefulWidget for ProjectDetail {
    type State = ProjectDetailState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        Clear.render(area, buf);

        let theme_style = THEME.get().unwrap();
        Block::default()
            .style(theme_style.background)
            .render(area, buf);

        let style = choose_font_style(&state.category, theme_style);

        Block::default()
            .borders(Borders::ALL)
            .title(" 项目详情 ")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded)
            .style(style)
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

        Paragraph::new(state.name.clone())
            .block(
                Block::default()
                    .title(" 🐝 项目名称 ")
                    .borders(Borders::ALL),
            )
            .style(style)
            .render(project_name_layout[0], buf);

        Paragraph::new(state.url.clone())
            .block(
                Block::default()
                    .title(" 🏁 项目地址 ")
                    .borders(Borders::ALL),
            )
            .style(style)
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
            .style(style)
            .render(project_stars_layout[0], buf);
        Paragraph::new(format!("👀 Watch: {}", state.watch))
            .block(Block::default().borders(Borders::ALL))
            .style(style)
            .render(project_stars_layout[1], buf);
        Paragraph::new(format!("🌸 Fork: {}", state.fork))
            .block(Block::default().borders(Borders::ALL))
            .style(style)
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
            .style(style)
            .render(layout[2], buf);
    }
}

#[cfg(test)]
mod test {

    #[test]
    #[ignore]
    fn test_open_image() {
        let url = "https://github.com/amodm/webbrowser-rs";

        assert!(webbrowser::open(url).is_ok());
    }
}
