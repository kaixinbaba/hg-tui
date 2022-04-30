use std::fmt::format;

use image::{DynamicImage, GenericImageView, Rgba};
use tui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, StatefulWidget, Widget},
};

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
    image: Option<String>,
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
            image: project.image,
        }
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

        Paragraph::new(format!("🐝 项目名称：{}", state.name))
            .block(Block::default().borders(Borders::ALL))
            .render(project_name_layout[0], buf);

        Paragraph::new(format!("🏁 项目地址：{}", state.url))
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

        Paragraph::new(format!("🌟 Star: {}", state.star))
            .block(Block::default().borders(Borders::ALL))
            .render(project_stars_layout[0], buf);
        Paragraph::new(format!("👀 Watch: {}", state.watch))
            .block(Block::default().borders(Borders::ALL))
            .render(project_stars_layout[1], buf);
        Paragraph::new(format!("🌸 Fork: {}", state.fork))
            .block(Block::default().borders(Borders::ALL))
            .render(project_stars_layout[2], buf);

        // project desc
        let project_desc_layout = Layout::default()
            .direction(tui::layout::Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(layout[2]);

        let text = vec![
            Spans::from(vec![Span::raw(state.desc.as_str())]),
            // Spans::from(Span::styled("Second line", Style::default().fg(Color::Red))),
        ];

        let desc_wrap = sub_strings(state.desc.clone(), 35);

        Paragraph::new(desc_wrap)
            .block(
                Block::default()
                    .title("🍗 简介:")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL),
            )
            .render(project_desc_layout[0], buf);

        Paragraph::new(vec![Spans::from(Span::raw("暂无图片"))])
            // Paragraph::new(image)
            .block(
                Block::default()
                    .title("🎮 图片:")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL),
            )
            .render(project_desc_layout[1], buf);
    }
}

// Splits a string into a vector of strings to appeal to a width (used for word wrap)
pub fn sub_strings<'a>(string: String, split_len: usize) -> Vec<Spans<'a>> {
    // Case if "" is passed
    if string.len() == 0 {
        return vec![Spans::from(Span::raw(""))];
    }
    let mut subs: Vec<Spans> = Vec::with_capacity(string.len() / split_len);
    let mut iter = string.chars();
    let mut pos = 0;

    while pos < string.len() {
        let mut len = 0;
        for ch in iter.by_ref().take(split_len) {
            len += ch.len_utf8();
        }
        subs.push(Spans::from(Span::raw(
            (&string[pos..pos + len]).to_string(),
        )));
        pos += len;
    }
    subs
}

pub fn image_fit_size(img: &DynamicImage, term_w: u32, term_h: u32) -> (u32, u32) {
    let (img_width, img_height) = img.dimensions();
    let (w, h) = get_dimensions(img_width, img_height, term_w, term_h);
    let h = if h == term_h { h - 1 } else { h };
    (w, h)
}

pub fn get_dimensions(width: u32, height: u32, bound_width: u32, bound_height: u32) -> (u32, u32) {
    let bound_height = 2 * bound_height;

    if width <= bound_width && height <= bound_height {
        return (width, std::cmp::max(1, height / 2 + height % 2));
    }

    let ratio = width * bound_height;
    let nratio = bound_width * height;

    let use_width = nratio <= ratio;
    let intermediate = if use_width {
        height * bound_width / width
    } else {
        width * bound_height / height
    };

    if use_width {
        (bound_width, std::cmp::max(1, intermediate / 2))
    } else {
        (intermediate, std::cmp::max(1, bound_height / 2))
    }
}

#[cfg(test)]
mod test {
    use std::io::{BufRead, BufReader};

    use image::{GenericImage, ImageBuffer};

    use super::*;

    #[test]
    fn test_open_image() {
        let url = "https://github.com/amodm/webbrowser-rs";
        webbrowser::open(url);
    }
}
