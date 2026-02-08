use crate::app::app::App;
use crate::app::memory::memory::{Memory, SendObject};
use crate::app::render::render::Render;
use color_eyre::owo_colors::OwoColorize;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Style};
use ratatui::widgets::{Block, Paragraph};
use ratatui::Frame;
use std::fmt;

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

    pub fn on_submit(button_type: First_Control_Button, memory: &Memory) {
        let mut send_object = SendObject::default();
        match button_type {
            First_Control_Button::Settings => return,
            First_Control_Button::DecreaseBpm => send_object.decrease_bpm = true,
            First_Control_Button::IncreaseBpm => send_object.increase_bpm = true,
            First_Control_Button::BecomeMaster => send_object.become_master = true,
        }
        memory.sender.send(send_object).unwrap();
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
        let centered_content = Render::center_vertically(area, 3, 1);

        let style = Self::get_button_style(app, button_type);

        let widget = Paragraph::new(button_type.label())
            .block(Block::bordered())
            .style(style)
            .centered();

        frame.render_widget(widget, centered_content);
    }

    pub fn render_button_color(
        app: &App,
        frame: &mut Frame,
        area: Rect,
        button_type: First_Control_Button,
        color: Color,
    ) {
        let centered_content = Render::center_vertically(area, 3, 1);

        let style = Self::get_button_style(app, button_type).fg(color);

        let widget = Paragraph::new(button_type.label())
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
