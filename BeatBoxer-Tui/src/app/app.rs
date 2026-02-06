use memmap2::MmapMut;
use std::fs::OpenOptions;
use std::{hint, thread};
use std::sync::{Arc, Mutex};

pub struct App {
     track_title: String,
    artist: String,
    pub bpm: f64,
    is_playing: bool,
    pub small_counter: u8,
    pub total_counter: u64,
    shared_state : Arc<Mutex<ReceiveObject>>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct ReceiveObject {
    pub sequence: u64,
    pub bpm: f64,
    pub small_counter: u8,
    pub total_counter: u64,
}

impl App {
    const FILE_PATH: &str = "../fromEngien_shm.bin";


    pub fn update(&mut self) {
        if let Ok(guard) = self.shared_state.lock() {
            self.bpm = guard.bpm;
            self.small_counter = guard.small_counter;
            self.total_counter = guard.total_counter;
        }
    }

    pub fn new() -> Self {
        let shared_data = Arc::new(Mutex::new(ReceiveObject::default()));
        let thread_shared_data = shared_data.clone();

        let handle = thread::spawn(move || {
            println!("Reading Thread start");
            let file_result = OpenOptions::new()
                .read(true)
                .write(true)
                .open(Self::FILE_PATH);

            let file = match file_result {
                Ok(file) => file,
                Err(error) => panic!("Problem opening the file: {error:?}"),
            };

            let mmap_result = unsafe { MmapMut::map_mut(&file) };

            let mmap = match mmap_result {
                Ok(mmap) => mmap,
                Err(error) => panic!("Problem opening the file: {error:?}"),
            };

            let ptr = mmap.as_ptr();

            let receive_ptr = ptr as *const ReceiveObject;

            let mut last_sequence: u64 = 0;

            loop {
                unsafe {
                    let current_sequence = std::ptr::read_volatile(&(*receive_ptr).sequence);

                    if current_sequence > last_sequence {
                        let data = std::ptr::read_volatile(receive_ptr);
                        last_sequence = current_sequence;

                        println!("BPM :{}", data.bpm);

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

        Self {
            track_title: "Wait on CDJ..".to_string(),
            artist: "xxx".to_string(),
            bpm: 0.0,
            total_counter: 0,
            is_playing: false,
            small_counter: 0,
            shared_state: shared_data,
        }
    }
}
