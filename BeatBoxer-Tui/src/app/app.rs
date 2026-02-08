use std::sync::{Arc, Mutex};

use crate::app::buttons::{Button, FirstControlButton, SecondControlButton};
use crate::app::memory::memory::{Memory, ReceiveObject};

use crate::app::interactions::keyboard_interactions::KeyBoardInteractions;
use crate::app::render::render::Render;
use color_eyre::Result;

pub enum AppAction {
    Quit,
    NextMode,
    PreviousMode,
    Submit,
    FirstMode,
    SecondMode,
    None,
}


pub struct App {
    pub bpm: f64,
    pub small_counter: u8,
    pub total_counter: u64,
    shared_state: Arc<Mutex<ReceiveObject>>,
    pub first_control_mode: FirstControlButton,
    pub second_control_mode: SecondControlButton,
    pub is_master: bool,
    pub memory: Memory,
    pub key_board_interactions: KeyBoardInteractions,
}

impl App {
    pub fn update(&mut self) {
        if let Ok(guard) = self.shared_state.lock() {
            self.bpm = guard.bpm;
            self.small_counter = guard.small_counter;
            self.total_counter = guard.total_counter;
            self.is_master = guard.is_master;
        }
    }

    pub fn new() -> Result<()> {
        let shared_data = Arc::new(Mutex::new(ReceiveObject::default()));
        let thread_shared_data = shared_data.clone();

        let memory = Memory::new(thread_shared_data);

        Render::run(Self {
            bpm: 0.0,
            small_counter: 0,
            total_counter: 0,
            shared_state: shared_data,
            first_control_mode: FirstControlButton::get_first_mode(),
            second_control_mode: SecondControlButton::get_first_mode(),
            is_master: false,
            memory,
            key_board_interactions: KeyBoardInteractions::new(),
        })
    }

    pub fn next_mode(&mut self, state: AppAction) {
        match state {
            AppAction::FirstMode =>         self.first_control_mode = self.first_control_mode.next(),
            AppAction::SecondMode => self.second_control_mode = self.second_control_mode.next(),
            _ => {}
        }
    }

    pub fn previous_mode(&mut self, state: AppAction) {
        match state {
            AppAction::FirstMode =>         self.first_control_mode = self.first_control_mode.previous(),
            AppAction::SecondMode => self.second_control_mode = self.second_control_mode.previous(),
            _ => {}
        }
    }

    pub fn submit(&mut self) {}
}
