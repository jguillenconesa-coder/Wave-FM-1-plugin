# Validation record · Preview 0.2.0 · 2026-10-23

For the upcoming 0.2.1 source changes: local checks add MIDI routing and optional
community downloads. Original external packages were downloaded from 64 author
GitHub releases; 62 passed basic engine/LCD/eight-second audio checks, while
BEETLE/PIGEON were silent with the generic stimulus. See
[exact external results](../../docs/COMMUNITY_COMPATIBILITY.md).
Native 0.2.1 release gates include an explicit real HTTPS download and hash
verification. The completed native evidence below remains specific to 0.2.0
preview.4 until a separate 0.2.1 result is recorded.

Emulator: `72adb810c4409c4b7302ca56906c05083834faee`; wrapper: v0.16.0.
Exact source revision, desktop fingerprint, firmware packages, matching sources,
notices and binary hashes are recorded by each build; compiler output is in CI. Release asset
hashes are in `SHA256SUMS` and `SHA256SUMS-macos` on the release page.

| Check | Observed result |
| --- | --- |
| Python dependency/packaging integrity tests | 16 passed |
| Emulator CPU / SoC baseline | 58 + 38 passed |
| Windows plugin unit tests under Wine 10 | 9 passed; all six panel palettes render distinctly and preserve LCD pixels |
| 23 real firmware packages, Windows engine | All booted, drew LCD, produced 352,800 finite audio frames each; no CPU halt |
| Independent instance configuration and firmware state | Passed; catalogue save encode/decode checked for every package |
| Portable save round-trip and transactional replacement, real Felucca fixture | 2 passed |
| Official Windows VST3 validator, final 23-package cross-build | 47 passed, 0 failed |
| Native Windows MSVC build, firmware tests, installer/factory/uninstall | Preview.4 passed native Windows firmware/state tests, explicit completed official validation, installer/factory/uninstall and release publication |
| Install/reinstall to standard VST3 path, uninstall and HKLM registration (Wine) | Passed again on final expanded package; all installed bundle hashes matched; factory load and removal verified |
| Silent rejection of unowned directory and legacy 0.1.0 registration | Passed; existing fixture preserved |
| Ableton program/preferences/project, other VST3 and global CLAP fixtures | All five unchanged after install/reinstall/uninstall |
| Native macOS Universal build/signature/validator | Preview.4 passed native firmware/state tests, Universal build/signature checks and 47 official VST3 tests with zero failures |
| Interactive plugin window under Wine | Not validated: software graphics could not provide the required accelerated OpenGL context |
| User's actual Live installation / native audio drivers / long sessions | Not tested here; user confirmed the prior plugin worked in Live Windows |

The firmware test applies MIDI on every channel, a timed PLAY-button press and a
panel note (REC note-entry mode for Ketch), in groups of four independent engine instances. Eight seconds per
package checks basic operation, not every menu, sequencer, input, sample bank,
USB/BLE/SD function or sustained real-time performance. Some firmware features
require hardware unavailable to the emulator. A finite peak is not a listening
test or endorsement of sound quality. Alpha/beta upstream versions retain their
experimental status.

Windows local evidence uses Rust 1.99.1, MinGW-w64 GCC 12.2 POSIX, Wine 10 and
Inno Setup 7.1.0 x64. Native release builds use the Windows runner toolchain.
macOS builds both architectures, while native execution tests run only the
runner architecture shown in Actions. Ad-hoc signatures are not Apple notarization.

Wine fixtures do not access the user's computer or important projects. Test the
update in an empty Set with backups. No experimental in-process plugin can
promise zero risk. This is not manufacturer certification or a complete security
audit; the dependency-maintenance warnings remain in the security review.

Native baseline evidence: [Windows run 39939039661](https://github.com/jguillenconesa-coder/Wave-FM-1-plugin/actions/runs/39939039661)
and [macOS run 39939039128](https://github.com/jguillenconesa-coder/Wave-FM-1-plugin/actions/runs/39939039128).
The release tag runs these checks again against its own exact source revision.

The macOS validator path in previews 2/3 was incorrect for the SDK's Ninja
output layout; the pipeline did not propagate that error and wrote an empty
report. Those workflow successes are **not** accepted as macOS VST3 validation.
Preview.4 uses `bin/Release/validator`, captures errors, requires a positive
completed-test count with zero failures, and enables pipeline failure propagation.

Final macOS preview.4: [native build and publication 39945061489](https://github.com/jguillenconesa-coder/Wave-FM-1-plugin/actions/runs/39945061489),
source `3d4a472`. The published ZIP and PKG were downloaded and their release
hashes checked. All 986 bundle files match the manifest; both Mach-O binaries
contain Intel and arm64 slices, and the included report records 47 passed tests,
zero failed. The ZIP includes all 23 firmware packages and the attribution files.

Final Windows preview.4: [native MSVC build and publication 39945061480](https://github.com/jguillenconesa-coder/Wave-FM-1-plugin/actions/runs/39945061480),
source `3d4a472`. The actual EXE and portable ZIP downloads match their published
SHA-256 values. All 340 bundle files match the native build manifest; both plugin
binaries are x64 PE files. The portable ZIP contains all 23 firmware packages,
the original upstream notice texts and the explicit maintainer/author declaration.
The release workflow installed, loaded the factory and completely uninstalled
the exact installer on its disposable native Windows runner before publication.

The downloaded native MSVC VST3 was also run through the official Windows
validator under Wine: **47 passed, 0 failed**. Raw reports are retained for
[Windows](../../docs/validation/windows-preview4-vst3.txt) and
[macOS](../../docs/validation/macos-preview4-vst3.txt). The Windows report checks
the delivered binary pair, in addition to the native Windows release workflow;
it does not represent a test on the user's computer or in Ableton Live.
