use crate::app::memory::memory::Memory;
use crate::app::memory::objects_main_data::SendObject;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Style};
use ratatui::widgets::Block;
use ratatui::Frame;

pub struct Knob {
    name: String,
    pub value: u8,
    pub midi_value: u8,
    pub send_object: SendObject,
}
impl Knob {
    pub fn new(name: String, value: u8, midi_value: u8, send_object: SendObject) -> Self {
        Self {
            name,
            value,
            midi_value,
            send_object,
        }
    }
}

pub struct Knobs {
    pub knobs: [Knob; 3],
}

impl Knobs {
    fn render_knob_with_progress_border(frame: &mut Frame, area: &Rect, label: &str, value: u8) {
        let block = Block::bordered()
            .title(label)
            .border_style(Style::default().fg(Color::DarkGray));

        frame.render_widget(block, *area);

        let buf = frame.buffer_mut();

        let left = area.x;
        let right = area.x + area.width - 1;
        let top = area.y;
        let bottom = area.y + area.height - 1;

        let mut border_path = Vec::new();

        for y in (top..=bottom).rev() {
            border_path.push((left, y));
        }
        for x in (left + 1)..=right {
            border_path.push((x, top));
        }
        for y in (top + 1)..=bottom {
            border_path.push((right, y));
        }
        for x in (left + 1..right).rev() {
            border_path.push((x, bottom));
        }

        let total_cells = border_path.len();
        let active_cells = (value as f32 / 127.0 * total_cells as f32).round() as usize;

        for (i, (x, y)) in border_path.iter().enumerate() {
            if i < active_cells {
                if let Some(cell) = buf.cell_mut((*x, *y)) {
                    cell.set_fg(Color::Cyan);
                }
            }
        }

        let text = format!("{}", value);
        let text_x = area.x + (area.width.saturating_sub(text.len() as u16)) / 2;
        let text_y = area.y + area.height / 2;
        buf.set_string(text_x, text_y, text, Style::default().fg(Color::White));
    }

    pub fn draw_midi_knobs(&self, frame: &mut Frame, area: Rect) {
        let constraints = vec![Constraint::Length(12); self.knobs.len()];

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .spacing(1)
            .constraints(constraints)
            .split(area);

        for (knob, chunk) in self.knobs.iter().zip(chunks.iter()) {
            Self::render_knob_with_progress_border(frame, chunk, &knob.name, knob.value);
        }
    }

    pub fn check_midi_knobs(&mut self, message: &[u8], memory: &Memory) {
        for knop in &mut self.knobs {
            if knop.midi_value == message[1] {
                knop.value = message[2];
                let mut send_object = knop.send_object;
                send_object.knob_value = message[2];
                memory.sender.send(send_object).unwrap();
            }
        }
    }

    pub fn new() -> Self {
        let echo_knob = Knob::new("Echo".to_string(), 0, 74, SendObject::knop_echo());
        let reverb_knob = Knob::new("Reverb".to_string(), 0, 18, SendObject::knop_reverb());
        let distortion_knob = Knob::new(
            "Distortion".to_string(),
            0,
            71,
            SendObject::knop_distortion(),
        );

        let knobs = [echo_knob, reverb_knob, distortion_knob];

        Self { knobs }
    }
}
