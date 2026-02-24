# Keyboard

| KEY           | Description                                                                                   | Rust Action             | optional - memoryCommand |
|:--------------|:----------------------------------------------------------------------------------------------|:------------------------|:-------------------------|
| q             | Quit TUI                                                                                      | AppAction::Quit         | -                        |     
| 1             | Pressing 1 Bar (DrumPad). Add selected Sound to 0 beat                                        | AppAction::Bar1         | ADD_BEAT_SMALL           |     
| 2             | Pressing 2 Bar (DrumPad). Add selected Sound to 1 beat                                        | AppAction::Bar2         | ADD_BEAT_SMALL           |     
| 3             | Pressing 3 Bar (DrumPad). Add selected Sound to 2 beat                                        | AppAction::Bar3         | ADD_BEAT_SMALL           |     
| 4             | Pressing 4 Bar (DrumPad). Add selected Sound to 3 beat                                        | AppAction::Bar4         | ADD_BEAT_SMALL           |     
| 5             | Pressing 5 Bar (DrumPad). Add selected Sound to 4 beat                                        | AppAction::Bar5         | ADD_BEAT_SMALL           |     
| 6             | Pressing 6 Bar (DrumPad). Add selected Sound to 5 beat                                        | AppAction::Bar6         | ADD_BEAT_SMALL           |     
| 7             | Pressing 7 Bar (DrumPad). Add selected Sound to 6 beat                                        | AppAction::Bar7         | ADD_BEAT_SMALL           |     
| 8             | Pressing 8 Bar (DrumPad). Add selected Sound to 7 beat                                        | AppAction::Bar8         | ADD_BEAT_SMALL           |     
| →             | Arrow Righ moves first navigation bar to Right (Setting, BPM-Section, FileBrowser)            | AppAction::NextMode     | -                        |     
| ←             | Arrow Left moves first navigation bar to Left (Setting, BPM-Section, FileBrowser)             | AppAction::PreviousMode | -                        |     
| ↑             | Arrow Up moves second navigation bar to Right (BPM-Lock, BARS)                                | AppAction::NextMode     | -                        |     
| ↓             | Arrow Down moves second navigation bar to Left (BPM-Lock, BARS)                               | AppAction::PreviousMode | -                        |     
| ENTER         | ENTER Selects Sound when is slected in filebrowser.                                           | AppAction::Submit       | -                        |     
| BACKSPACE     | BACKSPACE navigates to the folder before (FileBrowser like cd ..)                             | AppAction::Backspace    | -                        |     
| TAB           | TAB aktivate shift-mode (not hold tab is toggle) Shift mode allows second bindings on buttons | AppAction::Shift        | -                        |     
| ShiftMode + 1 | Pressing 1 Bar (DrumPad) with shiftmode. Remove selected Sound to 0 beat                      | AppAction::Bar1         | REMOVE_BEAT_SMALL        |     
| ShiftMode + 2 | Pressing 2 Bar (DrumPad) with shiftmode. Remove selected Sound to 1 beat                      | AppAction::Bar2         | REMOVE_BEAT_SMALL        |     
| ShiftMode + 3 | Pressing 3 Bar (DrumPad) with shiftmode. Remove selected Sound to 2 beat                      | AppAction::Bar3         | REMOVE_BEAT_SMALL        |     
| ShiftMode + 4 | Pressing 4 Bar (DrumPad) with shiftmode. Remove selected Sound to 3 beat                      | AppAction::Bar4         | REMOVE_BEAT_SMALL        |     
| ShiftMode + 5 | Pressing 5 Bar (DrumPad) with shiftmode. Remove selected Sound to 4 beat                      | AppAction::Bar5         | REMOVE_BEAT_SMALL        |     
| ShiftMode + 6 | Pressing 6 Bar (DrumPad) with shiftmode. Remove selected Sound to 5 beat                      | AppAction::Bar6         | REMOVE_BEAT_SMALL        |     
| ShiftMode + 7 | Pressing 7 Bar (DrumPad) with shiftmode. Remove selected Sound to 6 beat                      | AppAction::Bar7         | REMOVE_BEAT_SMALL        |     
| ShiftMode + 8 | Pressing 8 Bar (DrumPad) with shiftmode. Remove selected Sound to 7 beat                      | AppAction::Bar8         | REMOVE_BEAT_SMALL        |   

# Midi

| KEY                  | Description                                                                                     | Rust Action             | optional - memoryCommand |
|:---------------------|:------------------------------------------------------------------------------------------------|:------------------------|:-------------------------|
| [1] = 36             | Pressing 1 (36) Bar (DrumPad). Add selected Sound to 0 beat                                     | AppAction::Bar1         | ADD_BEAT_SMALL           |     
| [1] = 37             | Pressing 2 (37) Bar (DrumPad). Add selected Sound to 1 beat                                     | AppAction::Bar2         | ADD_BEAT_SMALL           |     
| [1] = 38             | Pressing 3 (38) Bar (DrumPad). Add selected Sound to 2 beat                                     | AppAction::Bar3         | ADD_BEAT_SMALL           |     
| [1] = 39             | Pressing 4 (39) Bar (DrumPad). Add selected Sound to 3 beat                                     | AppAction::Bar4         | ADD_BEAT_SMALL           |     
| [1] = 40             | Pressing 5 (40) Bar (DrumPad). Add selected Sound to 4 beat                                     | AppAction::Bar5         | ADD_BEAT_SMALL           |     
| [1] = 41             | Pressing 6 (41) Bar (DrumPad). Add selected Sound to 5 beat                                     | AppAction::Bar6         | ADD_BEAT_SMALL           |     
| [1] = 42             | Pressing 7 (42) Bar (DrumPad). Add selected Sound to 6 beat                                     | AppAction::Bar7         | ADD_BEAT_SMALL           |     
| [1] = 43             | Pressing 8 (43) Bar (DrumPad). Add selected Sound to 7 beat                                     | AppAction::Bar8         | ADD_BEAT_SMALL           |     
| [1] = 114            | Knopp (114) moves first navigation bar to Right (Setting, BPM-Section, FileBrowser)             | AppAction::NextMode     | -                        |     
| [1] = 114 + shift    | Knopp (114) + Shiftmode moves first navigation bar to Left (Setting, BPM-Section, FileBrowser)  | AppAction::PreviousMode | -                        |     
| [1] = 112            | Knopp (112) moves second navigation bar to Right (BPM-Lock, BARS)                               | AppAction::NextMode     | -                        |     
| [1] = 112 + shift    | Knopp (112) + Shiftmode moves second navigation bar to Left (BPM-Lock, BARS)                    | AppAction::PreviousMode | -                        |     
| [1] = 113 [2] = 127  | Pressing Knop 113 Selects Sound when is slected in filebrowser.                                 | AppAction::Submit       | -                        |     
| [1] = 115 [2] = 127  | Pressing Knop 113 Selects Sound when is slected in filebrowser.                                 | AppAction::Submit       | -                        |     
| [1] = 48             | 48 aktivate shift-mode on hold (NOT LIKE KEYBOARD) Shift mode allows second bindings on buttons | AppAction::Shift        | -                        |     
| ShiftMode + [1] = 36 | Pressing 1 (36) Bar (DrumPad) with shiftmode. Remove selected Sound to 0 beat                   | AppAction::Bar1         | REMOVE_BEAT_SMALL        |     
| ShiftMode + [1] = 37 | Pressing 2 (37) Bar (DrumPad) with shiftmode. Remove selected Sound to 1 beat                   | AppAction::Bar2         | REMOVE_BEAT_SMALL        |     
| ShiftMode + [1] = 38 | Pressing 3 (38) ar (DrumPad) with shiftmode. Remove selected Sound to 2 beat                    | AppAction::Bar3         | REMOVE_BEAT_SMALL        |     
| ShiftMode + [1] = 39 | Pressing 4 (39) Bar (DrumPad) with shiftmode. Remove selected Sound to 3 beat                   | AppAction::Bar4         | REMOVE_BEAT_SMALL        |     
| ShiftMode + [1] = 40 | Pressing 5 (40) Bar (DrumPad) with shiftmode. Remove selected Sound to 4 beat                   | AppAction::Bar5         | REMOVE_BEAT_SMALL        |     
| ShiftMode + [1] = 41 | Pressing 6 (41) Bar (DrumPad) with shiftmode. Remove selected Sound to 5 beat                   | AppAction::Bar6         | REMOVE_BEAT_SMALL        |     
| ShiftMode + [1] = 42 | Pressing 7 (42) Bar (DrumPad) with shiftmode. Remove selected Sound to 6 beat                   | AppAction::Bar7         | REMOVE_BEAT_SMALL        |     
| ShiftMode + [1] = 43 | Pressing 8 (43) Bar (DrumPad) with shiftmode. Remove selected Sound to 7 beat                   | AppAction::Bar8         | REMOVE_BEAT_SMALL        |  