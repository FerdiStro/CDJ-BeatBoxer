mod app;

use crate::app::app::App;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::Constraint::Ratio;
use ratatui::layout::{Direction, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, Paragraph};
use ratatui::DefaultTerminal;
use ratatui::Frame;
use std::time::Duration;

fn main() -> Result<()> {
    color_eyre::install()?;
    let app = App::new();
    let terminal = ratatui::init();
    let result = run(terminal, app);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, mut app: App) -> Result<()> {
    loop {
        terminal.draw(|f| render(f, &app))?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break Ok(()),
                        _ => {}
                    }
                }
            }
        }

        app.update()
    }
}

fn render(frame: &mut Frame, app: &App) {
    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Ratio(2, 3), Ratio(1, 3)]);

    let [content_grid, management_grid] = body.areas(frame.area());

    frame.render_widget(Block::bordered().title("Manage-Section"), management_grid);

    let content_horizon = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Ratio(1, 8), Ratio(3, 8), Ratio(2, 8), Ratio(2, 8)]);

    let [header_grid, audio_render_grid, button_grid, utils_grid] =
        content_horizon.areas(content_grid);

    render_header(frame, header_grid, app);

    frame.render_widget(Block::bordered().title("Render"), audio_render_grid);
    frame.render_widget(Block::bordered().title("Buttons"), button_grid);
    frame.render_widget(Block::bordered().title("Utils"), utils_grid);
}

fn render_header(frame: &mut Frame, area: Rect, app: &App) {
    //frame
    let block = Block::bordered().title("Header");
    frame.render_widget(block.clone(), area);

    let inner_area = block.inner(area);
    let header_content = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Ratio(1, 8), Ratio(1, 8), Ratio(6, 8)]);

    let [setting_area, counter_container, _rest] = header_content.areas(inner_area);

    // Settings
    frame.render_widget(Paragraph::new("Settings"), setting_area);

    // Small-counter
    let small_counter_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Ratio(1, 4), Ratio(1, 4), Ratio(1, 4), Ratio(1, 4)]);

    let [count_0, count_1, count_2, count_3] = small_counter_layout.areas(counter_container);

    let areas = [count_0, count_1, count_2, count_3];
    for (i, area) in areas.into_iter().enumerate() {
        let is_active = i == app.small_counter as usize;

        let block = if is_active {
            Block::bordered().style(Style::default().bg(Color::Red))
        } else {
            Block::bordered().style(Style::default().fg(Color::DarkGray))
        };

        frame.render_widget(block, area);
    }
}
