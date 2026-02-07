use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::Block;
use crate::app::app::App;

pub fn render_manage_section(frame: &mut Frame, area: Rect, app: &App){
    frame.render_widget(Block::bordered().title("Manage-Section"), area);
}