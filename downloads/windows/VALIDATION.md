# Validation · 0.2.7 startup recovery · 2026-10-25

| Platform | Promoted assets | Runtime source | Native workflow |
| --- | --- | --- | --- |
| Windows x64 | v0.2.7-preview.2 EXE and portable ZIP | `af369eb` | [40191194461: success](https://github.com/jguillenconesa-coder/Wave-FM-1-plugin/actions/runs/40191194461) |

A deterministic regression reproduces the startup-silence recovery failure at
exactly 1x production, with both 64- and 512-sample host blocks. Priming startup
silence on the producer avoids indefinitely discarding subsequent real audio;
it does not fill in missing synthesis after first DMA output. Source, behavior
and limits are described in `docs/STARTUP_AUDIO_INVESTIGATION.md`.

Both actual X0X configurations run for 12 seconds with the original PLAY,
sustained notes, repeated percussion and physical-key workload. Each must
produce finite nonzero audio, remain unhalted and miss zero steady frames.
The source preserves the assertions, the 16,384-frame ring, chosen reserve,
reported latency, firmware DSP and process/host priority. The 64-sample Safe
configuration reports 1,179 host samples / 24.56 ms at 48 kHz, unchanged.

Actual published Windows ZIP report, source `af369eb`:

| Host block | Startup misses | Steady misses | Final debt | Engine | Callback max | Early delivery max gain |
| --- | --- | --- | --- | --- | --- | --- |
| 512 | 0 | 0 | 0 | 2.058x | 60 µs | 1,825 µs |
| 64 | 0 | 0 | 0 | 2.044x | 9 µs | 1,489 µs |

All promoted public files were downloaded and SHA-256 checked. Bundle hashes
and exact filesets, executable architectures, all 23 bundled firmware records,
83 community references / 64 author alternatives, and original notices were
verified against the source manifest. The actual Windows download passed all
47 official SDK tests and nine empty-VST3 silence/zero-latency configurations
under Wine, in addition to its native release gates. Windows CI installed the
EXE, loaded the installed factory and completely removed the private bundle
and uninstall registration. The installer changes no Ableton files/settings.
Source gitleaks 8.24.2 scan: no secrets found. Reports are in
`docs/validation/*-027-*`; source is immutable tag `v0.2.7-preview.2`.

Completed DMA halves are now published before cpu1's following work, with a
fallback after the outer slice for pending startup buffers or cpu0-held ticks.
The host-owned Vec is cleared/reused instead of reallocated each half. A real
X0X equivalence regression compared 88,576 nonzero-capable raw audio frames,
both CPUs, exact cycles, all RAM, flash and LCD against the original runner.
All matched; this is delivery work, not a changed synthesis algorithm.

Windows tests now use a private high-resolution waitable timer to wake the
synthetic host instead of occupying a logical CPU between callbacks. This is
compiled only into tests, not into the DLL; it changes no global timer or
priority. The zero-missing-frame assertions remain. The old busy host is kept
as an explicit control. Native standalone run 40191143694 (source af369eb)
passed both 64/512 configurations in both host modes, with zero startup/steady
misses and 1,031/1,041 us maximum early-delivery gains in the timer cases.
Reported thread CPU time is coarse Windows accounting, not zero CPU overhead.
Its exact public annotation excerpts are `windows-027-early-native.txt`.

Negative evidence is retained: preview.1 source 6e2a8f0 passed the standalone
startup check but failed the full native Windows gate (55,428 / 7,938 steady
misses at 512/64) and native macOS 64 (152); neither was published. The current
local Wine early-delivery case passed at 64 but missed 8,430 at 512 with a
31,616 us maximum run. See `*-027-preview1-failed.txt` and
`windows-027-early-local.txt`. No failed check was retried into a release.

The standalone native startup diagnostic (source e3a39e9, run 40188372683) also
passed 64/512 with zero startup/steady misses and zero final debt. Its exact
public annotation excerpt is `windows-027-startup-native.txt`.

These results identify and fix software recovery/delivery failures. They do **not** establish
that all of the user's persistent X0X dropouts are resolved inside Ableton on
their i7-10700F. No global CPU-use or fidelity improvement is claimed. The
previous Windows timing/wake experiments remain on performance-investigation;
this patch does not promote those failed experiments. Original authorship is
unchanged; source/licences remain bundled.

macOS 0.2.7 preview.2 was **not published**: native X0X missed
1,123 / 68 steady frames at 512 / 64. The assertions were retained; the
failure report is `macos-027-preview2-failed.txt`. macOS downloads remain the
independently verified v0.2.6-preview.6 / source 123425b assets.

## Archived 0.2.6 results (historical, superseded for updated platforms)

# Validation record · Preview 0.2.6 · 2026-10-24

Recommended downloads intentionally use **different, verified platform tags**.
The Windows experiments that failed timing are not promoted as an installer.
`main` retains the Windows preview.4 runtime; subsequent source work is preserved
in [performance-investigation](https://github.com/jguillenconesa-coder/Wave-FM-1-plugin/tree/performance-investigation).
No claim is made that the user's persistent X0X underruns are fixed.

| Platform | Recommended assets | Exact source | Native build/publication |
| --- | --- | --- | --- |
| Windows x64 | v0.2.6-preview.4 EXE and portable ZIP | `53c396648fddff8a4ea32a23340b361021ba5645` | [40065127227: success](https://github.com/jguillenconesa-coder/Wave-FM-1-plugin/actions/runs/40065127227) |
| macOS Universal 2 | v0.2.6-preview.6 ZIP and PKG | `123425b51d6a20d1e24aa93b919e286edf0b61e0` | [40068475978: success, 6m 7s](https://github.com/jguillenconesa-coder/Wave-FM-1-plugin/actions/runs/40068475978) |

All four actual public downloads matched their published SHA-256 lists. Both
Windows modules are x64; both macOS modules contain Intel/arm64. The exact file
sets and **346 Windows / 992 macOS bundle hashes** matched their manifests.
The Windows desktop fingerprint also matches source `53c3966` with its Windows
checkout line endings. Both use emulator `72adb810`. Original attribution,
23 firmware packages and matching notices/sources, 83 community references,
64 original author download links and the DAW guide were verified in the ZIPs.
The firmware binaries and legal notices are unchanged.

The actual downloaded Windows VST3 additionally passed the official SDK validator
under Wine: **47 passed, zero failed**. The downloaded macOS report records the
same native result. Reports: [Windows](../../docs/validation/windows-026-preview4-vst3.txt),
[macOS](../../docs/validation/macos-026-preview6-vst3.txt).
Both actual bundles also passed **nine empty-VST3 audio configurations**:
44.1/48/96 kHz × 64/512/2048 samples, zero reported latency before/after processing
and silent output. Reports: [Windows](../../docs/validation/windows-026-preview4-empty-latency.txt),
[macOS](../../docs/validation/macos-026-preview6-empty-latency.txt).
The published 0.2.5 negative control failed as expected on nonzero empty latency.

Windows native gates include integrity/dependency audits, 41 unit tests,
16 Python packaging/integrity tests, two catalogue tests covering all 23 real
firmwares, two portable-state tests, real author downloads, official validation,
empty latency, install/factory/uninstall and complete packaging. macOS native
gates include 41 unit tests, the same catalogue/state/author/validator checks,
24 Python checks (including eight timing-gate boundary tests), Universal
architecture/ad-hoc signature checks and packaging. Clean tracked-source
Gitleaks 8.24.2 scans of the two recommended runtime source revisions found no
secrets. These focused checks are not a complete audit of all upstream code.

## Realtime observations, not a user-machine guarantee

The diagnostic runs actual X0X 1.1.0 at 48 kHz / 512 samples for 12 seconds,
virtual CPU 240 MHz and Safe lead 1495 firmware frames. It includes PLAY,
a held panel key, note-on/off messages on all 16 channels (X0X routes its parts
by channel), and repeated 909/808 percussion. The first two seconds are startup;
the following ten require finite non-silent output, no halt and zero missing
frames. Both recommended native packages passed. Neither run reproduces a
full Ableton/ASIO session, all firmware functions or arbitrary CPU workloads.

| Native runner | Engine speed (last measured interval) | Startup missing | Steady missing | Maximum callback | Maximum fake-host lateness |
| --- | --- | --- | --- | --- | --- |
| Windows preview.4 | 2.029x | 8467 | 0 | 24 µs | 160 µs |
| macOS preview.6 | 2.424x | 4703 | 0 | 391 µs | 13741 µs |

Exact reports: [Windows](../../docs/validation/windows-026-preview4-x0x.txt),
[macOS](../../docs/validation/macos-026-preview6-x0x.txt). Both peaks were 0.39877.
The macOS gate passed on its **first** attempt; no retry was needed. Its newer
CI wrapper permits at most two repeated attempts only after a steady-window
fake-host deadline violation, no halt/overload and an in-budget callback.
A qualified audio failure, missing/malformed report, zero tests or three
unsuccessful attempts blocks publication. The zero-steady-miss assertion stays
strict. Every attempt remains in its portable ZIP report.

## Windows experimental failures remain unresolved

Later Windows candidates were blocked, even though their functional tests and
builds passed. Preview.5 recorded **35,858** steady missing frames with only
310 µs steady fake-host lateness. Preview.6 recorded **7,247** with 17 µs lateness;
its separate timed-sleep control also failed (**6,733**). These are qualified
audio failures, not grounds for automatically retrying until green. No Windows
installer was published for those failed revisions. Their macOS counterparts
passed independently; that does not validate Windows.

A [separate native diagnostic](https://github.com/jguillenconesa-coder/Wave-FM-1-plugin/actions/runs/40070562776)
(source `cf07cd7`) compared cold/warmed workloads and park/sleep waits on one
runner. All four audio tests failed: 29,495 / 2,572 / 2,279 / 2,877 steady misses.
Warm runs still missed frames after the startup interval. Individual render
slices peaked around 6–8 ms while the longest interval until audio refill was
about 79–86 ms (that maximum includes startup). Thus average compute headroom
above 1x alone does not guarantee each delivery deadline. The collection job's
green status means **evidence collection completed**, not that audio passed;
it has read-only permissions and cannot publish. Full evidence and experimental
source remain on the investigation branch. Earlier macOS scheduling failures
and local timing variability are also retained there.

The i7-10700F user's persistent failure has not been verified as resolved.
One instance still depends mainly on a serial emulation worker, not all 16
logical CPUs. Raising virtual MHz increases emulation work; it is not an
allocation of extra host cores. No change to synthesis fidelity or oversampling
is claimed. First test an empty Set; keep CPU clock at 240 MHz and Safe until
Audio late stays clear after startup, then try Balanced with Live stopped.

## Changes included in the recommended builds

Both provide zero empty latency and Low / Balanced / Safe reserves. At
48 kHz / maximum host block 512, loaded reported latency is 582 / 1070 / 1627
host samples (12.1 / 22.3 / 33.9 ms), excluding ASIO and firmware response.
Safe remains the legacy default; lower modes require more scheduling margin.
Queue/host compensation may briefly settle after a stopped-transport change.
Existing host parameter IDs and VST3 identity are preserved; the new reserve
parameter is non-automatable.

The FM-1 panel caches unchanged controls and skips identical texture uploads.
A regression compares full/incremental pixels through control changes, releases
and all six skins. A release-mode drawing benchmark of 120 frames measured
2,614,827 → 11,008 µs for LCD-only changes and 2,681,517 → 177,563 µs for LCD plus
one turning knob and one LED. [Raw measurements](../../docs/validation/windows-026-panel-benchmark.txt).
This measures panel drawing, excluding egui, texture upload, emulation and Live;
it is not a claimed overall CPU percentage or DSP-speed increase.

Windows preview.4 uses a modest per-worker Above normal (+1) priority with
fallback and scoped restoration, below ordinary highest/time-critical audio
priorities. It does not use MMCSS quotas or a realtime process class. The
recommended macOS preview.6 additionally wakes its producer after consumption
and reuses a monotonic housekeeping timestamp per slice; those later delivery
experiments are **not** in the recommended Windows preview.4 package.
No global power, affinity, timer, driver or Live settings change. The installer
scope and sound algorithm/resampler remain unchanged. Existing signing,
trusted-firmware and dependency-maintenance limitations remain in the security
review. Close Live during installation and use backups for important projects.

---

# Validation record · Preview 0.2.5 · 2026-10-24

Published release: **v0.2.5-preview.3**, source `6f933c2`. Subsequent
updates to links and this record are documentation-only commits.

- [Native Windows build/publication 40043421910](https://github.com/jguillenconesa-coder/Wave-FM-1-plugin/actions/runs/40043421910): success, 14m 35s. Includes installer/factory/uninstall checks.
- [Native Universal macOS build/publication 40043421827](https://github.com/jguillenconesa-coder/Wave-FM-1-plugin/actions/runs/40043421827): success, 5m 57s. Includes Universal architecture/signature and official VST3 checks.
- All four actual public downloads matched their release SHA-256 values.
  Both source manifests identify `6f933c2`; every one of the **346 Windows** and
  **992 macOS** bundle file hashes matches. Both Windows modules are x64;
  both macOS modules contain Intel/arm64. The Windows desktop fingerprint also
  matches the reviewed source with its Windows checkout line endings.
- Original notices and author declarations, 23 bundled firmware packages,
  83 community references, 64 original author release links and the offline
  DAW guide were verified in the downloads. Firmware binaries/licences are unchanged.
- The published native macOS report records **47 passed, zero failed** official
  VST3 tests. The actual downloaded native Windows bundle additionally passed
  **47 official tests under Wine**, as well as its native CI gate. Reports:
  [Windows](../../docs/validation/windows-025-preview3-vst3.txt) and
  [macOS](../../docs/validation/macos-025-preview3-vst3.txt).
- Local checks passed **37 unit tests**, **16 Python integrity/packaging tests**,
  two portable-state tests and two catalogue tests exercising all 23 real
  firmwares. Both native gates repeat functional tests and actual Felucca/Melodee
  author downloads with hash/parse/save verification. A clean tracked-source
  export of `6f933c2` passed Gitleaks 8.24.2 with no findings.

## X0X at 48 kHz / 512 samples

The isolated diagnostic runs the real bundled X0X CPU/JIT and audio queue for
12 seconds, with PLAY, a held note and repeated GM percussion. The first two
seconds are startup; the following ten seconds require zero missing frames,
finite non-silent audio and no CPU halt. Virtual CPU: 240 MHz, lead: 1495
firmware frames. These are observations on each runner, not comparative CPU
ratings or an Ableton/ASIO driver benchmark.

| Native runner | Engine speed | Missing frames during startup | Missing frames after startup | Maximum callback time | Maximum fake-host deadline lateness |
| --- | --- | --- | --- | --- | --- |
| Windows | 1.997x | 8937 | 0 | 1502 µs | 9538 µs |
| macOS | 2.368x | 4703 | 0 | 18 µs | 15191 µs |

Both peaks were 0.39877 and both engines finished without a halt. Exact
reports: [Windows](../../docs/validation/windows-025-preview3-x0x.txt),
[macOS](../../docs/validation/macos-025-preview3-x0x.txt) and
[local MinGW/Wine](../../docs/validation/windows-025-local-x0x.txt).
`REALTIME_X0X.txt` is also included at the top level of each portable ZIP.

The first two macOS preview attempts were blocked before publication: the fake
host's sleep-based pacing missed deadlines by up to 84.6 ms. Preview.3 yields
until absolute deadlines in this opt-in test instead. That pacing loop is
**test-only**, not shipped audio processing. These earlier failures do not
establish a product root cause or successful realtime operation. The final
reports retain the actual deadline lateness, including remaining runner jitter.

## Display and acceptance limits

A rendered-pixel regression confirms stable button tint around the previous
label-colour threshold despite small LED duty-cycle changes, while full on/off
flashes remain immediate. The per-window display filter emits no input and
changes no MIDI/firmware state. The audio recovery budget scales with the host
block; worker slices are shorter. Reported latency, host IDs and installer scope
are preserved. The dedicated footer shows Engine, Audio late, CPU and Host,
with an explicit Copy audio status button.

The user's persistent Windows failure was not reproduced with the previous
released scheduling policy locally. A passing 12-second diagnostic does not
prove their machine is fixed or validate long sessions, every firmware function,
a graphical Ableton session, controller mapping or native audio drivers. Startup
misses remain visible in the plugin. User-machine acceptance is still required.
Close Live before updating and first try an empty Set with important projects
backed up. Existing signing and dependency-maintenance limits remain documented
in the security review; no zero-risk guarantee is made.

## Historical record · Preview 0.2.4 · 2026-10-23

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
