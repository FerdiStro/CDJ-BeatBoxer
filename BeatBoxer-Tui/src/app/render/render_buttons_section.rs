use crate::app::app::App;
use ratatui::layout::Constraint::{Max, Min, Ratio};
use ratatui::layout::{Direction, Layout, Rect};
use ratatui::widgets::Block;
use ratatui::Frame;

pub fn render_buttons_section(frame: &mut Frame, area: Rect, app: &App) {
    frame.render_widget(Block::bordered().title("Buttons"), area);

    let [button_area, numbers_area, big_button_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Ratio(1, 8), Ratio(1, 8), Ratio(6, 8)])
        .areas(area);

    //Button Area
    render_control_buttons(frame, button_area, app);
}

fn render_control_buttons(frame: &mut Frame, area: Rect, app: &App) {
    // Layout::default()
    //     .direction(Direction::Horizontal)
    //     .constraints([
    //         Min(0)
    //     ])


    // Button::render_button(app, frame, area )



}
