use crate::app::app::App;
use ratatui::layout::{Constraint, Direction, Flex, Layout, Rect};
use ratatui::prelude::{Color, Style};
use ratatui::widgets::{Block, Paragraph};
use ratatui::Frame;
use std::fmt;
use ratatui::layout::Constraint::Ratio;
use ratatui::macros::constraints;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum First_Control_Button {
    Settings,
    IncreaseBpm,
    DecreaseBpm,
    BecomeMaster,
}

impl First_Control_Button {
    pub fn next(&self) -> Self {
        match self {
            First_Control_Button::Settings => First_Control_Button::DecreaseBpm,
            First_Control_Button::DecreaseBpm => First_Control_Button::IncreaseBpm,
            First_Control_Button::IncreaseBpm => First_Control_Button::BecomeMaster,
            First_Control_Button::BecomeMaster => First_Control_Button::Settings,
        }
    }

    pub fn previous(&self) -> Self {
        match self {
            First_Control_Button::Settings => First_Control_Button::BecomeMaster,
            First_Control_Button::BecomeMaster => First_Control_Button::IncreaseBpm,
            First_Control_Button::IncreaseBpm => First_Control_Button::DecreaseBpm,
            First_Control_Button::DecreaseBpm => First_Control_Button::Settings,
        }
    }

    pub fn label(&self) -> &str {
        match self {
            First_Control_Button::Settings => "Settings ->",
            First_Control_Button::DecreaseBpm => "-",
            First_Control_Button::IncreaseBpm => "+",
            First_Control_Button::BecomeMaster => "Master",
        }
    }

    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::Settings,
            Self::DecreaseBpm,
            Self::IncreaseBpm,
            Self::BecomeMaster,
        ]
        .into_iter()
    }
}

pub struct Button {}
impl Button {
    fn get_button_style(app: &App, button_type: First_Control_Button) -> Style {
        let is_selected = app.current_mode == button_type;
        if is_selected {
            Style::default().bg(Color::Yellow).fg(Color::Black)
        } else {
            Style::default().fg(Color::Gray)
        }
    }

    pub fn render_button(
        app: &App,
        frame: &mut Frame,
        area: Rect,
        button_type: First_Control_Button,
    ) {
        let centered_vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),

            ])
            .flex(Flex::Center)
            .split(area)[0];

        let centered_content  = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Ratio(1, 8), Ratio(6, 8), Ratio(1, 8)])
            .areas::<3>(centered_vertical)[1];



        let style = Self::get_button_style(app, button_type);

        let widget = Paragraph::new(First_Control_Button::Settings.label())
            .block(Block::bordered())
            .style(style)
            .centered();

        frame.render_widget(widget, centered_content);
    }
}

impl fmt::Display for First_Control_Button {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label().trim())
    }
}
