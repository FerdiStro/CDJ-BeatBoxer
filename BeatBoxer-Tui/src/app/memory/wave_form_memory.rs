use crate::app::memory::objects_wave_from_data::WaveformData;
use crossbeam_channel::{bounded, Sender};
use memmap2::MmapMut;
use ratatui::prelude::Color;
use std::fs::OpenOptions;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{env, thread};

//WaveForm  Values
const HEADER_SIZE: usize = 16;
const MAX_WAVEFORM_SIZE: usize = 150_000;

// 4 (TrackID) + 4 (WFLen) + 150000 (WF) + 4 (BeatLen) + 40000 (Beats) + 80000 (Time)
const TRACK_BUFFER_SIZE: usize = 270_012;
const BUFFER_0_OFFSET: usize = HEADER_SIZE;
const BUFFER_1_OFFSET: usize = HEADER_SIZE + TRACK_BUFFER_SIZE;

pub struct WaveFormMemory {
    pub wave_form_cdj_1_terminal_sender: Option<Sender<usize>>,
    pub wave_form_cdj_2_terminal_sender: Option<Sender<usize>>,
}

impl WaveFormMemory {
    pub fn new(
        cdj_1_shared_data: Arc<Mutex<WaveformData>>,
        cdj_2_shared_data: Arc<Mutex<WaveformData>>,
    ) -> Self {
        let reading_path = env::var("BEATBOXER_READ_CDJ_PATH")
            .expect("CRITICAL: 'BEATBOXER_READ_CDJ_PATH' not set in ENV!");

        let cdj_1_path = reading_path.replace("XxX", "1");
        let wave_form_cdj_1_terminal_sender =
            Some(Self::start_wave_reading_trad(cdj_1_path, cdj_1_shared_data));

        let cdj_2_path = reading_path.replace("XxX", "2");
        let wave_form_cdj_2_terminal_sender =
            Some(Self::start_wave_reading_trad(cdj_2_path, cdj_2_shared_data));

        Self {
            wave_form_cdj_1_terminal_sender,
            wave_form_cdj_2_terminal_sender,
        }
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
                                BUFFER_0_OFFSET
                            } else {
                                BUFFER_1_OFFSET
                            };

                            //Read Track
                            let track_id =
                                u32::from_le_bytes(m_map[offset..offset + 4].try_into().unwrap());

                            let waveform_len = u32::from_le_bytes(
                                m_map[offset + 4..offset + 8].try_into().unwrap(),
                            ) as usize;

                            if waveform_len == 0
                                || waveform_len > MAX_WAVEFORM_SIZE
                                || offset + 8 + waveform_len > m_map.len()
                            {
                                thread::sleep(Duration::from_millis(1));
                                continue;
                            }

                            let wave_raw_data = &m_map[offset + 8..offset + 8 + waveform_len];
                            let wave_start = scroll_offset.min(waveform_len);
                            let wave_end = (wave_start + zoom_width).min(waveform_len);
                            let visible_data = &wave_raw_data[wave_start..wave_end];

                            if visible_data.is_empty() {
                                return;
                            }

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

                            //Read Grid

                            //4 == trackId; 4 == wavelength
                            let grid_offset = offset + MAX_WAVEFORM_SIZE + 4 + 4;

                            let beat_len = u32::from_le_bytes(
                                m_map[grid_offset..grid_offset + 4].try_into().unwrap(),
                            );

                            let safe_beat_len = beat_len.min(10_000) as usize;

                            let mut beat_data: Vec<u32> = vec![0; safe_beat_len];
                            let mut time_data: Vec<u64> = vec![0; safe_beat_len];

                            let beat_bytes_start = offset + 150028;
                            let beat_bytes_end = beat_bytes_start + (safe_beat_len * 4);
                            let beat_bytes = &m_map[beat_bytes_start..beat_bytes_end];

                            for (i, chunk) in beat_bytes.chunks_exact(4).enumerate() {
                                beat_data[i] = u32::from_le_bytes(chunk.try_into().unwrap());
                            }

                            let time_bytes_start = offset + 190028;
                            let time_bytes_end = time_bytes_start + (safe_beat_len * 8);
                            let time_bytes = &m_map[time_bytes_start..time_bytes_end];

                            for (i, chunk) in time_bytes.chunks_exact(8).enumerate() {
                                time_data[i] = u64::from_le_bytes(chunk.try_into().unwrap());
                            }

                            //Map grid to colors
                            let mut precise_grid_colors = vec![None; terminal_width];

                            let window_size = wave_end - wave_start;

                            for (i, &time_ms) in time_data.iter().enumerate() {
                                let byte_idx = ((time_ms * 150) / 1000) as usize;

                                if byte_idx >= wave_start && byte_idx < wave_end {
                                    let relative_pos = byte_idx - wave_start;
                                    let mut col = ((relative_pos as f64 / window_size as f64)
                                        * terminal_width as f64)
                                        as usize;

                                    col = col.min(terminal_width - 1);

                                    let is_downbeat = beat_data[i] == 1;

                                    if is_downbeat {
                                        precise_grid_colors[col] = Some(Color::Red);
                                    } else {
                                        precise_grid_colors[col] = Some(Color::Gray);
                                    }
                                }
                            }

                            if let Ok(mut guard) = thread_shared_data.lock() {
                                guard.amplitudes = amplitudes;
                                guard.track_id = track_id;
                                guard.gird_colors = precise_grid_colors;
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
}
