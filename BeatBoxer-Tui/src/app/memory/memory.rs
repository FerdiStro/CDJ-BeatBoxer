use memmap2::MmapMut;
use std::fs::OpenOptions;
use std::sync::{Arc, Mutex};
use std::{hint, thread};

#[derive(Debug, Copy, Clone, Default)]
pub struct Memory {}

impl Memory {
    pub fn new(shared_state: Arc<Mutex<ReceiveObject>>) {
        Self::start_reading_thread(shared_state);
        Self::start_writing_thread();
    }

    const FILE_PATH_READING: &str =
        "/home/ferdinoond/CDJ-BeatBoxer/BeatBoxer-Engien/fromEngien_shm.bin";

    fn start_reading_thread(thread_shared_data: Arc<Mutex<ReceiveObject>>) {
        thread::spawn(move || {
            println!("Reading Thread start");
            let file_result = OpenOptions::new()
                .read(true)
                .write(true)
                .open(Self::FILE_PATH_READING);

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
                        }
                    } else {
                        hint::spin_loop();
                    }
                }
            }
        });
    }

    fn start_writing_thread(){

    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct ReceiveObject {
    pub sequence: u64,
    pub bpm: f64,
    pub small_counter: u8,
    pub total_counter: u64,
}
