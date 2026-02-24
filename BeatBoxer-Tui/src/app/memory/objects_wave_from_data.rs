use ratatui::style::Color;

#[derive(Default)]
pub struct WaveformData {
    pub track_id: u32,
    pub amplitudes: Vec<u32>,
    pub gird_colors: Vec<Option<Color>>,
    pub play_header: Option<u64>,
}
