use crate::app::memory::objects_main_data::{ReceiveObject, SendObject};
use crossbeam_channel::{bounded, Sender};
use memmap2::MmapMut;
use std::fs::OpenOptions;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{env, thread};


const FILE_SIZE: u64 = 4096;

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
        Self {
            sender,
        }
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
