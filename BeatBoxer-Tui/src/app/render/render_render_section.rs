use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::Block;
use crate::app::app::App;

pub fn render_render_section(frame: &mut Frame, area: Rect, app: &App) {
    let message = "Debug: ".to_string() + &app.debug_message;


    frame.render_widget(Block::default().title(message), area);

}
