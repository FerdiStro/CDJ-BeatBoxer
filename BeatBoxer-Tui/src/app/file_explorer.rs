use ratatui::layout::Rect;
use ratatui::widgets::ListState;
use std::path::PathBuf;
use std::{env, fs};

use crate::app::app::App;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, List, ListItem};

pub struct FileExplorer {
    pub current_dir: PathBuf,
    pub items: Vec<PathBuf>,
    pub state: ListState,
}

impl FileExplorer {
    pub fn render_files(app: &mut App, frame: &mut ratatui::Frame, area: Rect) {
        let items: Vec<ListItem> = app
            .file_explorer
            .items
            .iter()
            .map(|path| {
                let path_string = path.to_string_lossy();

                if path_string == ".." {
                    return ListItem::new("../").style(
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    );
                }

                let file_name = path.file_name().unwrap_or_default().to_string_lossy();

                let is_active_sound = *path == app.selected_sound;

                if is_active_sound {
                    ListItem::new(format!("🔊 {}", file_name))
                        .style(Style::default().fg(Color::Green).bold())
                } else {
                    let (icon, style) = if path.is_dir() {
                        (
                            "📁",
                            Style::default()
                                .fg(Color::White)
                                .add_modifier(Modifier::BOLD),
                        )
                    } else {
                        ("📄", Style::default().fg(Color::White))
                    };

                    ListItem::new(format!("{} {}", icon, file_name)).style(style)
                }
            })
            .collect();

        let list = List::new(items)
            .block(Block::bordered().title("File Browser"))
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");
        frame.render_stateful_widget(list, area, &mut app.file_explorer.state)
    }

    pub fn new() -> Self {
        let sound_lib_folder = env::var("BEATBOXER_FILE_EXPLORER_PATH")
            .expect("CRITICAL: 'BEATBOXER_FILE_EXPLORER_PATH' not set in ENV!");

        let mut explorer = Self {
            current_dir: PathBuf::from(sound_lib_folder),
            items: vec![],
            state: ListState::default(),
        };
        explorer.read_dir();
        explorer
    }

    pub fn read_dir(&mut self) {
        self.items.clear();

        if self.current_dir.parent().is_some() {
            self.items.push(PathBuf::from(".."));
        }

        if let Ok(entries) = fs::read_dir(&self.current_dir) {
            for entry in entries.flatten() {
                self.items.push(entry.path());
            }
        }
        self.items.sort();
        self.state.select(Some(0));
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn is_start(&self) -> bool {
        self.state.selected() == Some(0)
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
