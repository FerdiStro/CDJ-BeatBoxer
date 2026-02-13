use crate::app::app::AppAction;
use crossterm::event::KeyCode;
use midir::{Ignore, MidiInput, MidiInputConnection, MidiOutput, MidiOutputConnection};
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
// Maybe needed in Linux
// sudo apt install libasound2-dev
// sudo modprobe snd-seq

#[derive(Copy, Clone)]
pub enum MidiColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Stay,
}

impl MidiColor {
    fn get_color_hex(&self) -> u8 {
        match self {
            MidiColor::Black => 0x00,
            MidiColor::Red => 0x01,
            MidiColor::Green => 0x04,
            MidiColor::Yellow => 0x05,
            MidiColor::Blue => 0x10,
            MidiColor::Magenta => 0x11,
            MidiColor::Cyan => 0x14,
            MidiColor::White => 0x7F,
            MidiColor::Stay => MidiColor::Red.get_color_hex(),
        }
    }
}

pub struct KeyBoardInteractions {
    pub first_control_button_last: bool,
    midi_pads: [MidiColor; 16],
    midi_output_connection: Option<MidiOutputConnection>,
    midi_input_connection: Option<MidiInputConnection<()>>,
    pub midi_receiver: Option<Receiver<Vec<u8>>>,
    midi_key_values: HashMap<u8, u8>,
    pub midi_available: bool,
}

impl KeyBoardInteractions {
    fn build_message(pad: u8, color: MidiColor) -> Vec<u8> {
        vec![
            0xF0,
            0x00,
            0x20,
            0x6B,
            0x7F,
            0x42,
            0x02,
            0x00,
            0x10,
            0x70 | pad,
            color.get_color_hex(),
            0xF7,
        ]
    }

    fn send_update_color_to_midi(&mut self) {
        if self.midi_available {
            let connection = self.midi_output_connection.as_mut().unwrap();
            self.midi_pads
                .iter()
                .enumerate()
                .for_each(|(i, pad_color)| {
                    let midi_message = Self::build_message(i as u8, *pad_color);
                    connection.send(&midi_message).unwrap()
                })
        }
    }

    pub fn update_midi_pad_color(&mut self, pad: u8, color: MidiColor) {
        self.midi_pads.iter_mut().for_each(|pad| match pad {
            MidiColor::Stay => {}
            _ => *pad = MidiColor::Black,
        });
        self.midi_pads[pad as usize] = color;
    }

    pub fn update_with_send_midi_pad_color(&mut self, pad: u8, color: MidiColor) {
        self.update_midi_pad_color(pad, color);
        self.send_update_color_to_midi();
    }

    pub fn new() -> Self {
        let mut midi_available: bool = true;

        let midi_out = MidiOutput::new("BeatBoxer-Midi-Output").unwrap();
        let out_ports = midi_out.ports();

        let mut conn_out: Option<MidiOutputConnection> = None;
        let mut midi_key_values: HashMap<u8, u8> = HashMap::new();

        match out_ports.get(0) {
            None => {
                midi_available = false;
            }
            Some(out) => {
                conn_out = midi_out.connect(out, "midir-send-output").ok();

                let mut midi_in = MidiInput::new("BeatBoxer-Midi-Input").unwrap();
                midi_in.ignore(Ignore::None);

                //counter for midi input latency
                midi_key_values.insert(0, 0);
                //Shift state  1 true 0 false
                midi_key_values.insert(1, 0);
                //navigate knops state
                midi_key_values.insert(74, 0);
                midi_key_values.insert(18, 0);
            }
        }

        Self {
            first_control_button_last: true,
            midi_pads: [MidiColor::Black; 16],
            midi_output_connection: conn_out,
            midi_input_connection: None,
            midi_receiver: None,
            midi_key_values,
            midi_available,
        }
    }

    pub fn init_midi(&mut self) {
        if self.midi_available {
            let (tx, rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = channel();

            self.midi_receiver = Some(rx);

            let midi_in = midir::MidiInput::new("Input").unwrap();
            let port = &midi_in.ports()[0];

            let conn = midi_in
                .connect(
                    port,
                    "midir-in",
                    move |_, message, _| {
                        let _ = tx.send(message.to_vec());
                    },
                    (),
                )
                .unwrap();

            self.midi_input_connection = Some(conn);
        }
    }

    fn map_message_to_key_code(&mut self, message: &[u8]) -> KeyCode {
        match message[1] {
            //smallest c not to shift mode
            48 => {
                let shift_mode = !(self.midi_key_values[&1] != 0) as u8;
                self.midi_key_values.insert(1, shift_mode);
                KeyCode::Null
            }
            36 => KeyCode::Char('1'),
            37 => KeyCode::Char('2'),
            38 => KeyCode::Char('3'),
            39 => KeyCode::Char('4'),
            40 => KeyCode::Char('5'),
            41 => KeyCode::Char('6'),
            42 => KeyCode::Char('7'),
            43 => KeyCode::Char('8'),
            //Enter - knop two
            113 => match message[2] {
                127 => KeyCode::Enter,
                _ => KeyCode::Null,
            },
            //Enter - knop one
            115 => match message[2] {
                127 => KeyCode::Enter,
                _ => KeyCode::Null,
            },
            _ => {
                let mut midi_skip = self.midi_key_values[&0];
                midi_skip += 1;
                self.midi_key_values.insert(0, midi_skip);

                if midi_skip % 16 == 0 {
                    self.midi_key_values.insert(0, 0);

                    let shift_mode = self.midi_key_values[&1] != 0;

                    return match message[1] {
                        112 => {
                            if shift_mode {
                                KeyCode::Down
                            } else {
                                KeyCode::Up
                            }
                        }
                        114 => {
                            if shift_mode {
                                KeyCode::Left
                            } else {
                                KeyCode::Right
                            }
                        }
                        _ => KeyCode::Null,
                    };
                }
                KeyCode::Null
            }
        }
    }

    pub fn on_midi_code(&mut self, midi_message: &[u8]) -> [AppAction; 2] {
        let midi_as_key_code = self.map_message_to_key_code(midi_message);

        self.on_key_code(midi_as_key_code)
    }

    pub fn on_key_code(&mut self, code: KeyCode) -> [AppAction; 2] {
        [
            match code {
                KeyCode::Char('q') => AppAction::Quit,
                KeyCode::Char('1') => AppAction::Bar1,
                KeyCode::Char('2') => AppAction::Bar2,
                KeyCode::Char('3') => AppAction::Bar3,
                KeyCode::Char('4') => AppAction::Bar4,
                KeyCode::Char('5') => AppAction::Bar5,
                KeyCode::Char('6') => AppAction::Bar6,
                KeyCode::Char('7') => AppAction::Bar7,
                KeyCode::Char('8') => AppAction::Bar8,
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
