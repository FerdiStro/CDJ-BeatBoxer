use crate::app::app::App;
use crate::app::buttons::{Button, SecondControlButton};
use ratatui::layout::Constraint::{Fill, Length};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Color;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

pub fn render_buttons_section(frame: &mut Frame, area: Rect, app: &App) {
    let [button_area, numbers_area, big_button_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Length(3), Length(1), Fill(0)])
        .areas(area);

    render_control_buttons(frame, button_area, app);
    render_number_row(frame, numbers_area, app);
    render_pad_row(frame, big_button_area, app);
}

fn render_pad_row(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 8); 8])
        .split(area);

    SecondControlButton::render_bar_button(SecondControlButton::Bar1, app, frame, chunks[0]);
    SecondControlButton::render_bar_button(SecondControlButton::Bar2, app, frame, chunks[1]);
    SecondControlButton::render_bar_button(SecondControlButton::Bar3, app, frame, chunks[2]);
    SecondControlButton::render_bar_button(SecondControlButton::Bar4, app, frame, chunks[3]);
    SecondControlButton::render_bar_button(SecondControlButton::Bar5, app, frame, chunks[4]);
    SecondControlButton::render_bar_button(SecondControlButton::Bar6, app, frame, chunks[5]);
    SecondControlButton::render_bar_button(SecondControlButton::Bar7, app, frame, chunks[6]);
    SecondControlButton::render_bar_button(SecondControlButton::Bar8, app, frame, chunks[7]);
}

fn render_number_row(frame: &mut Frame, area: Rect, _app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 8); 8])
        .split(area);

    for (i, chunk) in chunks.iter().enumerate() {
        let label = (i + 1).to_string();
        let widget = Paragraph::new(label).centered();
        frame.render_widget(widget, *chunk);
    }
}

fn render_control_buttons(frame: &mut Frame, area: Rect, app: &App) {
    let [button_1, button_2, button_3, midi_area] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Length(8), Length(8), Length(8), Fill(0)])
        .areas(area);

    let [is_lock_active_color, is_lock_inactive_flag_color] = if app.is_lock {
        [Color::Red, Color::Gray]
    } else {
        [Color::White, Color::White]
    };

    SecondControlButton::render_button_color(
        app,
        frame,
        button_1,
        SecondControlButton::BarLock,
        is_lock_active_color,
    );

    SecondControlButton::render_button_color(
        app,
        frame,
        button_2,
        SecondControlButton::PreviousBar,
        is_lock_inactive_flag_color,
    );
    SecondControlButton::render_button_color(
        app,
        frame,
        button_3,
        SecondControlButton::NextBar,
        is_lock_inactive_flag_color,
    );

    app.knobs.draw_midi_knobs(frame, midi_area)
}
