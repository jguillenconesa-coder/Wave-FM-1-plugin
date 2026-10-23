# Credits and third-party notices

Wave FM-1 is an independent Windows packaging project. It is not an official
M-VAVE product and is not endorsed by M-VAVE, Ableton, Steinberg or the authors
listed here. Their product names remain their respective owners' marks.

| Component | Credit | Licence / use |
| --- | --- | --- |
| [fm1-emu](https://github.com/joakimeriksson/fm1-emu) | Joakim Eriksson and contributors | MIT; CPU/SoC emulation, firmware loading, instrument engine and original interactive panel. Pinned to `72adb810c4409c4b7302ca56906c05083834faee`. |
| [clap-wrapper](https://github.com/free-audio/clap-wrapper) | defiantnerd (Timo Kaluza), Paul Walker and contributors | MIT; VST3 wrapper. Pinned to `1cca996e96f29ab2be7ae9f8cfe532bbc92e1dd6` (v0.16.0). |
| [nih-plug](https://github.com/robbert-vdh/nih-plug) | Robbert van der Helm and contributors | ISC; Rust CLAP plugin framework. Upstream disables its GPL VST3 bindings; our VST3 uses clap-wrapper. |
| [CLAP SDK](https://github.com/free-audio/clap) | CLAP contributors | MIT; plugin interface. |
| [VST3 SDK](https://github.com/steinbergmedia/vst3sdk) | Steinberg Media Technologies GmbH and contributors | MIT in v3.8.0; VST3 integration. |
| [egui](https://github.com/emilk/egui) | Emil Ernerfeldt and contributors | MIT option for the Rust UI libraries; bundled fonts have additional terms preserved in the distribution. |
| [profiling](https://github.com/aclysma/profiling) | Philip Degarmo and contributors | MIT option. |
| [clipboard-win](https://github.com/DoumanAsh/clipboard-win) | Douman and contributors | Boost Software License 1.0. |
| [Inno Setup](https://jrsoftware.org/isinfo.php) | Jordan Russell, Martijn Laan and contributors | Installer compiler; its own terms apply, not this project's MIT licence. |
| [MinGW-w64](https://www.mingw-w64.org/) and GCC runtime libraries (cross build) | MinGW-w64 and GCC contributors, Free Software Foundation | Original notices and GPL-3.0/GCC Runtime Library Exception are included in `licenses/toolchain/`. The preview links runtime libraries statically and does not install shared runtimes. |

The project's root MIT licence covers its own packaging code, not all bundled
components. Original source notices are retained in `LICENSES/` where needed;
binary packages include `licenses/`, with Rust package SPDX declarations,
versions, authors and repositories in `licenses/rust/inventory.json`. This
inventory includes build dependencies and platform dependencies, not only code
that ends up in a binary. Original upstream terms govern any redistribution.

No firmware, factory sample bank, manufacturer artwork or third-party release
binary is committed as part of this repository. Firmware authors retain their
rights; importing or sharing a firmware-containing `.fm1save` does not change
its licence. Felucca-family firmware may be GPL-3.0; do not describe it as MIT.

Research references (not code dependencies):

- [aroum/fm1-custom-fw](https://github.com/aroum/fm1-custom-fw), maintained by aroum
  and contributors: firmware catalogue and hardware/protocol research.
- [M-VAVE FM-1 workbench](https://open.m-vave.com/fm1-workbench): manufacturer resource.
- [Quixotic7/FM1VST](https://github.com/Quixotic7/FM1VST): alternative approach using
  recompiled firmware cores. No code from it is included in this project.

## Supplemental licence provenance

These unmodified texts supply notices omitted from some Cargo crate archives:

- `LICENSES/egui-MIT.txt`: emilk/egui, commit
  `1669e52a7ccfc3489c1b0999b9ed48894a0b3887`, `LICENSE-MIT` (0.31.1).
- `LICENSES/profiling-MIT.txt`: aclysma/profiling, commit
  `8271551172eb6fa4cba47369aedd93790c623df9`, `LICENSE-MIT` (crate 1.0.18).
- `LICENSES/clipboard-win-BSL-1.0.txt`: DoumanAsh/clipboard-win, commit
  `3b27cf2bfd1adcfa6e0264eb51c1025ddaf0f342`, `LICENSE` (crate 5.4.1).
- `LICENSES/winapi-MIT.txt`: original winapi-rs Developers MIT text from the
  Cargo-checksummed `winapi 0.3.9` crate; also retained for the Windows GNU
  import-library companion, whose manifest declares the same MIT/Apache-2.0 terms.

When updating these crate versions, review and refresh the corresponding
supplemental licence text too. The files are fetched from immutable upstream
commit URLs during maintenance, not from mutable branches during packaging.

The wrapper has a local, hash-checked patch in `tools/private_companion.py` to
load only the CLAP inside this preview's private VST3 bundle. This changes the
Windows loading path, not the firmware engine. The emulator sources are unmodified.
