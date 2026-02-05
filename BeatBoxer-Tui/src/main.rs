mod app;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::Constraint::{Ratio};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Paragraph};
use ratatui::DefaultTerminal;
use ratatui::Frame;
use std::time::Duration;

use crate::app::app::App;

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

        // app.update_metadata();
    }
}

fn render(frame: &mut Frame, app: &App) {

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Ratio(2, 3),
            Ratio(1, 3)
        ]);

    let [content_grid, management_grid] = body.areas(frame.area());

    frame.render_widget(Block::bordered().title("Manage-Section"), management_grid);


    let content_horizon = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Ratio(1, 8),
            Ratio(3, 8),
            Ratio(2, 8),
            Ratio(2, 8)
        ]);

    let [header_grid, audio_render_grid, button_grid, utils_grid] = content_horizon.areas(content_grid);


    render_header(frame, header_grid, app);

    frame.render_widget(Block::bordered().title("Render"), audio_render_grid);
    frame.render_widget(Block::bordered().title("Buttons"), button_grid);
    frame.render_widget(Block::bordered().title("Utils"), utils_grid);
}

fn render_header(frame: &mut Frame, area: Rect, _app: &App) {

    //settings
    let header_text = "press 'p' to quit ".to_string();
    //play state


    let block = Block::bordered().title("Header");
    let paragraph = Paragraph::new(header_text).block(block);

    frame.render_widget(paragraph, area);
}