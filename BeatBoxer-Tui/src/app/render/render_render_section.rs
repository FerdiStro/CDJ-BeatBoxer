use crate::app::app::App;
use crate::app::render::render_wave_form::{render_status_button, render_wave_form};
use ratatui::layout::Constraint::{Fill, Length};
use ratatui::layout::{Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::Block;
use ratatui::Frame;

pub fn render_render_section(frame: &mut Frame, area: Rect, app: &mut App) {
    let [mode_areas, mode_content_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Length(3), Fill(0)])
        .areas(area);

    render_mode_areas(frame, mode_areas, app);

    if app.is_cdj_mode {
        render_content_cdj(frame, mode_content_area, app);
    } else {
        render_content_offline(frame, mode_content_area, app);
    };
}

fn render_content_cdj(frame: &mut Frame, area: Rect, app: &mut App) {
    if let Some(sender) = &app.wave_form_memory.wave_form_cdj_1_terminal_sender {
        let _ = sender.try_send(area.width as usize);
    }
    if let Some(sender) = &app.wave_form_memory.wave_form_cdj_2_terminal_sender {
        let _ = sender.try_send(area.width as usize);
    }

    let [cdj_1_area, cjd_2_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Length(5), Length(5)])
        .areas(area);

    render_wave_form(
        frame,
        cdj_1_area,
        1,
        app.track_ids[0],
        app.amplitudes[0],
        app.gird_colors[0],
    );
    render_wave_form(
        frame,
        cjd_2_area,
        2,
        app.track_ids[1],
        app.amplitudes[1],
        app.gird_colors[1],
    );


    let center_x = area.x + (area.width / 2);

    frame.render_widget(
        Block::default().style(Style::default().bg(Color::Red)),
        Rect {
            x: center_x,
            y: cdj_1_area.y,
            width: 1,
            height: cdj_1_area.height,
        },
    );
    frame.render_widget(
        Block::default().style(Style::default().bg(Color::Red)),
        Rect {
            x: center_x,
            y: cjd_2_area.y,
            width: 1,
            height: cjd_2_area.height,
        },
    );
}

fn render_content_offline(frame: &mut Frame, area: Rect, app: &App) {
    frame.render_widget(Block::default().title("Todo: OFFLINE"), area);
}

fn render_mode_areas(frame: &mut Frame, area: Rect, app: &App) {
    let [cdj_mode, offline_mode] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Length(10), Length(14)])
        .areas(area);

    let cdj_mode_style = if app.is_cdj_mode {
        Style::default().fg(Color::Red)
    } else {
        Style::default().fg(Color::Gray)
    };

    render_status_button(frame, cdj_mode, "CDJ", cdj_mode_style);

    let offline_mode_style = if !app.is_cdj_mode {
        Style::default().fg(Color::Red)
    } else {
        Style::default().fg(Color::Gray)
    };

    render_status_button(frame, offline_mode, "Offline", offline_mode_style);
}
