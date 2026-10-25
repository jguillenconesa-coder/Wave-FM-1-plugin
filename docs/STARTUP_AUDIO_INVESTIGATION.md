# Startup audio recovery · 2026-10-25

User report: Windows, Intel i7-10700F, X0X 1.1.0, 48 kHz / 64 samples,
Safe, 240 MHz, above-normal worker. Reported engine 1.00x, audio late 44,982
frames/s, plugin latency 1,179 samples / 24.56 ms. The existing counter measures
misses per approximately one-second update, not a precisely normalized rate.
This report alone does not establish insufficient host CPU capacity.

The deterministic regression in `audio_timing.rs` reproduces a software failure:
after 150 ms without DMA audio, an exactly real-time producer cannot retire
startup debt while the consumer continues advancing. Almost all subsequently
produced audio is discarded, despite maintaining real-time production. This
occurs with both 64- and 512-sample host blocks.

The candidate supplies elapsed boot silence on the producer until the new
firmware's first DMA half is ready. The worker then appends that first half
without discarding it. It retains a pending first half if the ring is full.
The consumer publishes its monotonic source-frame cursor once per callback
with an instance-owned lock-free atomic. Priming has a fixed iteration limit
and returns immediately at ring capacity. It stops permanently for that boot
after first DMA output, so it cannot conceal a later synthesis underrun.

The callback's bounded recovery, interpolation, MIDI timestamps and honest
latency calculation remain in use. The patch does not raise host/worker
priority, grow the chosen reserve, alter the firmware binary, change guest
instruction timing, use other instances' state, or introduce I/O or waits in
the audio callback. Core emulation and firmware authors retain their existing
credits; this recovery change is an MIT integration addition.

The real-X0X diagnostic now exercises both 48 kHz / 64 samples (Safe lead
1,083 firmware frames) and 48 kHz / 512 (lead 1,495). It keeps the original
12-second workload, musical event times, positive finite-audio requirement and
zero steady-missing-frame assertion. A standalone read-only Windows workflow
runs both serially and cannot publish an installer. A deterministic regression
passing does not establish that all of the user's persistent dropouts are fixed.

Local validation so far: 43 Windows-target Rust unit tests passed under Wine,
including startup recovery at exactly 1x and bounded/full-ring handling;
16 Python packaging/integrity tests passed. Native Windows results and the
actual 64-sample X0X outcome must be assessed before promotion.
