# CDJ-BeatBoxer

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

## Build and Release

Project CI/CD listen to git tag `- 'v*'`.  <br>
To build on push just add new tag:

```bash
git tag vX.X.X
git push origin vX.X.X
```

### Binary-Structs
Communication between TUI → Engine and Engine → TUI uses shared-memory-files. The binar-struct is documented in the [Binary-Structs.md](Binary-Structs.md)

### System dependencies

`sudo apt install libasound2-dev`



---

