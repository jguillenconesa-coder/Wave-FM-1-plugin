// MIT. DAW clock additions; MIDI forwarding derives from fm1-emu by Joakim Eriksson.
use nih_plug::prelude::*;
use std::sync::atomic::{AtomicU32, AtomicU8, Ordering};

#[derive(Enum, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Source {
    #[name = "Internal / firmware"]
    Internal,
    #[name = "Ableton / DAW"]
    Daw,
}

/// Copied from the host at the beginning of an audio block. No wall-clock timers.
#[derive(Clone, Copy)]
pub struct Block {
    pub playing: bool,
    pub tempo: Option<f64>,
    pub beats: Option<f64>,
    pub sample_rate: f64,
    pub samples: u32,
}

impl Block {
    pub fn from_host(host: &Transport, samples: u32) -> Self {
        Self { playing: host.playing, tempo: host.tempo, beats: host.pos_beats(),
            sample_rate: host.sample_rate as f64, samples }
    }
    fn valid_tempo(self) -> Option<f64> {
        self.tempo.filter(|bpm| bpm.is_finite() && (1.0..=1000.0).contains(bpm))
            .filter(|_| self.sample_rate.is_finite() && (8000.0..=768000.0).contains(&self.sample_rate))
    }
    pub fn status(self, source: Source) -> u8 {
        if source == Source::Internal { INTERNAL }
        else if self.valid_tempo().is_none() { UNAVAILABLE }
        else if self.playing { FOLLOWING }
        else { STOPPED }
    }
}

pub const INTERNAL: u8 = 0;
pub const STOPPED: u8 = 1;
pub const FOLLOWING: u8 = 2;
pub const UNAVAILABLE: u8 = 3;
pub const QUEUE_FULL: u8 = 4;
pub const WAITING_FIRMWARE: u8 = 5;

/// Audio-thread publication, read by the editor without locks or allocation.
#[derive(Default)]
pub struct Feedback { state: AtomicU8, bpm: AtomicU32 }

impl Feedback {
    pub fn publish(&self, state: u8, tempo: Option<f64>) {
        self.bpm.store(tempo.filter(|v| v.is_finite()).map_or(0, |v| (v * 100.0).round() as u32), Ordering::Relaxed);
        self.state.store(state, Ordering::Relaxed);
    }
    pub fn label(&self, source: Source) -> String {
        if source == Source::Internal { return "Firmware clock".into(); }
        match self.state.load(Ordering::Relaxed) {
            FOLLOWING => format!("DAW · {:.2} BPM · playing", self.bpm.load(Ordering::Relaxed) as f64 / 100.0),
            STOPPED => "DAW · stopped".into(),
            WAITING_FIRMWARE => "DAW · waiting for firmware audio".into(),
            QUEUE_FULL => "MIDI queue full · resync pending".into(),
            _ => "DAW tempo unavailable · waiting for host".into(),
        }
    }
}

#[derive(Default)]
pub struct Clock {
    running: bool,
    expected_beat: Option<f64>,
    last_tick: Option<f64>,
    delivery_fault: bool,
    last_source: Option<Source>,
    firmware_generation: Option<u64>,
    suspended: bool,
}

impl Clock {
    /// A newly booted guest must receive transport state even if Live never
    /// stopped. References are per instance; host PPQ remains authoritative.
    pub fn observe_firmware(&mut self, generation: u64) {
        if self.firmware_generation != Some(generation) {
            self.reset();
            self.last_source = None;
            self.firmware_generation = Some(generation);
        }
    }

    pub fn set_firmware_ready(&mut self, ready: bool) {
        if !ready && !self.suspended {
            self.delivery_fault |= self.running;
            self.expected_beat = None;
            self.last_tick = None;
        }
        self.suspended = !ready;
    }

    pub fn reset(&mut self) {
        self.delivery_fault |= self.running;
        self.running = false;
        self.expected_beat = None;
        self.last_tick = None;
    }

    pub fn delivery_failed(&mut self) { self.delivery_fault = true; }

    /// Iterator work is bounded by block length at the accepted tempo/rate range.
    /// Start/Stop and 24-PPQN Clock bytes share the timestamped note-event queue.
    pub fn events(&mut self, source: Source, block: Block) -> Events<'_> {
        let tempo = block.valid_tempo();
        let active = source == Source::Daw && block.playing && tempo.is_some();
        let mut prefix = [0; 2];
        let mut prefix_len = 0;
        let mut start_tick = 0.0;
        let mut step = 0.0;
        let mut next_tick = 0.0;
        if block.samples > 0 && !(source == Source::Daw && self.suspended) {
            let entering_daw = source == Source::Daw && self.last_source != Some(Source::Daw);
            self.last_source = Some(source);
            if active {
                let per_sample = tempo.unwrap() / (60.0 * block.sample_rate);
                let beat = block.beats.filter(|v| v.is_finite() && v.abs() < 1.0e9)
                    .unwrap_or(self.expected_beat.unwrap_or(0.0));
                let seek = self.expected_beat.is_some_and(|expected|
                    (beat - expected).abs() > (per_sample * 2.0).max(1.0e-6));
                if !self.running || seek || self.delivery_fault {
                    // Start resets the sequence by itself. A Stop+Start pair
                    // in one firmware poll makes X0X clear its pending Start.
                    prefix[prefix_len] = 0xfa; prefix_len += 1;
                    self.last_tick = None;
                }
                self.running = true;
                self.expected_beat = Some(beat + block.samples as f64 * per_sample);
                start_tick = beat * 24.0;
                step = per_sample * 24.0;
                next_tick = (start_tick - 1.0e-7).ceil();
                if let Some(last) = self.last_tick { next_tick = next_tick.max(last + 1.0); }
            } else {
                if self.running || self.delivery_fault || entering_daw { prefix[0] = 0xfc; prefix_len = 1; }
                self.running = false;
                self.expected_beat = None;
                self.last_tick = None;
            }
            self.delivery_fault = false;
        }
        Events { clock: self, prefix, prefix_len, prefix_at: 0, start_tick, step,
            next_tick, samples: block.samples }
    }
}

pub struct Events<'a> {
    clock: &'a mut Clock,
    prefix: [u8; 2],
    prefix_len: usize,
    prefix_at: usize,
    start_tick: f64,
    next_tick: f64,
    step: f64,
    samples: u32,
}

impl Iterator for Events<'_> {
    type Item = (u32, u8);
    fn next(&mut self) -> Option<Self::Item> {
        if self.prefix_at < self.prefix_len {
            let byte = self.prefix[self.prefix_at]; self.prefix_at += 1;
            return Some((0, byte));
        }
        if self.step == 0.0 || self.next_tick >= self.start_tick + self.samples as f64 * self.step - 1.0e-7 {
            return None;
        }
        let at = ((self.next_tick - self.start_tick) / self.step).max(0.0).floor() as u32;
        self.clock.last_tick = Some(self.next_tick);
        self.next_tick += 1.0;
        Some((at.min(self.samples - 1), 0xf8))
    }
}

/// Merge the host's notes and generated clock in time order. A FIFO fed with
/// all clocks before all notes would delay early notes until the final clock.
pub fn send_block(
    clock: &mut Clock, source: Source, block: Block, ctx: &mut impl ProcessContext<crate::Fm1>,
    midi: Option<&mut rtrb::Producer<crate::engine::Midi>>, start: u64, ratio: f64, lead: usize, channel: u8,
) -> bool {
    send_events(clock, source, block, || ctx.next_event(), midi, start, ratio, lead, channel)
}

fn send_events(
    clock: &mut Clock, source: Source, block: Block,
    mut next_event: impl FnMut() -> Option<PluginNoteEvent<crate::Fm1>>,
    midi: Option<&mut rtrb::Producer<crate::engine::Midi>>, start: u64, ratio: f64, lead: usize, channel: u8,
) -> bool {
    let needs_transport_recovery = source == Source::Daw || clock.running || clock.delivery_fault;
    let mut clocks = clock.events(source, block);
    let mut tick = clocks.next();
    let mut event = next_event();
    let mut midi = midi;
    let mut success = true;
    while tick.is_some() || event.is_some() {
        let (at, bytes, len) = if tick.is_some_and(|(at, _)| event.as_ref().is_none_or(|ev| at <= ev.timing())) {
            let (at, byte) = tick.take().unwrap();
            tick = clocks.next();
            (at, [byte, 0, 0], 1)
        } else {
            let ev = event.take().unwrap();
            event = next_event();
            let Some(nih_plug::midi::MidiResult::Basic(mut bytes)) = ev.as_midi() else { continue; };
            let len = crate::midi_len(bytes[0]);
            if len == 0 || ev.timing() >= block.samples { continue; }
            if channel != 0 { bytes[0] = (bytes[0] & 0xf0) | (channel - 1); }
            (ev.timing(), bytes, len)
        };
        let frame = start + (at as f64 * ratio) as u64 + lead as u64;
        if let Some(queue) = midi.as_mut() {
            success &= queue.push(crate::engine::Midi { frame, bytes, len }).is_ok();
        } else { success = false; }
    }
    drop(clocks);
    if !success && needs_transport_recovery { clock.delivery_failed(); }
    success
}

#[cfg(test)]
mod tests {
    use super::*;
    fn block(playing: bool, tempo: f64, beats: Option<f64>, samples: u32) -> Block {
        Block { playing, tempo: Some(tempo), beats, sample_rate: 48000.0, samples }
    }

    #[test]
    fn entering_daw_stopped_sends_stop_once_for_auto_starting_firmwares() {
        let mut clock = Clock::default();
        let stopped = block(false, 120.0, Some(0.0), 512);
        clock.observe_firmware(1);
        assert_eq!(clock.events(Source::Daw, stopped).collect::<Vec<_>>(), [(0, 0xfc)]);
        assert!(clock.events(Source::Daw, stopped).next().is_none());
        assert!(clock.events(Source::Internal, stopped).next().is_none());
        assert_eq!(clock.events(Source::Daw, stopped).collect::<Vec<_>>(), [(0, 0xfc)]);
        clock.observe_firmware(2);
        assert_eq!(clock.events(Source::Daw, stopped).collect::<Vec<_>>(), [(0, 0xfc)]);
    }

    #[test]
    fn loading_a_guest_with_transport_running_reissues_start_without_repeating_it_each_block() {
        let mut clock = Clock::default();
        clock.observe_firmware(1);
        let begin = block(true, 120.0, Some(8.0), 24000);
        assert_eq!(clock.events(Source::Daw, begin).next(), Some((0, 0xfa)));
        // Consume a complete block, as the real callback does.
        clock.events(Source::Daw, block(true, 120.0, Some(9.0), 24000)).for_each(drop);
        clock.observe_firmware(2);
        let restarted: Vec<_> = clock.events(Source::Daw, block(true, 120.0, Some(10.0), 24000)).collect();
        assert_eq!(&restarted[..2], &[(0, 0xfa), (0, 0xf8)]);
        clock.observe_firmware(2);
        let next: Vec<_> = clock.events(Source::Daw, block(true, 120.0, Some(11.0), 24000)).collect();
        assert_eq!(next.len(), 24); assert!(next.iter().all(|(_, b)| *b == 0xf8));
        let mut internal = Clock::default();
        internal.observe_firmware(1);
        assert!(internal.events(Source::Internal, begin).next().is_none());
        internal.observe_firmware(2);
        assert!(internal.events(Source::Internal, begin).next().is_none());
    }

    #[test]
    fn cold_boot_and_rebuffer_wait_without_losing_notes_or_issuing_adjacent_stop_start() {
        let mut clock = Clock::default(); clock.observe_firmware(1);
        clock.set_firmware_ready(false);
        let b = block(true, 120.0, Some(0.0), 24000);
        assert!(clock.events(Source::Daw, b).next().is_none());
        clock.set_firmware_ready(true);
        assert_eq!(clock.events(Source::Daw, b).collect::<Vec<_>>()[..2], [(0, 0xfa), (0, 0xf8)]);
        clock.set_firmware_ready(false);
        let (mut tx, mut rx) = rtrb::RingBuffer::new(8);
        let mut note = std::iter::once(NoteEvent::NoteOn { timing: 100, voice_id: None,
            channel: 0, note: 60, velocity: 1.0 });
        assert!(send_events(&mut clock, Source::Daw, block(true, 120.0, Some(1.0), 24000),
            || note.next(), Some(&mut tx), 44100, 44100.0/48000.0, 1495, 0));
        assert_eq!(rx.pop().unwrap().bytes[0], 0x90); assert!(rx.pop().is_err());
        clock.set_firmware_ready(true);
        let resumed: Vec<_> = clock.events(Source::Daw, block(true, 120.0, Some(2.0), 24000)).collect();
        assert_eq!(&resumed[..2], &[(0, 0xfa), (0, 0xf8)]);
        assert!(!resumed.iter().any(|(_, byte)| *byte == 0xfc));
    }

    #[test]
    fn clock_has_24_pulses_per_quarter_independent_of_block_size_or_sample_rate() {
        for rate in [44100.0, 48000.0, 96000.0] {
            for size in [1, 127, 512, 4096] {
                let mut clock = Clock::default();
                let mut at = 0u32; let mut pulses = Vec::new(); let mut starts = 0;
                let total = rate as u32 * 2;
                while at < total {
                    let n = size.min(total - at);
                    let b = Block { sample_rate: rate, ..block(true, 120.0, Some(at as f64 / rate * 2.0), n) };
                    for (offset, byte) in clock.events(Source::Daw, b) {
                        if byte == 0xfa { starts += 1; }
                        if byte == 0xf8 { pulses.push(at + offset); }
                    }
                    at += n;
                }
                assert_eq!(starts, 1);
                assert_eq!(pulses.len(), 96);
                for (i, &sample) in pulses.iter().enumerate() {
                    assert!((sample as f64 - i as f64 * rate / 48.0).abs() <= 1.01);
                }
            }
        }
    }

    #[test]
    fn transport_start_stop_restart_and_mode_changes_are_explicit() {
        let mut clock = Clock::default();
        assert!(clock.events(Source::Internal, block(true, 120.0, Some(0.0), 1000)).next().is_none());
        assert_eq!(clock.events(Source::Daw, block(false, 120.0, Some(0.0), 1000)).collect::<Vec<_>>(), [(0, 0xfc)]);
        let a: Vec<_> = clock.events(Source::Daw, block(true, 120.0, Some(0.0), 1000)).collect();
        assert_eq!(a, [(0, 0xfa), (0, 0xf8)]);
        assert_eq!(clock.events(Source::Daw, block(false, 120.0, None, 1000)).collect::<Vec<_>>(), [(0, 0xfc)]);
        assert!(clock.events(Source::Daw, block(false, 120.0, None, 1000)).next().is_none());
        assert_eq!(clock.events(Source::Daw, block(true, 120.0, Some(4.0), 1000)).collect::<Vec<_>>(), [(0, 0xfa), (0, 0xf8)]);
        assert_eq!(clock.events(Source::Internal, block(true, 120.0, None, 1000)).collect::<Vec<_>>(), [(0, 0xfc)]);
    }

    #[test]
    fn tempo_changes_loop_seeks_missing_position_and_instances_keep_separate_phase() {
        let mut clock = Clock::default(); let mut other = Clock::default();
        let first: Vec<_> = clock.events(Source::Daw, block(true, 120.0, Some(0.0), 24000)).collect();
        assert_eq!(first.iter().filter(|(_, b)| *b == 0xf8).count(), 24);
        let faster: Vec<_> = clock.events(Source::Daw, block(true, 240.0, Some(1.0), 24000)).collect();
        assert_eq!(faster.len(), 48); assert!(faster.iter().all(|(_, b)| *b == 0xf8));
        let looped: Vec<_> = clock.events(Source::Daw, block(true, 120.0, Some(0.0), 1000)).collect();
        assert_eq!(looped, [(0, 0xfa), (0, 0xf8)]);
        assert!(other.events(Source::Internal, block(true, 120.0, None, 1000)).next().is_none());
        let mut no_position = Clock::default();
        let a: Vec<_> = no_position.events(Source::Daw, block(true, 120.0, None, 24000)).collect();
        let b: Vec<_> = no_position.events(Source::Daw, block(true, 120.0, None, 24000)).collect();
        assert_eq!(a.len(), 25); assert_eq!(b.len(), 24);
    }

    #[test]
    fn a_long_fractional_tempo_does_not_drift_with_irregular_host_buffers() {
        let mut clock = Clock::default();
        let tempo = 123.456;
        let rate = 44100.0;
        let total = 60 * 44100u32;
        let mut offset = 0; let mut pulses = 0;
        for size in [127u32, 512, 17, 2048, 255, 4096].into_iter().cycle() {
            if offset == total { break; }
            let count = size.min(total - offset);
            let b = Block { sample_rate: rate,
                ..block(true, tempo, Some(offset as f64 * tempo / (60.0 * rate)), count) };
            for (at, byte) in clock.events(Source::Daw, b) {
                if byte == 0xf8 {
                    let ideal = pulses as f64 * 60.0 * rate / (tempo * 24.0);
                    assert!(((offset + at) as f64 - ideal).abs() <= 1.01);
                    pulses += 1;
                }
            }
            offset += count;
        }
        assert_eq!(pulses, (tempo * 24.0).ceil() as usize);
    }

    #[test]
    fn unavailable_host_data_and_queue_failure_stop_or_resynchronize_without_bursts() {
        let mut clock = Clock::default();
        clock.events(Source::Daw, block(true, 120.0, Some(0.0), 1000)).for_each(drop);
        for tempo in [f64::NAN, f64::INFINITY, 0.0, -120.0, 1.0e9] {
            let events: Vec<_> = clock.events(Source::Daw, block(true, tempo, None, 512)).collect();
            assert!(events.is_empty() || events == [(0, 0xfc)]);
        }
        clock.delivery_failed();
        assert_eq!(clock.events(Source::Daw, block(true, 120.0, Some(4.0), 1000)).collect::<Vec<_>>(),
            [(0, 0xfa), (0, 0xf8)]);
        clock.reset();
        assert_eq!(clock.events(Source::Internal, block(false, 120.0, None, 512)).collect::<Vec<_>>(), [(0, 0xfc)]);
        assert!(clock.events(Source::Daw, block(true, 120.0, None, 0)).next().is_none());
    }

    #[test]
    fn notes_and_clock_share_ordered_frame_stamps_and_clock_bytes_are_never_channel_remapped() {
        let mut clock = Clock::default();
        let (mut tx, mut rx) = rtrb::RingBuffer::new(32);
        let mut notes = [
            NoteEvent::NoteOn { timing: 100, voice_id: None, channel: 5, note: 60, velocity: 0.8 },
            NoteEvent::NoteOff { timing: 1100, voice_id: None, channel: 5, note: 60, velocity: 0.0 },
        ].into_iter();
        let ratio = 44100.0 / 48000.0;
        assert!(send_events(&mut clock, Source::Daw, block(true, 120.0, Some(0.0), 2400),
            || notes.next(), Some(&mut tx), 100, ratio, 2048, 3));
        let mut received = Vec::new();
        while let Ok(event) = rx.pop() { received.push(event); }
        assert_eq!(received.iter().map(|e| e.bytes[0]).collect::<Vec<_>>(), [0xfa, 0xf8, 0x92, 0xf8, 0x82, 0xf8]);
        assert!(received.windows(2).all(|p| p[0].frame <= p[1].frame));
        assert_eq!(received[2].frame, 100 + (100.0 * ratio) as u64 + 2048);
        assert_eq!(received[4].frame, 100 + (1100.0 * ratio) as u64 + 2048);
        assert_eq!(received[2].bytes[1], 60);
        assert_eq!(received[0].len, 1);

        let (mut tiny, mut drain) = rtrb::RingBuffer::new(1);
        let mut failed = Clock::default();
        assert!(!send_events(&mut failed, Source::Daw, block(true, 120.0, Some(0.0), 1000),
            || None, Some(&mut tiny), 0, 1.0, 0, 0));
        while drain.pop().is_ok() {}
        assert!(send_events(&mut failed, Source::Internal, block(false, 120.0, None, 1000),
            || None, Some(&mut tiny), 1000, 1.0, 0, 0));
        assert_eq!(drain.pop().unwrap().bytes[0], 0xfc);
        // Overloaded note traffic in internal mode must not stop its sequencer.
        let mut internal = Clock::default();
        tiny.push(crate::engine::Midi::default()).unwrap();
        let mut note = std::iter::once(NoteEvent::NoteOn { timing: 0, voice_id: None,
            channel: 0, note: 60, velocity: 1.0 });
        assert!(!send_events(&mut internal, Source::Internal, block(true, 120.0, None, 1000),
            || note.next(), Some(&mut tiny), 0, 1.0, 0, 0));
        assert!(!internal.delivery_fault);
        assert!(internal.events(Source::Internal, block(true, 120.0, None, 1000)).next().is_none());
    }
}
