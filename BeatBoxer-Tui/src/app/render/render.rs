use crate::app::app::{App, AppAction};
use crate::app::buttons::{Button, FirstControlButton, SecondControlButton};
use crate::app::render::render_buttons_section::render_buttons_section;
use crate::app::render::render_header_section::render_header_section;
use crate::app::render::render_manage_section::render_manage_section;
use crate::app::render::render_render_section::render_render_section;
use crossterm::event;
use crossterm::event::{Event, KeyEventKind};
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
            terminal.draw(|f| render(f, &mut app))?;

            if event::poll(Duration::from_millis(50))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        let [action, state] = app.key_board_interactions.on_key_code(key.code);

                        match action {
                            AppAction::Quit => break Ok(()),
                            AppAction::NextMode => app.next_mode(state),
                            AppAction::PreviousMode => app.previous_mode(state),
                            AppAction::Submit => match state {
                                AppAction::FirstMode => match app.first_control_mode {
                                    FirstControlButton::FileBrowser => {
                                        if let Some(index) = app.file_explorer.state.selected() {
                                            let selected_path = &app.file_explorer.items[index];
                                            if selected_path.to_string_lossy() == ".." {
                                                if let Some(parent) =
                                                    app.file_explorer.current_dir.parent()
                                                {
                                                    app.file_explorer.current_dir =
                                                        parent.to_path_buf();
                                                    app.file_explorer.read_dir(); // Liste neu laden
                                                }
                                            } else if selected_path.is_dir() {
                                                app.file_explorer.current_dir =
                                                    selected_path.clone();
                                                app.file_explorer.read_dir();
                                            } else {
                                                app.selected_sound = selected_path.clone();
                                            }
                                        }
                                    }
                                    _ => FirstControlButton::submit(
                                        &app.first_control_mode,
                                        &app.memory,
                                    ),
                                },
                                AppAction::SecondMode => SecondControlButton::submit(
                                    &app.second_control_mode,
                                    &app.memory,
                                ),
                                _ => {}
                            },
                            AppAction::Backspace => match app.first_control_mode {
                                FirstControlButton::FileBrowser => {
                                    if let Some(parent) = app.file_explorer.current_dir.parent() {
                                        app.file_explorer.current_dir = parent.to_path_buf();
                                        app.file_explorer.read_dir();
                                    }
                                }
                                _ => {}
                            },
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

fn render(frame: &mut Frame, app: &mut App) {
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
