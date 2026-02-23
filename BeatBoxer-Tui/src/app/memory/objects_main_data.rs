
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ReceiveObject {
    pub sequence: u64,
    pub bpm: f64,
    pub small_counter: u8,
    pub is_master: bool,
    pub is_on_shoot_modus: bool,
    pub total_counter: u64,
    //Sounds
    pub sounds: [SoundEntry; 10],
}

impl ReceiveObject {
    pub fn default() -> Self {
        Self::new(
            0,
            0.0,
            0,
            false,
            false,
            0,
            [SoundEntry {
                path: [0; 256],
                assigned_slot: 0,
                _padding: [0; 7],
            }; 10],
        )
    }

    fn new(
        sequence: u64,
        bpm: f64,
        small_counter: u8,
        is_master: bool,
        is_on_shoot_modus: bool,
        total_counter: u64,
        sounds: [SoundEntry; 10],
    ) -> Self {
        Self {
            sequence,
            bpm,
            small_counter,
            is_master,
            is_on_shoot_modus,
            total_counter,
            sounds,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SoundEntry {
    pub path: [u8; 256],
    pub assigned_slot: u8,
    _padding: [u8; 7],
}

impl SoundEntry {
    pub fn get_path_string(&self) -> String {
        let len = self.path.iter().position(|&c| c == 0).unwrap_or(256);
        String::from_utf8_lossy(&self.path[..len]).to_string()
    }
    pub fn is_active_in_beat(&self, beat_index: usize) -> bool {
        ((self.assigned_slot >> beat_index) & 1) == 1
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SendObject {
    pub sequence: u64,
    pub increase_bpm: bool,
    pub decrease_bpm: bool,
    pub become_master: bool,
    pub on_shoot_modus: bool,
    _padding_1: [u8; 2],
    pub small_counter: u8,
    pub add_sound_on_small_beat: bool,
    pub selected_sound_path: [u8; 256],
    pub remove_sound_on_small_beat: bool,
    _padding_2: [u8; 7],
    pub remove_sound_path: [u8; 256],
    pub knob_value: u8,
    pub knob_echo: bool,
    pub knob_reverb: bool,
    pub knob_distortion: bool,
}

impl SendObject {
    pub fn knop_reverb() -> Self {
        Self::new(
            0, false, false, false, false, "", 0, false, false, "", 0, false, true, false,
        )
    }
    pub fn knop_distortion() -> Self {
        Self::new(
            0, false, false, false, false, "", 0, false, false, "", 0, false, false, true,
        )
    }
    pub fn knop_echo() -> Self {
        Self::new(
            0, false, false, false, false, "", 0, false, false, "", 0, true, false, false,
        )
    }

    pub fn default() -> Self {
        Self::new(
            0, false, false, false, false, "", 0, false, false, "", 0, false, false, false,
        )
    }

    pub fn convert_string_byte(path: &str) -> [u8; 256] {
        let mut path_bytes = [0u8; 256];
        let path_bytes_b = path.as_bytes();
        let len = std::cmp::min(path_bytes_b.len(), 256);
        path_bytes[..len].copy_from_slice(&path_bytes_b[..len]);
        path_bytes
    }

    pub fn new(
        sequence: u64,
        increase: bool,
        decrease: bool,
        master: bool,
        on_shoot_modus: bool,
        path: &str,
        small_counter: u8,
        add_sound_on_small_beat: bool,
        remove_sound_on_small_beat: bool,
        remove_sound_path: &str,
        knob_value: u8,
        knob_echo: bool,
        knob_reverb: bool,
        knob_distortion: bool,
    ) -> Self {
        let selected_sound_path = Self::convert_string_byte(path);
        let remove_sound_path = Self::convert_string_byte(remove_sound_path);

        Self {
            sequence,
            increase_bpm: increase,
            decrease_bpm: decrease,
            become_master: master,
            on_shoot_modus,
            _padding_1: [0; 2],
            small_counter,
            add_sound_on_small_beat,
            selected_sound_path,
            remove_sound_on_small_beat,
            _padding_2: [0; 7],
            remove_sound_path,
            knob_value,
            knob_echo,
            knob_reverb,
            knob_distortion,
        }
    }
}