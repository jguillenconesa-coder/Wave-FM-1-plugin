# Wave FM-1: Windows VST3 prototype

Windows x64 packaging of [fm1-emu](https://github.com/joakimeriksson/fm1-emu),
Joakim Eriksson's MIT-licensed M-VAVE FM-1 hardware emulator. The upstream
instrument loads `.fwsc` firmware packages and keeps firmware and written flash
in each plugin instance's host state. This project reuses that implementation;
it does not claim authorship of the emulator or any firmware.

The first target is Ableton Live on Windows. **Experimental: Windows compilation
and Ableton acceptance have not yet been verified.** No installer or tested
Windows binary is available from this checkout yet.

## Build

Install Git, Python 3, Rust (the repository selects 1.99.1) with the MSVC toolchain, CMake >= 3.24,
and Visual Studio 2022 Build Tools with Desktop development with C++ and a Windows SDK.
From PowerShell 7 or later at this repository's root:

```powershell
./tools/build-windows.ps1
```

Alternatively, the Windows GitHub Actions workflow builds a downloadable artifact
when run in GitHub. This workflow has not been executed by this local setup.

`tools/bootstrap.py` fetches a fixed upstream commit into ignored `.deps/fm1-emu`.
It refuses to replace a different revision or tracked edits. The wrapper is fixed
at clap-wrapper v0.16.0's commit. Cargo uses upstream's lockfiles. Wrapper SDK and
library downloads follow clap-wrapper's own dependency definitions.

See [Windows installation and acceptance checks](docs/WINDOWS.md).

To generate a Windows installer after compilation, install Inno Setup 6.3+ and
run `./tools/package-windows.ps1`. This packages both modules, documentation,
licences, a Microsoft-signed Visual C++ runtime and an uninstaller. The Windows
workflow runs an installation/factory-load/uninstallation smoke test before
uploading the artifact. These Windows steps remain unexecuted in this Linux environment.

## Scope and limitations

- Each instance owns an emulator and can load its own firmware.
- Firmware compatibility and CPU use vary; arbitrary future firmware is not guaranteed.
- Save edits through the firmware's own Save command before saving the Live set.
  Host state retains firmware and flash, not a complete running RAM snapshot.
- The upstream plugin reports latency and resamples to the host sample rate.
- Host transport/tempo synchronization and broad parameter automation are not
  acceptance claims of this prototype; upstream exposes Master automation.
- This first Windows package consists of a VST3 wrapper **and a CLAP module**.
  Both are required even though Live loads only the VST3.
- Firmware files are not bundled. Obtain them from their authors; their licences
  are independent of the MIT emulator and packaging code.

## Linux development checks

```sh
python3 tools/bootstrap.py
cargo test --manifest-path .deps/fm1-emu/Cargo.toml --locked --release -p fm1-core -p fm1-soc -j 2
```

This exercises the CPU and peripheral implementation; it does not validate
the Windows plugin or Ableton. Use the existing checkout in each isolated cloud
task; do not create a Git worktree unless explicitly requested.

## Project information

- [Credits and third-party licences](THIRD_PARTY_NOTICES.md)
- [Security policy](SECURITY.md) and [review scope](docs/SECURITY_REVIEW.md)
- [Contribution guidelines](CONTRIBUTING.md)

This project is independent of M-VAVE and Ableton. Only load trusted firmware:
emulation runs inside your DAW and is not a security sandbox.
