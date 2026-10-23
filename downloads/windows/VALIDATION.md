# Validation record · Preview 0.2.4 · 2026-10-23

Published release: **v0.2.4-preview.1**, source `a7b7e4a`. Subsequent
documentation-only commits update links and this record.

- [Native Windows build/publication 40024024923](https://github.com/jguillenconesa-coder/Wave-FM-1-plugin/actions/runs/40024024923): success, 15m 3s.
- [Native Universal macOS build/publication 40024025060](https://github.com/jguillenconesa-coder/Wave-FM-1-plugin/actions/runs/40024025060): success, 8m 12s.
- Local checks passed **36 plugin unit tests**, **16 Python integrity/packaging
  tests**, both portable-state tests and both catalogue tests. The catalogue
  test boots all **23 real packages**, including Skiff 1.4 and X0X 1.1.0, and
  checks eight seconds of finite audio, MIDI/panel input, LCD and no CPU halt.
  Native gates rerun the unit, firmware and state tests against the tagged source.
- Timing regressions force missing audio and verify that the host timeline
  advances, late replacements are discarded and callback recovery has a finite
  budget. Clock tests check 24 PPQN across several sample rates and buffer sizes,
  one minute at fractional BPM with irregular blocks and no accumulating drift
  (within one host sample of the pulse grid), tempo changes, Start/Stop, loops,
  invalid/missing transport data, queue failure, instance isolation, ordered
  note/clock frames and emulated-cycle scheduling without DMA-block rounding.
  Knob pixel/reopen regressions from 0.2.3 are retained.
- A clean tracked-source export passed Gitleaks 8.24.2 with no findings. Upstream
  pins, original checkout, author notices and firmware binaries remain intact.
  A hash-verified timing patch applies only to the separate generated SoC copy.
- All four public downloads matched their release checksums. Both manifests
  identify source `a7b7e4a`; all **346 Windows** and **992 macOS** bundle file
  hashes match, including the new offline DAW_SYNC.md guide. Both Windows
  modules are x64; both macOS modules contain Intel/arm64. Original credits,
  23 bundled packages, 83 community references and 64 author links were checked.
- The published native macOS report records **47 passed, zero failed** official
  VST3 tests. The actual published native Windows bundle additionally passed
  **47 official tests under Wine**, in addition to its native CI gate. Reports:
  [macOS](../../docs/validation/macos-024-preview1-vst3.txt) and
  [Windows](../../docs/validation/windows-024-preview1-vst3.txt).
- Native gates also require actual Felucca and Melodee author HTTPS downloads,
  SHA/package checks and file saving. Windows requires installer/factory/uninstall
  checks. Installer scope and VST3/previous parameter identities are unchanged;
  the new transport_sync parameter defaults to internal for existing projects.

No interactive Ableton/controller session was automated. The firmware smoke test
is not a measurement of each firmware's external-clock musical response. The
clock and recovery tests verify the common scheduling paths; the user's reported
tempo variation still requires confirmation in their Live setup. The firmware
must support external TRS/MIDI clock and be configured accordingly. Song Position
Pointer and within-block tempo ramps are not implemented. CPU overload can still
cause audio gaps; preserving the timeline does not create missing samples. See
[DAW sync and troubleshooting](../../docs/DAW_SYNC.md).

Close Live before updating and first test an empty Set with important projects
backed up. Windows has no own Authenticode signature; macOS uses ad-hoc signatures
without Developer ID/notarization and its PKG is not Apple-signed. Zero risk is
not guaranteed. This is not a complete independent security audit.

## Historical record · Preview 0.2.3 · 2026-10-23


Published release: **v0.2.3-preview.1**, source `d166566`. Subsequent
documentation-only commits update links and this record.

- [Native Windows build/publication 40021097292](https://github.com/jguillenconesa-coder/Wave-FM-1-plugin/actions/runs/40021097292): success, 13m 40s.
- [Native Universal macOS build/publication 40021097255](https://github.com/jguillenconesa-coder/Wave-FM-1-plugin/actions/runs/40021097255): success, 7m 29s.
- **26 plugin unit tests** passed locally. The new regression tests observe all
  seven host references, check editor reopening and instance separation, preserve
  endless mouse movement, and verify actual rendered indicator pixels at MIDI
  references 0, 64 and 127 without mouse commands. Restoring those references
  emits no engine events. Both native gates run these tests plus the two
  portable-state and two catalogue integration tests.
- All **16 Python integrity/packaging tests** passed. A clean export of tracked
  source passed Gitleaks 8.24.2 with no findings. No dependency, firmware binary,
  serialized host ID, VST3 identity, audio callback or installer scope changed.
- All four public downloads were retrieved and matched their release checksums.
  Both build manifests identify source `d166566`; all **345 Windows** and
  **991 macOS** bundle hashes match. Both Windows modules are x64; both macOS
  modules contain Intel/arm64. Original author notices, 23 bundled packages,
  83 community references and 64 original author release links were verified.
- The published native macOS report records **47 passed, zero failed** official
  VST3 tests. The actual downloaded native Windows bundle additionally passed
  **47 official tests under Wine**, in addition to the native CI gate. Reports:
  [macOS](../../docs/validation/macos-023-preview1-vst3.txt) and
  [Windows](../../docs/validation/windows-023-preview1-vst3.txt).
- The native gates also require actual Felucca and Melodee HTTPS downloads,
  reference hashes, package parsing and saving through the primary resolver.
  Windows additionally requires installer/factory/uninstall checks.

No interactive Ableton/controller session was automated. These tests verify
host references, rendering and unchanged routing; mapping/Takeover settings
and feedback from the user's specific controller require user-machine acceptance.
The indicator represents the host's 0–127 reference, not a firmware parameter's
internal musical value. Close Live before updating and first try an empty Set
with important projects backed up. Windows has no own Authenticode signature;
macOS uses ad-hoc signatures without Developer ID/notarization and its PKG is
not Apple-signed. Zero risk is not guaranteed.

## Historical record · Preview 0.2.2 · 2026-10-23


Published release: **v0.2.2-preview.1**, source `79f89b8`. Subsequent
documentation-only commits update links and this record.

- [Native Windows build/publication 39992303466](https://github.com/jguillenconesa-coder/Wave-FM-1-plugin/actions/runs/39992303466): success, 15m 8s.
- [Native Universal macOS build/publication 39992303478](https://github.com/jguillenconesa-coder/Wave-FM-1-plugin/actions/runs/39992303478): success, 8m 59s.
- Both native gates now download **Felucca and Melodee using the same source
  resolver as the primary community button**, verify the reference SHA-256,
  parse the package and assert it was saved. The original Melodee Hub URL
  reproduced HTTP 403; its original GitHub release downloads successfully.
  Melodee 1.4.5 SHA-256:
  `a18f40d6f8c8087b25f115ab11286f85c7f04383de836227c463c6ce4745ce20`.
- All four public downloads were retrieved and matched their release checksums.
  Both build manifests identify source `79f89b8`; all **345 Windows** and
  **991 macOS** bundle file hashes match. Both Windows modules are x64 and
  both macOS modules contain Intel/arm64. Packaged original credits, 23 bundled
  packages, 83 references and 64 author release links were verified.
- The published native macOS report records **47 passed, zero failed** official
  VST3 tests. The actual downloaded native Windows bundle additionally passed
  **47 official tests under Wine**, in addition to its native CI gate. Reports:
  [macOS](../../docs/validation/macos-022-preview1-vst3.txt) and
  [Windows](../../docs/validation/windows-022-preview1-vst3.txt).
- Local checks passed: **24 unit tests** and **16 Python integrity/packaging
  tests**. Both native gates also run the two portable-state tests and two
  catalogue integration tests. Regression tests
  cover blocked Hub download routing, same-version/hash fallback, no retry after
  an integrity failure, preservation of different existing files, reuse of an
  identical file and retained author links after a Hub catalogue refresh. A clean
  tracked-source export passed Gitleaks 8.24.2 with no findings.
- Native Windows CI also requires installer/factory/uninstall checks. Installer
  scope and host identities are unchanged; this fix adds no runtime dependencies,
  services, drivers or firmware binaries. Existing state IDs and credits remain.

No interactive Ableton session or save-dialog click sequence was automated.
The native tests exercise the actual download, validation and file-save path;
user-machine acceptance of the interface remains necessary. Close Live before
updating and try an empty Set with important projects backed up. No zero-risk
guarantee is made. Windows has no own Authenticode signature; macOS uses ad-hoc
signatures without Developer ID/notarization and its PKG is not Apple-signed.

## Historical record · Preview 0.2.1 · 2026-10-23

Published release: **v0.2.1-preview.2**, source `2e8ceea`. Subsequent documentation-only commits update links and this record.

- [Native Windows build/publication 39986788981](https://github.com/jguillenconesa-coder/Wave-FM-1-plugin/actions/runs/39986788981): success, 12m 37s.
- [Native Universal macOS build/publication 39986788463](https://github.com/jguillenconesa-coder/Wave-FM-1-plugin/actions/runs/39986788463): success, 9m 14s.
- Both gates require engine/state tests, controller regressions, a real HTTPS
  author download with hash/package verification and completed official VST3
  validation with zero failures. Windows also installs, loads the private factory,
  uninstalls and verifies application/registration cleanup.
- All four public downloads were retrieved and matched their release checksums.
  Metadata identifies source `2e8ceea`; every hash of the Windows ZIP's **345**
  bundle files and macOS ZIP's **991** bundle files matches. Both Windows modules
  are x64; both macOS modules contain Intel/arm64. Packaged original credits,
  23 bundled packages, 83 references and 64 author release links were verified.
- The published macOS native report records **47 tests passed, zero failed**.
  The actual downloaded native Windows bundle additionally passed **47 official
  VST3 tests under Wine**, as well as its native CI gate. Reports:
  `docs/validation/macos-021-preview2-vst3.txt` and
  `docs/validation/windows-021-preview2-vst3.txt`.
- Local checks: 21 unit tests, two portable-state tests, two catalogue integration
  tests and 16 source/packaging integrity tests passed. A clean tracked-source
  export passed Gitleaks 8.24.2 with no findings. A temporary egui linker object
  in the build cache triggered a binary-symbol false positive; it is not tracked
  source or a shipped file.
- Local installer smoke: all 345 installed bundle hashes matched, private factory
  loaded, uninstallation removed the application directory, and five protected
  fixtures representing Live, preferences, a project, another VST3 and a global
  CLAP stayed unchanged. This used a disposable Wine prefix. Installer logic is
  unchanged between preview.1 and preview.2; the latter fixes controller-state publication.
- Separately, 64 original author packages were downloaded: 62 passed basic
  boot/LCD/eight-second audio; BEETLE and PIGEON were silent with the generic
  sequence. [Exact external results](../../docs/COMMUNITY_COMPATIBILITY.md).
  These external binaries are not redistributed. A smoke test does not establish
  full feature/security verification or permission to redistribute assets.

No interactive Ableton/controller session was automated. Try an empty Set with
important projects closed/backed up. Windows has no own Authenticode signature.
macOS uses ad-hoc signatures without Developer ID/notarization, and the PKG is
not Apple-signed; Gatekeeper may block it. Zero risk is not guaranteed. No new
Rust/runtime dependencies, host changes, drivers or services were introduced.
The CI Node 20 warning concerns the pinned download-artifact action, which
completed under GitHub's Node 24 enforcement; it is unrelated to plugin runtime.

## Historical record · Preview 0.2.0 · 2026-10-23

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
