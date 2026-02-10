use crate::app::app::App;
use crate::app::memory::memory::{Memory, SendObject};
use crate::app::render::render::Render;
use color_eyre::owo_colors::OwoColorize;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Style};
use ratatui::widgets::{Block, Paragraph};
use ratatui::Frame;
use std::str::FromStr;

pub trait Button {
    fn label(&self) -> &str;

    fn get_first_mode() -> Self;
    fn get_button_style(&self, app: &App) -> Style;

    fn next(&self, exclude: &[&Self]) -> Self;
    fn previous(&self, exclude: &[&Self]) -> Self;
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
    BAR_1,
    BAR_2,
    BAR_3,
    BAR_4,
    BAR_5,
    BAR_6,
    BAR_7,
    BAR_8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FirstControlButton {
    Settings,
    IncreaseBpm,
    DecreaseBpm,
    BecomeMaster,
    FileBrowser,
}

impl Button for FirstControlButton {
    fn label(&self) -> &str {
        match self {
            FirstControlButton::Settings => "Settings ->",
            FirstControlButton::DecreaseBpm => "-",
            FirstControlButton::IncreaseBpm => "+",
            FirstControlButton::BecomeMaster => "Master",
            FirstControlButton::FileBrowser => "file_browser_mode_label_not_displayed",
        }
    }

    fn get_first_mode() -> FirstControlButton {
        FirstControlButton::FileBrowser
    }

    fn get_button_style(&self, app: &App) -> Style {
        let is_selected = app.first_control_mode.label() == self.label();
        if is_selected && app.key_board_interactions.first_control_button_last {
            Style::default().bg(Color::Yellow).fg(Color::Black)
        } else {
            Style::default().fg(Color::Gray)
        }
    }

    fn next(&self, exclude: &[&FirstControlButton]) -> Self {
        match self {
            FirstControlButton::Settings => FirstControlButton::DecreaseBpm,
            FirstControlButton::DecreaseBpm => FirstControlButton::IncreaseBpm,
            FirstControlButton::IncreaseBpm => FirstControlButton::BecomeMaster,
            FirstControlButton::BecomeMaster => FirstControlButton::FileBrowser,
            FirstControlButton::FileBrowser => FirstControlButton::Settings,
        }
    }

    fn previous(&self, exclude: &[&FirstControlButton]) -> Self {
        match self {
            FirstControlButton::Settings => FirstControlButton::FileBrowser,
            FirstControlButton::BecomeMaster => FirstControlButton::IncreaseBpm,
            FirstControlButton::IncreaseBpm => FirstControlButton::DecreaseBpm,
            FirstControlButton::DecreaseBpm => FirstControlButton::Settings,
            FirstControlButton::FileBrowser => FirstControlButton::BecomeMaster,
        }
    }

    fn submit(&self, memory: &Memory) {
        let mut send_object = SendObject::default();

        match self {
            FirstControlButton::Settings => return,
            FirstControlButton::DecreaseBpm => send_object.decrease_bpm = true,
            FirstControlButton::IncreaseBpm => send_object.increase_bpm = true,
            FirstControlButton::BecomeMaster => send_object.become_master = true,
            FirstControlButton::FileBrowser => return,
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
            SecondControlButton::BAR_1 => "0",
            SecondControlButton::BAR_2 => "1",
            SecondControlButton::BAR_3 => "2",
            SecondControlButton::BAR_4 => "3",
            SecondControlButton::BAR_5 => "4",
            SecondControlButton::BAR_6 => "5",
            SecondControlButton::BAR_7 => "6",
            SecondControlButton::BAR_8 => "7",
        }
    }

    fn get_first_mode() -> SecondControlButton {
        SecondControlButton::BarLock
    }

    fn get_button_style(&self, app: &App) -> Style {
        let is_selected = app.second_control_mode.label() == self.label();
        if is_selected && !app.key_board_interactions.first_control_button_last {
            Style::default().bg(Color::Yellow).fg(Color::Black)
        } else {
            Style::default().fg(Color::Gray)
        }
    }

    fn next(&self, exclude: &[&SecondControlButton]) -> Self {
        let mut candidate = *self;

        loop {
            candidate = match candidate {
                SecondControlButton::BarLock => SecondControlButton::PreviousBar,
                SecondControlButton::PreviousBar => SecondControlButton::NextBar,
                SecondControlButton::NextBar => SecondControlButton::BAR_1,
                SecondControlButton::BAR_1 => SecondControlButton::BAR_2,
                SecondControlButton::BAR_2 => SecondControlButton::BAR_3,
                SecondControlButton::BAR_3 => SecondControlButton::BAR_4,
                SecondControlButton::BAR_4 => SecondControlButton::BAR_5,
                SecondControlButton::BAR_5 => SecondControlButton::BAR_6,
                SecondControlButton::BAR_6 => SecondControlButton::BAR_7,
                SecondControlButton::BAR_7 => SecondControlButton::BAR_8,
                SecondControlButton::BAR_8 => SecondControlButton::BarLock,
            };
            if !exclude.contains(&&candidate) {
                return candidate;
            }
            if candidate == *self {
                return *self;
            }
        }
    }

    fn previous(&self, exclude: &[&SecondControlButton]) -> Self {
        let mut candidate = *self;
        loop {
            candidate = match candidate {
                SecondControlButton::BarLock => SecondControlButton::BAR_8,
                SecondControlButton::PreviousBar => SecondControlButton::BarLock,
                SecondControlButton::NextBar => SecondControlButton::PreviousBar,
                SecondControlButton::BAR_1 => SecondControlButton::NextBar,
                SecondControlButton::BAR_2 => SecondControlButton::BAR_1,
                SecondControlButton::BAR_3 => SecondControlButton::BAR_2,
                SecondControlButton::BAR_4 => SecondControlButton::BAR_3,
                SecondControlButton::BAR_5 => SecondControlButton::BAR_4,
                SecondControlButton::BAR_6 => SecondControlButton::BAR_5,
                SecondControlButton::BAR_7 => SecondControlButton::BAR_6,
                SecondControlButton::BAR_8 => SecondControlButton::BAR_7,
            };
            if !exclude.contains(&&candidate) {
                return candidate;
            }
            if candidate == *self {
                return *self;
            }
        }
    }

    fn submit(&self, memory: &Memory) {}
}

impl SecondControlButton {
    pub fn render_bar_button(self, app: &App, frame: &mut Frame, area: Rect) {
        let bar_index = u8::from_str(self.label()).unwrap();
        let style = if app.bar_counter == bar_index {
            Style::default().bg(Color::Red)
        } else {
            Style::default().fg(Color::Gray)
        };

        let select_test = if app.second_control_mode.label() == self.label()
            && !app.key_board_interactions.first_control_button_last
        {
            "🔊"
        } else {
            ""
        };

        let widget = Paragraph::new(select_test)
            .block(Block::bordered().style(style))
            .style(style)
            .centered();
        frame.render_widget(widget, area);
    }
}
