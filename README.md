# CDJ-BeatBoxer
<!-- ALL-CONTRIBUTORS-BADGE:START - Do not remove or modify this section -->
[![All Contributors](https://img.shields.io/badge/all_contributors-1-orange.svg?style=flat-square)](#contributors-)
<!-- ALL-CONTRIBUTORS-BADGE:END -->

A drum machine fully synced to your CDJs via ProDJ Link using beat-link. Better version
of [CDJ-BeatBox](https://github.com/FerdiStro/CDJ-BeatBox).Compile and run in a single bin for performative TUI build in
Rust.
<br>

[![Java](https://img.shields.io/badge/Java-%23ED8B00.svg?logo=openjdk&logoColor=white)](#) [![Rust](https://img.shields.io/badge/Rust-%23000000.svg?e&logo=rust&logoColor=white)](#)

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/FerdiStro/CDJ-BeatBoxer/build-binary.yaml)
![GitHub Release](https://img.shields.io/github/v/release/FerdiStro/CDJ-BeatBoxer)

## Dev-Setup

1. Install [mise](https://mise.jdx.dev/) and follow instruction guides
2. Run-`mise trust`
3. Run-`mise install`

---

## Build and Run local

To build just run:

```bash
./build-combined-bin.sh
```

After that execute the `CDJ-BeatBoxer`:
```bash
./CDJ-BeatBoxer
```

### Engine

For building and starting the engine just use Gradle. Go to `/BeatBoxer-Engien` and use one of these:

```bash
#Build
./gradlew build

#Build and ZIP
./gradlew bundleApp

#Build and run 
./gradlew run
```

### TUI

For building and starting the TUI just use cargo. Go to `/BeatBoxer-Tui` and use one of these:

```bash
#Build
cargo build

#Run main TUI. Select current env 
cargo run --bin beatboxer-tui

#Production build. before run these make sure you executed ./gradlew bundleApp
cargo build --release --bin prod_binary

#Test TUI for testing the engine
cargo run --bin test_ui
```

---

## Build and Release

Project CI/CD listen to git tag `- 'v*'`.  <br>
To build on push just add new tag:

```bash
git tag vX.X.X
git push origin vX.X.X
```

### Binary-Structs

Communication between TUI → Engine and Engine → TUI uses shared-memory-files. The binar-struct is documented in
the [Binary-Structs.md](Binary-Structs.md)

### System dependencies

`sudo apt install libasound2-dev`



---


## Contributors

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://pharrellkaim.github.io"><img src="https://avatars.githubusercontent.com/u/79710229?v=4?s=100" width="100px;" alt="Pharrell Kaim"/><br /><sub><b>Pharrell Kaim</b></sub></a><br /><a href="https://github.com/FerdiStro/CDJ-BeatBoxer/commits?author=PharrellKaim" title="Code">💻</a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

