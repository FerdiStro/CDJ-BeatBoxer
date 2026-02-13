use crate::app::buttons::{Button, FirstControlButton, SecondControlButton};
use crate::app::file_explorer::FileExplorer;
use crate::app::interactions::keyboard_interactions::{KeyBoardInteractions, MidiColor};
use crate::app::memory::memory::{Memory, ReceiveObject, SoundEntry};
use crate::app::render::render::Render;
use color_eyre::Result;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub enum AppAction {
    Quit,
    NextMode,
    PreviousMode,
    Submit,
    FirstMode,
    SecondMode,
    FileMode,
    None,
    Backspace,
    Shift,
    Bar1,
    Bar2,
    Bar3,
    Bar4,
    Bar5,
    Bar6,
    Bar7,
    Bar8,
}

pub struct SoundBar {
    pub paths: [String; 5],
    pub size: u8,
}

impl SoundBar {
    pub fn default() -> Self {
        Self {
            paths: std::array::from_fn(|_| String::new()),
            size: 0,
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            paths: self.paths.clone(),
            size: self.size,
        }
    }

    pub fn new(beat_index: usize, memory_sequence: &[SoundEntry; 10]) -> Self {
        let mut paths: [String; 5] = std::array::from_fn(|_| String::new());
        let mut count = 0;

        let mut size = 0;

        for sound in memory_sequence {
            if sound.is_active_in_beat(beat_index) {
                let path = sound.get_path_string();

                if !path.is_empty() {
                    paths[count] = path;
                    count += 1;
                    size += 1;
                    if count >= 5 {
                        break;
                    }
                }
            }
        }

        Self { paths, size }
    }
}

pub struct App {
    //Information from memory
    pub bpm: f64,
    pub is_master: bool,
    pub is_lock: bool,
    //Beat count
    pub small_counter: u8,
    pub total_counter: u64,
    pub bar_counter: u8,
    last_counter: u8,
    //Memory
    shared_state: Arc<Mutex<ReceiveObject>>,
    pub memory: Memory,
    //Keyboard/MIDI
    pub shift_mode: bool,
    pub first_control_mode: FirstControlButton,
    pub second_control_mode: SecondControlButton,
    pub key_board_interactions: KeyBoardInteractions,
    pub key_help_counter: u8,
    //File
    pub file_explorer: FileExplorer,
    pub selected_sound: PathBuf,
    //Beats
    pub beat_sequence: [SoundBar; 8],
    // Debug/Console message
    pub debug_message: String,
}

impl App {
    pub fn update(&mut self) {
        if let Ok(guard) = self.shared_state.lock() {
            //mapping  4 beat loop to 8 beat
            self.bar_counter = if self.small_counter != self.last_counter {
                self.last_counter = self.small_counter;
                let next_value = (self.bar_counter + 1) % 8;
                if (next_value % 4) == (self.small_counter) {
                    next_value
                } else {
                    self.small_counter
                }
            } else {
                self.bar_counter
            };

            self.bpm = guard.bpm;
            self.small_counter = guard.small_counter;
            self.total_counter = guard.total_counter;
            self.is_master = guard.is_master;

            for i in 0..8 {
                let sound_bar = SoundBar::new(i, &guard.sounds);
                if sound_bar.size != 0 {
                    self.key_board_interactions
                        .update_midi_pad_color(i as u8, MidiColor::Stay);
                } else {
                    self.key_board_interactions
                        .update_midi_pad_color(i as u8, MidiColor::Black);
                }
                self.beat_sequence[i] = SoundBar::new(i, &guard.sounds);
            }
            self.key_board_interactions
                .update_with_send_midi_pad_color(self.bar_counter, MidiColor::Cyan);

            self.debug_message = guard.sounds[0].get_path_string()
                + "Bit Mask: "
                + &guard.sounds[0].assigned_slot.to_string()
                + " SoundBarValue: "
                + &self.beat_sequence[0].paths[0]
        }
    }

    pub fn new() -> Result<()> {
        let shared_data = Arc::new(Mutex::new(ReceiveObject::default()));
        let thread_shared_data = shared_data.clone();

        let memory = Memory::new(thread_shared_data);

        Render::run(Self {
            bpm: 0.0,
            small_counter: 0,
            last_counter: 0,
            bar_counter: 0,
            total_counter: 0,
            shared_state: shared_data,
            first_control_mode: FirstControlButton::get_first_mode(),
            second_control_mode: SecondControlButton::get_first_mode(),
            is_master: false,
            is_lock: true,
            memory,
            key_board_interactions: KeyBoardInteractions::new(),
            key_help_counter: 0,
            shift_mode: false,
            file_explorer: FileExplorer::new(),
            selected_sound: PathBuf::default(),
            beat_sequence: std::array::from_fn(|_| SoundBar::default()),
            debug_message: "".to_string(),
        })
    }

    fn check_bar_next(&mut self, bar_button: SecondControlButton) {
        let i: u8 = bar_button.label().parse().expect("ERROR. No bar selected");
        if self.beat_sequence[i as usize].size > self.key_help_counter {
            self.key_help_counter += 1
        } else {
            self.key_help_counter = 0
        }
    }

    pub fn next_mode(&mut self, state: AppAction) {
        match state {
            AppAction::FirstMode => match self.first_control_mode {
                FirstControlButton::BecomeMaster => {
                    //overwork when BecomeMaster-button isn't previous
                    self.first_control_mode = FirstControlButton::FileBrowser;
                    self.file_explorer.next()
                }
                FirstControlButton::FileBrowser => self.file_explorer.next(),
                _ => self.first_control_mode = self.first_control_mode.next(&[]),
            },
            AppAction::SecondMode => {
                match self.second_control_mode {
                    SecondControlButton::Bar1 => self.check_bar_next(SecondControlButton::Bar1),
                    SecondControlButton::Bar2 => self.check_bar_next(SecondControlButton::Bar2),
                    SecondControlButton::Bar3 => self.check_bar_next(SecondControlButton::Bar3),
                    SecondControlButton::Bar4 => self.check_bar_next(SecondControlButton::Bar4),
                    SecondControlButton::Bar5 => self.check_bar_next(SecondControlButton::Bar5),
                    SecondControlButton::Bar6 => self.check_bar_next(SecondControlButton::Bar6),
                    SecondControlButton::Bar7 => self.check_bar_next(SecondControlButton::Bar7),
                    SecondControlButton::Bar8 => self.check_bar_next(SecondControlButton::Bar8),
                    _ => self.key_help_counter = 0,
                }

                if self.key_help_counter != 0 {
                    return;
                }

                if self.is_lock {
                    self.second_control_mode = self.second_control_mode.next(&[
                        &SecondControlButton::PreviousBar,
                        &SecondControlButton::NextBar,
                    ]);
                    return;
                }
                self.second_control_mode = self.second_control_mode.next(&[])
            }
            _ => {}
        }
    }

    pub fn previous_mode(&mut self, state: AppAction) {
        match state {
            AppAction::FirstMode => match self.first_control_mode {
                FirstControlButton::FileBrowser => {
                    if self.file_explorer.is_start() {
                        self.first_control_mode = self.first_control_mode.previous(&[])
                    } else {
                        self.file_explorer.previous();
                    }
                }
                _ => self.first_control_mode = self.first_control_mode.previous(&[]),
            },
            AppAction::SecondMode => {
                if self.is_lock {
                    self.second_control_mode = self.second_control_mode.previous(&[
                        &SecondControlButton::PreviousBar,
                        &SecondControlButton::NextBar,
                    ]);
                    return;
                }
                self.second_control_mode = self.second_control_mode.previous(&[])
            }
            _ => {}
        }
    }

    pub fn submit(&mut self, state: AppAction) {
        match state {
            AppAction::FirstMode => match self.first_control_mode {
                FirstControlButton::FileBrowser => {
                    if let Some(index) = self.file_explorer.state.selected() {
                        let selected_path = &self.file_explorer.items[index];
                        if selected_path.to_string_lossy() == ".." {
                            if let Some(parent) = self.file_explorer.current_dir.parent() {
                                self.file_explorer.current_dir = parent.to_path_buf();
                                self.file_explorer.read_dir();
                            }
                        } else if selected_path.is_dir() {
                            self.file_explorer.current_dir = selected_path.clone();
                            self.file_explorer.read_dir();
                        } else {
                            self.selected_sound = selected_path.clone();
                        }
                    }
                }
                _ => FirstControlButton::submit(&self.first_control_mode, &self.memory),
            },
            AppAction::SecondMode => match self.second_control_mode {
                SecondControlButton::Bar1 => SecondControlButton::Bar1.remove_bar_submit(
                    self.key_help_counter,
                    &self.beat_sequence,
                    &self.memory,
                ),
                SecondControlButton::Bar2 => SecondControlButton::Bar2.remove_bar_submit(
                    self.key_help_counter,
                    &self.beat_sequence,
                    &self.memory,
                ),
                SecondControlButton::Bar3 => SecondControlButton::Bar3.remove_bar_submit(
                    self.key_help_counter,
                    &self.beat_sequence,
                    &self.memory,
                ),
                SecondControlButton::Bar4 => SecondControlButton::Bar4.remove_bar_submit(
                    self.key_help_counter,
                    &self.beat_sequence,
                    &self.memory,
                ),
                SecondControlButton::Bar5 => SecondControlButton::Bar5.remove_bar_submit(
                    self.key_help_counter,
                    &self.beat_sequence,
                    &self.memory,
                ),
                SecondControlButton::Bar6 => SecondControlButton::Bar6.remove_bar_submit(
                    self.key_help_counter,
                    &self.beat_sequence,
                    &self.memory,
                ),
                SecondControlButton::Bar7 => SecondControlButton::Bar7.remove_bar_submit(
                    self.key_help_counter,
                    &self.beat_sequence,
                    &self.memory,
                ),
                SecondControlButton::Bar8 => SecondControlButton::Bar8.remove_bar_submit(
                    self.key_help_counter,
                    &self.beat_sequence,
                    &self.memory,
                ),
                _ => SecondControlButton::submit(&self.second_control_mode, &self.memory),
            },
            _ => {}
        }
    }
}
