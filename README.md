# CDJ-BeatBoxer

https://ratatui.rs/

### Binary-Struct

Engine -> TUI <br><br>
**IN:** `fromEngien_shm.bin` <br>
**Total-length:** `4096 Bytes`

| Postion | Length (Bytes) | Value        | Type (Java) | Type (Rust) |
|---------|----------------|--------------|-------------|-------------|
| 0 - 7   | 8              | SequenceId   | long        | u64         |
| 8 - 15  | 8              | BPM          | double      | f64         |
| 16      | 1              | smallCounter | byte        | u8          |
| 17      | 1              | isMaster     | boolean     | bool        |
| 18 - 23 | 6              | Padding      | -           | [u8; 6]     |
| 24 - 31 | 8              | totalCounter | long        | u64         |

 ---

TUI -> Engine<br><br>
**IN:** `toEngien_shm.bin` <br>
**Total-length:** `4096 Bytes`

| Postion  | Length (Byte) | Value                   | Type (Java) | Type (Rust) |
|----------|---------------|-------------------------|-------------|-------------|
| 0 - 7    | 8             | SequenceId              | long        | u64         |
| 8        | 1             | increase_bpm            | boolean     | bool        |
| 9        | 1             | decrease_bpm            | boolean     | bool        |
| 10       | 1             | become_master           | boolean     | bool        |
| 11 - 13  | 3             | Padding                 | -           | [u8; 3]     |
| 14       | 1             | small_counter           | byte        | u8          |
| 15       | 1             | add_sound_on_small_beat | boolean     | bool        |
| 16 - 271 | 256           | selected_sound_path     | byte[]      | [u8; 256]   |

