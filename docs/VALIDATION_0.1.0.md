# Validation record — Windows Preview 0.1.0

Source commit: `64df594446bc69cee5777764ecab237018129c7b`. Emulator commit:
`72adb810c4409c4b7302ca56906c05083834faee`. Binary hashes are in build-info.json; download hashes
are in SHA256SUMS. Build date: 2026-10-23.

Built with Rust 1.99.1, MinGW-w64 GCC 12.2 POSIX, CMake 3.31.6, clap-wrapper
v0.16.0 and Steinberg VST3 SDK 3.8.0. The local wrapper patch verifies the original
source SHA-256 and restricts Windows CLAP loading to the private bundle. Runtime
libraries are statically linked. Final installer: Inno Setup 7.1.0 x64 from the
official release, whose Authenticode signature was verified before use. This
preview installer itself is unsigned.

| Check | Actual result |
| --- | --- |
| Packaging and dependency integrity tests | 13 passed |
| Emulator CPU / SoC tests on Linux | 58 + 38 passed |
| Windows plugin unit tests under Wine 10 | 7 passed |
| Windows firmware boot / MIDI audio / screen tests with Felucca 1.6.1 | 2 passed |
| Windows portable save and replacement tests | 2 passed |
| Official Windows VST3 validator, final private bundle and preview class ID | 47 passed, 0 failed |
| Installed private VST3 factory load | Passed; 1 registered class, clean shutdown |
| Missing private CLAP with a valid global CLAP present | Refused to load, as intended |
| Install and reinstall, Inno Setup 7.1.0 x64 package under Wine 10 | Passed; per-user HKCU, no restart |
| Installed app / Add or remove programs entry | Display name and uninstaller inspected |
| Uninstall private application and registration | Passed; directory and registry entry removed |
| Protected fixtures (Ableton files, preferences, project, other VST3, global CLAP) | All 5 unchanged |
| Native Windows desktop and Ableton Live acceptance | NOT RUN |

Wine validation is not a test of the user's actual Windows installation,
GPU/audio drivers or important projects. First use an empty Set with backups;
no experimental in-process plugin can guarantee zero risk. See the security
review for remaining dependency-maintenance advisories. No firmware is bundled.
