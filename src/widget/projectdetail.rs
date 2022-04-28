use tui::{buffer::Buffer, layout::Rect, widgets::StatefulWidget};

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
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {}
}
