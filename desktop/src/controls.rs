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
        assert_eq!(ids.len(), 24);
        assert!(ids.iter().any(|id| id == "master"));
        assert!(ids.iter().any(|id| id == "channel"));
        assert!(ids.iter().any(|id| id == "clock"));
        assert!(ids.iter().any(|id| id == "enc_7"));
        assert!(ids.iter().any(|id| id == "btn_14"));
        let mut unique = ids.clone(); unique.sort(); unique.dedup();
        assert_eq!(unique.len(), ids.len());
    }
}
