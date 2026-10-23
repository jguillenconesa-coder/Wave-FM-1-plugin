# Licensing

Ketch is free software under the GNU General Public License, version 3 only (`GPL-3.0-only`, full text in
`LICENSE`). You may use, study, change and share it. If you give someone a firmware built from it, give them its
complete source under the same licence too.

## Who made what

| | |
| --- | --- |
| The platform: `firmware/hal/`, `firmware/loader/`, `crt0.S`, `app.ld`, `firmware/src/` (`usb.c`, `ota.c`, `recovery.c`, `lcd.c`, `gfx.c`, `libc.c`, `bootguard.h`, `uac_fir.h`), `tools/` | Copyright (C) 2026 Leo Kuroshita (@kurogedelic), Hügelton Instruments ([Felucca](https://github.com/hugelton/Felucca)), with the changes of [SLOOP](https://github.com/isod89/sloop-fm1) (isod89) |
| The sound layer `firmware/sound/` (engines, parameters, modulation, voices, slicer, effects) | Felucca, Copyright (C) 2026 Leo Kuroshita (@kurogedelic), Hügelton Instruments; the third-party parts below |
| The installer `web/install/` (`fm1pkg.js`, `fm1ota.js`, the page), `tools/make_site.py` | Felucca, Leo Kuroshita; Ketch's changes |
| The wiring `platform.c`, the host mode `tests/host.c`, the browser emulator `web/emu/` | [Dinghy](https://github.com/zvenson/dinghy), zvenson; the emulator's faceplate from [sloop-web-emu](https://github.com/sabliran/sloop-web-emu) (GPL-3.0) |
| The tracker `firmware/src/tracker.c`, `commands.c`, `sound.c` and the rest of the app, the docs | Adrien Risser-Zhuang |

## Third-party material

| What | Licence | Where |
| --- | --- | --- |
| The PHASE engine's waveforms: a C port of the oscillator of CrispyZebra (<https://github.com/hugelton/CrispyZebra>) | GPL-3.0 | `firmware/sound/eng_phase.c` |
| DaisySP by Electrosmith, Corp and Emilie Gillet (<https://github.com/electro-smith/DaisySP>): the PHYS engine's modal and string models and the resonator, ported to fixed point | MIT | `LICENSES/MIT-DaisySP.txt` |
| Rings by Emilie Gillet (<https://github.com/pichenettes/eurorack>): the PHYS engine's sympathetic strings, ported to fixed point | MIT | `LICENSES/MIT-Rings.txt` |
| msfa by Google Inc. and Pascal Gauthier, from Dexed (<https://github.com/asb2m10/dexed>): the FM6 engine's synthesis, ported to integer C | Apache-2.0 | `LICENSES/Apache-2.0-msfa.txt` |
| Terminus font 8x16 (ter-u16n), rendered into `firmware/gen/felucca_font.h` | SIL OFL 1.1 | `assets/fonts/Terminus-LICENSE.txt` |
| JieLi AC79 SDK: `uboot.boot`, `cfg_tool.bin`, `eq_cfg_hw.bin` are read from your SDK checkout at build time and placed in every `.fwsc` package; no SDK files are in this tree | Apache-2.0, copyright JieLi Technology | `LICENSES/Apache-2.0.txt`, <https://gitee.com/Jieli-Tech/fw-AC79_AIoT_SDK> |

## Trademarks

"Felucca" and "Hügelton Instruments" are names of Hügelton Instruments; "M-VAVE" and "FM-1" belong to their
owners. Ketch is not made or endorsed by them.
