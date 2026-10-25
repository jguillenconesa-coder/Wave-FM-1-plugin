// MIT. Host control integration; see desktop/README.md and AUTHORS.md.
//! Physical control names are stable across firmwares. They are not synth parameter names.
use nih_plug::prelude::*;
use std::sync::{Arc, atomic::{AtomicI32, AtomicU64, Ordering}};
use std::time::{Duration, Instant};

// Same visual order and hardware encoder numbers as panel::ENCS.
pub const ENCODERS: [(&str, u8); 7] = [
    ("Select", 0), ("Presets", 6), ("Algorithm", 1),
    ("Knob 1", 2), ("Knob 2", 3), ("Knob 3", 4), ("Knob 4", 5),
];
pub const BUTTONS: [&str; 14] = [
    "OCT -", "OCT +", "FX", "SCL", "ENV", "LFO", "EDIT", "GLO",
    "HOME", "SAVE", "ARP", "SEQ", "PLAY", "REC",
];

/// Editor-only feedback. Observing host values must never send another encoder
/// event, including when a window reopens or a Set restores its references.
#[derive(Default)]
pub struct EncoderDisplay {
    positions: Option<[i32; 7]>,
    pub angles: [f32; 7],
}

impl EncoderDisplay {
    pub fn sync(&mut self, positions: [i32; 7]) -> [bool; 7] {
        let changed = std::array::from_fn(|i| self.positions.is_some_and(|last| last[i] != positions[i]));
        for i in 0..7 {
            if self.positions.is_none() || changed[i] {
                self.angles[i] = (-135.0 + 270.0 * positions[i].clamp(0, 127) as f32 / 127.0).to_radians();
            }
        }
        self.positions = Some(positions);
        changed
    }

    /// Explicit relative/navigation gesture (Alt). Ordinary mouse turns use
    /// turn_bounded and the same saved parameter as host/controller changes.
    pub fn turn(&mut self, i: usize, clicks: i32) {
        self.angles[i] = (self.angles[i] + (clicks as f32 * 18.0).to_radians())
            .rem_euclid(std::f32::consts::TAU);
    }

    pub fn turn_bounded(&mut self, i: usize, clicks: i32) -> (i32, i32) {
        let positions = self.positions.get_or_insert([64; 7]);
        let previous = positions[i];
        let target = previous.saturating_add(clicks).clamp(0, 127);
        positions[i] = target;
        self.angles[i] = (-135.0 + 270.0 * target as f32 / 127.0).to_radians();
        (target, target - previous)
    }
}

#[derive(Params)]
pub struct EncoderParam {
    #[id = "enc"]
    pub value: IntParam,
}

#[derive(Params)]
pub struct ButtonParam {
    #[id = "btn"]
    pub value: BoolParam,
    rises: Arc<AtomicU64>,
}

#[derive(Params)]
pub struct PanelParams {
    // Never reorder these arrays: enc_1..7 / btn_1..14 are saved host IDs.
    #[nested(array)]
    pub encoders: [EncoderParam; 7],
    #[nested(array)]
    pub buttons: [ButtonParam; 14],
    epoch: AtomicU64,
    sequence: AtomicU64,
    published_positions: [AtomicI32; 7],
    published_rises: [AtomicU64; 14],
    published_epoch: AtomicU64,
}

impl Default for PanelParams {
    fn default() -> Self {
        Self {
            encoders: std::array::from_fn(|i| EncoderParam {
                value: IntParam::new(ENCODERS[i].0, 64, IntRange::Linear { min: 0, max: 127 }),
            }),
            buttons: std::array::from_fn(|i| {
                let rises = Arc::new(AtomicU64::new(0));
                let counter = rises.clone();
                ButtonParam {
                    value: BoolParam::new(format!("{} button", BUTTONS[i]), false)
                        .with_callback(Arc::new(move |down| {
                            if down { counter.fetch_add(1, Ordering::Relaxed); }
                        })),
                    rises,
                }
            }),
            epoch: AtomicU64::new(0),
            sequence: AtomicU64::new(0),
            published_positions: std::array::from_fn(|_| AtomicI32::new(64)),
            published_rises: std::array::from_fn(|_| AtomicU64::new(0)),
            published_epoch: AtomicU64::new(0),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Snapshot {
    positions: [i32; 7],
    rises: [u64; 14],
    epoch: u64,
}

impl PanelParams {
    /// Only initialize()/process() publish. NIH-plug serializes those calls with
    /// state restoration. Never read partially restored host parameters directly.
    pub fn publish(&self) {
        self.sequence.fetch_add(1, Ordering::SeqCst); // odd: publication in progress
        for i in 0..7 { self.published_positions[i].store(self.encoders[i].value.value(), Ordering::SeqCst); }
        for i in 0..14 { self.published_rises[i].store(self.buttons[i].rises.load(Ordering::Relaxed), Ordering::SeqCst); }
        self.published_epoch.store(self.epoch.load(Ordering::Relaxed), Ordering::SeqCst);
        self.sequence.fetch_add(1, Ordering::SeqCst);
    }

    /// One bounded read attempt; no audio-thread locks, spinning or allocation.
    pub fn snapshot(&self) -> Option<Snapshot> {
        let sequence = self.sequence.load(Ordering::SeqCst);
        if sequence & 1 != 0 { return None; }
        let snapshot = Snapshot {
            positions: std::array::from_fn(|i| self.published_positions[i].load(Ordering::SeqCst)),
            rises: std::array::from_fn(|i| self.published_rises[i].load(Ordering::SeqCst)),
            epoch: self.published_epoch.load(Ordering::SeqCst),
        };
        (self.sequence.load(Ordering::SeqCst) == sequence).then_some(snapshot)
    }

    /// Restoring positions establishes a reference; it must not turn all encoders on boot.
    pub fn resync(&self) {
        self.epoch.fetch_add(1, Ordering::Relaxed);
        self.publish();
    }
}

impl Default for Snapshot {
    // A read interrupted by publication must establish a baseline on its next
    // coherent read, including a firmware boot while the host is processing.
    fn default() -> Self { Self { positions: [64; 7], rises: [0; 14], epoch: u64::MAX } }
}

#[derive(Debug, PartialEq)]
pub enum Event { Encoder { enc: u8, clicks: i32 }, Key { id: u8, down: bool } }

/// Runs exclusively on the machine thread. Parameter callbacks only increment atomics,
/// so fast host button presses are retained without allocating on the audio thread.
pub struct Routing {
    last: Snapshot,
    until: [Option<Instant>; 14],
    manual: [bool; 64],
}

impl Routing {
    pub fn new(snapshot: Snapshot) -> Self {
        Self { last: snapshot, until: [None; 14], manual: [false; 64] }
    }

    pub fn reset(&mut self, snapshot: Snapshot, mut emit: impl FnMut(Event)) {
        for id in 0..14 {
            if self.until[id].is_some() && !self.manual[id] {
                emit(Event::Key { id: id as u8, down: false });
            }
        }
        self.until = [None; 14];
        self.last = snapshot;
    }

    pub fn manual_key(&mut self, id: u8, down: bool, mut emit: impl FnMut(Event)) {
        let i = id as usize;
        if i >= self.manual.len() || self.manual[i] == down { return; }
        self.manual[i] = down;
        // A host pulse and a mouse hold share a key; either source can keep it pressed.
        if i >= 14 || self.until[i].is_none() { emit(Event::Key { id, down }); }
    }

    pub fn poll(&mut self, snapshot: Snapshot, now: Instant, mut emit: impl FnMut(Event)) {
        if snapshot.epoch != self.last.epoch { self.reset(snapshot, emit); return; }
        for i in 0..7 {
            let clicks = snapshot.positions[i] - self.last.positions[i];
            if clicks != 0 { emit(Event::Encoder { enc: ENCODERS[i].1, clicks }); }
        }
        for i in 0..14 {
            if snapshot.rises[i] != self.last.rises[i] {
                if self.until[i].is_none() && !self.manual[i] {
                    emit(Event::Key { id: i as u8, down: true });
                }
                // A finite pulse cannot leave SAVE/REC held if the editor closes or a
                // controller sends only 127. A new trigger extends an active pulse.
                self.until[i] = Some(now + Duration::from_millis(100));
            }
            if self.until[i].is_some_and(|until| now >= until) {
                self.until[i] = None;
                if !self.manual[i] { emit(Event::Key { id: i as u8, down: false }); }
            }
        }
        self.last = snapshot;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn initial() -> Snapshot { PanelParams::default().snapshot().unwrap() }

    #[test]
    fn mouse_limits_do_not_wrap_or_accumulate_overshoot_and_share_saved_references() {
        let mut display = EncoderDisplay::default();
        display.sync([64; 7]);
        for i in 0..7 {
            assert_eq!(display.turn_bounded(i, i32::MAX), (127, 63));
            assert_eq!(display.turn_bounded(i, 1000), (127, 0));
            assert!((display.angles[i].to_degrees() - 135.0).abs() < 0.001);
            assert_eq!(display.turn_bounded(i, -1), (126, -1));
            assert_eq!(display.turn_bounded(i, i32::MIN), (0, -126));
            assert_eq!(display.turn_bounded(i, -1000), (0, 0));
            assert!((display.angles[i].to_degrees() + 135.0).abs() < 0.001);
            assert_eq!(display.turn_bounded(i, 1), (1, 1));
        }
        // These are the host parameter values written by the mouse setter;
        // reopening reads those same saved values and emits no engine event.
        let mut reopened = EncoderDisplay::default();
        reopened.sync([1; 7]);
        assert_eq!(reopened.angles, display.angles);
        display.sync([99; 7]); // controller takes over from the current reference
        assert_eq!(display.turn_bounded(3, 1), (100, 1));
    }

    #[test]
    fn host_knobs_repaint_without_mouse_events_and_reopen_at_the_saved_reference() {
        let mut display = EncoderDisplay::default();
        let mut other = EncoderDisplay::default();
        assert_eq!(display.sync([64; 7]), [false; 7]);
        other.sync([64; 7]);
        let mut positions = [64; 7];
        positions[3..].copy_from_slice(&[0, 32, 96, 127]);
        assert_eq!(display.sync(positions), [false, false, false, true, true, true, true]);
        assert!((display.angles[3].to_degrees() + 135.0).abs() < 0.001);
        assert!((display.angles[6].to_degrees() - 135.0).abs() < 0.001);
        assert!(display.angles[3..].windows(2).all(|pair| pair[0] < pair[1]));
        assert_eq!(other.sync([64; 7]), [false; 7]);
        assert_ne!(other.angles[3], display.angles[3]);

        // A repaint does not undo an endless mouse turn. A new host position
        // does reanchor it, and a new editor immediately shows restored values.
        display.turn(3, 40);
        let manual = display.angles[3];
        assert_eq!(display.sync(positions), [false; 7]);
        assert_eq!(display.angles[3], manual);
        positions[3] = 127;
        display.sync(positions);
        let mut reopened = EncoderDisplay::default();
        assert_eq!(reopened.sync(positions), [false; 7]);
        assert_eq!(reopened.angles, display.angles);
        positions[..3].copy_from_slice(&[0, 127, 32]);
        assert_eq!(display.sync(positions), [true, true, true, false, false, false, false]);
    }

    #[test]
    fn rendered_knob_indicators_follow_host_values_without_replaying_engine_controls() {
        let mut params = PanelParams::default();
        let mut display = EncoderDisplay::default();
        let mut routing = Routing::new(params.snapshot().unwrap());
        let lcd = vec![0; 240 * 240];
        let st = crate::panel::State { lcd: &lcd, enc_clicks: &[0; 7], pot: 512,
            down: &[false; 41], leds: &[0.0; 41], turning: &[0; 8] };
        let base = crate::panel::base();
        let mut image = base.clone();
        for position in [0, 127, 64] {
            for i in 0..7 {
                params.encoders[i].value = IntParam::new(ENCODERS[i].0, position,
                    IntRange::Linear { min: 0, max: 127 });
            }
            params.resync(); // restoring references is visual feedback, not a turn
            display.sync(std::array::from_fn(|i| params.encoders[i].value.value()));
            crate::panel::draw_with_encoder_angles(&mut image, &base, &st,
                &crate::panel::GRAPHITE, &display.angles);
            for (_, _, x, y, radius) in crate::panel::ENCS {
                // Pointer halfway along its radius: low = lower left,
                // high = lower right, centre = upwards.
                let angle = match position { 0 => -135.0_f32, 127 => 135.0, _ => 0.0 }.to_radians();
                let px = (x + angle.sin() * radius * 0.4).round() as usize;
                let py = (y - angle.cos() * radius * 0.4).round() as usize;
                assert_eq!(image[py * crate::panel::W + px], 0xeeeae2,
                    "host reference {position} did not move the rendered indicator at {x},{y}");
            }
            routing.poll(params.snapshot().unwrap(), Instant::now(),
                |_| panic!("visual synchronization replayed a saved control"));
        }
    }

    #[test]
    fn partially_restored_host_state_is_not_exposed_before_initialize() {
        let mut params = PanelParams::default();
        let mut routing = Routing::new(params.snapshot().unwrap());
        params.encoders[0].value = IntParam::new("Select", 100, IntRange::Linear { min: 0, max: 127 });
        params.buttons[9].rises.fetch_add(1, Ordering::Relaxed);
        routing.poll(params.snapshot().unwrap(), Instant::now(), |_| panic!("partially restored state replayed a control"));
        params.resync();
        routing.poll(params.snapshot().unwrap(), Instant::now(), |_| panic!("saved values must not trigger controls"));
        params.encoders[0].value = IntParam::new("Select", 101, IntRange::Linear { min: 0, max: 127 });
        params.buttons[2].rises.fetch_add(1, Ordering::Relaxed);
        params.publish();
        let mut events = Vec::new();
        routing.poll(params.snapshot().unwrap(), Instant::now(), |e| events.push(e));
        assert_eq!(events, [Event::Encoder { enc: 0, clicks: 1 }, Event::Key { id: 2, down: true }]);
    }

    #[test]
    fn an_in_progress_publication_is_skipped_without_spinning() {
        let params = PanelParams::default();
        params.sequence.store(1, Ordering::SeqCst);
        assert!(params.snapshot().is_none());
        params.sequence.store(2, Ordering::SeqCst);
        assert!(params.snapshot().is_some());
        let mut routing = Routing::new(Snapshot::default());
        params.published_positions[0].store(100, Ordering::SeqCst);
        routing.poll(params.snapshot().unwrap(), Instant::now(), |_| panic!("an interrupted boot replayed the encoder reference"));
    }

    #[test]
    fn all_encoder_routes_are_relative_and_use_the_physical_matrix() {
        let first = initial();
        let mut routing = Routing::new(first);
        let mut next = first;
        next.positions = [63, 66, 61, 68, 59, 70, 57];
        let mut events = Vec::new();
        routing.poll(next, Instant::now(), |e| events.push(e));
        assert_eq!(events, ENCODERS.iter().enumerate().map(|(i, &(_, enc))|
            Event::Encoder { enc, clicks: next.positions[i] - 64 }).collect::<Vec<_>>());
        events.clear();
        routing.poll(next, Instant::now(), |e| events.push(e));
        assert!(events.is_empty(), "unchanged values must not keep turning");
    }

    #[test]
    fn a_short_host_press_is_retained_and_always_released() {
        let mut next = initial();
        let mut routing = Routing::new(next);
        let now = Instant::now();
        next.rises[13] = 1; // on and off both arrived between engine polls
        let mut events = Vec::new();
        routing.poll(next, now, |e| events.push(e));
        routing.poll(next, now + Duration::from_millis(99), |e| events.push(e));
        assert_eq!(events, [Event::Key { id: 13, down: true }]);
        routing.poll(next, now + Duration::from_millis(100), |e| events.push(e));
        assert_eq!(events.last(), Some(&Event::Key { id: 13, down: false }));
    }

    #[test]
    fn restoring_a_set_or_switching_firmware_does_not_replay_controls() {
        let mut next = initial();
        let mut routing = Routing::new(next);
        next.positions = [127; 7]; next.rises = [5; 14]; next.epoch += 1;
        let mut events = Vec::new();
        routing.poll(next, Instant::now(), |e| events.push(e));
        routing.poll(next, Instant::now(), |e| events.push(e));
        assert!(events.is_empty());
    }

    #[test]
    fn independent_instances_and_overlapping_mouse_presses() {
        let mut next = initial();
        let mut first = Routing::new(next);
        let mut second = Routing::new(next);
        let now = Instant::now();
        let mut events = Vec::new();
        first.manual_key(2, true, |e| events.push(e));
        next.rises[2] += 1;
        first.poll(next, now, |e| events.push(e));
        first.poll(next, now + Duration::from_millis(110), |e| events.push(e));
        assert_eq!(events, [Event::Key { id: 2, down: true }]);
        first.manual_key(2, false, |e| events.push(e));
        assert_eq!(events.last(), Some(&Event::Key { id: 2, down: false }));
        second.poll(initial(), now, |_| panic!("another instance received an event"));
    }

    #[test]
    fn stable_host_ids_are_unique_and_old_parameters_remain_present() {
        let params = crate::Fm1Params::default();
        let ids: Vec<_> = params.param_map().into_iter().map(|(id, _, _)| id).collect();
        assert_eq!(ids.len(), 26);
        assert!(ids.iter().any(|id| id == "master"));
        assert!(ids.iter().any(|id| id == "channel"));
        assert!(ids.iter().any(|id| id == "clock"));
        assert!(ids.iter().any(|id| id == "transport_sync"));
        assert!(ids.iter().any(|id| id == "audio_reserve"));
        assert!(ids.iter().any(|id| id == "enc_7"));
        assert!(ids.iter().any(|id| id == "btn_14"));
        let mut unique = ids.clone(); unique.sort(); unique.dedup();
        assert_eq!(unique.len(), ids.len());
    }
}
