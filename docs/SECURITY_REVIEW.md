# Security review — 2026-10-23

Scope: the initial Windows packaging source, CI workflow, installer and pinned
fm1-emu dependency manifests. This was a focused source/configuration review,
not an independent penetration test or a complete audit of upstream Rust/C++
code. No Windows executable has been validated by this Linux session.

## Changes made

- GitHub Actions use immutable full commit pins, read-only repository permissions,
  no persisted checkout credentials, time limits and bounded artifact retention.
  No signing secret or write-capable token is requested. Dependabot checks Actions
  weekly. PR workflows do not use `pull_request_target` or elevated secrets.
- Emulator and wrapper revisions are fixed. Bootstrap rejects a linked checkout,
  changed revisions, modified tracked files and extra nonignored source files.
  Cargo uses the upstream lockfiles. TLS and checksum verification stay enabled;
  CPM's bootstrap uses its upstream SHA-256 verification. Some transitive CMake
  dependencies still use upstream release tags; complete hermetic builds are not claimed.
- A build manifest records hashes of both plugin binaries and the emulator/source
  revisions. Packaging rejects a missing companion, non-x64 PE file or changed
  binary pair. A failed rebuild invalidates the previous manifest.
- The installer compiler is versioned in CI. Microsoft's runtime is fetched over
  HTTPS and must have a valid Microsoft Authenticode signature before packaging.
  Compiler failure and missing output are fatal. Installer SHA-256 is emitted.
- The installer uses administrator-owned Common Files locations. It checks its
  ownership record and previous file hashes before replacing installed modules.
  Uninstallation removes shared-location files only while their hashes match its
  ownership record; externally replaced files are preserved. It leaves user
  projects, firmware and the shared Microsoft runtime alone. These installer
  behaviors still need execution on Windows.
- Own packaging licence and original third-party notices are separate. Windows
  Rust metadata resolves 121 packages; their notices and font licences are collected
  for distribution, and missing notices fail the build.
- Secrets, keys, build caches, downloaded sources and binaries are excluded from
  the source repository. Only source, tests, documentation and licence notices
  are intended for the initial Git commit.

## Checks executed

- 9 packaging/dependency integrity tests pass, including rejected binary mutation,
  missing companion, wrong architecture, truncated PE, wrong revision and local
  upstream modifications. Tests use synthetic files, not fabricated plugin builds.
- The pinned emulator's 58 CPU and 38 SoC tests passed on Linux (96 total).
- PowerShell parser accepted all three scripts; workflow and Dependabot YAML
  parsed, and every Actions reference was checked for a full commit pin.
- Rust licence collection completed for all 121 Windows-target metadata packages.
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

1. Execute the Windows build, installation/factory-loading/uninstallation tests
   and Steinberg's VST3 validator; complete the Ableton multi-instance/state/audio
   checklist in WINDOWS.md. Merely uploading source does not validate an installer.
2. Address or explicitly reassess the open dependency findings before calling a
   release stable. Review trust in transitive CMake downloads and the Chocolatey
   installer-compiler package when updating toolchains.
3. Review upstream native JIT/parser boundaries. The Windows JIT uses RWX memory;
   firmware execution occurs inside the DAW, not in an OS sandbox. Only trusted
   firmware, saved setups and DAW projects should be opened. No firmware binaries
   are bundled and no hardware flashing is performed by this packaging project.
4. Sign/notarize as appropriate for distribution: Windows Authenticode signing is
   not configured. Do not disable SmartScreen or antivirus. Installer hashes are
   integrity metadata, not proof of a known publisher. No signing credentials
   were created or committed.
5. Enable private vulnerability reporting, secret scanning/push protection and
   main-branch protection with CI checks in GitHub settings when available. This
   session's Git access does not establish permission to configure these server
   settings, so they are not claimed as enabled.
