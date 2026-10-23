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

#[cfg(test)]
mod tests {
    use super::*;
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
