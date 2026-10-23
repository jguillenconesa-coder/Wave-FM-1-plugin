//! Real packages, CPU/JIT, MIDI/panel input, LCD and audio. Missing data is a failure.
use fm1_plugin::{catalogue, engine::{self, Cmd, Midi, Shared, View}, Fm1Params};
use std::{sync::{mpsc, Arc, Mutex, atomic::Ordering}, time::{Duration, Instant}};

#[test]
fn every_bundled_firmware_boots_draws_and_produces_finite_audio() {
    let directory = std::env::var_os("FM1_CATALOGUE_TEST_DIR").map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../firmware/catalogue"));
    for group in catalogue::ENTRIES.chunks(4) {
        std::thread::scope(|scope| {
            for entry in group {
                let directory = &directory;
                scope.spawn(move || exercise_entry(entry, directory));
            }
        });
    }
}

pub fn exercise_entry(entry: &catalogue::Entry, directory: &std::path::Path) {
        let firmware = catalogue::read_at(directory, entry).expect(entry.name);
        let shared = Arc::new(Shared {
            view: Mutex::new(View { lcd: vec![0; 240 * 240], leds: [0.0; 41], pads: [0; 16],
                board: fm1_soc::Board::Fm1, booted: false, halted: false, speed: 0.0, starved: 0, late: 0 }),
            quit: Default::default(),
        });
        let params = Arc::new(Fm1Params::default());
        let (tx, rx) = mpsc::channel();
        let (mut midi, midi_rx) = rtrb::RingBuffer::new(1024);
        let (audio_tx, mut audio) = rtrb::RingBuffer::new(16384);
        let sh = shared.clone(); let par = params.clone();
        let thread = std::thread::spawn(move || engine::run(sh, par, rx, midi_rx, audio_tx));
        tx.send(Cmd::Lead(2048)).unwrap();
        let (reply, answer) = mpsc::channel();
        tx.send(Cmd::Load { firmware: firmware.clone(), sectors: Vec::new(), reply }).unwrap();
        let boot = answer.recv_timeout(Duration::from_secs(20));
        if !matches!(&boot, Ok(Ok(()))) {
            shared.quit.store(true, Ordering::Relaxed); thread.join().unwrap();
            panic!("{} failed to load: {:?}", entry.name, boot);
        }
        // The same import command used by the menu stores a portable package.
        let stored = params.firmware.lock().unwrap().clone();
        assert_eq!(stored.name, firmware.name);
        assert_eq!(stored.pkg, firmware.pkg);
        let save = fm1_plugin::savefile::SaveData { firmware: stored, sectors: Vec::new() };
        let bytes = fm1_plugin::savefile::encode(&save).unwrap();
        assert_eq!(fm1_plugin::savefile::decode(&bytes).unwrap().firmware.pkg, firmware.pkg);
        for channel in 0..16 {
            midi.push(Midi { frame: 3 * 44100, bytes: [0x90 | channel, 60, 100], len: 3 }).unwrap();
        }
        for channel in 0..16 {
            midi.push(Midi { frame: 5 * 44100, bytes: [0x80 | channel, 60, 0], len: 3 }).unwrap();
        }
        let start = Instant::now(); let mut frames = 0; let mut peak = 0.0f32; let mut finite = true;
        let mut played = false; let mut released_play = false; let mut keyed = false;
        // Ketch boots into tracker navigation. Its documented REC button enables
        // note entry/audition; its beta does not route incoming MIDI to voices.
        let play_button = if entry.id == "ketch" { 13 } else { 12 };
        while frames < 8 * 44100 && start.elapsed() < Duration::from_secs(40) {
            if frames >= 2 * 44100 && !played {
                tx.send(Cmd::Key { id: play_button, down: true }).unwrap();
                played = true;
            }
            // Matrix buttons must stay down across emulated scan ticks. Sending
            // down/up in the same command drain is an invisible zero-length tap.
            if frames >= 2 * 44100 + 4410 && !released_play {
                tx.send(Cmd::Key { id: play_button, down: false }).unwrap(); released_play = true;
            }
            if frames >= 3 * 44100 && !keyed {
                tx.send(Cmd::Key { id: 26, down: true }).unwrap(); keyed = true;
            }
            match audio.pop() {
                Ok(f) => { frames += 1; for v in f { finite &= v.is_finite(); peak = peak.max(v.abs()); } }
                Err(_) => std::thread::sleep(Duration::from_millis(1)),
            }
        }
        shared.quit.store(true, Ordering::Relaxed); thread.join().unwrap();
        let view = shared.view.lock().unwrap();
        let lit = view.lcd.iter().filter(|&&p| p != 0).count();
        eprintln!("{} {}: {frames} frames, peak {peak:.5}, {lit} lit pixels, halted {}", entry.name, entry.version, view.halted);
        assert!(view.booted && !view.halted, "{} CPU halted", entry.name);
        assert!(lit > 1000, "{} screen stayed dark", entry.name);
        assert_eq!(frames, 8 * 44100, "{} audio timed out", entry.name);
        assert!(finite && peak > 0.0001 && peak < 32.0, "{} invalid or silent audio: {peak}", entry.name);
}

#[test]
fn skins_and_firmware_are_owned_by_each_instance() {
    let a = Fm1Params::default(); let b = Fm1Params::default();
    *a.skin.lock().unwrap() = "lilac".into();
    a.firmware.lock().unwrap().name = "instance A".into();
    assert_eq!(*b.skin.lock().unwrap(), "graphite");
    assert!(b.firmware.lock().unwrap().name.is_empty());
}
