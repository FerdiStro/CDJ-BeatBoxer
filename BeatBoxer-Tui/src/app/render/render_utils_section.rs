use crate::app::app::App;
use ratatui::layout::Rect;
use ratatui::widgets::Block;
use ratatui::Frame;

pub fn render_utils_section(frame: &mut Frame, area: Rect, app: &App) {
    let message = "Debug: ".to_string() + &app.debug_message;

    // let message    =  app.beat_sequence[1]

    frame.render_widget(Block::default().title(message), area);
}
