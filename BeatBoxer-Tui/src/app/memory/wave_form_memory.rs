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

        let zoom_width = 500;

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
            let mut terminal_width = 100;
            let mut force_redraw = false;
            let mut last_play_header_index = 0;

            let mut cached_beat_data: Vec<u32> = Vec::new();
            let mut cached_time_data: Vec<u64> = Vec::new();

            loop {
                while let Ok(width) = rx.try_recv() {
                    if terminal_width != width {
                        terminal_width = width;
                        force_redraw = true;
                    }
                }

                let current_active_buffer = m_map[0];
                let mut current_play_header_index = last_play_header_index;

                if let Ok(bytes) = m_map[8..16].try_into() {
                    current_play_header_index = u64::from_le_bytes(bytes);
                }

                let is_new_track = current_active_buffer != last_active_buffer;
                let play_head_moved = current_play_header_index != last_play_header_index;

                if is_new_track ||  play_head_moved || force_redraw {

                    let offset = if current_active_buffer == 0 {
                        BUFFER_0_OFFSET
                    } else {
                        BUFFER_1_OFFSET
                    };

                    let track_id =
                        u32::from_le_bytes(m_map[offset..offset + 4].try_into().unwrap());
                    let waveform_len =
                        u32::from_le_bytes(m_map[offset + 4..offset + 8].try_into().unwrap())
                            as usize;

                    if waveform_len == 0
                        || waveform_len > MAX_WAVEFORM_SIZE
                        || offset + 8 + waveform_len > m_map.len()
                    {
                        thread::sleep(Duration::from_millis(1));
                        continue;
                    }

                    if is_new_track {
                        let grid_offset = offset + MAX_WAVEFORM_SIZE + 4 + 4;
                        let beat_len = u32::from_le_bytes(
                            m_map[grid_offset..grid_offset + 4].try_into().unwrap(),
                        );
                        let safe_beat_len = beat_len.min(10_000) as usize;

                        cached_beat_data = vec![0; safe_beat_len];
                        cached_time_data = vec![0; safe_beat_len];

                        let beat_bytes_start = offset + 150028;
                        let beat_bytes_end = beat_bytes_start + (safe_beat_len * 4);
                        for (i, chunk) in m_map[beat_bytes_start..beat_bytes_end]
                            .chunks_exact(4)
                            .enumerate()
                        {
                            cached_beat_data[i] = u32::from_le_bytes(chunk.try_into().unwrap());
                        }

                        let time_bytes_start = offset + 190028;
                        let time_bytes_end = time_bytes_start + (safe_beat_len * 8);
                        for (i, chunk) in m_map[time_bytes_start..time_bytes_end]
                            .chunks_exact(8)
                            .enumerate()
                        {
                            cached_time_data[i] = u64::from_le_bytes(chunk.try_into().unwrap());
                        }
                    }

                    let zoom_width_i64 = zoom_width as i64;
                    let half_window = zoom_width_i64 / 2;
                    let play_head_i64 = current_play_header_index as i64;

                    let conceptual_start = play_head_i64 - half_window;
                    let conceptual_end = play_head_i64 + half_window;

                    let valid_start = conceptual_start.max(0) as usize;
                    let valid_end = (conceptual_end.max(0) as usize).min(waveform_len);

                    let wave_raw_data = &m_map[offset + 8..offset + 8 + waveform_len];
                    let visible_data = if valid_start < valid_end {
                        &wave_raw_data[valid_start..valid_end]
                    } else {
                        &[]
                    };

                    let mut amplitudes = vec![0; terminal_width];

                    if !visible_data.is_empty() {
                        for (i, &b) in visible_data.iter().enumerate() {
                            let byte_idx = valid_start as i64 + i as i64;

                            let relative_pos = byte_idx - conceptual_start;

                            let mut col = ((relative_pos as f64 / zoom_width_i64 as f64)
                                * terminal_width as f64)
                                as usize;
                            col = col.min(terminal_width - 1);

                            let amp = (b & 0x1F) as u32;
                            if amp > amplitudes[col] {
                                amplitudes[col] = amp;
                            }
                        }
                    }

                    let mut precise_grid_colors = vec![None; terminal_width];

                    for (i, &time_ms) in cached_time_data.iter().enumerate() {
                        //todo: get bytes per second better ??? depend on track
                        let byte_idx = ((time_ms * 75) / 1000) as i64;



                        if byte_idx >= conceptual_start && byte_idx < conceptual_end {
                            let relative_pos = byte_idx - conceptual_start;
                            let mut col = ((relative_pos as f64 / zoom_width_i64 as f64)
                                * terminal_width as f64)
                                as usize;
                            col = col.min(terminal_width - 1);

                            let is_downbeat = cached_beat_data[i] == 1;
                            if is_downbeat {
                                precise_grid_colors[col] = Some(Color::Red);
                            }
                        }
                    }

                    if let Ok(mut guard) = thread_shared_data.lock() {
                        guard.amplitudes = amplitudes;
                        guard.track_id = track_id;
                        guard.gird_colors = precise_grid_colors;
                        guard.play_header = Some(current_play_header_index);
                    }

                    last_active_buffer = current_active_buffer;
                    last_play_header_index = current_play_header_index;
                    force_redraw = false;
                }


                thread::sleep(Duration::from_millis(1));
            }
        });
        tx
    }
}
