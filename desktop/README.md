# Desktop UI fork

`src/editor.rs` and `src/panel.rs` derive from Joakim Eriksson's fm1-emu
commit `ba11e2e68f3934a2c5c98a0eb55607387e262a49`, Copyright (c) 2026
Joakim Eriksson, MIT. The original licence is in the dependency checkout and
distributed as `licenses/fm1-emu-MIT.txt`.

Changes by Wave-FM-1-plugin contributors (2026), MIT: offline firmware dropdown,
verified sidecar packages, confirmation before replacing a running setup,
per-instance skin persisted with the host project, recolourable panel, clearly
labelled MIDI/Live view, 21 new host parameters, and an optional community browser.
`tools/prepare-desktop.py` verifies the upstream baseline and generates a
separate build tree. It never edits the pinned upstream checkout. The core
and file loader remain unchanged. Verified patches to the engine route host
encoder changes and finite button pulses on its worker thread; its audio callback
does not perform network/file I/O or allocate for these controls.

From 0.2.4, the separate generated SoC has a hash-checked MIT timing patch:
it records the last audio DMA deadline and inserts timestamped MIDI input into
the existing peripheral event queue, invalidating its JIT deadline cache. The
pristine pinned SoC checkout is unchanged. `host_sync.rs` generates optional
24-PPQN MIDI Clock and transport from host blocks; `midi_timing.rs` schedules it
on emulated cycles. `audio_timing.rs` preserves the host timeline during missing
audio and discards late frames with a finite recovery budget. The additional
clock-source parameter defaults to internal and keeps every prior host ID.

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

From 0.2.6, an instance without firmware reports zero latency. A bounded
`audio_reserve` parameter chooses Low / Balanced / Safe (legacy default),
applied while stopped. Loaded latency remains honestly reported to the host.
The instance worker uses public Windows thread-priority/macOS QoS APIs with fallback and
thread-scoped cleanup. No global host or installer setting changes. The FM-1
GUI caches unchanged controls, with a full-render pixel regression across all
six skins and an opt-in drawing benchmark. New host-side VST3 checks verify
actual empty audio processing and reported latency across nine configurations.
These integration changes are MIT; original emulator/panel authorship remains
Joakim Eriksson's. The resampler and firmware sound algorithms are unchanged.

From 0.2.7, boot-only producer silence priming prevents initial missing DMA
frames from poisoning bounded recovery at exactly real-time production. The
callback publishes its source position atomically; the worker retains its first
DMA buffer when space is unavailable. Priming ends for that boot after first
output. Recovery after later underruns remains observable. Native release gates
exercise actual X0X at both 48 kHz / 64 and 512 host samples. The emulator and
firmware authors retain the credits above; the integration change is MIT.

Completed host-owned DMA copies are published before CPU1's following work,
with a post-run fallback. Their Vec capacity is reused. The real-X0X regression
compares samples and complete guest RAM/flash/LCD, registers and cycles against
the original runner. Windows tests use a private high-resolution host timer;
the timer FFI is cfg(test) and is absent from the plugin DLL. These delivery
changes are MIT integration additions; original CPU/SoC/DSP authorship remains
unchanged.

Version 0.2.8 adds a producer/consumer pause handshake after a mid-stream underrun
exhausts the reserve. The consumer flushes only after the producer acknowledges;
the worker re-primes audio at the monotonic host position without rebooting the
guest or clearing patches. Future MIDI deadlines are rebased on the worker.
Internal transport is preserved; DAW mode resynchronizes its transport clock.
A real guest halt is separately reported and is not concealed by recovery.
Upstream ba11e2e also restores global interrupt enable on icfg writes and fixes
branch flag liveness/self-modifying RAM invalidation; its original authors retain
credit. Test-only forced stalls are excluded from production builds.

Ordinary mouse encoders now use the existing saved host parameters with bounded
0–127 reference positions. Shift gives finer motion; Alt retains relative menu
navigation. Host parameter IDs and the VST3 class identity are unchanged.
