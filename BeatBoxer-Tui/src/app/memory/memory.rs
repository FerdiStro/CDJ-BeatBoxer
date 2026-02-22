use crossbeam_channel::{bounded, Sender};
use memmap2::MmapMut;
use ratatui::prelude::Color;
use std::fs::OpenOptions;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{env, thread};

//WaveForm  Values
const WAVEFORM_MAX_SIZE: usize = 150_000;
const WAVEFORM_HEADER_SIZE: usize = 4;
const WAVEFORM_BUFFER_0_OFFSET: usize = WAVEFORM_HEADER_SIZE;
const WAVEFORM_BUFFER_1_OFFSET: usize = WAVEFORM_HEADER_SIZE + 8 + WAVEFORM_MAX_SIZE;

//SharedMemory to engine
const FILE_SIZE: u64 = 4096;

#[derive(Default)]
pub struct WaveformData {
    pub track_id: u32,
    pub amplitudes: Vec<u32>,
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

#[derive(Debug, Clone)]
pub struct Memory {
    pub sender: Sender<SendObject>,
    pub wave_form_cdj_1_terminal_sender: Option<Sender<usize>>,
    pub wave_form_cdj_2_terminal_sender: Option<Sender<usize>>,
}

impl Memory {
    pub fn new(shared_state: Arc<Mutex<ReceiveObject>>) -> Memory {
        let reading_path = env::var("BEATBOXER_READ_PATH")
            .expect("CRITICAL: 'BEATBOXER_READ_PATH' not set in ENV!");

        let writing_path = env::var("BEATBOXER_WRITE_PATH")
            .expect("CRITICAL: 'BEATBOXER_WRITE_PATH' not set in ENV!");

        Self::start_reading_thread(reading_path, shared_state);
        let sender = Self::start_writing_thread(writing_path);
        Self {
            sender,
            wave_form_cdj_1_terminal_sender: None,
            wave_form_cdj_2_terminal_sender: None,
        }
    }

    pub fn get_data_from_cdj(
        &mut self,
        cdj_1_shared_data: Arc<Mutex<WaveformData>>,
        cdj_2_shared_data: Arc<Mutex<WaveformData>>,
    ) {
        let reading_path = env::var("BEATBOXER_READ_CDJ_PATH")
            .expect("CRITICAL: 'BEATBOXER_READ_CDJ_PATH' not set in ENV!");
        let cdj_1_path = reading_path.replace("XxX", "1");
        self.wave_form_cdj_1_terminal_sender =
            Some(Self::start_wave_reading_trad(cdj_1_path, cdj_1_shared_data));

        let cdj_2_path = reading_path.replace("XxX", "2");
        self.wave_form_cdj_2_terminal_sender =
            Some(Self::start_wave_reading_trad(cdj_2_path, cdj_2_shared_data));
    }

    fn start_wave_reading_trad(
        wave_file_path: String,
        thread_shared_data: Arc<Mutex<WaveformData>>,
    ) -> Sender<usize> {
        let (tx, rx) = bounded::<usize>(1);

        //move them auto??
        let scroll_offset = 0;
        let zoom_width = 975;

        thread::spawn(move || {
            let file_result = OpenOptions::new()
                .read(true)
                .write(true)
                .open(wave_file_path);

            let file = match file_result {
                Ok(file) => file,
                Err(error) => {
                    eprintln!("Problem opening the file: {error:?}");
                    return;
                }
            };

            let m_map_result = unsafe { MmapMut::map_mut(&file) };

            let m_map = match m_map_result {
                Ok(m_map) => m_map,
                Err(error) => {
                    eprintln!("Problem opening the file: {error:?}");
                    return;
                }
            };

            let mut last_active_buffer: u8 = 255;

            loop {
                let current_active_buffer = m_map[0];

                if current_active_buffer != last_active_buffer {
                    last_active_buffer = current_active_buffer;

                    match rx.recv() {
                        Ok(terminal_width) => {
                            let offset = if current_active_buffer == 0 {
                                WAVEFORM_BUFFER_0_OFFSET
                            } else {
                                WAVEFORM_BUFFER_1_OFFSET
                            };

                            let track_id = u32::from_le_bytes(
                                m_map[offset..offset + 4].try_into().ok().unwrap(),
                            );
                            let len = u32::from_le_bytes(
                                m_map[offset + 4..offset + 8].try_into().ok().unwrap(),
                            ) as usize;

                            if len == 0 || len > WAVEFORM_MAX_SIZE || offset + 8 + len > m_map.len()
                            {
                                return;
                            }

                            let raw_data = &m_map[offset + 8..offset + 8 + len];

                            let start = scroll_offset.min(len);
                            let end = (start + zoom_width).min(len);

                            let visible_data = &raw_data[start..end];

                            if visible_data.is_empty() {
                                return;
                            }

                            //zoomed in data
                            let mut amplitudes = vec![0; terminal_width];
                            let mut grid_colors = vec![None; terminal_width];

                            for (i, &b) in visible_data.iter().enumerate() {
                                let mut col = ((i as f64 / visible_data.len() as f64)
                                    * terminal_width as f64)
                                    as usize;
                                col = col.min(terminal_width - 1);

                                // 150 bytes === 1 s
                                // 0 - 4 amplitud
                                // value 0 - 31  (0x1F)
                                let amp = (b & 0x1F) as u32;
                                if amp > amplitudes[col] {
                                    amplitudes[col] = amp;
                                }

                                // 6  is bar (first Beat)
                                let is_bar = (b & 0x40) != 0;

                                let prev_is_bar = if i > 0 {
                                    (visible_data[i - 1] & 0x40) != 0
                                } else {
                                    false
                                };

                                if is_bar && !prev_is_bar {
                                    grid_colors[col] = Some(Color::Red);
                                } else {
                                    grid_colors[col] = Some(Color::DarkGray);
                                }
                            }

                            if let Ok(mut guard) = thread_shared_data.lock() {
                                guard.amplitudes = amplitudes;
                                guard.track_id = track_id;
                            }
                        }
                        Err(_) => {
                            break;
                        }
                    }
                } else {
                    thread::sleep(Duration::from_millis(1));
                }
            }
        });
        tx
    }

    fn start_writing_thread(writing_path: String) -> Sender<SendObject> {
        let (tx, rx) = bounded::<SendObject>(1024);

        thread::spawn(move || {
            println!("Writing Thread start");
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(writing_path)
                .expect("Can't open file");

            if file.metadata().unwrap().len() < FILE_SIZE {
                file.set_len(FILE_SIZE).expect("Can't set file size (4096)");
            }

            let mut m_map = unsafe { MmapMut::map_mut(&file).expect("error on m_map ") };

            let ptr = m_map.as_mut_ptr();
            let mut sequence: u64 = 0;

            //reset file before using it
            unsafe {
                std::ptr::write_bytes(ptr, 0, FILE_SIZE as usize);
            }

            //writing loop
            loop {
                match rx.recv() {
                    Ok(data) => {
                        sequence += 1;
                        unsafe {
                            let increase_bpm_ptr = ptr.add(8) as *mut bool;
                            std::ptr::write_volatile(increase_bpm_ptr, data.increase_bpm);

                            let decrease_bpm_ptr = ptr.add(9) as *mut bool;
                            std::ptr::write_volatile(decrease_bpm_ptr, data.decrease_bpm);

                            let become_master_ptr = ptr.add(10) as *mut bool;
                            std::ptr::write_volatile(become_master_ptr, data.become_master);

                            let on_shoot_modus_ptr = ptr.add(11) as *mut bool;
                            std::ptr::write_volatile(on_shoot_modus_ptr, data.on_shoot_modus);

                            let small_counter_ptr = ptr.add(14);
                            std::ptr::write_volatile(small_counter_ptr, data.small_counter);

                            let add_sound_on_small_beat_ptr = ptr.add(15) as *mut bool;
                            std::ptr::write_volatile(
                                add_sound_on_small_beat_ptr,
                                data.add_sound_on_small_beat,
                            );

                            let path_dest_ptr = ptr.add(16);
                            let path_src_ptr = data.selected_sound_path.as_ptr();
                            std::ptr::copy_nonoverlapping(path_src_ptr, path_dest_ptr, 256);

                            let remove_sound_on_small_beat_ptr = ptr.add(272) as *mut bool;
                            std::ptr::write_volatile(
                                remove_sound_on_small_beat_ptr,
                                data.remove_sound_on_small_beat,
                            );

                            let remove_sound_path_dest_ptr = ptr.add(280);
                            let remove_sound_path_ptr_ptr = data.remove_sound_path.as_ptr();
                            std::ptr::copy_nonoverlapping(
                                remove_sound_path_ptr_ptr,
                                remove_sound_path_dest_ptr,
                                256,
                            );

                            let knob_value_ptr = ptr.add(536);
                            std::ptr::write_volatile(knob_value_ptr, data.knob_value);

                            let knob_echo_ptr = ptr.add(537) as *mut bool;
                            std::ptr::write_volatile(knob_echo_ptr, data.knob_echo);

                            let knob_reverb_ptr = ptr.add(538) as *mut bool;
                            std::ptr::write_volatile(knob_reverb_ptr, data.knob_reverb);

                            let knob_distortion_ptr = ptr.add(539) as *mut bool;
                            std::ptr::write_volatile(knob_distortion_ptr, data.knob_distortion);

                            let seq_ptr = ptr as *mut u64;
                            std::ptr::write_volatile(seq_ptr, sequence);
                        }
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
        });
        tx
    }

    fn start_reading_thread(reading_path: String, thread_shared_data: Arc<Mutex<ReceiveObject>>) {
        thread::spawn(move || {
            let file_result = OpenOptions::new().read(true).write(true).open(reading_path);

            let file = match file_result {
                Ok(file) => file,
                Err(error) => {
                    eprintln!("Problem opening the file: {error:?}");
                    return;
                }
            };

            let m_map_result = unsafe { MmapMut::map_mut(&file) };

            let m_map = match m_map_result {
                Ok(m_map) => m_map,
                Err(error) => {
                    eprintln!("Problem opening the file: {error:?}");
                    return;
                }
            };

            let receive_ptr = m_map.as_ptr() as *const ReceiveObject;
            let mut last_sequence: u64 = 0;

            loop {
                unsafe {
                    let current_sequence = std::ptr::read_volatile(&(*receive_ptr).sequence);

                    if current_sequence > last_sequence {
                        let data = std::ptr::read_volatile(receive_ptr);
                        last_sequence = current_sequence;

                        if let Ok(mut guard) = thread_shared_data.lock() {
                            guard.bpm = data.bpm;
                            guard.total_counter = data.total_counter;
                            guard.small_counter = data.small_counter;
                            guard.is_master = data.is_master;
                            guard.is_on_shoot_modus = data.is_on_shoot_modus;
                            guard.sounds = data.sounds;
                        }
                    } else {
                        thread::sleep(Duration::from_millis(1));
                    }
                }
            }
        });
    }
}
