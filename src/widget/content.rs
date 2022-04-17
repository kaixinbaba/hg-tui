use tui::widgets::{Widget, Paragraph, Block, Borders, BorderType};
use tui::buffer::{Buffer};
use tui::layout::Rect;

/// 数据表格展示
#[derive(Debug)]
pub struct Content {

}

impl Default for Content {
    fn default() -> Content {
        Content {  }
    }
}


impl Widget for &Content {
    fn render(self, area: Rect, buf: &mut Buffer) {

        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .render(area, buf);
    }
}
