use crate::app::render::render::Render;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Style};
use ratatui::widgets::canvas::Canvas;
use ratatui::widgets::{Block, Paragraph};
use ratatui::{symbols, Frame};

pub fn render_status_button(frame: &mut Frame, area: Rect, label: &str, style: Style) {
    let centered_content = Render::center_vertically(area, 3, 1);
    let widget = Paragraph::new(label)
        .block(Block::bordered())
        .style(style)
        .centered();

    frame.render_widget(widget, centered_content);
}

pub fn render_wave_form(
    frame: &mut Frame,
    area: Rect,
    cdj_number: u32,
    track_id: u32,
    amplitudes: [u32; 400],
    precise_grid_colors: [Option<Color>; 400],
) {
    let terminal_width = area.width as usize;

    let title = format!(" CDJ-{} Waveform | Track ID: {} ", cdj_number, track_id);

    let canvas = Canvas::default()
        .block(Block::default())
        .marker(symbols::Marker::Braille)
        .x_bounds([0.0, terminal_width as f64])
        .y_bounds([0.0, 31.0])
        .paint(|ctx| {
            for i in 0..terminal_width {
                let amp = amplitudes[i];

                if let Some(_) = precise_grid_colors[i]
                    && i != 0
                {
                    ctx.draw(&ratatui::widgets::canvas::Line {
                        x1: i as f64,
                        y1: 0f64,
                        x2: i as f64,
                        y2: 31f64,
                        color: Color::Red,
                    });
                    continue;
                }

                if amp > 0 && amp != 31 {
                    let w_color = if amp >= 24 {
                        Color::Blue
                    } else if amp >= 16 {
                        Color::Cyan
                    } else {
                        Color::White
                    };
                    let x = i as f64;

                    let amp_f64 = amp as f64;

                    let center = 15.5;

                    let half_amp = amp_f64 / 2.0;

                    let y_start = center - half_amp;
                    let y_stop = center + half_amp;

                    ctx.draw(&ratatui::widgets::canvas::Line {
                        x1: x,
                        y1: y_start,
                        x2: x,
                        y2: y_stop,
                        color: w_color,
                    });
                }

            }
        });
    frame.render_widget(canvas, area);
}
