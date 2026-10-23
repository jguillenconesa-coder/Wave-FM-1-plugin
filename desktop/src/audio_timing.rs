// Resampler derived from fm1-emu by Joakim Eriksson (MIT); timing recovery additions, 2026.
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
                  mut pop: impl FnMut() -> Option<[f32; 2]>) -> u64 {
        let mut missed = 0;
        let mut recovery_budget = 16384;
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
        let mut playback = Playback { debt: 1_000_000, ..Default::default() };
        let mut left = [0.0; 128]; let mut right = [0.0; 128]; let mut reads = 0;
        assert_eq!(playback.render(&mut [&mut left, &mut right], 1.0,
            || { reads += 1; Some([0.0; 2]) }), 128);
        assert_eq!(reads, 16384);
        assert_eq!(playback.position, 128);
    }
}
