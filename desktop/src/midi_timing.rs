// MIT. Timestamped MIDI delivery on the emulated CPU timeline, not DMA boundaries.
use crate::engine::Midi;
use fm1_soc::{Action, periph::Periph};

pub fn schedule(periph: &mut Periph, made: u64, event: Midi) -> bool {
    if periph.audio_halves == 0 { return false; }
    // The last DMA deadline corresponds to the end of `made` output frames.
    // Integer arithmetic avoids accumulating a rounded clock-pulse period.
    let delta = (event.frame as i128 - made as i128) * periph.hz as i128 / 44100;
    let cycle = (periph.last_audio_cycle as i128 + delta).clamp(0, u64::MAX as i128) as u64;
    for &byte in &event.bytes[..event.len.min(3) as usize] {
        periph.schedule_input(cycle, Action::Midi { b: byte });
    }
    true
}

/// A queue recovery establishes a new host-frame origin at the current guest
/// DMA boundary. Keep pending MIDI bytes in order without touching hardware
/// timers, RAM, voices, patches, or non-MIDI scripted actions.
pub fn rebase(periph: &mut Periph, old_end: u64, new_end: u64) {
    let offset = (old_end as i128 - new_end as i128) * periph.hz as i128 / 44100;
    for (cycle, action) in &mut periph.script {
        if matches!(action, Action::Midi { .. }) {
            *cycle = (*cycle as i128 + offset).clamp(0, u64::MAX as i128) as u64;
        }
    }
    // Stable ordering is essential: the three bytes of a MIDI message may
    // share a timestamp. This work runs only on the paused producer.
    periph.script.make_contiguous().sort_by_key(|(cycle, _)| *cycle);
    periph.invalidate_input_deadline();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn recovery_rebases_midi_stably_without_retiming_hardware_or_keys() {
        let mut soc = fm1_soc::Soc::new(1 << 20, vec![]);
        let p = &mut soc.periph;
        p.hz = 240_000_000;
        p.schedule_input(360_000_000, Action::Key { id: 12, down: false });
        for b in [0x90, 60, 100] { p.schedule_input(480_000_000, Action::Midi { b }); }
        let cycles = p.cycles;
        rebase(p, 44100, 88200);
        assert_eq!(p.cycles, cycles);
        assert_eq!(p.script.iter().map(|(at, _)| *at).collect::<Vec<_>>(),
            [240_000_000, 240_000_000, 240_000_000, 360_000_000]);
        assert_eq!(p.script.iter().take(3).map(|(_, a)| a.clone()).collect::<Vec<_>>(),
            [Action::Midi { b: 0x90 }, Action::Midi { b: 60 }, Action::Midi { b: 100 }]);
        assert_eq!(p.script.back().unwrap().1, Action::Key { id: 12, down: false });
        rebase(p, 0, 88200);
        assert_eq!(p.script.iter().take(3).map(|(at, _)| *at).collect::<Vec<_>>(), [0, 0, 0]);
    }
    #[test]
    fn midi_deadlines_do_not_snap_to_audio_dma_blocks() {
        let mut soc = fm1_soc::Soc::new(1 << 20, vec![]);
        let p = &mut soc.periph;
        p.hz = 240_000_000;
        p.audio_halves = 1;
        p.last_audio_cycle = 240_000_000;
        let mut deadlines = Vec::new();
        for frame in [44100, 45018, 45937, 46856, 47775] {
            assert!(schedule(p, 44100, Midi { frame, bytes: [0xf8, 0, 0], len: 1 }));
            deadlines.push(p.script.back().unwrap().0);
        }
        for (i, &cycle) in deadlines.iter().enumerate() {
            let expected = 240_000_000 + ([44100u64, 45018, 45937, 46856, 47775][i] - 44100) * 240_000_000 / 44100;
            assert_eq!(cycle, expected);
        }
        let mut delivered = Vec::new();
        while let Some((at, _)) = p.script.front() {
            let at = *at;
            let due = p.advance(at - p.cycles);
            assert_eq!(due.actions.len(), 1);
            delivered.push(p.cycles);
        }
        assert_eq!(delivered, deadlines);
        // Late data is applied in its order without replacing the queue's future
        // deadline. A following timestamp must not hide an earlier input.
        p.schedule_input(p.cycles + 1000, Action::Midi { b: 0xf8 });
        p.schedule_input(p.cycles + 10, Action::Midi { b: 0xfc });
        assert_eq!(p.advance(10).actions, [Action::Midi { b: 0xfc }]);
    }
}
