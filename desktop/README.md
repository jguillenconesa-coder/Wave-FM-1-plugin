# Desktop UI fork

`src/editor.rs` and `src/panel.rs` derive from Joakim Eriksson's fm1-emu
commit `72adb810c4409c4b7302ca56906c05083834faee`, Copyright (c) 2026
Joakim Eriksson, MIT. The original licence is in the dependency checkout and
distributed as `licenses/fm1-emu-MIT.txt`.

Changes by Wave-FM-1-plugin contributors (2026), MIT: offline firmware dropdown,
verified sidecar packages, confirmation before replacing a running setup,
per-instance skin persisted with the host project, recolourable panel, clearly
labelled MIDI/Live view, 21 new host parameters, and an optional community browser.
`tools/prepare-desktop.py` verifies the upstream baseline and generates a
separate build tree. It never edits the pinned upstream checkout. The core,
SoC and file loader remain unchanged. Verified patches to the engine route host
encoder changes and finite button pulses on its worker thread; its audio callback
does not perform network/file I/O or allocate for these controls.

`controls.rs` and `online.rs` are integration additions (MIT). The community
index metadata derives from `web/firmwares.json` of the same fm1-emu revision,
by Joakim Eriksson (MIT); firmware authors are named individually. Alternate
URLs and hashes identify original public GitHub releases inspected for this
preview. They are references to third-party downloads, not bundled firmware
or a grant of redistribution rights. The Hub service is credited to its
maintainer zvenson as recorded by upstream; no affiliation is claimed.

Network access is opt-in for each check/download, using Windows WinHTTP or
macOS's built-in `/usr/bin/curl` with no shell or user curl configuration.
TLS remains enabled, hosts and redirects are allowlisted, bytes/time are bounded,
downloads are parsed and saved without overwriting an existing file. They are
not run automatically. A hash for a pinned author release verifies integrity
against that reference, not authorship, safety or full functionality.

Firmware packages are separate interpreted programs in `firmware/catalogue`,
not linked into the plugin. Each retains its own licence and corresponding
source; see `firmware/README.md` and `firmware/manifest.json`.
