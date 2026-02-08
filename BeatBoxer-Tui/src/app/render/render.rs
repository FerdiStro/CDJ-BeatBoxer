use crate::app::app::App;
use crate::app::buttons::First_Control_Button;
use crate::app::render::render_buttons_section::render_buttons_section;
use crate::app::render::render_header_section::render_header_section;
use crate::app::render::render_manage_section::render_manage_section;
use crate::app::render::render_render_section::render_render_section;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::layout::Constraint::Ratio;
use ratatui::layout::{Constraint, Direction, Flex, Layout, Rect};
use ratatui::Frame;
use std::time::Duration;

#[derive(Debug, Copy, Clone, Default)]
pub struct Render {}
impl Render {
    pub fn run(mut app: App) -> color_eyre::Result<()> {
        let mut terminal = ratatui::init();
        loop {
            terminal.draw(|f| render(f, &app))?;

            if event::poll(Duration::from_millis(50))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') => break Ok(()),
                            KeyCode::Right => app.next_mode(),
                            KeyCode::Left => app.previous_mode(),
                            KeyCode::Enter => First_Control_Button::on_submit(app.current_mode, &app.memory),

                            _ => {}
                        }
                    }
                }
            }

            app.update()
        }
    }

    pub fn center_vertically(area: Rect, height: u16, padding: u32) -> Rect {
        let centered_vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(height)])
            .flex(Flex::Center)
            .split(area)[0];

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Ratio(padding, 8),
                Ratio(8 - padding * 2, 8),
                Ratio(padding, 8),
            ])
            .areas::<3>(centered_vertical)[1]
    }
}

fn render(frame: &mut Frame, app: &App) {
    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Ratio(2, 3), Ratio(1, 3)]);

    let [content_grid, management_grid] = body.areas(frame.area());
    render_manage_section(frame, management_grid, app);

    let content_horizon = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Ratio(1, 8), Ratio(3, 8), Ratio(2, 8), Ratio(2, 8)]);

    let [header_grid, audio_render_grid, button_grid, utils_grid] =
        content_horizon.areas(content_grid);

    render_header_section(frame, header_grid, app);
    render_render_section(frame, audio_render_grid, app);
    render_buttons_section(frame, button_grid, app);
    render_buttons_section(frame, utils_grid, app);
}
