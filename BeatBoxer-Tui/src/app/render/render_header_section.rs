use crate::app::app::App;
use crate::app::buttons::{Button, FirstControlButton};
use crate::app::render::render::Render;
use ratatui::layout::Constraint::Ratio;
use ratatui::layout::{Direction, Layout, Rect};
use ratatui::prelude::{Color, Style};
use ratatui::widgets::{Block, Paragraph};
use ratatui::Frame;

pub fn render_header_section(frame: &mut Frame, area: Rect, app: &App) {
    //frame
    let block = Block::bordered().title("Header");
    frame.render_widget(block.clone(), area);

    let inner_area = block.inner(area);
    let header_content = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Ratio(2, 8), Ratio(1, 8), Ratio(5, 8)]);

    let [setting_area, counter_container, _rest] = header_content.areas(inner_area);

    // Settings
    FirstControlButton::render_button(app, frame, setting_area, FirstControlButton::Settings);

    // Small-counter
    let small_counter_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Ratio(1, 4), Ratio(1, 4), Ratio(1, 4), Ratio(1, 4)]);

    let [count_0, count_1, count_2, count_3] = small_counter_layout.areas(counter_container);

    let areas = [count_0, count_1, count_2, count_3];
    for (i, area) in areas.into_iter().enumerate() {
        let is_active = i == (app.small_counter as usize);
        let (symbol, color) = if is_active {
            ("⬤", Color::Red)
        } else {
            ("⬤", Color::White)
        };
        frame.render_widget(
            Paragraph::new(symbol)
                .style(Style::default().fg(color))
                .centered(),
            Render::center_vertically(area, 1, 0),
        );
    }
}
