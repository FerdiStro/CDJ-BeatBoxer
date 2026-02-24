use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
    ,
    ExecutableCommand,
};
use memmap2::MmapOptions;
use ratatui::widgets::canvas::Canvas;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders},
};
use std::{fs::File, io::stdout, time::Duration};


/// DEPRECATED BYRE-MAP CHANGED






















const MAX_WAVEFORM_SIZE: usize = 150_000;
const HEADER_SIZE: usize = 4;
const BUFFER_0_OFFSET: usize = HEADER_SIZE;
const BUFFER_1_OFFSET: usize = HEADER_SIZE + 8 + MAX_WAVEFORM_SIZE;

struct WaveformData {
    track_id: u32,
    amplitudes: Vec<u32>,
}

fn read_shared_memory(
    mmap: &memmap2::Mmap,
    terminal_width: usize,
    scroll_offset: usize,
    zoom_width: usize,
) -> Option<WaveformData> {
    let active_buffer = mmap[0];
    let offset = if active_buffer == 0 {
        BUFFER_0_OFFSET
    } else {
        BUFFER_1_OFFSET
    };

    let track_id = u32::from_le_bytes(mmap[offset..offset + 4].try_into().ok()?);
    let len = u32::from_le_bytes(mmap[offset + 4..offset + 8].try_into().ok()?) as usize;

    if len == 0 || len > MAX_WAVEFORM_SIZE || offset + 8 + len > mmap.len() {
        return None;
    }
    let raw_data = &mmap[offset + 8..offset + 8 + len];

    let start = scroll_offset.min(len);
    let end = (start + zoom_width).min(len);

    let visible_data = &raw_data[start..end];

    if visible_data.is_empty() {
        return Some(WaveformData {
            track_id,
            amplitudes: vec![0; terminal_width],
        });
    }

    //zoomed in data
    let mut amplitudes = vec![0; terminal_width];
    let mut grid_colors = vec![None; terminal_width];

    for (i, &b) in visible_data.iter().enumerate() {
        let mut col = ((i as f64 / visible_data.len() as f64) * terminal_width as f64) as usize;
        col = col.min(terminal_width - 1);

        // 150 bytes === 1 s

        // 0 - 4 amplitud
        // value 0 - 31  (0x1F)

        let amp = (b & 0x1F) as u32;
        if amp > amplitudes[col] {
            amplitudes[col] = amp;
        }

        // 6  is bar (first Beat)
        let is_bar = (b & 0x40) != 0;

        let prev_is_bar = if i > 0 {
            (visible_data[i - 1] & 0x40) != 0
        } else {
            false
        };

        if is_bar && !prev_is_bar {
            grid_colors[col] = Some(Color::Red);
        } else {
            grid_colors[col] = Some(Color::DarkGray);
        }
    }

    Some(WaveformData {
        track_id,
        amplitudes,
    })
}

fn main() -> std::io::Result<()> {
    let file = File::open("/Users/maintenance/Projects/CDJ-BeatBoxer/2_player_wave_form.bin")
        .expect("File not found");
    let mmap = unsafe { MmapOptions::new().map(&file)? };

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut scroll_offset: usize = 0;

    //6.5 sec default
    let mut zoom_width: usize = 975;

    let mut last_active_buffer: u8 = 255;
    let mut needs_redraw = true;

    loop {
        let current_active_buffer = mmap[0];
        if current_active_buffer != last_active_buffer {
            needs_redraw = true;
            last_active_buffer = current_active_buffer;
        }

        if needs_redraw {
            terminal.draw(|frame| {
                let area = frame.size();
                let layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(10), Constraint::Min(0)])
                    .split(area);

                let waveform_area = layout[0];
                let terminal_width = waveform_area.width as usize;

                if let Some(waveform) =
                    read_shared_memory(&mmap, terminal_width, scroll_offset, zoom_width)
                {
                    let title = format!(
                        " CDJ Waveform | Track ID: {} | Zoom: {} | [Pfeile] Scroll | [+/-] Zoom ",
                        waveform.track_id, zoom_width
                    );

                    let canvas = Canvas::default()
                        .block(Block::default().title(title).borders(Borders::ALL))
                        .marker(symbols::Marker::Braille)
                        .x_bounds([0.0, terminal_width as f64])
                        .y_bounds([0.0, 31.0])
                        .paint(|ctx| {
                            for i in 0..terminal_width {
                                let amp = waveform.amplitudes[i];

                                if amp > 0 && amp != 31 {
                                    let w_color = if amp >= 24 {
                                        Color::White
                                    } else if amp >= 16 {
                                        Color::Cyan
                                    } else {
                                        Color::Blue
                                    };
                                    let x = i as f64;

                                    let amp_f64 = amp as f64;

                                    let center = 15.5;

                                    let half_amp = amp_f64 / 2.0;

                                    let y_start = center - half_amp;
                                    let y_stop  = center + half_amp;

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

                    frame.render_widget(canvas, waveform_area);
                }
            })?;

            needs_redraw = false;
        }

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                needs_redraw = true;
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Right => scroll_offset = scroll_offset.saturating_add(150),
                    KeyCode::Left => scroll_offset = scroll_offset.saturating_sub(150),
                    KeyCode::Char('+') | KeyCode::Up => {
                        zoom_width = zoom_width.saturating_sub(150).max(150)
                    }
                    KeyCode::Char('-') | KeyCode::Down => {
                        zoom_width = zoom_width.saturating_add(150).min(MAX_WAVEFORM_SIZE)
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
