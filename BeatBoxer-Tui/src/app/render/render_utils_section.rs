use crate::app::app::{App, SoundBar};
use crate::app::buttons::Button;
use crate::app::render::render::Render;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Style};
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use std::path::Path;

pub fn render_utils_section(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 8); 8])
        .split(area);

    for (i, chunk) in chunks.iter().enumerate() {
        let key_board_sel = app.second_control_mode.label().eq(&i.to_string());

        render_audios_in_chunk(
            frame,
            *chunk,
            app,
            app.beat_sequence[i].clone(),
            key_board_sel,
        );
    }
}

fn render_audios_in_chunk(
    frame: &mut Frame,
    area: Rect,
    app: &App,
    sound_bar: SoundBar,
    key_board_sel: bool,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(1, 5); 5])
        .split(area);

    for (i, path) in sound_bar.paths.iter().enumerate() {
        let style = if key_board_sel && i + 1 == app.key_help_counter as usize {
            Style::default().bg(Color::Yellow).fg(Color::Black)
        } else {
            Style::default().fg(Color::Gray)
        };

        let file_name = Path::new(path).file_stem().and_then(|name| name.to_str());
        let label = file_name.unwrap_or("").to_string();
        let paragraph = Paragraph::new(label).style(style);

        frame.render_widget(paragraph, Render::center_vertically(chunks[i], 1, 1))
    }
}
