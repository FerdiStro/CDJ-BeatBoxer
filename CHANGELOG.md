# CDJ-BeatBoxer

All notable changes to this project will be documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

-

### Changed

-

### Fixed

-

### Removed

## [0.0.2] - unreleased

### Added

- BeatBoxer-Engine: Direct OnShoots
- BeatBoxer-Engine: Direct OnShoots on next beat
- 

### Fixed
- BeatBoxer-Engine: Break Loop on`[SHMR] ERROR  Error in Hot-Loop`
- 

## [0.0.1] - 21.02.2026

### Added

- CHANGELOG.md update for newest future version documentations.\
- CI/CD workflow listen to `v*` git tags.
- CDJ-BeatBoxer-Mac-ARM64 binary - can be used to test prod on ARM64-Mac (m-chips)
- CDJ-BeatBoxer-Pi-ARM64 binary - can be used to test prod env on ARM64 (linux)
- BeatBoxer-Engine: Add sound on 8 bar gird.
- BeatBoxer-Engine: Add on-Shoot sounds on 8 bar grid.
- BeatBoxer-Engine: Apply effects on the main mix (include Reverb, Echo, Distortion)
- BeatBoxer-Engine: Communicate to XDJ-700 and read meta-data
- BeatBoxer-Engine: Set Master to Engine
- BeatBoxer-Engine: Revive data from shared memory.
- BeatBoxer-Engine: Send data to shared memory.
- BeatBoxer-TUI: Add basic Terminal-UI. and pre define structure for future devs.
- BeatBoxer-TUI: Add File System to Explore local sounds files.
- BeatBoxer-TUI: Send data to shared memory.
- BeatBoxer-TUI: Revive data from shared memory.
- BeatBoxer-TUI: Read Keyboard-inputs.
- BeatBoxer-TUI: Read MIDI-inputs. (Midi map hardcoded Arturia MiniLab mkII)
- BeatBoxer-TUI: Send MIDI-messages to control colored buttons.
- BeatBoxer-TUI: Add/Remove sounds to 8 Bar-grid.
- BeatBoxer-TUI: Add local test ui for testing drum-commands manuel `test_ui`
- BeatBoxer-TUI: Add production script `prod_binary`
- BeatBoxer-TUI: Add local-dev env script `beatboxer-tui`, runs app with selected envs 