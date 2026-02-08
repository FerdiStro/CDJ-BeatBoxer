use crate::app::app::AppAction;
use crossterm::event::KeyCode;

pub struct KeyBoardInteractions {
    first_control_button_last: bool,
}

impl KeyBoardInteractions {
    pub fn new() -> Self {
        Self {
            first_control_button_last: true,
        }
    }

    pub fn on_key_code(&mut self, code: KeyCode) -> [AppAction; 2] {
        let mut first_return: AppAction = AppAction::None;
        match code {
            KeyCode::Char('q') => first_return = AppAction::Quit,

            KeyCode::Right => {
                self.first_control_button_last = true;
                first_return = AppAction::NextMode;
            }

            KeyCode::Left => {
                self.first_control_button_last = true;
                first_return = AppAction::PreviousMode
            }

            KeyCode::Up => {
                self.first_control_button_last = false;
                first_return = AppAction::NextMode
            }
            KeyCode::Down => {
                self.first_control_button_last = false;
                first_return = AppAction::PreviousMode
            }
            KeyCode::Enter => first_return = AppAction::Submit,
            _ => first_return = AppAction::None,
        }
        let second_return = if self.first_control_button_last {
            AppAction::FirstMode
        } else {
            AppAction::SecondMode
        };

        [first_return, second_return]
    }
}
