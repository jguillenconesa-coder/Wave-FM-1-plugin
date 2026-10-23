# Security review — 2026-10-23

Scope: the initial Windows packaging source, CI workflow, installer and pinned
fm1-emu dependency manifests. This was a focused source/configuration review,
not an independent penetration test or a complete audit of upstream Rust/C++
code. Windows x64 binaries have been built and checked under Wine 10; native
release workflows gate publication. User-machine acceptance remains separate.

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
