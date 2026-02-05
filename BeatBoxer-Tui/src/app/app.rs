pub struct App {
    track_title: String,
    artist: String,
    bpm: f64,
    is_playing: bool,
    small_counter: u8,
}

impl App {
    pub fn new() -> Self {
        Self {
            track_title: "Wait on CDJ..".to_string(),
            artist: "xxx".to_string(),
            bpm: 0.0,
            is_playing: false,
            small_counter: 0,
        }
    }
}
