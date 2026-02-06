# CDJ-BeatBoxer

https://ratatui.rs/

### Binary-Struct

Engine -> TUI <br><br>
**IN:** `fromEngien_shm.bin` <br>
**Total-length:** `4096 Bytes`

| Postion | Byte Postion | Length (Byte) | Value        | Type (Java) | Type (Rust) |
|---------|--------------|---------------|--------------|-------------|-------------|
| 0  - 7  | 1            | 8             | SequenceId   | long        | u64         |
| 8  - 15 | 2            | 8             | BPM          | double      | f64         |
| 16 - 19 | 3            | 4             | smallCounter | byte        | u8          |
| 20 - 23 | 3            | 4             | Padding      | -           | -           |
| 24 - 31 | 4            | 8             | totalCounter | long        | u64         |
|         |              |               |              |             |             |

 ---

TUI -> Engine
...
