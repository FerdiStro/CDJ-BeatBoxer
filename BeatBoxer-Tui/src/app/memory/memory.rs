use crossbeam_channel::{bounded, Sender};
use memmap2::MmapMut;
use std::fs::OpenOptions;
use std::sync::{Arc, Mutex};
use std::{hint, thread};

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct SendObject {
    pub sequence: u64,
    pub increase_bpm: bool,
    pub decrease_bpm: bool,
    pub become_master: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct ReceiveObject {
    pub sequence: u64,
    pub bpm: f64,
    pub small_counter: u8,
    pub is_master: bool,
    pub total_counter: u64,
}

#[derive(Debug, Clone)]
pub struct Memory {
    pub sender: Sender<SendObject>,
}

impl Memory {
    pub fn new(shared_state: Arc<Mutex<ReceiveObject>>) -> Memory {
        Self::start_reading_thread(shared_state);
        let sender = Self::start_writing_thread();
        Self { sender }
    }
    const FILE_PATH_WRITING: &str =
        "/home/ferdinoond/CDJ-BeatBoxer/BeatBoxer-Engien/toEngien_shm.bin";
    const FILE_PATH_READING: &str =
        "/home/ferdinoond/CDJ-BeatBoxer/BeatBoxer-Engien/fromEngien_shm.bin";
    // const FILE_PATH_READING: &str = "/Users/maintenance/Projects/CDJ-BeatBoxer/fromEngien_shm.bin";
    // const FILE_PATH_WRITING: &str = "/Users/maintenance/Projects/CDJ-BeatBoxer/toEngien_shm.bin";

    const FILE_SIZE: u64 = 4096;

    fn start_writing_thread() -> Sender<SendObject> {
        let (tx, rx) = bounded::<SendObject>(1024);

        thread::spawn(move || {
            println!("Writing Thread start");
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(Self::FILE_PATH_WRITING)
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

                            let increase_bpm_ptr = ptr.add(9) as *mut bool;
                            std::ptr::write_volatile(increase_bpm_ptr, data.decrease_bpm);

                            let increase_bpm_ptr = ptr.add(10) as *mut bool;
                            std::ptr::write_volatile(increase_bpm_ptr, data.become_master);

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
                            guard.is_master = data.is_master;
                        }
                    } else {
                        hint::spin_loop();
                    }
                }
            }
        });
    }
}
