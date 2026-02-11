# CDJ-BeatBoxer

https://ratatui.rs/

### Binary-Struct

Engine -> TUI <br><br>
**IN:** `fromEngien_shm.bin` <br>
**Total-length:** `4096 Bytes`

| Position (Start - End) | Length (Bytes) | Field Name            | Type (Java) | Type (Rust) |
|:-----------------------|:---------------|:----------------------|:------------|:------------|
| **0 - 7**              | 8              | `SequenceId`          | `long`      | `u64`       |
| **8 - 15**             | 8              | `BPM`                 | `double`    | `f64`       |
| **16**                 | 1              | `smallCounter`        | `byte`      | `u8`        |
| **17**                 | 1              | `isMaster`            | `boolean`   | `bool`      |
| **18 - 23**            | 6              | *Padding*             | -           | `[u8; 6]`   |
| **24 - 31**            | 8              | `totalCounter`        | `long`      | `u64`       |
|                        |                | **--- SOUND 1 ---**   |             |             |
| **32 - 287**           | 256            | `sound_1_path`        | `byte[]`    | `[u8; 256]` |
| **288**                | 1              | `sound_1_slot`        | `byte`      | `u8`        |
| **289 - 295**          | 7              | *Padding (Align 264)* | -           | `[u8; 7]`   |
|                        |                | **--- SOUND 2 ---**   |             |             |
| **296 - 551**          | 256            | `sound_2_path`        | `byte[]`    | `[u8; 256]` |
| **552**                | 1              | `sound_2_slot`        | `byte`      | `u8`        |
| **553 - 559**          | 7              | *Padding (Align 264)* | -           | `[u8; 7]`   |
|                        |                | **--- SOUND 3 ---**   |             |             |
| **560 - 815**          | 256            | `sound_3_path`        | `byte[]`    | `[u8; 256]` |
| **816**                | 1              | `sound_3_slot`        | `byte`      | `u8`        |
| **817 - 823**          | 7              | *Padding (Align 264)* | -           | `[u8; 7]`   |
|                        |                | **--- SOUND 4 ---**   |             |             |
| **824 - 1079**         | 256            | `sound_4_path`        | `byte[]`    | `[u8; 256]` |
| **1080**               | 1              | `sound_4_slot`        | `byte`      | `u8`        |
| **1081 - 1087**        | 7              | *Padding (Align 264)* | -           | `[u8; 7]`   |
|                        |                | **--- SOUND 5 ---**   |             |             |
| **1088 - 1343**        | 256            | `sound_5_path`        | `byte[]`    | `[u8; 256]` |
| **1344**               | 1              | `sound_5_slot`        | `byte`      | `u8`        |
| **1345 - 1351**        | 7              | *Padding (Align 264)* | -           | `[u8; 7]`   |
|                        |                | **--- SOUND 6 ---**   |             |             |
| **1352 - 1607**        | 256            | `sound_6_path`        | `byte[]`    | `[u8; 256]` |
| **1608**               | 1              | `sound_6_slot`        | `byte`      | `u8`        |
| **1609 - 1615**        | 7              | *Padding (Align 264)* | -           | `[u8; 7]`   |
|                        |                | **--- SOUND 7 ---**   |             |             |
| **1616 - 1871**        | 256            | `sound_7_path`        | `byte[]`    | `[u8; 256]` |
| **1872**               | 1              | `sound_7_slot`        | `byte`      | `u8`        |
| **1873 - 1879**        | 7              | *Padding (Align 264)* | -           | `[u8; 7]`   |
|                        |                | **--- SOUND 8 ---**   |             |             |
| **1880 - 2135**        | 256            | `sound_8_path`        | `byte[]`    | `[u8; 256]` |
| **2136**               | 1              | `sound_8_slot`        | `byte`      | `u8`        |
| **2137 - 2143**        | 7              | *Padding (Align 264)* | -           | `[u8; 7]`   |
|                        |                | **--- SOUND 9 ---**   |             |             |
| **2144 - 2399**        | 256            | `sound_9_path`        | `byte[]`    | `[u8; 256]` |
| **2400**               | 1              | `sound_9_slot`        | `byte`      | `u8`        |
| **2401 - 2407**        | 7              | *Padding (Align 264)* | -           | `[u8; 7]`   |
|                        |                | **--- SOUND 10 ---**  |             |             |
| **2408 - 2663**        | 256            | `sound_10_path`       | `byte[]`    | `[u8; 256]` |
| **2664**               | 1              | `sound_10_slot`       | `byte`      | `u8`        |
| **2665 - 2671**        | 7              | *Padding (Align 264)* | -           | `[u8; 7]`   |

 ---

TUI -> Engine<br><br>
**IN:** `toEngien_shm.bin` <br>
**Total-length:** `4096 Bytes`

| Postion   | Length (Byte) | Value                      | Type (Java) | Type (Rust) |
|-----------|---------------|----------------------------|-------------|-------------|
| 0 - 7     | 8             | SequenceId                 | long        | u64         |
| 8         | 1             | increase_bpm               | boolean     | bool        |
| 9         | 1             | decrease_bpm               | boolean     | bool        |
| 10        | 1             | become_master              | boolean     | bool        |
| 11 - 13   | 3             | Padding                    | -           | [u8; 3]     |
| 14        | 1             | small_counter              | byte        | u8          |
| 15        | 1             | add_sound_on_small_beat    | boolean     | bool        |
| 16 - 271  | 256           | selected_sound_path        | byte[]      | [u8; 256]   |
| 272       | 1             | remove_sound_on_small_beat | boolean     | bool        |
| 273 - 279 | 7             | Padding                    | -           | [u8; 7]     |
| 280 - 535 | 256           | remove_sound_path          | byte[]      | [u8; 256]   |