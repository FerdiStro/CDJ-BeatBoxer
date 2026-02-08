use std::sync::{Arc, Mutex};

use crate::app::buttons::First_Control_Button;
use crate::app::memory::memory::{Memory, ReceiveObject};

use crate::app::render::render::Render;
use color_eyre::Result;

pub struct App {
    track_title: String,
    artist: String,
    pub bpm: f64,
    is_playing: bool,
    pub small_counter: u8,
    pub total_counter: u64,
    shared_state: Arc<Mutex<ReceiveObject>>,
    pub current_mode: First_Control_Button,
    pub is_master: bool,
    pub memory: Memory
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

       let memory =  Memory::new(thread_shared_data);

        Render::run(Self {
            track_title: "Wait on CDJ..".to_string(),
            artist: "xxx".to_string(),
            bpm: 0.0,
            total_counter: 0,
            is_playing: false,
            small_counter: 0,
            shared_state: shared_data,
            current_mode: First_Control_Button::Settings,
            is_master: false,
            memory: memory,
        })
    }

    pub fn next_mode(&mut self) {
        self.current_mode = self.current_mode.next();
    }

    pub fn previous_mode(&mut self) {
        self.current_mode = self.current_mode.previous();
    }

    pub fn submit(&mut self) {}
}
