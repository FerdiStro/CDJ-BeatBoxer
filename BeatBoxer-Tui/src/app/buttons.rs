use crate::app::app::App;
use crate::app::memory::memory::{Memory, SendObject};
use crate::app::render::render::Render;
use color_eyre::owo_colors::OwoColorize;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Style};
use ratatui::widgets::{Block, Paragraph};
use ratatui::Frame;

pub trait Button {
    fn label(&self) -> &str;

    fn get_first_mode() -> Self;
    fn get_button_style(&self, app: &App) -> Style;

    fn next(&self) -> Self;
    fn previous(&self) -> Self;
    fn submit(&self, memory: &Memory);

    fn render_button(app: &App, frame: &mut Frame, area: Rect, button_type: impl Button) {
        let centered_content = Render::center_vertically(area, 3, 1);

        let style = button_type.get_button_style(app);

        let widget = Paragraph::new(button_type.label())
            .block(Block::bordered())
            .style(style)
            .centered();

        frame.render_widget(widget, centered_content);
    }

    fn render_button_color(
        app: &App,
        frame: &mut Frame,
        area: Rect,
        button_type: impl Button,
        color: Color,
    ) {
        let centered_content = Render::center_vertically(area, 3, 1);

        let style = button_type.get_button_style(app).fg(color);

        let widget = Paragraph::new(button_type.label())
            .block(Block::bordered())
            .style(style)
            .centered();

        frame.render_widget(widget, centered_content);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SecondControlButton {
    BarLock,
    PreviousBar,
    NextBar,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FirstControlButton {
    Settings,
    IncreaseBpm,
    DecreaseBpm,
    BecomeMaster,
}

impl Button for FirstControlButton {
    fn label(&self) -> &str {
        match self {
            FirstControlButton::Settings => "Settings ->",
            FirstControlButton::DecreaseBpm => "-",
            FirstControlButton::IncreaseBpm => "+",
            FirstControlButton::BecomeMaster => "Master",
        }
    }

    fn get_first_mode() -> FirstControlButton {
        FirstControlButton::Settings
    }

    fn get_button_style(&self, app: &App) -> Style {
        let is_selected = app.first_control_mode.label() == self.label();
        if is_selected {
            Style::default().bg(Color::Yellow).fg(Color::Black)
        } else {
            Style::default().fg(Color::Gray)
        }
    }

    fn next(&self) -> Self {
        match self {
            FirstControlButton::Settings => FirstControlButton::DecreaseBpm,
            FirstControlButton::DecreaseBpm => FirstControlButton::IncreaseBpm,
            FirstControlButton::IncreaseBpm => FirstControlButton::BecomeMaster,
            FirstControlButton::BecomeMaster => FirstControlButton::Settings,
        }
    }

    fn previous(&self) -> Self {
        match self {
            FirstControlButton::Settings => FirstControlButton::BecomeMaster,
            FirstControlButton::BecomeMaster => FirstControlButton::IncreaseBpm,
            FirstControlButton::IncreaseBpm => FirstControlButton::DecreaseBpm,
            FirstControlButton::DecreaseBpm => FirstControlButton::Settings,
        }
    }

    fn submit(&self, memory: &Memory) {
        let mut send_object = SendObject::default();
        match self {
            FirstControlButton::Settings => return,
            FirstControlButton::DecreaseBpm => send_object.decrease_bpm = true,
            FirstControlButton::IncreaseBpm => send_object.increase_bpm = true,
            FirstControlButton::BecomeMaster => send_object.become_master = true,
        }
        memory.sender.send(send_object).unwrap();
    }
}

impl Button for SecondControlButton {
    fn label(&self) -> &str {
        match self {
            SecondControlButton::BarLock => "Lock",
            SecondControlButton::PreviousBar => "<-",
            SecondControlButton::NextBar => "->",
        }
    }

    fn get_first_mode() -> SecondControlButton {
        SecondControlButton::BarLock
    }

    fn get_button_style(&self, app: &App) -> Style {
        let is_selected = app.second_control_mode.label() == self.label();
        if is_selected {
            Style::default().bg(Color::Yellow).fg(Color::Black)
        } else {
            Style::default().fg(Color::Gray)
        }
    }

    fn next(&self) -> Self {
        todo!()
    }

    fn previous(&self) -> Self {
        todo!()
    }

    fn submit(&self, memory: &Memory) {
        todo!()
    }
}
