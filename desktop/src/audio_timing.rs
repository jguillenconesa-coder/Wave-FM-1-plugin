// Resampler derived from fm1-emu by Joakim Eriksson (MIT); timing recovery additions, 2026.
use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Default)]
pub struct Diagnostics { rate: AtomicU32, block: AtomicU32 }

impl Diagnostics {
    pub fn configure(&self, rate: f32, block: u32) {
        self.rate.store(rate.to_bits(), Ordering::Relaxed);
        self.block.store(block, Ordering::Relaxed);
    }
    pub fn observe_block(&self, samples: u32) { self.block.store(samples, Ordering::Relaxed); }
    pub fn host(&self) -> (f32, u32) {
        (f32::from_bits(self.rate.load(Ordering::Relaxed)), self.block.load(Ordering::Relaxed))
    }
}

#[derive(Default)]
pub struct Playback {
    pub position: u64,
    frac: f64,
    cur: [f32; 2],
    next: [f32; 2],
    debt: u64,
}

impl Playback {
    /// Silence missing audio and discard its late replacement when it arrives.
    /// Holding the frame counter on an underrun stretches every later note/beat.
    /// Recovery work is bounded, even after a long period without machine audio.
    pub fn render(&mut self, out: &mut [&mut [f32]], ratio: f64,
                  pop: impl FnMut() -> Option<[f32; 2]>) -> u64 {
        // Recovery must scale with this callback, with enough headroom to reduce
        // existing debt as the host continues advancing. Limiting it to only one
        // block can keep re-adding the same debt indefinitely after startup.
        let budget = ((out[0].len() as f64 * ratio).ceil().max(1.0) as usize).saturating_mul(2);
        self.render_with_budget(out, ratio, pop, budget)
    }

    fn render_with_budget(&mut self, out: &mut [&mut [f32]], ratio: f64,
                          mut pop: impl FnMut() -> Option<[f32; 2]>, mut recovery_budget: usize) -> u64 {
        let mut missed = 0;
        for i in 0..out[0].len() {
            self.frac += ratio;
            while self.frac >= 1.0 {
                self.frac -= 1.0;
                self.position = self.position.saturating_add(1);
                while self.debt > 0 && recovery_budget > 0 {
                    if pop().is_none() { break; }
                    self.debt -= 1;
                    recovery_budget -= 1;
                }
                let frame = if self.debt == 0 { pop() } else { None };
                self.cur = self.next;
                self.next = match frame {
                    Some(frame) => frame,
                    None => {
                        self.debt = self.debt.saturating_add(1);
                        missed += 1;
                        [0.0; 2]
                    }
                };
            }
            let t = self.frac as f32;
            for c in 0..2 { out[c][i] = self.cur[c] + (self.next[c] - self.cur[c]) * t; }
        }
        missed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn an_underrun_does_not_slow_the_timeline_or_replay_late_audio() {
        let mut reference = Playback::default(); let mut stalled = Playback::default();
        let mut stream = (0..100).map(|i| [i as f32; 2]);
        let mut a = [0.0; 12]; let mut b = [0.0; 12];
        reference.render(&mut [&mut a, &mut b], 1.0, || stream.next());
        let mut first = (0..2).map(|i| [i as f32; 2]);
        assert_eq!(stalled.render(&mut [&mut a, &mut b], 1.0, || first.next()), 10);
        assert_eq!(stalled.position, 12);
        assert!(a[3..].iter().all(|&v| v == 0.0));
        let mut late = (2..100).map(|i| [i as f32; 2]);
        let mut expected = [0.0; 8]; let mut scratch = [0.0; 8];
        reference.render(&mut [&mut expected, &mut scratch], 1.0, || stream.next());
        let mut actual = [0.0; 8];
        assert_eq!(stalled.render(&mut [&mut actual, &mut scratch], 1.0, || late.next()), 0);
        assert_eq!(&actual[1..], &expected[1..]);
        assert_eq!(stalled.position, reference.position);
        assert_eq!(stalled.debt, 0);
    }

    #[test]
    fn host_rate_and_missing_audio_do_not_change_clock_speed_or_other_instances() {
        for host_rate in [44100.0, 48000.0, 96000.0] {
            let mut playback = Playback::default();
            let untouched = Playback::default();
            let mut left = [0.0; 127]; let mut right = [0.0; 127];
            let ratio = 44100.0 / host_rate;
            for _ in 0..1000 { playback.render(&mut [&mut left, &mut right], ratio, || None); }
            assert!((playback.position as f64 - 127000.0 * ratio).abs() <= 1.0);
            assert_eq!(untouched.position, 0);
            assert!(left.iter().all(|&v| v == 0.0));
        }
    }

    #[test]
    fn recovering_a_long_gap_has_a_finite_callback_work_budget() {
        for size in [32, 64, 128, 512, 8192] {
            let mut playback = Playback { debt: 1_000_000, ..Default::default() };
            let mut left = vec![0.0; size]; let mut right = vec![0.0; size]; let mut reads = 0;
            assert_eq!(playback.render(&mut [&mut left, &mut right], 1.0,
                || { reads += 1; Some([0.0; 2]) }), size as u64);
            assert_eq!(reads, size * 2);
            assert_eq!(playback.position, size as u64);
        }
    }

    #[test]
    #[ignore = "Opt-in wall-clock diagnostic: real X0X, 48 kHz, 512-sample callbacks"]
    fn realtime_x0x_48k_512() {
        use crate::{engine::{self, Cmd, Shared, View}, Fm1Params};
        use std::{sync::{mpsc, Arc, Mutex, atomic::Ordering}, time::{Duration, Instant}};
        let directory = std::env::var_os("FM1_CATALOGUE_TEST_DIR").map(std::path::PathBuf::from)
            .unwrap_or_else(|| std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../firmware/catalogue"));
        let entry = crate::catalogue::ENTRIES.iter().find(|e| e.id == "x0x").unwrap();
        let firmware = crate::catalogue::read_at(&directory, entry).unwrap();
        let shared = Arc::new(Shared { view: Mutex::new(View {
            lcd: vec![0; 240 * 240], leds: [0.0; 41], pads: [0; 16], board: fm1_soc::Board::Fm1,
            booted: false, halted: false, speed: 0.0, starved: 0, late: 0 }), quit: Default::default() });
        let params = Arc::new(Fm1Params::default());
        let (cmd, commands) = mpsc::channel();
        let (mut midi, midi_rx) = rtrb::RingBuffer::new(4096);
        let (audio_tx, mut audio) = rtrb::RingBuffer::new(16384);
        let worker_shared = shared.clone(); let worker_params = params.clone();
        let worker = std::thread::spawn(move || engine::run(worker_shared, worker_params, commands, midi_rx, audio_tx));
        let lead = std::env::var("FM1_REALTIME_LEAD").ok().and_then(|s| s.parse().ok()).unwrap_or(1495);
        cmd.send(Cmd::Lead(lead)).unwrap();
        let (reply, answer) = mpsc::channel();
        cmd.send(Cmd::Load { firmware, sectors: Vec::new(), reply }).unwrap();
        let boot = answer.recv_timeout(Duration::from_secs(20));
        if !matches!(&boot, Ok(Ok(()))) {
            shared.quit.store(true, Ordering::Relaxed); worker.join().unwrap();
            panic!("X0X boot failed: {boot:?}");
        }
        let mut playback = Playback::default();
        let mut left = [0.0; 512]; let mut right = [0.0; 512];
        let ratio = 44100.0 / 48000.0;
        let started = Instant::now(); let mut startup_missed = 0; let mut steady_missed = 0;
        let mut max_callback = Duration::ZERO; let mut max_host_late = Duration::ZERO; let mut peak = 0.0f32;
        let legacy = std::env::var_os("FM1_REALTIME_LEGACY").is_some();
        for block in 0..1125u32 {
            let deadline = started + Duration::from_secs_f64(block as f64 * 512.0 / 48000.0);
            // A real host calls from its audio driver, not a coalesced sleep
            // timer. Darwin CI can oversleep nanosleep by tens of milliseconds,
            // making this fake host burst many callbacks and starve any queue.
            // Yield until the deadline in this isolated, opt-in diagnostic only;
            // the shipped callback/worker do not busy-wait.
            while Instant::now() < deadline { std::thread::yield_now(); }
            max_host_late = max_host_late.max(Instant::now().saturating_duration_since(deadline));
            if block == 188 { cmd.send(Cmd::Key { id: 12, down: true }).unwrap(); }
            if block == 198 { cmd.send(Cmd::Key { id: 12, down: false }).unwrap(); }
            if block == 282 || block == 750 {
                cmd.send(Cmd::Key { id: 26, down: block == 282 }).unwrap();
                for channel in 0..16 {
                    midi.push(engine::Midi { frame: playback.position + lead as u64,
                        bytes: [(if block == 282 { 0x90 } else { 0x80 }) | channel, 60, 100], len: 3 }).unwrap();
                }
            }
            if block >= 282 && block % 12 == 0 {
                // Default X0X GM inputs: 909 on channel 10, 808 on channel 11.
                // Keep exercising voices instead of timing a silent/idle synth.
                for (channel, note) in [(9, 36), (9, 42), (10, 42), (10, 38)] {
                    midi.push(engine::Midi { frame: playback.position + lead as u64,
                        bytes: [0x90 | channel, note, 100], len: 3 }).unwrap();
                }
            }
            let call = Instant::now();
            let missed = if legacy {
                playback.render_with_budget(&mut [&mut left, &mut right], ratio, || audio.pop().ok(), 16384)
            } else { playback.render(&mut [&mut left, &mut right], ratio, || audio.pop().ok()) };
            max_callback = max_callback.max(call.elapsed());
            if block < 188 { startup_missed += missed; } else { steady_missed += missed; }
            for &sample in left.iter().chain(&right) { assert!(sample.is_finite()); peak = peak.max(sample.abs()); }
        }
        shared.quit.store(true, Ordering::Relaxed); worker.join().unwrap();
        let view = shared.view.lock().unwrap();
        eprintln!("REALTIME X0X 48k/512 lead={lead} legacy={legacy}: startup_missing={startup_missed}, steady_missing={steady_missed}, debt={}, position={}, callback_max_us={}, host_late_max_us={}, engine_speed={:.3}, peak={peak:.5}, halted={}",
            playback.debt, playback.position, max_callback.as_micros(), max_host_late.as_micros(), view.speed, view.halted);
        assert!(view.booted && !view.halted && peak > 0.0001);
        assert_eq!(steady_missed, 0, "steady playback missed frames; inspect diagnostic timing/load");
    }
}
