use crossterm::event;
use crossterm::event::{Event, KeyCode};
use ratatui::widgets::Borders;
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, List, ListItem, ListState},
    Terminal,
};
use std::error::Error;
use std::io;
use std::sync::{Arc, Mutex};
use BeatBoxer_Tui::app::memory::memory::{Memory, ReceiveObject, SendObject};
use BeatBoxer_Tui::dev_state::{DevState, DevStatus};

fn main() -> Result<(), Box<dyn Error>> {
    let mut dev_state = DevState::new();

    match dev_state.run_selection_window()? {
        DevStatus::Dev => {
            dev_state.set_dev_env();
        }
        _ => {}
    }

    let shared_data = Arc::new(Mutex::new(ReceiveObject::default()));
    let thread_shared_data = shared_data.clone();

    let memory = Memory::new(thread_shared_data);

    crossterm::terminal::enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    let commands = vec![
        "Quit",
        "Add-Bar-1",
        "Remove-Bar-1",
        "Knop-Echo-64",
        "Knop-Echo-0",
        "Knop-Dis-120",
        "Knop-Dis-0",
        "Knop-Reverb-64",
        "Knop-Reverb-0",
    ];
    let mut state = ListState::default();
    state.select(Some(0));

    loop {
        terminal.draw(|f| {
            let items: Vec<ListItem> = commands.iter().map(|c| ListItem::new(*c)).collect();
            let list = List::new(items)
                .block(
                    Block::default()
                        .title("Drum-machine-commands")
                        .borders(Borders::ALL),
                )
                .highlight_symbol(" >> ");

            f.render_stateful_widget(list, f.area(), &mut state);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Up => {
                    let i = state.selected().unwrap_or(0);
                    state.select(Some(i.saturating_sub(1)));
                }
                KeyCode::Down => {
                    let i = state.selected().unwrap_or(0);
                    if i < commands.len() - 1 {
                        state.select(Some(i + 1));
                    }
                }
                KeyCode::Enter => {
                    let selected = commands[state.selected().unwrap_or(0)];
                    let mut send_object = SendObject::default();

                    match selected {
                        "Quit" => break,
                        "Add-Bar-1" => {
                            send_object.selected_sound_path = SendObject::convert_string_byte(
                                "/Users/maintenance/Projects/CDJ-BeatBoxer/BeatBoxer-Sounds/airy-groove-kick.wav",
                            );
                            send_object.add_sound_on_small_beat = true;
                            send_object.small_counter = 0;
                            memory.sender.send(send_object)?;
                        }
                        "Remove-Bar-1" => {
                            send_object.remove_sound_path = SendObject::convert_string_byte(
                                "/Users/maintenance/Projects/CDJ-BeatBoxer/BeatBoxer-Sounds/airy-groove-kick.wav",
                            );
                            send_object.remove_sound_on_small_beat = true;
                            send_object.small_counter = 0;
                            memory.sender.send(send_object)?;
                        }
                        "Knop-Echo-64" => {
                            send_object.knob_echo = true;
                            send_object.knob_value = 64;
                            memory.sender.send(send_object)?;
                        }
                        "Knop-Echo-0" => {
                            send_object.knob_echo = true;
                            send_object.knob_value = 0;
                            memory.sender.send(send_object)?;
                        }

                        "Knop-Reverb-64" => {
                            send_object.knob_reverb = true;
                            send_object.knob_value = 64;
                            memory.sender.send(send_object)?;
                        }
                        "Knop-Reverb-0" => {
                            send_object.knob_reverb = true;
                            send_object.knob_value = 0;
                            memory.sender.send(send_object)?;
                        }
                        "Knop-Dis-120" => {
                            send_object.knob_distortion = true;
                            send_object.knob_value = 120;
                            memory.sender.send(send_object)?;
                        }
                        "Knop-Dis-0" => {
                            send_object.knob_distortion = true;
                            send_object.knob_value = 0;
                            memory.sender.send(send_object)?;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}
