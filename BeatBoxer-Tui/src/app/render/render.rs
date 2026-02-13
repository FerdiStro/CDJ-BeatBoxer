use crate::app::app::{App, AppAction};
use crate::app::buttons::{FirstControlButton, SecondControlButton};
use crate::app::render::render_buttons_section::render_buttons_section;
use crate::app::render::render_header_section::render_header_section;
use crate::app::render::render_manage_section::render_manage_section;
use crate::app::render::render_render_section::render_render_section;
use crate::app::render::render_utils_section::render_utils_section;
use color_eyre::Report;
use crossterm::event;
use crossterm::event::{Event, KeyEventKind};
use ratatui::layout::Constraint::{Fill, Length, Ratio};
use ratatui::layout::{Constraint, Direction, Flex, Layout, Rect};
use ratatui::Frame;
use std::time::Duration;

#[derive(Debug, Copy, Clone, Default)]
pub struct Render {}
impl Render {
    fn key_inputs(_app: &mut App, actions: [AppAction; 2]) -> color_eyre::Result<()> {
        let [action, state] = actions;
        match action {
            AppAction::Quit => Err(Report::msg("Application quited")),

            AppAction::Shift => {
                _app.shift_mode = !_app.shift_mode;
                Ok(())
            }
            AppAction::Bar1 => {
                SecondControlButton::Bar1.on_bar_press(
                    &_app.selected_sound,
                    &_app.memory,
                    &_app.shift_mode,
                    &_app.beat_sequence,
                );
                Ok(())
            }
            AppAction::Bar2 => {
                SecondControlButton::Bar2.on_bar_press(
                    &_app.selected_sound,
                    &_app.memory,
                    &_app.shift_mode,
                    &_app.beat_sequence,
                );
                Ok(())
            }
            AppAction::Bar3 => {
                SecondControlButton::Bar3.on_bar_press(
                    &_app.selected_sound,
                    &_app.memory,
                    &_app.shift_mode,
                    &_app.beat_sequence,
                );
                Ok(())
            }
            AppAction::Bar4 => {
                SecondControlButton::Bar4.on_bar_press(
                    &_app.selected_sound,
                    &_app.memory,
                    &_app.shift_mode,
                    &_app.beat_sequence,
                );
                Ok(())
            }
            AppAction::Bar5 => {
                SecondControlButton::Bar5.on_bar_press(
                    &_app.selected_sound,
                    &_app.memory,
                    &_app.shift_mode,
                    &_app.beat_sequence,
                );
                Ok(())
            }
            AppAction::Bar6 => {
                SecondControlButton::Bar6.on_bar_press(
                    &_app.selected_sound,
                    &_app.memory,
                    &_app.shift_mode,
                    &_app.beat_sequence,
                );
                Ok(())
            }
            AppAction::Bar7 => {
                SecondControlButton::Bar7.on_bar_press(
                    &_app.selected_sound,
                    &_app.memory,
                    &_app.shift_mode,
                    &_app.beat_sequence,
                );
                Ok(())
            }
            AppAction::Bar8 => {
                SecondControlButton::Bar8.on_bar_press(
                    &_app.selected_sound,
                    &_app.memory,
                    &_app.shift_mode,
                    &_app.beat_sequence,
                );
                Ok(())
            }
            AppAction::NextMode => {
                _app.next_mode(state);
                Ok(())
            }
            AppAction::PreviousMode => {
                _app.previous_mode(state);
                Ok(())
            }
            AppAction::Submit => {
                _app.submit(state);
                Ok(())
            }
            AppAction::Backspace => match _app.first_control_mode {
                FirstControlButton::FileBrowser => {
                    if let Some(parent) = _app.file_explorer.current_dir.parent() {
                        _app.file_explorer.current_dir = parent.to_path_buf();
                        _app.file_explorer.read_dir();
                    }
                    Ok(())
                }
                _ => Ok(()),
            },
            _ => Ok(()),
        }
    }

    pub fn run(mut _app: App) -> color_eyre::Result<()> {
        let mut terminal = ratatui::init();
        _app.key_board_interactions.init_midi();

        loop {
            terminal.draw(|f| render(f, &mut _app))?;

            let mut pending_midi_messages = Vec::new();

            //Midi
            if let Some(rx) = &_app.key_board_interactions.midi_receiver {
                for message in rx.try_iter() {
                    pending_midi_messages.push(message);
                }
            }
            for message in pending_midi_messages {
                let app_actions = _app.key_board_interactions.on_midi_code(&message);

                match Self::key_inputs(&mut _app, app_actions) {
                    Err(_e) => break,
                    _ => {}
                }
            }

            //Keyboard
            if event::poll(Duration::from_millis(50))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        let app_actions = _app.key_board_interactions.on_key_code(key.code);

                        match Self::key_inputs(&mut _app, app_actions) {
                            Err(_e) => break Ok(()),
                            _ => {}
                        }
                    }
                }
            }

            _app.update()
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
        .constraints([Ratio(1, 8), Ratio(3, 8), Length(7), Fill(0)]);

    let [header_grid, audio_render_grid, button_grid, utils_grid] =
        content_horizon.areas(content_grid);

    render_header_section(frame, header_grid, app);
    render_render_section(frame, audio_render_grid, app);
    render_buttons_section(frame, button_grid, app);
    render_utils_section(frame, utils_grid, app);
}
