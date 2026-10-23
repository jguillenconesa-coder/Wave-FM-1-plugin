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
    for name in ("editor.rs", "catalogue.rs", "skins.rs", "controls.rs", "online.rs", "host_sync.rs", "audio_timing.rs", "midi_timing.rs"):
        shutil.copyfile(ROOT / "desktop/src" / name, plugin / "src" / name)
    shutil.copyfile(ROOT / "firmware/community-index.json", plugin / "src/community-index.json")
    shutil.copyfile(ROOT / "desktop/src/panel.rs", DEST / "crates/fm1-live/src/panel.rs")
    shutil.copyfile(ROOT / "desktop/tests/catalogue.rs", plugin / "tests/catalogue.rs")
    (plugin / "examples").mkdir(exist_ok=True)
    shutil.copyfile(ROOT / "desktop/examples/check-community.rs", plugin / "examples/check-community.rs")
    lib = plugin / "src/lib.rs"
    text = lib.read_text()
    text = "// Desktop additions: firmware catalogue and per-instance skin. MIT; see desktop/README.md.\n" + text
    text = text.replace("mod editor;", "mod editor;\npub mod catalogue;\nmod skins;\nmod controls;\nmod online;\npub mod host_sync;\nmod audio_timing;\nmod midi_timing;")
    text = text.replace('#[persist = "flash"]', '#[persist = "skin"]\n    pub skin: Mutex<String>,\n    #[persist = "flash"]')
    text = text.replace("firmware: Mutex::new(Firmware::default()),", 'firmware: Mutex::new(Firmware::default()),\n            skin: Mutex::new("graphite".into()),')
    text = replace_once(text, '#[id = "master"]', '#[nested]\n    pub controls: controls::PanelParams,\n    #[id = "master"]')
    text = replace_once(text, 'master: FloatParam::new(', 'controls: controls::PanelParams::default(),\n            master: FloatParam::new(')
    text = replace_once(text, '    pub clock: EnumParam<Clock>,', '''    pub clock: EnumParam<Clock>,
    #[id = "transport_sync"]
    pub transport_sync: EnumParam<host_sync::Source>,
    pub transport_feedback: host_sync::Feedback,
    pub audio_diagnostics: audio_timing::Diagnostics,''')
    text = replace_once(text, '            clock: EnumParam::new("CPU clock", Clock::Mhz240).non_automatable(),', '''            clock: EnumParam::new("CPU clock", Clock::Mhz240).non_automatable(),
            transport_sync: EnumParam::new("Musical clock", host_sync::Source::Internal).non_automatable(),
            transport_feedback: host_sync::Feedback::default(),
            audio_diagnostics: audio_timing::Diagnostics::default(),''')
    text = replace_once(text, '    ratio: f64,', '    ratio: f64,\n    musical_clock: host_sync::Clock,')
    text = replace_once(text, '            ratio: 1.0,', '            ratio: 1.0,\n            musical_clock: host_sync::Clock::default(),')
    text = replace_once(text, '''    taken: u64,
    frac: f64,
    cur: [f32; 2],
    next: [f32; 2],''', '    playback: audio_timing::Playback,')
    text = replace_once(text, '''            taken: 0,
            frac: 0.0,
            cur: [0.0; 2],
            next: [0.0; 2],''', '            playback: audio_timing::Playback::default(),')
    text = replace_once(text, '        let link = self.start();\n        self.ratio = RATE / cfg.sample_rate as f64;',
                        '        self.params.controls.resync();\n        self.musical_clock.reset();\n        self.params.audio_diagnostics.configure(cfg.sample_rate, cfg.max_buffer_size);\n        let link = self.start();\n        self.ratio = RATE / cfg.sample_rate as f64;')
    text = replace_once(text, '        let start = self.taken;', '        self.params.controls.publish();\n        self.params.audio_diagnostics.observe_block(buffer.samples() as u32);\n        let start = self.playback.position;')
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
    text = text[:render_start] + '        let starved = self.playback.render(out, self.ratio, || rx.pop().ok());\n' + text[render_end:]
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
    text = replace_once(text, '    let mut m: Option<Machine> = None;',
                        '''    let mut controls = crate::controls::Routing::new(params.controls.snapshot().unwrap_or_default());
    // Publish audio frequently; large instruction batches can miss host deadlines
    // even when the firmware's average emulation speed exceeds real time.
    #[cfg(not(test))]
    let render_slice = 8_000;
    #[cfg(test)]
    let render_slice = std::env::var("FM1_TEST_ENGINE_SLICE").ok().and_then(|s| s.parse().ok()).unwrap_or(8_000);
    #[cfg(test)]
    let mut render_max = (0u128, 0usize);
    let mut m: Option<Machine> = None;''')
    text = replace_once(text, '                    active_pkg = m.as_ref().map(|_| pkg);',
                        '                    controls = crate::controls::Routing::new(params.controls.snapshot().unwrap_or_default());\n                    active_pkg = m.as_ref().map(|_| pkg);')
    text = replace_once(text, '                    v.booted = m.is_some();', '                    v.booted = m.is_some();\n                    v.late = 0;')
    text = replace_once(text, 'Cmd::Key { id, down } => apply(&mut m, Action::Key { id, down }),',
                        '''Cmd::Key { id, down } => {
                    if m.as_ref().is_some_and(|m| m.soc.board == Board::Fm1) {
                        controls.manual_key(id, down, |e| panel_event(&mut m, e));
                    } else { apply(&mut m, Action::Key { id, down }); }
                }''')
    text = replace_once(text, '                    m = Some(next);',
                        '                    controls = crate::controls::Routing::new(params.controls.snapshot().unwrap_or_default());\n                    m = Some(next);')
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
    text = replace_once(text, '            let a = std::mem::take(&mut mm.soc.periph.audio);', '''            #[cfg(test)]
            { render_max.0 = render_max.0.max(t0.elapsed().as_micros()); render_max.1 = render_max.1.max(mm.soc.periph.audio.len() / 2); }
            let a = std::mem::take(&mut mm.soc.periph.audio);''')
    text = replace_once(text, '''    }
}

fn apply(m: &mut Option<Machine>, a: Action) {''', '''    }
    #[cfg(test)]
    if std::env::var_os("FM1_REALTIME_DIAGNOSTIC").is_some() {
        eprintln!("RENDER slice={} max_us={} max_batch_frames={}", render_slice, render_max.0, render_max.1);
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
    manifest.write_text(replace_once(manifest.read_text(), 'version = "0.2.0"', 'version = "0.2.5"'))
    lock = plugin / "Cargo.lock"
    lock.write_text(replace_once(lock.read_text(), 'name = "fm1-plugin"\nversion = "0.2.0"',
                                'name = "fm1-plugin"\nversion = "0.2.5"'))
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
