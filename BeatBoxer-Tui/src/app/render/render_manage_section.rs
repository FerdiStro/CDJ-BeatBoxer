use crate::app::app::App;
use crate::app::buttons::{Button, First_Control_Button};
use crate::app::render::render::Render;
use ratatui::layout::Constraint::Ratio;
use ratatui::layout::{Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Paragraph};
use ratatui::Frame;

pub fn render_manage_section(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::bordered().title("Manage-Section");
    frame.render_widget(block.clone(), area);

    let inner_area = block.inner(area);

    let [control_section, browse_section] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Ratio(1, 8), Ratio(7,8)])
        .areas(inner_area);

    render_controll_Section(frame, control_section, app);
}

fn render_controll_Section(frame: &mut Frame, control_section: Rect, app: &App) {
    let [bpm, adjust_section, master_button] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Ratio(1, 3), Ratio(1, 3), Ratio(1, 3)])
        .areas(control_section);

    let is_master_color = if app.is_master {
        Color::Red
    } else {
        Color::White
    };

    //BPM
    let rounded_bpm_string = ((app.bpm * 100.0).round() / 100.0).to_string();
    let bpm_paragraph =
        Paragraph::new(rounded_bpm_string).style(Style::default().fg(is_master_color));

    frame.render_widget(bpm_paragraph, Render::center_vertically(bpm, 1, 1));

    //Adjust buttons
    let [minus_button_area, plus_button_area] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Ratio(1, 2), Ratio(1, 2)])
        .areas(adjust_section);

    Button::render_button(
        app,
        frame,
        minus_button_area,
        First_Control_Button::DecreaseBpm,
    );
    Button::render_button(
        app,
        frame,
        plus_button_area,
        First_Control_Button::IncreaseBpm,
    );

    //Master Button
    Button::render_button_color(
        app,
        frame,
        master_button,
        First_Control_Button::BecomeMaster,
        is_master_color
    );
}
