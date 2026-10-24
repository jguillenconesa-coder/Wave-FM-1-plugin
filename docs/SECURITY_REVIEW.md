# Security review — 2026-10-24

Scope: the initial Windows packaging source, CI workflow, installer and pinned
fm1-emu dependency manifests. This was a focused source/configuration review,
not an independent penetration test or a complete audit of upstream Rust/C++
code. Windows x64 binaries have been built and checked under Wine 10; native
release workflows gate publication. User-machine acceptance remains separate.

## Version 0.2.6 latency and worker policy (2026-10-24)

A non-automatable `audio_reserve` parameter adds bounded, validated queue targets;
Safe / legacy remains the default. Existing parameter IDs and VST3 identity are
preserved. Empty instances output silence and report zero latency; loaded
instances report the actual target, with coalesced host notifications. Reserve
changes are deferred while the transport plays. The callback adds only bounded
arithmetic and atomic access; no allocation, wait, I/O or new lock is added.
Clearing restored firmware cancels exports and clears the worker's active state.

Windows uses public Kernel32 thread-priority APIs on this instance's worker
only: Above normal (+1), below normal highest/time-critical audio priorities.
It does not enter a realtime class or MMCSS, which applies CPU quotas. Its
non-Send RAII guard restores a successful change on its creating thread; it
never lowers an existing higher priority. Failure is non-fatal. macOS uses the public
pthread QoS API for the same worker. Failure falls back to normal scheduling.
No process-wide priority, affinity, timer resolution, power settings, services,
drivers, installer privileges or host configuration changes are introduced.
The editor's per-window pixel cache is invalidated on skin/canvas/window changes;
changed controls and LCD retain the full renderer's pixels. It has no input or
firmware side effects. No dependency or bundled firmware changes are made.

The official-SDK host check processes the actual empty VST3 at 44.1/48/96 kHz
and 64/512/2048 samples, requiring zero latency and silence in all nine cases.
A negative control with the published 0.2.5 VST3 correctly failed on nonzero
empty latency. Real firmware boot/clear and latency notification/defer behavior
also have lifecycle tests. Native release gates and actual-asset verification
must complete before download links are promoted. A smaller reserve failed
local timing tests, so it remains an explicit tradeoff, not the default or a
claimed fix for sustained overload. No audio quality increase is claimed.

The promoted packages use different verified sources: Windows preview.4 at
`53c3966`, macOS preview.6 at `123425b`. All four downloaded assets, exact bundle
hashes, architectures, original credits and the Windows source fingerprint were
verified. Both recommended native X0X diagnostics passed with zero steady misses;
official validation records 47 passed / zero failed and both empty-latency
reports cover nine configurations. Source scans found no secrets. Later Windows
wake/clock experiments failed qualified timing checks and were not promoted.
`main` follows the verified Windows runtime; all experimental work/evidence is
preserved on `performance-investigation`. See `downloads/windows/VALIDATION.md`.
The user's persistent underruns are not claimed as resolved.

macOS preview.6 additionally uses a non-waiting producer notification through its
stored thread handle and timed parks, plus cached per-slice monotonic housekeeping
time. No callback allocation, blocking wait, file/network I/O or new mutex is
introduced. Its bounded timing wrapper can retry only a failed synthetic host's
own steady-window deadline violation with headroom and an in-budget callback;
it never retries away qualified misses, overload, halts, malformed reports or
zero tests. A complete strict pass remains mandatory; all attempts are retained.
These later changes are not in the promoted Windows preview.4 runtime.

## Version 0.2.5 audio scheduling and diagnostics (2026-10-24)

Generated worker render slices decrease from 20,000 to 8,000 instructions,
allowing the audio ring to be published more frequently. Recovery reads are
bounded by twice the current block's expected 44.1 kHz frames instead of a
fixed 16,384-frame budget. The audio timeline, ring size, reported latency and
host configuration are unchanged. Recovery must have headroom beyond one block
to reduce existing debt while the timeline advances. No wait, allocation, I/O
or additional mutex is introduced in the callback.

Host rate and current block size are published through relaxed atomics for
the editor. A dedicated footer keeps emulation speed and missing-frame counts
visible. Its explicit copy button copies only firmware name, version and audio
diagnostics to the clipboard; it sends nothing over the network. Clock guidance
explains that increasing virtual MHz costs host CPU. No new host parameters,
dependencies, downloads, installer permissions, services or firmware binaries
are added. Original attribution remains unchanged.

An isolated opt-in wall-clock regression runs the actual bundled X0X engine
with 48 kHz / 512-frame callbacks, PLAY, held notes and repeated GM percussion.
It requires finite non-silent output, no halt and zero missing frames after
two seconds of startup. The test-only fake host yields until each absolute
deadline instead of using sleep timers that macOS CI may coalesce by 85 ms.
This pacing loop is not used in shipped audio processing. Its measured host
lateness is retained to distinguish test scheduling from emulation speed.
Native Windows/macOS gates retain its timing report in
their ZIPs. Startup misses are measured separately, not hidden in the UI.
The persistent failure reported by the user was not reproduced locally under
the previous scheduling policy; improvement here does not prove that their
specific machine's symptom is resolved. Real Ableton/ASIO acceptance remains
necessary. See the validation record for completed measurements.

The FM-1 editor also applies an 8% brightness deadband to multiplexed LED
readings. Small duty-cycle jitter no longer continuously recolours button caps
or flips their label colour; full on/off transitions remain immediate.
This is per-window display state only. It does not send input, filter MIDI,
alter sequencer events or change the firmware. A rendered-pixel regression
checks stable tint around the previous ink-colour boundary and real flashes.

Native Windows and Universal macOS preview.3 gates completed successfully at
source `6f933c2`. All four public assets matched their release checksums;
346 Windows / 992 macOS bundle hashes, architectures and original notices were
verified. Both native X0X diagnostics passed at 48 kHz / 512 samples with zero
missing frames after startup. The macOS native VST3 report and additional
validation of the actual downloaded Windows VST3 under Wine each record 47
passed, zero failed. Local functional/integrity checks and the clean-source
secret scan also passed. Measurements, failed earlier test pacing and remaining
acceptance limits: `downloads/windows/VALIDATION.md`.

## Version 0.2.4 host clock and timing (2026-10-23)

An optional, non-automatable `transport_sync` parameter defaults to internal;
old host parameter IDs and the VST3 identity are preserved. DAW mode generates
24-PPQN MIDI Clock and Start/Stop in process-block time, merging with note events
in timestamp order. Stop is sent on leaving DAW mode or losing valid tempo;
loops/seeks restart explicitly. Invalid/non-finite tempo/rate data is rejected.
Missing beat position falls back to continuous tempo-based phase. Song Position
Pointer and per-sample tempo ramps are not implemented. Firmware external-clock
configuration is user-controlled and no firmware binary is changed.

The audio callback advances its timeline during an underrun and discards late
replacements, with a finite recovery budget. It does not allocate, wait for the
machine, perform network/file I/O or alter the host's audio settings. Missing
audio can still cause gaps under excess load. MIDI timestamps use the same
declared latency and are scheduled on the worker against the last DMA cycle,
using the existing peripheral event queue. A hash-checked patch is applied only
to the generated SoC copy: record that cycle, insert input in stable order and
invalidate the cached JIT deadline. Pending MIDI has backpressure; ring overflow
is reported and transport resynchronization retried. Free-running while the host
does not consume audio is restricted to internal mode and accounts for lead time.

Tests cover forced underruns, discard/recovery budgets, instance separation,
clock frequency at several rates/buffers, fractional tempo without drift,
tempo changes, Stop/Start/loops, invalid data, queue overflow, note/clock ordering
and exact emulated-cycle delivery. The standard real-firmware/state and native
VST3 gates also apply. There are no new dependencies, permissions, services or
installer-scope changes; original notices remain intact. These tests do not
establish controller/Live acceptance or every firmware's external-clock support.

Native Windows and Universal macOS 0.2.4 preview.1 publication gates completed
successfully at source `a7b7e4a`. All four actual public assets matched their
release checksums. Every one of the 346 Windows / 992 macOS bundle file hashes,
architectures, original author notices and offline DAW guide was verified.
The published macOS native report and additional official validation of the
actual published Windows bundle under Wine each record 47 passed, zero failed.
Local checks passed 36 unit tests, 16 integrity/packaging tests and real-firmware
and portable-state integration tests. The clean source export passed Gitleaks
with no findings. Exact workflow links, measurements and acceptance limits:
`downloads/windows/VALIDATION.md`.

## Version 0.2.3 knob display fix (2026-10-23)

The editor now observes the seven existing host encoder references to draw
their indicators over a 270-degree sweep, including host automation and reopened
windows. This is editor-only feedback: it emits no commands, writes no host
parameters and does not change the serialized control IDs, worker routing,
firmware state or audio callback. Manual mouse turns remain endless and the
next host reference change reanchors their display. FM-1 feedback is not applied
to the SMK-37 panel. Existing native drawing callers retain their click-based API.
Regression tests check actual indicator pixels at low/centre/high references,
all seven routes, editor reopening, instance separation and unchanged engine
behaviour during reference restoration. No new dependency, firmware binary,
network permission or installer scope is introduced.

## Version 0.2.2 download fix (2026-10-23)

The primary community button now resolves a pinned original author release
before using a Hub URL, addressing the reproduced Melodee Hub HTTP 403. Its
GitHub package was downloaded independently and matches the recorded SHA-256.
Native release gates exercise the same primary resolver with actual Felucca
and Melodee downloads, instead of testing only an explicitly selected alternative.
Transport TLS, host/redirect allowlists and size/time bounds remain unchanged.

Fallback is permitted only after a transport failure, for the same version and
the same pinned hash. Hash failures never trigger a weaker alternate download.
The existing-file path is bounded and compared byte for byte: an identical
regular file is reused; different content, links or directories are preserved
and rejected. No overwrite path is introduced. Failed/cancelled new downloads
clear the previous load target. Errors and the saved path are visible in the UI.
An explicit browser link offers a manual alternative when local network policy
blocks the plugin. Browser downloads are not claimed to be hash-verified by the
plugin. No firmware redistribution, new dependencies or installer/host permissions
were added. Existing project/controller state IDs remain unchanged.

The 0.2.2 preview.1 native Windows and macOS gates completed successfully at
source `79f89b8`, including real primary-button Felucca and Melodee downloads,
hash/package verification and saving to disk. All four public downloads matched
their release checksums. All 345 Windows and 991 macOS bundle file hashes,
architectures and packaged original credits were verified. The published native
macOS report and additional validation of the actual published Windows bundle
under Wine each record 47 passed tests and zero failures. Local regression and
packaging tests passed; a clean tracked-source export passed Gitleaks with no
findings. Exact evidence and acceptance limits: `downloads/windows/VALIDATION.md`.

## Version 0.2.1 additions (2026-10-23)

- MIDI/Live adds 21 stable host parameters without changing the existing
  parameter IDs or VST3 class identity. Encoder differences and 100 ms button
  pulses are routed on the emulator worker. Host callbacks only use atomic
  counters for pulses. Simultaneous host/mouse presses are combined; host pulses
  always expire. SMK-37 does not receive FM-1 automation control IDs.
- The optional community browser embeds 83 credited references and 64 direct
  author release URLs with pinned SHA-256 values. External firmware binaries
  are not bundled or relicensed. A separate report records basic boot/LCD/audio
  success for 62 of 64 exact original downloads; BEETLE and PIGEON remained
  silent in the generic test. This does not establish safety or full compatibility.
- Network activity requires explicit user actions. Transport uses OS TLS through
  WinHTTP or macOS's built-in curl with no shell/user curl config. Initial URLs
  and every redirect use exact public host allowlists; no custom hosts, ports,
  credentials, insecure protocols, private IP destinations or arbitrary programs.
  GitHub's asset CDN may use signed HTTPS query strings. Byte counts, redirects
  and time are bounded. No certificates are bypassed or ambient Windows
  credentials submitted. No network/file I/O is added to the audio callback.
- Downloads are parsed as data and saved to a new user-selected `.fwsc` using
  atomic no-overwrite persistence. They never load automatically, update Sets,
  flash hardware or replace bundled files. Pinned author release downloads must
  match the recorded hash; a new Hub package has no independent hash reference
  and is explicitly unverified. The emulator remains inside the DAW, without
  a security sandbox. Third-party network providers see normal request metadata.
- Instance destruction joins owned background jobs before returning to the host,
  so a download cannot keep running after the host unloads its plugin code.
  Closing an instance during a network request can wait for its timeout. Existing
  firmware import/export worker jobs now use the same lifetime ownership.
- Local unit tests cover host IDs, encoder routes, short pulses and release,
  instance separation, malformed/oversized catalogues, blocked addresses and
  redirects, package parsing, hashes and refusal to overwrite user files.
  Local Wine HTTPS hit DNS error 12007 in the container; it is not accepted as
  a successful network test. Native Windows/macOS release workflows explicitly
  require a real author HTTPS download, hash match and package parse in addition
  to their normal engine and official VST3 checks. No new Rust dependencies
  were introduced. Actual Live MIDI mapping requires user-machine acceptance.
- Preview.2 additionally publishes coherent atomic control snapshots only from
  the host's serialized initialize/process phases. The worker never polls raw
  host parameter values during state deserialization. Regression tests cover
  partially restored positions/button counters and interrupted snapshot reads;
  neither may replay saved controls into the old firmware. Publication uses no
  locks, allocation or spinning in the audio callback. macOS asset publication
  uses Ubuntu after the native macOS build to avoid a second macOS runner queue.

Final 0.2.1 preview.2 native Windows and macOS workflows completed successfully,
including the real HTTPS download test. All four actual published downloads
matched their release checksums; all 345 Windows / 991 macOS bundle file hashes,
source commit, architectures and author notices were verified. Both official
validator reports record 47 passed and zero failed tests; the native Windows
download additionally passed validation under Wine. A clean export of tracked
source passed Gitleaks with no findings. Evidence and practical limits:
`downloads/windows/VALIDATION.md`. Preview.1 is superseded by preview.2's coherent
host-state publication; use the latter for the new MIDI controls.

## Version 0.2.0 additions (2026-10-23)

- Twenty-three original firmware packages are distributed as separate interpreted
  programs, not linked into the plugin. Exact versions, authors and licences are
  listed in `firmware/README.md` and `manifest.json`. Complete corresponding
  sources, recursive submodules and original notices travel inside every bundle.
  Polly retains CC BY 4.0 rhythm credits and modification descriptions. The
  research ledger records excluded or pending candidates and licensing concerns;
  a catalogue's broad GPL label alone was not accepted as asset clearance.
- Catalogue loading is offline, fixed at build time, bounded to 8 MiB, and checks
  size and SHA-256 before sending the package to the existing transactional loader.
  It resolves only the loaded CLAP's own folder, not the host executable, current
  directory, user environment variable or an Internet manifest. Manual loading of
  modified/custom firmware remains available without a catalogue hash restriction.
- GUI work uses a hash-checked baseline and a separate MIT desktop fork. Original
  emulator sources and dependency lockfiles remain unchanged. Skin is persisted
  per instance and never recolours firmware screen pixels or changes sound.
- The build manifest now covers firmware, sources, licence files and desktop
  source fingerprint as well as both Windows binaries. Unexpected bundle files
  fail verification. A corrupted bundled package is rejected before replacing
  the current instance. Catalogue hash verification is integrity checking, not
  certification or a sandbox for malicious firmware.
- From preview.4, macOS is built natively through the macOS workflow for both Intel/arm64, with
  private CLAP loading. Packages are released only after native tests, official
  VST3 validation and bundle-signature checks. Ad-hoc code signatures are not
  Developer ID, installer signatures or Apple notarization. Native host/driver
  compatibility still requires user testing.
- Current release evidence is in `downloads/windows/VALIDATION.md`. The checks
  below are the prior 0.1.0 baseline where not explicitly superseded; they must
  not be read as new native Ableton or macOS results.

An additional source-notice completeness check compares every licence, notice,
credit and attribution text in the matching source archives against the directly
accessible `firmware-licenses` folder, including files named `MIT-*.txt` or
`Apache-*.txt` under `LICENSES/`. Builds reject missing or changed notices; no
copyright text is replaced with the repository maintainer's identity.

The final preview.4 native Windows and macOS workflows completed successfully.
The actual four published downloads (EXE, Windows ZIP, macOS ZIP and PKG) were
retrieved and verified against their release hashes. Both bundle manifests,
Windows PE architecture, both Universal Mach-O slices and the packaged original
credits were checked. macOS's included official validator report records 47
passed tests and zero failures. The downloaded native MSVC bundle additionally
passed 47 official Windows VST3 tests under Wine, with zero failures. Earlier macOS previews had an incorrect validator
path and masked pipeline status; those green workflows are not accepted as
validation. Preview.4 enables shell pipeline failure propagation and requires a
nonempty successful test-count report on both platforms. See the validation record
for native run links and practical limits.

Source secret scans were repeated with Gitleaks 8.24.2 on the working tree and
on safely extracted corresponding firmware source archives: no findings. No
upstream source build scripts were executed during catalogue research. This
scan cannot establish that a firmware is harmless or free of vulnerabilities.
The final expanded Windows installer, installed file hashes, factory loading,
complete uninstallation and five protected fixtures were rechecked under Wine.

## Changes made

- GitHub Actions use immutable full commit pins, read-only build-job permissions,
  no persisted checkout credentials, time limits and bounded artifact retention.
  Only the tag-triggered publishing job gets a short-lived GitHub token with
  contents-write permission to create the requested prerelease. No personal token
  or signing secret is requested. Dependabot checks Actions
  weekly. PR workflows do not use `pull_request_target` or elevated secrets.
- Emulator and wrapper revisions are fixed. Bootstrap rejects a linked checkout,
  changed revisions, modified tracked files and extra nonignored source files.
  Cargo uses the upstream lockfiles. TLS and checksum verification stay enabled;
  CPM's bootstrap uses its upstream SHA-256 verification. Some transitive CMake
  dependencies still use upstream release tags; complete hermetic builds are not claimed.
- A build manifest records hashes of both plugin binaries and the emulator/source
  revisions. Packaging rejects a missing companion, non-x64 PE file or changed
  binary pair. A failed rebuild invalidates the previous manifest.
- The installer compiler is versioned in CI. The tested cross-build uses the
  official Authenticode-verified Inno Setup 7.1.0 x64 compiler. Plugin runtime
  libraries are linked statically; the installer does not download or install
  shared system runtimes. Compiler failure and missing output are fatal.
- Version 0.2.0 installs its own bundle in `%CommonProgramFiles%\VST3\Wave FM-1.vst3`,
  requires elevation and registers its own uninstaller in HKLM. It changes no
  Ableton settings or application files. Unlike 0.1.0, the bundle is discoverable
  by the host's standard VST3 scan. Existing unowned folders are rejected in both
  interactive and silent mode; the legacy per-user 0.1.0 must be removed first. No services, drivers,
  antivirus exclusions or PATH changes are installed.
- A local patch verifies the exact wrapper source SHA-256 before adding private
  companion loading. The VST3 only loads the CLAP inside its own bundle. It does
  not fall back to another FM-1 installation in a shared folder.
- The preview has its own VST3 class ID
  `763245F44C814116B23B2268F75ADC65`, separate from upstream's instrument ID.
  It does not claim to be the same plugin when Live resolves existing projects.
- Own packaging licence and original third-party notices are separate. Windows
  Rust GNU-target metadata resolves 128 packages (121 on MSVC); their notices and font licences are collected
  for distribution, and missing notices fail the build.
- Secrets, keys, build caches and downloaded dependencies remain excluded. The
  intentionally published preview snapshots under `downloads/windows/` have
  SHA-256 checksums and a separate release tag; they are not build-cache uploads.

## Checks executed

- 15 packaging/dependency integrity tests pass, including rejected binary mutation,
  missing companion, wrong architecture, truncated PE, wrong revision and local
  upstream modifications. Tests use synthetic files, not fabricated plugin builds.
- Inno Setup 7.1.0's generated Windows x64 installer was installed and reinstalled
  under Wine 10. The registered Installed apps entry was inspected and the
  installed VST3 factory loaded successfully. Uninstallation removed the complete
  private application directory and its registration without rebooting. Five
  protected test fixtures representing Ableton files, preferences, an important
  project, another VST3 and another global FM-1 CLAP were hash-checked unchanged.
  These fixtures are not a test of the user's actual machine or projects.
- A negative runtime check removed the private companion while leaving a valid
  global FM-1 CLAP available: the VST3 refused to load instead of using that
  other installation. The private file was restored before uninstall testing.
- The pinned emulator's 58 CPU and 38 SoC tests passed on Linux (96 total).
- PowerShell parser accepted all three scripts; workflow and Dependabot YAML
  parsed, and every Actions reference was checked for a full commit pin.
- Rust licence collection completed for all 128 Windows GNU-target metadata packages,
  including font notices and the GCC/MinGW runtime notices and exception.
- Gitleaks 8.24.2 scanned the exact source-file publication set and reported no
  detected secrets. Build caches, downloaded dependencies and binary outputs are
  not in that set. This is a detection check, not a guarantee that no secret can exist.
- `cargo audit 0.22.2`, RustSec database commit
  `7ad117a308b144f2f7661fa555e4db5db9c5586e`, checked the emulator lockfile
  (180 packages) and plugin lockfile (355 packages). Both reported **0
  vulnerabilities** in the scanner's vulnerability category. This does not mean
  zero security-relevant findings; informational and registry warnings follow.

## Dependency findings still open

| Finding | Assessment / next action |
| --- | --- |
| `atty 0.2.14`, RUSTSEC-2021-0145 and RUSTSEC-2024-0375 | Potential unaligned read on Windows and no maintenance. RustSec states the usual Windows System allocator supplies sufficient alignment, but this is not a proof for every host/build. A supported upstream replacement should be investigated before a stable release. No advisory suppression was added. |
| `ttf-parser 0.25.1`, RUSTSEC-2026-0192 | Unmaintained parser used in the GUI dependency graph; follow upstream migration guidance and review updates. |
| `zbus_names`, `zvariant`, `zvariant_derive`, RUSTSEC-2026-0371/0370/0372 | Maintenance advisories in the all-platform lockfile. These packages are absent from the Windows-target metadata graph reviewed here; they still affect maintenance of other targets. |
| `instant 0.1.13`, RUSTSEC-2024-0384 | Maintenance advisory in the emulator workspace lockfile. |
| `clipboard-win 5.4.1` | Registry reports this locked version as yanked. The Windows dependency graph includes it. The reason and an upstream-compatible replacement remain to be reviewed before a stable release. Cargo's frozen checksums still authenticate the locked archive; yanking is not itself a malware diagnosis. |

The default audit command fails on vulnerability findings and reports the above
warnings. The CI workflow repeats the audit on both lockfiles on changes and
weekly. Maintenance warnings are documented rather than hidden or presented as
fixed. Audits of the Cargo graph do not scan the C++ SDK, runtime or compiler.

## Remaining release requirements

1. Windows x64 compilation and Steinberg's validator completed under Wine 10:
   47 tests pass, 0 fail. The Windows plugin's 7 unit tests, 2 firmware/MIDI/audio
   tests and 2 portable-state tests also pass under Wine 10 with Felucca 1.6.1
   obtained from the author's GitHub release (SHA-256
   `dff3a38f7e5b9833e15aba322558c9485e9e422e3ff813c616c91b3584a2d909`).
   These do not replace the native Ableton multi-instance/state/audio checklist
   in WINDOWS.md. The firmware is a local test input, not a distributed asset.
2. Address or explicitly reassess the open dependency findings before calling a
   release stable. Review trust in transitive CMake downloads and the Chocolatey
   installer-compiler package when updating toolchains.
3. Review upstream native JIT/parser boundaries. The Windows JIT uses RWX memory;
   firmware execution occurs inside the DAW, not in an OS sandbox. Only trusted
   firmware, saved setups and DAW projects should be opened. No firmware binaries
   are bundled and no hardware flashing is performed by this packaging project.
4. Sign as appropriate for Windows distribution: Authenticode signing is
   not configured. Do not disable SmartScreen or antivirus. Installer hashes are
   integrity metadata, not proof of a known publisher. No signing credentials
   were created or committed.
5. Enable private vulnerability reporting, secret scanning/push protection and
   main-branch protection with CI checks in GitHub settings when available. This
   session's Git access does not establish permission to configure these server
   settings, so they are not claimed as enabled.
