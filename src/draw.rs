use crate::app::App;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::terminal::Frame;
use tui::text::{Span, Spans, Text};
use tui::widgets::{Block, Paragraph};
use tui::style::{Color, Style};


pub fn redraw(app: &mut App) {
    let terminal = &mut app.terminal;

    terminal
        .draw(|f| {
            // layout[0] => title
            // layout[1] => input
            // layout[2] => content
            let layout = Layout::default()
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Max(97),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            f.render_widget(title(), layout[0]);
            f.render_widget(&app.input, layout[1]);
            f.render_widget(&app.content, layout[2]);
        })
        .unwrap();
}

fn title() -> Paragraph<'static> {
    Paragraph::new(
        // Text::from(Spans::from(vec![
        // Span::styled("HelloGiHub", Style::default().fg(Color::Yellow)),
        // Span::raw(""),
        Text::styled("HelloGiHub\n分享 GitHub 上有趣、入门级的开源项目", Style::default().fg(Color::Rgb(255, 192, 102)))
    )
    .alignment(Alignment::Center)
    .block(Block::default())
}


pub fn add_padding(mut rect: Rect, n: u16, direction: PaddingDirection) -> Rect {
    match direction {
        PaddingDirection::Top => {
            rect.y += n;
            rect.height = rect.height.saturating_sub(n);
            rect
        }
        PaddingDirection::Bottom => {
            rect.height = rect.height.saturating_sub(n);
            rect
        }
        PaddingDirection::Left => {
            rect.x += n;
            rect.width = rect.width.saturating_sub(n);
            rect
        }
        PaddingDirection::Right => {
            rect.width = rect.width.saturating_sub(n);
            rect
        }
        PaddingDirection::All => {
            rect.y += n;
            rect.height = rect.height.saturating_sub(n * 2);

            rect.x += n;
            rect.width = rect.width.saturating_sub(n * 2);

            rect
        }
    }
}

#[allow(dead_code)]
pub enum PaddingDirection {
    Top,
    Bottom,
    Left,
    Right,
    All,
}
