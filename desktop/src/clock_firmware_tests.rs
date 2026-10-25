// MIT integration regressions. Original X0X firmware: Charles Vestal and contributors, GPL-3.0-only.
// Runs the unmodified package on the actual CPU/SoC, including UART byte timing.
use super::*;
use crate::host_sync::{Block, Clock, Source};

#[test]
fn programmed_x0x_follows_daw_start_tempo_stop_restart_and_loop() {
    let directory = std::env::var_os("FM1_CATALOGUE_TEST_DIR").map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../firmware/catalogue"));
    let entry = crate::catalogue::ENTRIES.iter().find(|e| e.id == "x0x").unwrap();
    let firmware = crate::catalogue::read_at(&directory, entry).unwrap();
    let mut machine = Machine::boot(&firmware.pkg, &[], 240).unwrap();
    let hz = machine.soc.periph.hz;
    machine.soc.periph.apply(&Action::Adc { ch: 4, value: 900 });
    // X0X starts with EMPTY patterns. Follow the author's manual: black 1
    // selects the kick, white 1/5/9/13 write a four-on-the-floor pattern.
    for (time_ms, key) in [(1000u64, 15), (1200, 14), (1400, 21), (1600, 28), (1800, 35)] {
        machine.soc.periph.schedule_input(time_ms * hz / 1000, Action::Key { id: key, down: true });
        machine.soc.periph.schedule_input((time_ms + 90) * hz / 1000, Action::Key { id: key, down: false });
    }
    let mut clock = Clock::default();
    for frame in (0..12 * 48000u64).step_by(512) {
        let playing = (96000..336000).contains(&frame) || frame >= 432000;
        let tempo = if frame < 192000 { 120.0 } else { 90.0 };
        let elapsed = frame.min(336000).saturating_sub(96000);
        let fast = elapsed.min(96000);
        let mut beat = fast as f64 / 24000.0 + (elapsed - fast) as f64 / 32000.0;
        if frame >= 432000 { beat += (frame - 432000) as f64 / 32000.0; }
        if frame >= 504000 { beat = (frame - 504000) as f64 / 32000.0; }
        let block = Block { playing, tempo: Some(tempo), beats: Some(beat), sample_rate: 48000.0,
            samples: (12 * 48000 - frame).min(512) as u32 };
        for (offset, byte) in clock.events(Source::Daw, block) {
            // The first stopped state is sent after receiver setup; the
            // wrapper's cold-start/readiness behavior is checked separately.
            let at = if frame == 0 { hz * 900 / 1000 } else { (frame + offset as u64) * hz / 48000 };
            machine.soc.periph.schedule_input(at, Action::Midi { b: byte });
        }
    }
    let mut audio = vec![0.0f32; 12 * 44100];
    let began = Instant::now();
    while machine.soc.periph.cycles < 12 * hz && !machine.halted {
        assert!(began.elapsed() < Duration::from_secs(60), "firmware emulation timed out");
        machine.run(8000);
        let pending = &mut machine.soc.periph.audio;
        let frames = pending.len() / 2;
        let end = (machine.soc.periph.last_audio_cycle as u128 * 44100 / hz as u128) as usize;
        let start = end.saturating_sub(frames);
        for (i, pair) in pending.chunks_exact(2).enumerate() {
            if let Some(value) = audio.get_mut(start + i) { *value = pair[0] as f32 / 32768.0; }
        }
        pending.clear();
    }
    assert!(!machine.halted);
    // An onset must be preceded by 30ms below threshold. This rejects the
    // individual oscillations of the kick without counting its decay twice.
    let peak = audio.iter().map(|v| v.abs()).fold(0.0, f32::max);
    assert!(peak > 0.01);
    let threshold = peak * 0.1;
    let mut quiet = 12; let mut onsets = Vec::new();
    for (bin, chunk) in audio.chunks(110).enumerate() {
        if chunk.iter().any(|v| v.abs() > threshold) {
            if quiet >= 12 { onsets.push(bin as f64 * 110.0 / 44100.0); }
            quiet = 0;
        } else { quiet += 1; }
    }
    let intervals = |lo: f64, hi: f64| -> Vec<f64> {
        let selected: Vec<_> = onsets.iter().copied().filter(|t| *t >= lo && *t < hi).collect();
        selected.windows(2).map(|pair| pair[1] - pair[0]).collect()
    };
    let fast = intervals(2.1, 3.95); let slow = intervals(4.2, 6.95);
    eprintln!("DAW_FIRMWARE X0X: peak={peak:.5}, onsets={onsets:?}, 120bpm_intervals={fast:?}, 90bpm_intervals={slow:?}, dropped_midi={}", machine.soc.periph.midi_dropped());
    assert!(fast.len() >= 2 && fast.iter().all(|d| (*d - 0.5).abs() < 0.035), "not following 120 BPM");
    assert!(slow.len() >= 2 && slow.iter().all(|d| (*d - 2.0/3.0).abs() < 0.035), "not following 90 BPM");
    assert!(!onsets.iter().any(|t| (7.2..9.0).contains(t)), "sequence kept playing after Stop");
    assert!(onsets.iter().any(|t| (9.0..9.15).contains(t)), "sequence did not restart");
    assert!(onsets.iter().any(|t| (10.5..10.65).contains(t)), "sequence did not restart after host loop");
}
