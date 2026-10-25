// Resampler derived from fm1-emu by Joakim Eriksson (MIT); timing recovery additions, 2026.
use nih_plug::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};

#[derive(Enum, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Reserve {
    #[name = "Low latency"]
    Low,
    #[name = "Balanced"]
    Balanced,
    #[name = "Safe / legacy"]
    Safe,
}

/// Firmware frames, not host samples. Keep room for one maximum DMA half in
/// the ring. Lower reserves are optional: they require more scheduling headroom.
pub fn lead_for(ratio: f64, host_block: u32, reserve: Reserve) -> Option<usize> {
    if !ratio.is_finite() || ratio <= 0.0 || host_block == 0 { return None; }
    let block = (host_block as f64 * ratio).ceil();
    let extra = match reserve { Reserve::Low => 64.0, Reserve::Balanced => 512.0, Reserve::Safe => 1024.0 };
    if !block.is_finite() { return None; }
    // Hosts may negotiate huge blocks or unusual sample rates (including the
    // official VST3 format tests). Cap the reserve to the fixed ring capacity;
    // do not reject activation after setupProcessing accepted the format.
    Some((block + extra).clamp(512.0, (16384 - 512) as f64) as usize)
}

pub fn latency_samples(lead: usize, ratio: f64, loaded: bool) -> u32 {
    if loaded { (lead as f64 / ratio).round() as u32 } else { 0 }
}

#[derive(Default)]
pub struct Diagnostics {
    rate: AtomicU32, block: AtomicU32,
    lead: AtomicU32, latency: AtomicU32,
    loaded: AtomicBool, playing: AtomicBool,
    scheduling: AtomicU32,
    position: AtomicU64,
    ready: AtomicBool,
    recovery_request: AtomicU64,
    recovery_paused: AtomicU64,
    recovery_resume: AtomicU64,
    recovery_completed: AtomicU64,
    recovery_base: AtomicU64,
    recoveries: AtomicU64,
}

impl Diagnostics {
    pub fn publish_position(&self, position: u64) { self.position.store(position, Ordering::Release); }
    pub fn position(&self) -> u64 { self.position.load(Ordering::Acquire) }
    pub fn reset_stream(&self) {
        self.ready.store(false, Ordering::Release);
        let generation = self.recovery_request.load(Ordering::Acquire);
        self.recovery_completed.store(generation, Ordering::Release);
        self.recoveries.store(0, Ordering::Relaxed);
    }
    pub fn stream_halted(&self) {
        self.ready.store(false, Ordering::Release);
        let generation = self.recovery_request.load(Ordering::Acquire);
        self.recovery_completed.store(generation, Ordering::Release);
    }
    pub fn stream_ready(&self) {
        self.ready.store(true, Ordering::Release);
        let generation = self.recovery_request.load(Ordering::Acquire);
        if self.recovery_resume.load(Ordering::Acquire) == generation
            && self.recovery_completed.load(Ordering::Acquire) != generation {
            self.recoveries.fetch_add(1, Ordering::Relaxed);
            self.recovery_completed.store(generation, Ordering::Release);
        }
    }
    fn request_recovery(&self) -> Option<u64> {
        let previous = self.recovery_request.load(Ordering::Acquire);
        if previous != self.recovery_completed.load(Ordering::Acquire) || previous == u64::MAX { return None; }
        let generation = previous + 1;
        self.recovery_request.store(generation, Ordering::Release);
        Some(generation)
    }
    /// Called only by the producer at a slice boundary, after its last push.
    pub fn pending_recovery(&self) -> Option<u64> {
        let generation = self.recovery_request.load(Ordering::Acquire);
        (generation != self.recovery_completed.load(Ordering::Acquire)).then_some(generation)
    }
    pub fn pause_recovery(&self, generation: u64) {
        self.ready.store(false, Ordering::Release);
        self.recovery_paused.store(generation, Ordering::Release);
    }
    pub fn recovery_resume(&self, generation: u64) -> Option<u64> {
        (self.recovery_resume.load(Ordering::Acquire) == generation)
            .then(|| self.recovery_base.load(Ordering::Acquire))
    }
    pub fn recovery_count(&self) -> u64 { self.recoveries.load(Ordering::Relaxed) }
    pub fn recovering(&self) -> bool { self.pending_recovery().is_some() }
    pub fn configure(&self, rate: f32, block: u32) {
        self.rate.store(rate.to_bits(), Ordering::Relaxed);
        self.block.store(block, Ordering::Relaxed);
    }
    pub fn observe_block(&self, samples: u32) { self.block.store(samples, Ordering::Relaxed); }
    pub fn request_lead(&self, frames: usize) { self.lead.store(frames as u32, Ordering::Relaxed); }
    pub fn requested_lead(&self) -> usize { self.lead.load(Ordering::Relaxed) as usize }
    pub fn set_loaded(&self, loaded: bool) { self.loaded.store(loaded, Ordering::Relaxed); }
    pub fn loaded(&self) -> bool { self.loaded.load(Ordering::Relaxed) }
    pub fn publish_latency(&self, samples: u32) { self.latency.store(samples, Ordering::Relaxed); }
    pub fn latency(&self) -> u32 { self.latency.load(Ordering::Relaxed) }
    pub fn set_playing(&self, playing: bool) { self.playing.store(playing, Ordering::Relaxed); }
    pub fn playing(&self) -> bool { self.playing.load(Ordering::Relaxed) }
    pub fn set_scheduling(&self, status: u32) { self.scheduling.store(status, Ordering::Relaxed); }
    pub fn scheduling(&self) -> &'static str {
        match self.scheduling.load(Ordering::Relaxed) {
            1 => "Windows above-normal", 2 => "macOS user-interactive", _ => "Standard scheduling",
        }
    }
    pub fn host(&self) -> (f32, u32) {
        (f32::from_bits(self.rate.load(Ordering::Relaxed)), self.block.load(Ordering::Relaxed))
    }
}

/// Consumer/producer handshake for an underrun that cannot catch up at 1x.
/// Only the consumer flushes its ring, after the producer acknowledges a pause.
/// No waits, command allocations, locks or firmware reloads in the callback.
#[derive(Default)]
pub struct Recovery {
    pending: Option<u64>,
    waiting_pause: bool,
    stalled: u64,
    last_position: u64,
    last_request: Option<u64>,
}

impl Recovery {
    pub fn before_block(&mut self, diagnostics: &Diagnostics, playback: &mut Playback,
                        audio: Option<&mut rtrb::Consumer<[f32; 2]>>) -> bool {
        let Some(generation) = self.pending else { return false; };
        if diagnostics.recovery_completed.load(Ordering::Acquire) == generation {
            self.pending = None; self.waiting_pause = false;
            return false;
        }
        if !self.waiting_pause || diagnostics.recovery_paused.load(Ordering::Acquire) != generation { return false; }
        let Some(audio) = audio else { return false; };
        let available = audio.slots();
        if let Ok(chunk) = audio.read_chunk(available) { chunk.commit_all(); }
        else { return false; }
        playback.debt = 0; playback.cur = [0.0; 2]; playback.next = [0.0; 2];
        // Keep the host frame counter and fractional rate phase monotonic.
        diagnostics.recovery_base.store(playback.position, Ordering::Release);
        diagnostics.recovery_resume.store(generation, Ordering::Release);
        self.waiting_pause = false;
        self.stalled = 0;
        true
    }
    pub fn holding(&self) -> bool { self.waiting_pause }
    pub fn observe(&mut self, diagnostics: &Diagnostics, playback: &Playback,
                   missed: u64, lead: usize) {
        let elapsed = playback.position.saturating_sub(self.last_position);
        self.last_position = playback.position;
        if missed == 0 { self.stalled = 0; } else { self.stalled = self.stalled.saturating_add(elapsed); }
        if self.pending.is_some() || !diagnostics.loaded()
            || !diagnostics.ready.load(Ordering::Acquire) { return; }
        let cooled = self.last_request.is_none_or(|at| playback.position.saturating_sub(at) >= 44100);
        if missed > 0 && cooled && (playback.debt >= lead as u64 || self.stalled >= 11025) {
            if let Some(generation) = diagnostics.request_recovery() {
                self.pending = Some(generation); self.waiting_pause = true;
                self.last_request = Some(playback.position);
            }
        }
    }
}

/// Before a new firmware has produced its first DMA half, elapsed host time is
/// boot silence, not missing synthesis. Supply that silence on the producer so
/// the bounded consumer can retire startup debt without needing to emulate the
/// entire boot delay faster than real time. Also used after the paused-producer
/// handshake establishes a new host audio origin; never during normal rendering.
/// A full ring returns immediately; the worker retries without advancing the
/// guest or losing its first DMA buffer. Callback work remains unchanged.
pub fn prime_startup(made: &mut u64, position: u64, lead: usize,
                     mut push: impl FnMut([f32; 2]) -> bool) -> bool {
    let target = position.saturating_add(lead as u64);
    for _ in 0..16384 {
        if *made >= target { return true; }
        if !push([0.0; 2]) { return false; }
        *made = made.saturating_add(1);
    }
    *made >= target
}

/// Transfer a completed, host-owned DMA copy without changing guest memory or
/// interrupt state. Retain an unpublishable first half; reuse its allocation.
pub fn publish_dma(audio: &mut Vec<i16>, made: &mut u64,
                   out: &mut rtrb::Producer<[f32; 2]>, position: u64,
                   lead: usize, warming: &mut bool) -> bool {
    if audio.is_empty() { return true; }
    if *warming {
        if !prime_startup(made, position, lead, |f| out.push(f).is_ok())
            || out.slots() < audio.len() / 2 { return false; }
        *warming = false;
    }
    for f in audio.chunks_exact(2) {
        if out.push([f[0] as f32 / 32768.0, f[1] as f32 / 32768.0]).is_ok() {
            *made = made.saturating_add(1);
        }
    }
    audio.clear();
    true
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

    // A driver wakes its callback; it does not spend the whole block occupying
    // a logical CPU. Keep the old busy host as an explicit diagnostic control.
    // This timer exists only in the test executable, never in the plugin DLL.
    struct HostDeadline {
        #[cfg(windows)]
        timer: *mut std::ffi::c_void,
        busy: bool,
        #[cfg(windows)]
        cpu_started: u64,
    }

    #[cfg(windows)]
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn CreateWaitableTimerExW(attrs: *const std::ffi::c_void, name: *const u16, flags: u32, access: u32) -> *mut std::ffi::c_void;
        fn SetWaitableTimer(timer: *mut std::ffi::c_void, due: *const i64, period: i32,
            callback: *const std::ffi::c_void, arg: *const std::ffi::c_void, resume: i32) -> i32;
        fn WaitForSingleObject(handle: *mut std::ffi::c_void, millis: u32) -> u32;
        fn CloseHandle(handle: *mut std::ffi::c_void) -> i32;
        fn GetCurrentThread() -> *mut std::ffi::c_void;
        fn GetThreadTimes(handle: *mut std::ffi::c_void, creation: *mut u64,
            exit: *mut u64, kernel: *mut u64, user: *mut u64) -> i32;
    }

    #[cfg(windows)]
    fn host_cpu_ticks() -> u64 {
        let (mut creation, mut exit, mut kernel, mut user) = (0, 0, 0, 0);
        // FILETIME is two adjacent u32 words, represented here by aligned u64
        // storage on the supported little-endian x64 Windows target.
        assert_ne!(unsafe { GetThreadTimes(GetCurrentThread(), &mut creation, &mut exit, &mut kernel, &mut user) }, 0);
        kernel.saturating_add(user)
    }

    impl HostDeadline {
        fn new() -> Self {
            let busy = std::env::var_os("FM1_REALTIME_BUSY_HOST").is_some();
            #[cfg(windows)]
            {
                // High-resolution auto-reset timer since Windows 10 1803.
                // Thread-local test handle; no global timer-resolution change.
                let timer = if busy { std::ptr::null_mut() } else {
                    unsafe { CreateWaitableTimerExW(std::ptr::null(), std::ptr::null(), 2, 0x0010_0002) }
                };
                assert!(busy || !timer.is_null(), "high-resolution host timer unavailable: {}", std::io::Error::last_os_error());
                Self { timer, busy, cpu_started: host_cpu_ticks() }
            }
            #[cfg(not(windows))]
            Self { busy }
        }
        fn wait(&self, deadline: std::time::Instant) {
            #[cfg(windows)]
            if !self.busy {
                if let Some(remaining) = deadline.checked_duration_since(std::time::Instant::now()) {
                    let ticks = (remaining.as_nanos() / 100).clamp(1, i64::MAX as u128) as i64;
                    let due = -ticks;
                    assert_ne!(unsafe { SetWaitableTimer(self.timer, &due, 0, std::ptr::null(), std::ptr::null(), 0) }, 0);
                    assert_eq!(unsafe { WaitForSingleObject(self.timer, 1000) }, 0, "host timer wait failed");
                }
            }
            while std::time::Instant::now() < deadline { std::thread::yield_now(); }
        }
        fn report(&self) {
            #[cfg(windows)]
            eprintln!("HOST_CLOCK busy_control={} host_thread_cpu_us={}", self.busy, host_cpu_ticks().saturating_sub(self.cpu_started) / 10);
            #[cfg(not(windows))]
            eprintln!("HOST_CLOCK busy_control={} non_windows_yield_host=true", self.busy);
        }
    }

    impl Drop for HostDeadline {
        fn drop(&mut self) {
            #[cfg(windows)]
            if !self.timer.is_null() { assert_ne!(unsafe { CloseHandle(self.timer) }, 0); }
        }
    }
    #[test]
    fn empty_instances_have_no_signal_latency_and_reserves_use_host_units() {
        let ratio = 44100.0 / 48000.0;
        for (mode, frames, samples) in [(Reserve::Low, 535, 582), (Reserve::Balanced, 983, 1070), (Reserve::Safe, 1495, 1627)] {
            let lead = lead_for(ratio, 512, mode).unwrap();
            assert_eq!(lead, frames);
            assert_eq!(latency_samples(lead, ratio, false), 0);
            assert_eq!(latency_samples(lead, ratio, true), samples);
        }
        for ratio in [1.0, 44100.0 / 96000.0, 44100.0 / 192000.0] {
            for size in [16, 64, 128, 512, 2048, 8192] {
                let low = lead_for(ratio, size, Reserve::Low).unwrap();
                let safe = lead_for(ratio, size, Reserve::Safe).unwrap();
                assert!(low >= 512 && low <= safe && safe <= 16384 - 512);
            }
        }
        assert!(lead_for(f64::NAN, 512, Reserve::Safe).is_none());
        assert!(lead_for(0.0, 512, Reserve::Safe).is_none());
        assert!(lead_for(1.0, 0, Reserve::Safe).is_none());
        assert_eq!(lead_for(1.0, 16384, Reserve::Safe), Some(16384 - 512));
    }
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
    fn boot_silence_recovers_even_when_active_production_is_only_realtime() {
        // 150 ms without a DMA frame, then exactly 1x audio production. No OS
        // clock or fast test machine can conceal the startup-debt failure here.
        for host_size in [64, 512] {
            let ratio = 44100.0 / 48000.0;
            let lead = lead_for(ratio, host_size as u32, Reserve::Safe).unwrap();
            let simulate = |pad: bool| {
                let (mut tx, mut rx) = rtrb::RingBuffer::new(16384);
                let mut playback = Playback::default();
                let mut made = 0u64; let mut generated = 0u64;
                let mut left = vec![0.0; host_size]; let mut right = left.clone();
                let mut steady_missing = 0; let mut audible = 0;
                let blocks = 96000 / host_size;
                let boot_blocks = 7200 / host_size;
                for block in 0..blocks {
                    if pad && block <= boot_blocks {
                        assert!(prime_startup(&mut made, playback.position, lead, |f| tx.push(f).is_ok()));
                    }
                    if block >= boot_blocks {
                        let next = (((block - boot_blocks + 1) * host_size) as f64 * ratio).floor() as u64;
                        for _ in generated..next { tx.push([0.25, -0.25]).unwrap(); made += 1; }
                        generated = next;
                    }
                    let missing = playback.render(&mut [&mut left, &mut right], ratio, || rx.pop().ok());
                    if block * host_size >= 48000 {
                        steady_missing += missing;
                        audible += left.iter().filter(|&&s| s > 0.2).count();
                    }
                }
                (steady_missing, audible, playback.debt)
            };
            let unprimed = simulate(false);
            assert!(unprimed.0 > 43000 && unprimed.1 == 0 && unprimed.2 > 6000, "{unprimed:?}");
            assert_eq!(simulate(true), (0, 48000 / host_size * host_size, 0));
        }
    }

    #[test]
    fn startup_padding_stops_at_capacity_and_preserves_the_producer_cursor() {
        let (mut tx, mut rx) = rtrb::RingBuffer::new(64);
        let mut made = 10;
        assert!(!prime_startup(&mut made, 1_000_000, 1083, |f| tx.push(f).is_ok()));
        assert_eq!(made, 74);
        for _ in 0..64 { assert_eq!(rx.pop().unwrap(), [0.0; 2]); }
        let mut calls = 0;
        assert!(!prime_startup(&mut made, 1_000_000, 1083, |_| { calls += 1; true }));
        assert_eq!(calls, 16384);
        assert_eq!(made, 74 + 16384);
        assert!(prime_startup(&mut made, 0, 1083, |_| panic!("already ahead")));
    }

    #[test]
    fn a_full_ring_retains_the_first_dma_then_reuses_its_allocation() {
        let (mut tx, mut rx) = rtrb::RingBuffer::new(64);
        let mut made = 0; let mut warming = true;
        assert!(prime_startup(&mut made, 0, 64, |f| tx.push(f).is_ok()));
        let mut audio = vec![16384, -16384, 8192, -8192];
        let allocation = audio.as_ptr(); let capacity = audio.capacity();
        assert!(!publish_dma(&mut audio, &mut made, &mut tx, 0, 64, &mut warming));
        assert_eq!(audio, [16384, -16384, 8192, -8192]);
        assert!(warming); assert_eq!(made, 64);
        assert_eq!(rx.pop().unwrap(), [0.0; 2]);
        assert_eq!(rx.pop().unwrap(), [0.0; 2]);
        assert!(publish_dma(&mut audio, &mut made, &mut tx, 0, 64, &mut warming));
        assert!(!warming); assert_eq!(made, 66);
        assert_eq!(audio.capacity(), capacity); assert_eq!(audio.as_ptr(), allocation);
        assert!(audio.is_empty());
        for _ in 0..62 { assert_eq!(rx.pop().unwrap(), [0.0; 2]); }
        assert_eq!(rx.pop().unwrap(), [0.5, -0.5]);
        assert_eq!(rx.pop().unwrap(), [0.25, -0.25]);
        // Once running, an empty DMA copy cannot synthesize padding to disguise
        // later missing audio, even if the host cursor has moved far ahead.
        assert!(publish_dma(&mut audio, &mut made, &mut tx, 1_000_000, 64, &mut warming));
        assert!(rx.pop().is_err()); assert_eq!(made, 66);
    }

    #[test]
    fn a_midstream_stall_recovers_at_exactly_one_x_without_reload_or_timeline_rewind() {
        for block_size in [64, 512] {
            let ratio = 44100.0 / 48000.0;
            let lead = lead_for(ratio, block_size as u32, Reserve::Safe).unwrap();
            let diagnostics = Diagnostics::default();
            diagnostics.set_loaded(true); diagnostics.stream_ready();
            let (mut tx, mut rx) = rtrb::RingBuffer::new(16384);
            let mut made = lead as u64;
            for _ in 0..lead { tx.push([0.25, -0.25]).unwrap(); }
            let mut playback = Playback::default(); let mut recovery = Recovery::default();
            let mut left = vec![0.0; block_size]; let mut right = left.clone();
            let mut producer_recovering = None;
            let mut ready_again = false; let mut audible_after = 0; let mut resumed_callbacks = 0;
            for block in 0..144000 / block_size {
                let host_frame = block * block_size;
                let n = (((block + 1) * block_size) as f64 * ratio).floor() as usize
                    - ((block * block_size) as f64 * ratio).floor() as usize;
                // One 50 ms worker scheduling stall, followed by exactly 1x
                // production. A fast computer cannot hide the stuck-debt case.
                if !(48000..50400).contains(&host_frame) {
                    let mut paused = false;
                    if let Some(generation) = diagnostics.pending_recovery() {
                        if producer_recovering != Some(generation) {
                            diagnostics.pause_recovery(generation);
                            if let Some(base) = diagnostics.recovery_resume(generation) {
                                made = base; producer_recovering = Some(generation);
                                assert!(prime_startup(&mut made, diagnostics.position(), lead, |f| tx.push(f).is_ok()));
                            } else { paused = true; }
                        }
                    }
                    if !paused {
                        for _ in 0..n { tx.push([0.25, -0.25]).unwrap(); made += 1; }
                        if producer_recovering.take().is_some() { diagnostics.stream_ready(); ready_again = true; }
                    }
                }
                let previous = playback.position;
                if recovery.before_block(&diagnostics, &mut playback, Some(&mut rx)) { resumed_callbacks += 1; }
                assert_eq!(previous, playback.position, "recovery rewound the MIDI/host frame origin");
                let missed = if recovery.holding() {
                    playback.render(&mut [&mut left, &mut right], ratio, || None)
                } else { playback.render(&mut [&mut left, &mut right], ratio, || rx.pop().ok()) };
                recovery.observe(&diagnostics, &playback, missed, lead);
                diagnostics.publish_position(playback.position);
                if host_frame >= 96000 {
                    audible_after += left.iter().filter(|&&v| v > 0.2).count();
                    assert_eq!(missed, 0, "the stream remained stuck after recovery");
                }
            }
            assert!(ready_again);
            assert_eq!(resumed_callbacks, 1);
            assert_eq!(diagnostics.recovery_count(), 1);
            assert!(!diagnostics.recovering());
            assert_eq!(playback.debt, 0);
            assert!(audible_after >= 47000);
            assert_eq!(made - playback.position, rx.slots() as u64);
            assert!((playback.position as f64 - 144000.0 * ratio).abs() < block_size as f64);
        }
    }

    #[test]
    fn recovery_does_not_flush_a_running_producer_and_can_be_cancelled_on_firmware_change() {
        let diagnostics = Diagnostics::default();
        diagnostics.set_loaded(true); diagnostics.stream_ready();
        let (mut tx, mut rx) = rtrb::RingBuffer::new(16);
        tx.push([0.75; 2]).unwrap();
        let mut playback = Playback { position: 12000, debt: 4096, ..Default::default() };
        let mut recovery = Recovery::default();
        recovery.observe(&diagnostics, &playback, 64, 1083);
        assert!(recovery.holding());
        assert!(!recovery.before_block(&diagnostics, &mut playback, Some(&mut rx)));
        assert_eq!(rx.slots(), 1, "consumer flushed before producer acknowledgement");
        diagnostics.reset_stream();
        assert!(!recovery.before_block(&diagnostics, &mut playback, Some(&mut rx)));
        assert!(!recovery.holding()); assert!(!diagnostics.recovering());
        assert_eq!(rx.pop().unwrap(), [0.75; 2]);
        assert_eq!(diagnostics.recovery_count(), 0);
    }

    #[test]
    #[ignore = "Opt-in wall-clock diagnostic: real X0X, 48 kHz, 512-sample callbacks"]
    fn realtime_x0x_48k_512() {
        realtime_firmware(512, "x0x", false);
    }

    #[test]
    #[ignore = "Opt-in wall-clock diagnostic: real X0X, user's 48 kHz / 64-sample configuration"]
    fn realtime_x0x_48k_64() {
        realtime_firmware(64, "x0x", false);
    }

    #[test]
    #[ignore = "Opt-in real DUB-1 at 48 kHz, both block sizes"]
    fn realtime_dub_48k_both() {
        for block in [512, 64] { realtime_firmware(block, "dub13", false); }
    }

    #[test]
    #[ignore = "Opt-in forced 100ms producer stall; must recover without reloading"]
    fn stalled_engine_recovers_dub_and_x0x_both_blocks() {
        for id in ["dub13", "x0x"] {
            for block in [512, 64] { realtime_firmware(block, id, true); }
        }
    }

    fn realtime_firmware(block_size: usize, firmware_id: &str, inject_stall: bool) {
        use crate::{engine::{self, Cmd, Shared, View}, Fm1Params};
        use std::{sync::{mpsc, Arc, Mutex, atomic::Ordering}, time::{Duration, Instant}};
        // Real audio hosts use audio-priority callbacks. The synthetic host
        // must not run below its priority/QoS emulation worker: otherwise the OS
        // can delay the consumer or throttle the worker behind a busy normal-
        // priority fake host. Give both test threads the same worker scheduling policy.
        // This is test-only, lasts for this test thread and keeps the strict
        // zero-missing-frame assertion; it does not change plugin processing.
        let _host_scheduling = crate::worker::Scheduling::audio();
        let directory = std::env::var_os("FM1_CATALOGUE_TEST_DIR").map(std::path::PathBuf::from)
            .unwrap_or_else(|| std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../firmware/catalogue"));
        let entry = crate::catalogue::ENTRIES.iter().find(|e| e.id == firmware_id).unwrap();
        let firmware = crate::catalogue::read_at(&directory, entry).unwrap();
        let shared = Arc::new(Shared { view: Mutex::new(View {
            lcd: vec![0; 240 * 240], leds: [0.0; 41], pads: [0; 16], board: fm1_soc::Board::Fm1,
            booted: false, halted: false, speed: 0.0, starved: 0, late: 0 }), quit: Default::default() });
        let params = Arc::new(Fm1Params::default());
        params.audio_diagnostics.configure(48000.0, block_size as u32);
        let (cmd, commands) = mpsc::channel();
        let (mut midi, midi_rx) = rtrb::RingBuffer::new(4096);
        let (audio_tx, mut audio) = rtrb::RingBuffer::new(16384);
        let worker_shared = shared.clone(); let worker_params = params.clone();
        let worker = std::thread::spawn(move || engine::run(worker_shared, worker_params, commands, midi_rx, audio_tx));
        let lead = std::env::var("FM1_REALTIME_LEAD").ok().and_then(|s| s.parse().ok())
            .unwrap_or_else(|| lead_for(44100.0 / 48000.0, block_size as u32, Reserve::Safe).unwrap());
        cmd.send(Cmd::Lead(lead)).unwrap();
        let (reply, answer) = mpsc::channel();
        cmd.send(Cmd::Load { firmware, sectors: Vec::new(), reply }).unwrap();
        let boot = answer.recv_timeout(Duration::from_secs(20));
        if !matches!(&boot, Ok(Ok(()))) {
            shared.quit.store(true, Ordering::Relaxed); worker.join().unwrap();
            panic!("{firmware_id} boot failed: {boot:?}");
        }
        let mut playback = Playback::default();
        let mut recovery = Recovery::default();
        let mut fault_missing = 0; let mut recovered_peak = 0.0f32;
        let mut left = vec![0.0; block_size]; let mut right = left.clone();
        let ratio = 44100.0 / 48000.0;
        let started = Instant::now(); let mut startup_missed = 0; let mut steady_missed = 0;
        let mut max_callback = Duration::ZERO; let mut max_host_late = Duration::ZERO; let mut peak = 0.0f32;
        let legacy = std::env::var_os("FM1_REALTIME_LEGACY").is_some();
        let host_deadline = HostDeadline::new();
        for block in 0..576000 / block_size {
            let host_frame = block * block_size;
            let contains = |frame| host_frame <= frame && frame < host_frame + block_size;
            let deadline = started + Duration::from_secs_f64(host_frame as f64 / 48000.0);
            // A real host calls from its audio driver, not a coalesced sleep
            // timer. Darwin CI can oversleep nanosleep by tens of milliseconds,
            // making this fake host burst many callbacks and starve any queue.
            // Yield until the deadline in this isolated, opt-in diagnostic only;
            // the shipped callback/worker do not busy-wait.
            host_deadline.wait(deadline);
            max_host_late = max_host_late.max(Instant::now().saturating_duration_since(deadline));
            // DUB starts its arrangement automatically; PLAY would stop it.
            if firmware_id == "x0x" && contains(188 * 512) { cmd.send(Cmd::Key { id: 12, down: true }).unwrap(); }
            if firmware_id == "x0x" && contains(198 * 512) { cmd.send(Cmd::Key { id: 12, down: false }).unwrap(); }
            if contains(282 * 512) || contains(750 * 512) {
                let down = contains(282 * 512);
                cmd.send(Cmd::Key { id: 26, down }).unwrap();
                for channel in 0..16 {
                    midi.push(engine::Midi { frame: playback.position + lead as u64,
                        bytes: [(if down { 0x90 } else { 0x80 }) | channel, 60, 100], len: 3 }).unwrap();
                }
            }
            if host_frame >= 282 * 512 && host_frame % (12 * 512) == 0 {
                // Default X0X GM inputs: 909 on channel 10, 808 on channel 11.
                // Keep exercising voices instead of timing a silent/idle synth.
                for (channel, note) in [(9, 36), (9, 42), (10, 42), (10, 38)] {
                    midi.push(engine::Midi { frame: playback.position + lead as u64,
                        bytes: [0x90 | channel, note, 100], len: 3 }).unwrap();
                }
            }
            if inject_stall && contains(240000) { cmd.send(Cmd::TestStall(Duration::from_millis(100))).unwrap(); }
            let call = Instant::now();
            recovery.before_block(&params.audio_diagnostics, &mut playback, Some(&mut audio));
            let missed = if recovery.holding() {
                playback.render(&mut [&mut left, &mut right], ratio, || None)
            } else if legacy {
                playback.render_with_budget(&mut [&mut left, &mut right], ratio, || audio.pop().ok(), 16384)
            } else { playback.render(&mut [&mut left, &mut right], ratio, || audio.pop().ok()) };
            recovery.observe(&params.audio_diagnostics, &playback, missed, lead);
            params.audio_diagnostics.publish_position(playback.position);
            max_callback = max_callback.max(call.elapsed());
            if host_frame < 188 * 512 { startup_missed += missed; }
            else if inject_stall && (240000..288000).contains(&host_frame) { fault_missing += missed; }
            else { steady_missed += missed; }
            if inject_stall && (288000..360000).contains(&host_frame) {
                for &sample in &left { recovered_peak = recovered_peak.max(sample.abs()); }
            }
            for &sample in left.iter().chain(&right) { assert!(sample.is_finite()); peak = peak.max(sample.abs()); }
        }
        shared.quit.store(true, Ordering::Relaxed); worker.join().unwrap();
        host_deadline.report();
        let view = shared.view.lock().unwrap();
        eprintln!("REALTIME {firmware_id} 48k/{block_size} lead={lead} legacy={legacy}: startup_missing={startup_missed}, steady_missing={steady_missed}, debt={}, position={}, callback_max_us={}, host_late_max_us={}, engine_speed={:.3}, peak={peak:.5}, halted={}",
            playback.debt, playback.position, max_callback.as_micros(), max_host_late.as_micros(), view.speed, view.halted);
        eprintln!("RECOVERY {firmware_id} 48k/{block_size}: injected={inject_stall}, fault_missing={fault_missing}, recoveries={}, post_fault_peak={recovered_peak:.5}", params.audio_diagnostics.recovery_count());
        assert!(view.booted && !view.halted && peak > 0.0001);
        if inject_stall {
            assert!(fault_missing > 0, "fault did not exercise an underrun");
            assert_eq!(params.audio_diagnostics.recovery_count(), 1, "must complete exactly one recovery");
            assert!(recovered_peak > 0.0001, "audio never returned after stall");
            assert!(!params.audio_diagnostics.recovering());
            assert_eq!(playback.debt, 0);
        } else { assert_eq!(params.audio_diagnostics.recovery_count(), 0, "normal playback needed recovery"); }
        assert_eq!(steady_missed, 0, "steady playback missed frames; inspect diagnostic timing/load");
    }
}
