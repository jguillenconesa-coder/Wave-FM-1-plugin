# Desktop UI fork

`src/editor.rs` and `src/panel.rs` derive from Joakim Eriksson's fm1-emu
commit `72adb810c4409c4b7302ca56906c05083834faee`, Copyright (c) 2026
Joakim Eriksson, MIT. The original licence is in the dependency checkout and
distributed as `licenses/fm1-emu-MIT.txt`.

Changes by Wave-FM-1-plugin contributors (2026), MIT: offline firmware dropdown,
verified sidecar packages, confirmation before replacing a running setup,
per-instance skin persisted with the host project, and recolourable panel.
`tools/prepare-desktop.py` verifies the upstream baseline and generates a
separate build tree. It never edits the pinned upstream checkout. The core,
SoC, file loader and audio engine are unchanged.

Firmware packages are separate interpreted programs in `firmware/catalogue`,
not linked into the plugin. Each retains its own licence and corresponding
source; see `firmware/README.md` and `firmware/manifest.json`.
