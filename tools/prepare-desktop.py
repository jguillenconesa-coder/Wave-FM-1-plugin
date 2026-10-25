"""Prepare our MIT desktop UI fork beside the unchanged, pinned emulator."""
import hashlib
import json
from pathlib import Path
import shutil

from bootstrap import ROOT, SOURCE, main as verify_upstream

DEST = ROOT / ".deps/fm1-desktop"
BASE_HASHES = {
    "crates/fm1-plugin/src/lib.rs": "bacecec087841b1a589f09a0751fd51f336746217d37218c5bcb95c25cc1fbb6",
    "crates/fm1-plugin/src/engine.rs": "e5e48026467ace50a5b0bc31517687017115a8f5be04df672b1a406ce2085a12",
    "crates/fm1-plugin/src/editor.rs": "cee9c6c3036c9ab62505ec5bbe36217c9e932c0a4d14bd46bc871642b16e2df3",
    "crates/fm1-live/src/panel.rs": "8ea12e3383cca39ed8ace90bb04f465e6bff174e010a23899fec0e8e5985f931",
    "crates/fm1-soc/src/periph.rs": "4380f1efd1ba76a5dd6348f7e6899669a0674bee4024cdb0050cd6440a670d9f",
}


def main():
    verify_upstream()
    for name, expected in BASE_HASHES.items():
        # Git for Windows may check out CRLF; verify the same normalized source.
        data = (SOURCE / name).read_bytes().replace(b"\r\n", b"\n")
        if hashlib.sha256(data).hexdigest() != expected:
            raise SystemExit(f"Unexpected UI baseline: {name}")
    if DEST.is_symlink():
        raise SystemExit("Desktop build directory must not be a symlink")
    # Only generated source is refreshed. The target cache lives separately.
    if DEST.exists():
        shutil.rmtree(DEST)
    shutil.copytree(SOURCE, DEST, ignore=shutil.ignore_patterns(".git", "target", "reference", "node_modules"))
    plugin = DEST / "crates/fm1-plugin"
    reference = DEST / "reference/firmwares"
    reference.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(ROOT / "firmware/catalogue/felucca-1.6.1.fwsc", reference / "felucca.fwsc")
    for name in ("editor.rs", "catalogue.rs", "skins.rs", "controls.rs", "online.rs", "host_sync.rs", "audio_timing.rs", "midi_timing.rs", "worker.rs", "latency_tests.rs"):
        shutil.copyfile(ROOT / "desktop/src" / name, plugin / "src" / name)
    shutil.copyfile(ROOT / "firmware/community-index.json", plugin / "src/community-index.json")
    shutil.copyfile(ROOT / "desktop/src/panel.rs", DEST / "crates/fm1-live/src/panel.rs")
    shutil.copyfile(ROOT / "desktop/tests/catalogue.rs", plugin / "tests/catalogue.rs")
    (plugin / "examples").mkdir(exist_ok=True)
    shutil.copyfile(ROOT / "desktop/examples/check-community.rs", plugin / "examples/check-community.rs")
    lib = plugin / "src/lib.rs"
    text = lib.read_text()
    text = "// Desktop additions: firmware catalogue and per-instance skin. MIT; see desktop/README.md.\n" + text
    text = text.replace("mod editor;", "mod editor;\npub mod catalogue;\nmod skins;\nmod controls;\nmod online;\npub mod host_sync;\nmod audio_timing;\nmod midi_timing;\nmod worker;\n#[cfg(test)]\nmod latency_tests;")
    text = text.replace('#[persist = "flash"]', '#[persist = "skin"]\n    pub skin: Mutex<String>,\n    #[persist = "flash"]')
    text = text.replace("firmware: Mutex::new(Firmware::default()),", 'firmware: Mutex::new(Firmware::default()),\n            skin: Mutex::new("graphite".into()),')
    text = replace_once(text, '#[id = "master"]', '#[nested]\n    pub controls: controls::PanelParams,\n    #[id = "master"]')
    text = replace_once(text, 'master: FloatParam::new(', 'controls: controls::PanelParams::default(),\n            master: FloatParam::new(')
    text = replace_once(text, '    pub clock: EnumParam<Clock>,', '''    pub clock: EnumParam<Clock>,
    #[id = "transport_sync"]
    pub transport_sync: EnumParam<host_sync::Source>,
    pub transport_feedback: host_sync::Feedback,
    #[id = "audio_reserve"]
    pub audio_reserve: EnumParam<audio_timing::Reserve>,
    pub audio_diagnostics: audio_timing::Diagnostics,''')
    text = replace_once(text, '            clock: EnumParam::new("CPU clock", Clock::Mhz240).non_automatable(),', '''            clock: EnumParam::new("CPU clock", Clock::Mhz240).non_automatable(),
            transport_sync: EnumParam::new("Musical clock", host_sync::Source::Internal).non_automatable(),
            transport_feedback: host_sync::Feedback::default(),
            audio_reserve: EnumParam::new("Audio buffer", audio_timing::Reserve::Safe).non_automatable(),
            audio_diagnostics: audio_timing::Diagnostics::default(),''')
    text = replace_once(text, '    ratio: f64,', '    ratio: f64,\n    max_block: u32,\n    reported_latency: u32,\n    musical_clock: host_sync::Clock,')
    text = replace_once(text, '            ratio: 1.0,', '            ratio: 1.0,\n            max_block: 0,\n            reported_latency: 0,\n            musical_clock: host_sync::Clock::default(),')
    text = replace_once(text, '''    taken: u64,
    frac: f64,
    cur: [f32; 2],
    next: [f32; 2],''', '    playback: audio_timing::Playback,')
    text = replace_once(text, '''            taken: 0,
            frac: 0.0,
            cur: [0.0; 2],
            next: [0.0; 2],''', '            playback: audio_timing::Playback::default(),')
    init_start = text.index('        let link = self.start();\n        self.ratio = RATE / cfg.sample_rate as f64;')
    init_end = text.index('        true\n', init_start)
    text = text[:init_start] + '''        self.ratio = RATE / cfg.sample_rate as f64;
        let Some(lead) = audio_timing::lead_for(self.ratio, cfg.max_buffer_size, self.params.audio_reserve.value()) else { return false; };
        self.max_block = cfg.max_buffer_size;
        self.lead = lead;
        self.params.controls.resync();
        self.musical_clock.reset();
        self.params.audio_diagnostics.configure(cfg.sample_rate, cfg.max_buffer_size);
        self.params.audio_diagnostics.request_lead(lead);
        let fw = self.params.firmware.lock().unwrap().clone();
        let sectors = self.params.flash.lock().unwrap().clone();
        self.params.audio_diagnostics.set_loaded(!fw.pkg.is_empty());
        self.reported_latency = audio_timing::latency_samples(lead, self.ratio, !fw.pkg.is_empty());
        self.params.audio_diagnostics.publish_latency(self.reported_latency);
        ctx.set_latency_samples(self.reported_latency);
        let link = self.start();
        let _ = link.cmds.send(Cmd::Lead(lead));
        link.boot(&fw, &sectors);
''' + text[init_end:]
    text = replace_once(text, '        let start = self.taken;', '''        self.params.controls.publish();
        self.params.audio_diagnostics.observe_block(buffer.samples() as u32);
        let playing = ctx.transport().playing;
        self.params.audio_diagnostics.set_playing(playing);
        let loaded = self.params.audio_diagnostics.loaded();
        if let Some(latency) = self.update_audio_timing(playing, loaded, self.params.audio_reserve.value()) { ctx.set_latency_samples(latency); }
        let start = self.playback.position;''')
    text = replace_once(text, 'impl Fm1 {', '''impl Fm1 {
    // Latency changes are explicit, coalesced and applied at block boundaries.
    // The worker observes a bounded atomic target; no command allocation/wait.
    // A smaller target lets existing queued frames drain in order. Change the
    // reserve while stopped: the queue and host compensation may briefly settle.
    fn update_audio_timing(&mut self, playing: bool, loaded: bool, reserve: audio_timing::Reserve) -> Option<u32> {
        if !playing {
            if let Some(lead) = audio_timing::lead_for(self.ratio, self.max_block, reserve) {
                self.lead = lead;
                self.params.audio_diagnostics.request_lead(lead);
            }
        }
        let latency = audio_timing::latency_samples(self.lead, self.ratio, loaded);
        if latency == self.reported_latency { return None; }
        self.reported_latency = latency;
        self.params.audio_diagnostics.publish_latency(latency);
        Some(latency)
    }
''')
    midi_start = text.index('        let ch = self.params.channel.value() as u8;')
    midi_end = text.index('        let Some(rx) = self.audio.as_mut()', midi_start)
    if text[midi_start:midi_end].count('while let Some(ev) = ctx.next_event()') != 1:
        raise SystemExit('Unexpected MIDI processing baseline')
    text = text[:midi_start] + '''        let source = self.params.transport_sync.value();
        let block = host_sync::Block::from_host(ctx.transport(), buffer.samples() as u32);
        let delivered = host_sync::send_block(&mut self.musical_clock, source, block, ctx,
            self.midi.as_mut(), start, self.ratio, self.lead, self.params.channel.value() as u8);
        self.params.transport_feedback.publish(if delivered { block.status(source) } else { host_sync::QUEUE_FULL }, block.tempo);
''' + text[midi_end:]
    text = replace_once(text, 'use nih_plug::midi::MidiResult;\n', '')
    render_start = text.index('        let n = out[0].len();')
    render_end = text.index('        if starved > 0 {', render_start)
    if text[render_start:render_end].count('self.frac = self.frac.min(1.0);') != 1:
        raise SystemExit('Unexpected audio rendering baseline')
    text = text[:render_start] + '''        let starved = self.playback.render(out, self.ratio, || rx.pop().ok());
        self.params.audio_diagnostics.publish_position(self.playback.position);
        let starved = if loaded { starved } else {
            // Empty/restored-empty instances produce silence and need no PDC.
            // Continue draining the timeline so a later firmware starts in sync.
            for channel in out.iter_mut() { channel.fill(0.0); }
            0
        };
''' + text[render_end:]
    text = replace_once(text, '    pub running: Mutex<Option<Arc<Vec<u8>>>>,',
                        '    pub running: Mutex<Option<Arc<Vec<u8>>>>,\n    jobs: Mutex<Vec<std::thread::JoinHandle<()>>>,')
    text = replace_once(text, 'impl Link {', '''impl Link {
    // A host may unload the plugin DLL after the last instance closes. Keep job
    // handles on the instance and join before destruction can return to the host.
    pub fn background(&self, job: impl FnOnce() + Send + 'static) {
        let mut jobs = self.jobs.lock().unwrap();
        let mut index = 0;
        while index < jobs.len() {
            if jobs[index].is_finished() { let _ = jobs.swap_remove(index).join(); }
            else { index += 1; }
        }
        jobs.push(std::thread::spawn(job));
    }
''')
    text = replace_once(text, 'running: Mutex::new(None)', 'running: Mutex::new(None), jobs: Mutex::new(Vec::new())')
    text = replace_once(text, '        if fw.pkg.is_empty() || running.as_ref().is_some_and', '''        if fw.pkg.is_empty() {
            *running = None;
            self.shared.view.lock().unwrap().late = 0;
            let _ = self.cmds.send(Cmd::Clear);
            return;
        }
        if running.as_ref().is_some_and''')
    text = replace_once(text, '        if let Some(t) = self.thread.take() {\n            let _ = t.join();\n        }',
                        '''        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
        if let Some(link) = &self.link {
            let jobs: Vec<_> = link.jobs.lock().unwrap().drain(..).collect();
            for job in jobs { let _ = job.join(); }
        }''')
    lib.write_text(text)
    engine = plugin / "src/engine.rs"
    text = engine.read_text()
    text = replace_once(text, '    fn changed_sectors(&self) -> Sectors {', '''    // The SoC has already copied a completed DMA half before cpu1 renders
    // its next work. Publish that host-owned copy before running cpu1; taking
    // it does not alter guest RAM, cycles, interrupts or instruction order.
    fn run_audio(&mut self, n: u32, mut emit: impl FnMut(&mut Vec<i16>)) {
        let jit = self.cpu.jit_on;
        let Machine { cpu, cpu1, soc, pending_tick, .. } = self;
        if machine::run_dual(cpu1, soc, n, RAM_SIZE, jit, |soc, k| {
            let result = machine::run_blocks(cpu, soc, k, pending_tick);
            if !soc.periph.audio.is_empty() { emit(&mut soc.periph.audio); }
            result
        }).is_err() { self.halted = true; }
    }

    fn changed_sectors(&self) -> Sectors {''')
    text = replace_once(text, '    Lead(usize),', '    Lead(usize),\n    Clear,')
    text = replace_once(text, '    let mut m: Option<Machine> = None;', '''    let scheduling = crate::worker::Scheduling::audio();
    params.audio_diagnostics.set_scheduling(scheduling.status);
    let mut m: Option<Machine> = None;''')
    text = replace_once(text, '    let mut m: Option<Machine> = None;',
                        '''    let mut controls = crate::controls::Routing::new(params.controls.snapshot().unwrap_or_default());
    // Publish audio frequently; large instruction batches can miss host deadlines
    // even when the firmware's average emulation speed exceeds real time.
    #[cfg(not(test))]
    let render_slice = 8_000;
    #[cfg(test)]
    let render_slice = std::env::var("FM1_TEST_ENGINE_SLICE").ok().and_then(|s| s.parse().ok()).unwrap_or(8_000);
    #[cfg(test)]
    let mut render_max = (0u128, 0usize, 0u128);
    let mut warming = true;
    let mut m: Option<Machine> = None;''')
    text = replace_once(text, '                    active_pkg = m.as_ref().map(|_| pkg);',
                        '                    controls = crate::controls::Routing::new(params.controls.snapshot().unwrap_or_default());\n                    warming = true;\n                    active_pkg = m.as_ref().map(|_| pkg);')
    text = replace_once(text, '                    v.booted = m.is_some();', '                    v.booted = m.is_some();\n                    params.audio_diagnostics.set_loaded(m.is_some());\n                    v.late = 0;')
    text = replace_once(text, '                Cmd::Lead(l) => lead = l,', '''                Cmd::Lead(l) => { lead = l; params.audio_diagnostics.request_lead(l); }
                Cmd::Clear => {
                    cancel_snapshots(&mut pending);
                    m = None; active_pkg = None; due.clear(); saved.clear();
                    params.audio_diagnostics.set_loaded(false);
                    let mut v = shared.view.lock().unwrap();
                    v.booted = false; v.halted = false; v.speed = 0.0; v.late = 0;
                    v.lcd.fill(0); v.leds.fill(0.0); v.pads.fill(0);
                }''')
    text = replace_once(text, '        let Some(mm) = m.as_mut().filter(|m| !m.halted) else {', '''        let requested = params.audio_diagnostics.requested_lead();
        if requested > 0 && requested <= 16384 - 512 { lead = requested; }
        let Some(mm) = m.as_mut().filter(|m| !m.halted) else {''')
    text = replace_once(text, 'Cmd::Key { id, down } => apply(&mut m, Action::Key { id, down }),',
                        '''Cmd::Key { id, down } => {
                    if m.as_ref().is_some_and(|m| m.soc.board == Board::Fm1) {
                        controls.manual_key(id, down, |e| panel_event(&mut m, e));
                    } else { apply(&mut m, Action::Key { id, down }); }
                }''')
    text = replace_once(text, '                    m = Some(next);',
                        '                    controls = crate::controls::Routing::new(params.controls.snapshot().unwrap_or_default());\n                    warming = true;\n                    m = Some(next);')
    text = replace_once(text, '        snapshots(&mut pending, m.as_ref(), &params, Instant::now());', '''        if m.as_ref().is_some_and(|m| m.soc.board == Board::Fm1) {
            if let Some(snapshot) = params.controls.snapshot() {
                controls.poll(snapshot, Instant::now(), |e| panel_event(&mut m, e));
            }
        } else {
            // FM-1 control IDs must never be applied to an imported SMK-37 firmware.
            controls = crate::controls::Routing::new(params.controls.snapshot().unwrap_or_default());
        }
        snapshots(&mut pending, m.as_ref(), &params, Instant::now());''')
    text = replace_once(text, '''            while due.front().is_some_and(|e| e.frame <= made) {
                let e = due.pop_front().unwrap();
                for &b in &e.bytes[..e.len as usize] {
                    mm.soc.periph.apply(&Action::Midi { b });
                }
            }''', '''            while due.front().is_some_and(|e| mm.soc.periph.audio_halves > 0 || e.frame <= made) {
                let e = due.pop_front().unwrap();
                if !crate::midi_timing::schedule(&mut mm.soc.periph, made, e) {
                    for &b in &e.bytes[..e.len.min(3) as usize] { mm.soc.periph.apply(&Action::Midi { b }); }
                }
            }''')
    text = replace_once(text, '''        while let Ok(e) = midi_in.pop() {
            due.push_back(e);
        }''', '''        // Backpressure keeps future MIDI bounded if emulation cannot keep up.
        let scheduled = m.as_ref().map_or(0, |m| m.soc.periph.script.len());
        while due.len() + scheduled < 4096 {
            match midi_in.pop() { Ok(e) => due.push_back(e), Err(_) => break }
        }''')
    text = replace_once(text, 'fill >= lead && last_take.elapsed() >= Duration::from_millis(50)',
        'fill >= lead && last_take.elapsed() >= Duration::from_secs_f64((lead as f64 / RATE).max(0.05) + 0.025) && params.transport_sync.value() == crate::host_sync::Source::Internal')
    if text.count('mm.run(20_000);') != 2:
        raise SystemExit('Unexpected worker render-slice baseline')
    text = text.replace('mm.run(20_000);', 'mm.run(render_slice);')
    text = replace_once(text, '''        if fill >= lead && last_take.elapsed()''', '''        if !warming && fill >= lead && last_take.elapsed()''')
    text = replace_once(text, '''        } else if fill >= lead {''', '''        } else if !warming && fill >= lead {''')
    text = replace_once(text, '''            let t0 = Instant::now();
            mm.run(render_slice);
            busy += t0.elapsed();''', '''            if warming && !crate::audio_timing::prime_startup(&mut made,
                params.audio_diagnostics.position(), lead, |f| out.push(f).is_ok()) {
                std::thread::sleep(Duration::from_micros(500));
                continue;
            }
            let t0 = Instant::now();
            #[cfg(test)]
            let mut published_at = None;
            // Keep the first half pending if padding could not fit last time.
            // Do not advance or discard that first audible buffer while waiting.
            if mm.soc.periph.audio.is_empty() {
                mm.run_audio(render_slice, |audio| {
                    #[cfg(test)]
                    { render_max.1 = render_max.1.max(audio.len() / 2); }
                    if crate::audio_timing::publish_dma(audio, &mut made, &mut out,
                        params.audio_diagnostics.position(), lead, &mut warming) {
                        #[cfg(test)]
                        { published_at.get_or_insert_with(Instant::now); }
                    }
                });
            }
            busy += t0.elapsed();
            #[cfg(test)]
            {
                render_max.0 = render_max.0.max(t0.elapsed().as_micros());
                if let Some(at) = published_at { render_max.2 = render_max.2.max(at.elapsed().as_micros()); }
            }''')
    text = replace_once(text, '''            let a = std::mem::take(&mut mm.soc.periph.audio);
            for f in a.chunks_exact(2) {
                // (a full ring drops the frame: the host has stopped taking them)
                if out.push([f[0] as f32 / 32768.0, f[1] as f32 / 32768.0]).is_ok() {
                    made += 1;
                }
            }''', '''            #[cfg(test)]
            { render_max.1 = render_max.1.max(mm.soc.periph.audio.len() / 2); }
            // Fallback for pending first halves and cpu0-held peripheral ticks.
            if !crate::audio_timing::publish_dma(&mut mm.soc.periph.audio,
                &mut made, &mut out, params.audio_diagnostics.position(), lead, &mut warming) {
                std::thread::sleep(Duration::from_micros(500));
            }''')
    text = replace_once(text, '''    }
}

fn apply(m: &mut Option<Machine>, a: Action) {''', '''    }
    #[cfg(test)]
    if std::env::var_os("FM1_REALTIME_DIAGNOSTIC").is_some() {
        eprintln!("RENDER slice={} max_us={} max_batch_frames={} early_delivery_max_gain_us={}", render_slice, render_max.0, render_max.1, render_max.2);
    }
}

fn apply(m: &mut Option<Machine>, a: Action) {''')
    text += '''
// Host panel controls run on the machine thread, never the audio callback.
fn panel_event(m: &mut Option<Machine>, event: crate::controls::Event) {
    use crate::controls::Event;
    apply(m, match event {
        Event::Encoder { enc, clicks } => Action::Enc { enc, clicks },
        Event::Key { id, down } => Action::Key { id, down },
    });
}

#[cfg(test)]
mod early_delivery_tests {
    use super::*;
    #[test]
    fn x0x_samples_and_guest_state_match_the_original_runner_exactly() {
        let directory = std::env::var_os("FM1_CATALOGUE_TEST_DIR").map(std::path::PathBuf::from)
            .unwrap_or_else(|| std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../firmware/catalogue"));
        let entry = crate::catalogue::ENTRIES.iter().find(|e| e.id == "x0x").unwrap();
        let fw = crate::catalogue::read_at(&directory, entry).unwrap();
        let mut reference = Machine::boot(&fw.pkg, &[], 240).unwrap();
        let mut early = Machine::boot(&fw.pkg, &[], 240).unwrap();
        // Match the engine's Master ADC input; raw Machine boots at zero.
        reference.soc.periph.apply(&Action::Adc { ch: 4, value: 900 });
        early.soc.periph.apply(&Action::Adc { ch: 4, value: 900 });
        let mut received = Vec::new(); let mut total = 0; let mut peak = 0i32; let mut notes = false;
        for _ in 0..200_000 {
            reference.run(8000);
            early.run_audio(8000, |audio| { received.extend_from_slice(audio); audio.clear(); });
            received.extend_from_slice(&early.soc.periph.audio); early.soc.periph.audio.clear();
            let expected = std::mem::take(&mut reference.soc.periph.audio);
            assert_eq!(received, expected, "early delivery changed samples");
            total += expected.len() / 2;
            for sample in &expected { peak = peak.max((*sample as i32).abs()); }
            received.clear();
            assert_eq!(reference.cpu.pc, early.cpu.pc);
            assert_eq!(reference.cpu.regs, early.cpu.regs);
            assert_eq!(reference.soc.periph.cycles, early.soc.periph.cycles);
            assert_eq!(reference.soc.periph.audio_halves, early.soc.periph.audio_halves);
            assert_eq!(reference.halted, early.halted);
            match (&reference.cpu1, &early.cpu1) {
                (Some(a), Some(b)) => { assert_eq!(a.pc, b.pc); assert_eq!(a.regs, b.regs); }
                (None, None) => (), _ => panic!("different cpu1 release"),
            }
            if total >= 22050 && !notes {
                for channel in 0..16 {
                    for b in [0x90 | channel, 60, 100] {
                        reference.soc.periph.apply(&Action::Midi { b });
                        early.soc.periph.apply(&Action::Midi { b });
                    }
                }
                notes = true;
            }
            if total >= 88200 { break; }
        }
        assert!(total >= 88200 && peak > 100 && !early.halted,
            "frames={total} peak={peak} halted={}", early.halted);
        assert_eq!(reference.soc.flash.mem, early.soc.flash.mem);
        assert_eq!(reference.soc.lcd.fb, early.soc.lcd.fb);
        assert_eq!(reference.soc.ram.len(), early.soc.ram.len());
        for at in (0..reference.soc.ram.len() as u32).step_by(4) {
            assert_eq!(reference.soc.ram.read32(at), early.soc.ram.read32(at), "guest RAM differs");
        }
        eprintln!("BYTE_IDENTICAL X0X frames={total} peak={peak}; cpu0/cpu1, cycles, RAM, flash and LCD match");
    }
}
'''
    engine.write_text(text)
    periph = DEST / 'crates/fm1-soc/src/periph.rs'
    text = periph.read_text()
    text = replace_once(text, '    pub audio_halves: u64,', '    pub audio_halves: u64,\n    pub last_audio_cycle: u64,')
    text = replace_once(text, '            audio_halves: 0,', '            audio_halves: 0,\n            last_audio_cycle: 0,')
    text = replace_once(text, '            self.alnk_next += self.half_period();',
        '            self.last_audio_cycle = self.alnk_next;\n            self.alnk_next += self.half_period();')
    text = replace_once(text, '    pub fn apply(&mut self, a: &Action) {', '''    /// Insert timestamped input and invalidate the JIT's cached next deadline.
    pub fn schedule_input(&mut self, cycle: u64, action: Action) {
        let i = self.script.partition_point(|(at, _)| *at <= cycle);
        self.script.insert(i, (cycle, action));
        self.next_due = 0;
    }

    pub fn apply(&mut self, a: &Action) {''')
    periph.write_text(text)
    # Only this integration's plugin version changes; dependency pins remain intact.
    manifest = plugin / "Cargo.toml"
    manifest.write_text(replace_once(manifest.read_text(), 'version = "0.2.0"', 'version = "0.2.7"'))
    lock = plugin / "Cargo.lock"
    lock.write_text(replace_once(lock.read_text(), 'name = "fm1-plugin"\nversion = "0.2.0"',
                                'name = "fm1-plugin"\nversion = "0.2.7"'))
    entries = json.loads((ROOT / "firmware/manifest.json").read_text())["firmwares"]
    generated = "// Generated from firmware/manifest.json. Do not edit.\npub static ENTRIES: &[Entry] = &[\n"
    for e in entries:
        fields = ("id", "name", "version", "author", "license", "description", "file", "sha256")
        generated += "    Entry { " + ", ".join(f"{k}: {json.dumps(e[k], ensure_ascii=False)}" for k in fields)
        generated += f", size: {e['size']} }},\n"
    (plugin / "src/catalogue_entries.rs").write_text(generated + "];\n")
    print("Prepared desktop fork; pinned emulator sources remain unchanged")


def replace_once(text, old, new):
    if text.count(old) != 1:
        raise SystemExit(f"Unexpected desktop patch baseline: {old[:80]}")
    return text.replace(old, new, 1)


if __name__ == "__main__":
    main()
