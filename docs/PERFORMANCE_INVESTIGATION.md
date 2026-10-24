# Windows audio delivery investigation · 2026-10-24

This branch preserves experiments that are **not promoted as the recommended
Windows installer**. Use the platform-specific downloads linked from `main`.
The user's i7-10700F / X0X / 48 kHz / 512-sample failure is not considered fixed.
The owner maintains/publishes this integration; original emulator/firmware
credits remain in AUTHORS.md and the bundled notices.

Recommended Windows runtime: source `53c3966`, tag `v0.2.6-preview.4`.
Recommended macOS runtime: source `123425b`, tag `v0.2.6-preview.6`.
Each has independent native validation and exact source/file-hash provenance.

The GUI's incremental renderer preserves the reference pixels and substantially
reduces the measured cost of drawing unchanged controls. Empty instances report
zero signal latency; loaded modes remain honestly reported. Those changes are
in both recommended packages. Safe remains the legacy default; shorter reserves
can cause more underruns and do not fix insufficient emulation throughput.

Later experiments included direct producer notifications/timed parks, reduced
housekeeping clock queries, strict timing qualification and more instrumentation.
Windows preview.5 failed with 35,858 steady misses and 310 µs steady host lateness;
preview.6 failed with 7,247 / 17 µs. The latter's timed-sleep control also failed
with 6,733 misses. Qualified failures were not retried and neither Windows
candidate was published. The matching macOS revisions passed independently.
A complete pass still requires zero steady missing frames, finite positive audio,
no halt, compute headroom >1x and an in-budget callback.

The read-only `timing-026-1` workflow compared four scenarios on one Windows
runner, recording missed frames per second, per-second compute speed, wait/run/
other time, and refill intervals. All four audio tests failed, including warmed
workloads. Its green **collection-job** status does not mean audio success and
cannot publish any package. Exact evidence: `docs/validation/windows-026-timing-investigation.txt`.
This rejects a startup-only or wait-primitive-only explanation; it does not
establish a single product root cause. Average speed above 1x does not guarantee
all delivery deadlines. One instance still emulates its guest CPUs mainly on a
single host worker; 16 host logical CPUs are not automatically shared by it.

Further changes must preserve firmware/state isolation, correct host latency,
real MIDI timestamps and explicit ownership/lifetime of worker jobs. No release
may be unlocked by a diagnostic control passing after the actual gate failed.
No process-wide priority, power-plan, timer-resolution, host preference, driver
or firmware-binary changes were introduced. The synthesis/resampler are unchanged;
no fidelity or oversampling improvement is claimed. Full Ableton/ASIO acceptance
on the user's machine remains necessary.
