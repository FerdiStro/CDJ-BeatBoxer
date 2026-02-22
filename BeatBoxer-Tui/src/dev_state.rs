use crate::app::app::App;
use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use dotenvy::dotenv;
use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, List, ListState},
    DefaultTerminal,
};
use std::string::ToString;
use std::{env, fs, io};

pub enum DevStatus {
    App,
    Dev,
    DevSel,
    Sel,
}

pub struct DevState {
    pub status: DevStatus,
    items: Vec<&'static str>,
    list_state: ListState,
    path: String,
}

impl DevState {
    const FILE_PATH_WRITING_PROD: &str =
        "/home/ferdinoond/CDJ-BeatBoxer/BeatBoxer-Engien/toEngien_shm.bin";
    const FILE_PATH_READING_PROD: &str =
        "/home/ferdinoond/CDJ-BeatBoxer/BeatBoxer-Engien/fromEngien_shm.bin";

    const FILE_PATH_WRITING_DEV: &str =
        "/Users/maintenance/Projects/CDJ-BeatBoxer/toEngien_shm.bin";
    const FILE_PATH_READING_DEV: &str =
        "/Users/maintenance/Projects/CDJ-BeatBoxer/fromEngien_shm.bin";

    const SOUND_LIB_FOLDER_PROD: &str = "/home/ferdinoond/CDJ-BeatBoxer/BeatBoxer-Sounds";
    const SOUND_LIB_FOLDER_DEV: &str = "/Users/maintenance/Projects/CDJ-BeatBoxer/BeatBoxer-Sounds";

    const FILE_IGNORE_WRITING: &str = "ui_states/ignore_writing.bin";

    const BEATBOXER_READ_CDJ_PATH_PROD: &str =
        "/home/ferdinoond/CDJ-BeatBoxer/BeatBoxer-Engien/x_player_wave_form.bin";
    const BEATBOXER_READ_CDJ_PATH_DEV: &str =
        "/Users/maintenance/Projects/CDJ-BeatBoxer/XxX_player_wave_form.bin";

    fn get_paths() -> Vec<&'static str> {
        vec![
            "PROD",
            "ui_states/set_Beat_first_state.bin",
            "DEV_SEL",
            "EXIT",
        ]
    }

    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        let is_env_app: bool = env::var("BEATBOXER_ENV_APP")
            .unwrap_or("false".to_string())
            .parse()
            .unwrap_or(false);

        let is_env_select_always: bool = env::var("BEATBOXER_ENV_SELECT_ALWAYS")
            .unwrap_or("false".to_string())
            .parse()
            .unwrap_or(false);

        let mut path: String = "".to_string();

        let status: DevStatus = if is_env_app && !is_env_select_always {
            DevStatus::App
        } else if is_env_app && is_env_select_always {
            DevStatus::DevSel
        } else if is_env_select_always {
            DevStatus::Sel
        } else if !is_env_app && !is_env_select_always {
            let env_path = env::var("BEATBOXER_ENV_SELECTED_PATH").unwrap_or("SEL_NEW".to_string());
            path = env_path.clone();
            if env_path.eq("SEL_NEW") {
                DevStatus::Sel
            } else {
                DevStatus::Dev
            }
        } else {
            DevStatus::Sel
        };

        Self {
            status,
            items: Self::get_paths(),
            list_state,
            path,
        }
    }

    pub fn set_dev_env(&self) {
        unsafe {
            self.update_env_file("BEATBOXER_ENV_APP", "false").unwrap();
            self.update_env_file("BEATBOXER_ENV_SELECT_ALWAYS", "false")
                .unwrap();
            self.update_env_file("BEATBOXER_ENV_SELECTED_PATH", &self.path)
                .unwrap();
            self.update_env_file("BEATBOXER_READ_PATH", &self.path)
                .unwrap();
            self.update_env_file(
                "BEATBOXER_FILE_EXPLORER_PATH",
                DevState::SOUND_LIB_FOLDER_DEV,
            )
            .unwrap();
            self.update_env_file("BEATBOXER_WRITE_PATH", DevState::FILE_IGNORE_WRITING)
                .unwrap();
        }
    }

    pub fn run_dev(&self) -> Result<()> {
        self.set_dev_env();
        App::new()
    }

    unsafe fn update_env_file(&self, key: &str, value: &str) -> io::Result<()> {
        let env_path = dotenv().map_err(|e| io::Error::new(io::ErrorKind::NotFound, e))?;
        let content = fs::read_to_string(&env_path)?;

        let mut new_lines = Vec::new();
        let mut key_found = false;

        for line in content.lines() {
            if let Some((k, _)) = line.split_once('=') {
                if k.trim() == key {
                    new_lines.push(format!("{}={}", key, value));
                    key_found = true;
                    continue;
                }
            }
            new_lines.push(line.to_string());
        }

        if !key_found {
            new_lines.push(format!("{}={}", key, value));
        }

        let new_content = new_lines.join("\n");
        fs::write(&env_path, new_content)?;

        unsafe {
            env::set_var(key, value);
        }

        println!("Updated {} in file {:?}", key, env_path);
        Ok(())
    }

    pub fn run_selection_window(&mut self) -> Result<DevStatus> {
        match self.status {
            DevStatus::DevSel => Ok(DevStatus::DevSel),
            DevStatus::Dev => Ok(DevStatus::Dev),
            DevStatus::App => Ok(DevStatus::App),
            DevStatus::Sel => {
                let terminal = ratatui::init();

                let result = self.run_loop(terminal);

                ratatui::restore();

                match result {
                    Ok(Some(selection)) => {
                        if selection.eq("DEV_SEL") {
                            unsafe {
                                self.update_env_file("BEATBOXER_ENV_APP", "true")?;
                                self.update_env_file("BEATBOXER_ENV_SELECT_ALWAYS", "true")?;
                                self.update_env_file(
                                    "BEATBOXER_READ_PATH",
                                    &DevState::FILE_PATH_READING_DEV,
                                )?;
                                self.update_env_file(
                                    "BEATBOXER_WRITE_PATH",
                                    DevState::FILE_PATH_WRITING_DEV,
                                )?;
                                self.update_env_file(
                                    "BEATBOXER_FILE_EXPLORER_PATH",
                                    DevState::SOUND_LIB_FOLDER_DEV,
                                )?;
                                self.update_env_file(
                                    "BEATBOXER_READ_CDJ_PATH",
                                    DevState::BEATBOXER_READ_CDJ_PATH_DEV,
                                )?;
                            }
                            return Ok(DevStatus::DevSel);
                        }

                        if selection.eq("PROD") {
                            unsafe {
                                self.update_env_file("BEATBOXER_ENV_APP", "true")?;
                                self.update_env_file("BEATBOXER_ENV_SELECT_ALWAYS", "false")?;
                                self.update_env_file(
                                    "BEATBOXER_READ_PATH",
                                    &DevState::FILE_PATH_READING_PROD,
                                )?;
                                self.update_env_file(
                                    "BEATBOXER_WRITE_PATH",
                                    DevState::FILE_PATH_WRITING_PROD,
                                )?;
                                self.update_env_file(
                                    "BEATBOXER_FILE_EXPLORER_PATH",
                                    DevState::SOUND_LIB_FOLDER_PROD,
                                )?;
                                self.update_env_file(
                                    "BEATBOXER_READ_CDJ_PATH",
                                    DevState::BEATBOXER_READ_CDJ_PATH_PROD,
                                )?;
                            }
                            return Ok(DevStatus::App);
                        }
                        if selection.eq("EXIT") {
                            return Ok(DevStatus::Sel);
                        }
                        self.path = selection;
                        Ok(DevStatus::Dev)
                    }
                    Ok(None) => Ok(DevStatus::Sel),
                    Err(e) => Err(e),
                }
            }
        }
    }

    fn run_loop(&mut self, mut terminal: DefaultTerminal) -> Result<Option<String>> {
        loop {
            terminal.draw(|f| {
                let area = f.area();

                let list = List::new(self.items.clone())
                    .block(
                        Block::default()
                            .title(" ↑ Up ↓ Down ↵ Select ")
                            .borders(Borders::ALL),
                    )
                    .highlight_style(Style::default().bg(Color::Magenta).fg(Color::White))
                    .highlight_symbol(">> ");

                f.render_stateful_widget(list, area, &mut self.list_state);
            })?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Up => self.list_state.select_previous(),
                    KeyCode::Down => self.list_state.select_next(),
                    KeyCode::Enter => {
                        let index = self.list_state.selected().unwrap_or(0);

                        return Ok(Some(self.items[index].to_string()));
                    }
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(None),
                    _ => {}
                }
            }
        }
    }
}
