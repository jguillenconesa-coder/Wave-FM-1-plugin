# Credits and third-party notices

Wave FM-1 is an independent Windows/macOS instrument packaging project. It is not an official
M-VAVE product and is not endorsed by M-VAVE, Ableton, Steinberg or the authors
listed here. Their product names remain their respective owners' marks.

The repository owner is the maintainer/publisher, not the author of the original
emulator, plugin, firmware or hardware. See [AUTHORS.md](AUTHORS.md) for roles
and AI-assisted integration disclosure. Original copyright notices are retained.

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

The offline firmware catalogue distributes separate unmodified interpreted programs:

| Firmware | Credit | Licence |
| --- | --- | --- |
| Felucca 1.6.1 | Leo Kuroshita (@kurogedelic), Hügelton Instruments and contributors | GPL-3.0-only; MIT engines, CC0 samples, OFL fonts and Apache-2.0 SDK/DSP notices retained. |
| X0X 1.1.0 | Charles Vestal; Felucca platform by Leo Kuroshita; 9W9/8W8 by athousanddetails; ER-99 by Matthew Cieplak; Open303 by Robin Schmidt; TB-3PO contributors; BB Gen by mestela (author permission documented upstream) | GPL-3.0-only; third-party MIT, GPL, OFL and Apache notices retained. |
| AMB-1 1.3 | Charles Vestal; fm1-base; felucca-bsp by Leo Kuroshita / Hügelton Instruments; FoMni Dattorro plate lineage | MPL-2.0 for this release, with third-party notices retained. Earlier catalogue descriptions calling all versions GPL are outdated. |
| FoMni-1 0.4 | Charles Vestal; Felucca/X0X platform contributors; Chordian by Jan125 | GPL-3.0-only; Chordian data CC0; OFL fonts and Apache SDK notices retained. |
| Polly 0.5 | Charles Vestal; fm1-base and felucca-bsp contributors; Sonivox EAS (Sonic Network Inc., Android Open Source Project); Groove MIDI Dataset by Jon Gillick, Adam Roberts, Jesse Engel, Douglas Eck and David Bamman, Google Magenta / © Google LLC | GPL-3.0-only app, MPL-2.0 platform, Apache-2.0 EAS and SDK, CC BY 4.0 drum performances; upstream modifications and source credits in `firmware/licenses/polly/styles/README.md`. |
| Skiff 1.4 | illuminus5; Leo Kuroshita / Hügelton Instruments; DSP and font contributors listed in its LICENSING.md | GPL-3.0-only, with MIT, Apache-2.0, CC0 and OFL notices preserved. |
| DUB-1 1.2.0 | sbegueria; Charles Vestal (AMB-1/FoMni/X0X); Leo Kuroshita / Hügelton Instruments | GPL-3.0-only, with OFL and Apache-2.0 notices. |


The additional catalogue entries and their authors are listed in
[`firmware/README.md`](firmware/README.md): fm1-chord, CTL-1 (USB), FiMba-1,
MIN-1, Niagara Drift, TRIBE, Chipbox, Frodhes, FMini-1, Felucca 8 Patterns,
Pocket GammaWave, NOIZ DEMONS, Unholy Union, WaveLoop FM-1, Ketch (Adrien
Risser-Zhuang) and GR!MM (GRiMMBIENT), with zvenson, isod89 and Hügelton
Instruments foundation credits. Each has its own
`firmware/licenses/<id>/` notices, including inherited DSP, font and SDK credits.

Full upstream notices are in `firmware/licenses/<id>/`, and complete corresponding
sources with recursive submodules are in `firmware/sources/`. Both are also included
inside every VST3 package. See `firmware/manifest.json` for exact versions, source
revisions, original URLs and hashes. Their own licences apply; the wrapper's MIT
licence does not relicense firmware. The catalogue is an aggregation of independent
programs, not linked firmware code. Loading a firmware or sharing a `.fm1save`
does not change its licence. No proprietary stock firmware is distributed.

Desktop changes to the upstream MIT panel/editor are identified in `desktop/README.md`.
Skins are original drawn colour palettes, with no manufacturer artwork copied.

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

macOS Rust bindings also credit Steven Sheldon and the objc2 contributors. Some
registry archives omit a standalone licence; the shared upstream texts and
package authorship are retained in `LICENSES/Steven-Sheldon-MIT.txt`,
`objc2-MIT.txt`, `objc2-LICENSING.md`, `malloc-buf-MIT.txt` and
`macos-Rust-AUTHORS.txt`, and collected for each affected package. The MIT option
is selected where offered. Builds use the native Apple SDK under its own terms;
no Apple SDK is redistributed.
