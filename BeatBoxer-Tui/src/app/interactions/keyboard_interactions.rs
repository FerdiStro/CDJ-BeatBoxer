use crate::app::app::AppAction;
use crossterm::event::KeyCode;

pub struct KeyBoardInteractions {
    pub first_control_button_last: bool,
}

impl KeyBoardInteractions {
    pub fn new() -> Self {
        Self {
            first_control_button_last: true,
        }
    }

    pub fn on_key_code(&mut self, code: KeyCode) -> [AppAction; 2] {
        [
            match code {
                KeyCode::Char('q') => AppAction::Quit,
                KeyCode::Char('1') => AppAction::BAR_1,
                KeyCode::Right => {
                    self.first_control_button_last = true;
                    AppAction::NextMode
                }

                KeyCode::Left => {
                    self.first_control_button_last = true;
                    AppAction::PreviousMode
                }

                KeyCode::Up => {
                    self.first_control_button_last = false;
                    AppAction::NextMode
                }
                KeyCode::Down => {
                    self.first_control_button_last = false;
                    AppAction::PreviousMode
                }
                KeyCode::Backspace => AppAction::Backspace,

                KeyCode::Enter => AppAction::Submit,
                _ => AppAction::None,
            },
            if self.first_control_button_last {
                AppAction::FirstMode
            } else {
                AppAction::SecondMode
            },
        ]
    }
}
