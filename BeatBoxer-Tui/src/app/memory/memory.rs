use crossbeam_channel::{bounded, Sender};
use memmap2::MmapMut;
use std::fs::OpenOptions;
use std::sync::{Arc, Mutex};
use std::{env, hint, thread};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SendObject {
    pub sequence: u64,
    pub increase_bpm: bool,
    pub decrease_bpm: bool,
    pub become_master: bool,
    _padding_1: [u8; 3],
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
            0, false, false, false, "", 0, false, false, "", 0, false, true, false,
        )
    }
    pub fn knop_distortion() -> Self {
        Self::new(
            0, false, false, false, "", 0, false, false, "", 0, false, false, true,
        )
    }
    pub fn knop_echo() -> Self {
        Self::new(
            0, false, false, false, "", 0, false, false, "", 0, true, false, false,
        )
    }

    pub fn default() -> Self {
        Self::new(
            0, false, false, false, "", 0, false, false, "", 0, false, false, false,
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
            _padding_1: [0; 3],
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
        total_counter: u64,
        sounds: [SoundEntry; 10],
    ) -> Self {
        Self {
            sequence,
            bpm,
            small_counter,
            is_master,
            total_counter,
            sounds,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Memory {
    pub sender: Sender<SendObject>,
}

impl Memory {
    pub fn new(shared_state: Arc<Mutex<ReceiveObject>>) -> Memory {
        let reading_path = env::var("BEATBOXER_READ_PATH")
            .expect("CRITICAL: 'BEATBOXER_READ_PATH' not set in ENV!");

        let writing_path = env::var("BEATBOXER_WRITE_PATH")
            .expect("CRITICAL: 'BEATBOXER_WRITE_PATH' not set in ENV!");

        Self::start_reading_thread(reading_path, shared_state);
        let sender = Self::start_writing_thread(writing_path);
        Self { sender }
    }

    const FILE_SIZE: u64 = 4096;

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

            if file.metadata().unwrap().len() < Self::FILE_SIZE {
                file.set_len(Self::FILE_SIZE)
                    .expect("Can't set file size (4096)");
            }

            let mut m_map = unsafe { MmapMut::map_mut(&file).expect("error on m_map ") };

            let ptr = m_map.as_mut_ptr();
            let mut sequence: u64 = 0;

            loop {
                match rx.try_recv() {
                    Ok(data) => {
                        sequence += 1;
                        unsafe {
                            let increase_bpm_ptr = ptr.add(8) as *mut bool;
                            std::ptr::write_volatile(increase_bpm_ptr, data.increase_bpm);

                            let decrease_bpm_ptr = ptr.add(9) as *mut bool;
                            std::ptr::write_volatile(decrease_bpm_ptr, data.decrease_bpm);

                            let become_master_ptr = ptr.add(10) as *mut bool;
                            std::ptr::write_volatile(become_master_ptr, data.become_master);

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

                            let seq_ptr = ptr as *mut u64;
                            std::ptr::write_volatile(seq_ptr, sequence);
                        }
                    }
                    Err(crossbeam_channel::TryRecvError::Empty) => {
                        hint::spin_loop();
                    }
                    Err(crossbeam_channel::TryRecvError::Disconnected) => {
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
                            guard.sounds = data.sounds;
                        }
                    } else {
                        hint::spin_loop();
                    }
                }
            }
        });
    }
}
