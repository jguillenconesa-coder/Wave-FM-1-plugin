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
    for name in ("editor.rs", "catalogue.rs", "skins.rs", "controls.rs", "online.rs"):
        shutil.copyfile(ROOT / "desktop/src" / name, plugin / "src" / name)
    shutil.copyfile(ROOT / "firmware/community-index.json", plugin / "src/community-index.json")
    shutil.copyfile(ROOT / "desktop/src/panel.rs", DEST / "crates/fm1-live/src/panel.rs")
    shutil.copyfile(ROOT / "desktop/tests/catalogue.rs", plugin / "tests/catalogue.rs")
    (plugin / "examples").mkdir(exist_ok=True)
    shutil.copyfile(ROOT / "desktop/examples/check-community.rs", plugin / "examples/check-community.rs")
    lib = plugin / "src/lib.rs"
    text = lib.read_text()
    text = "// Desktop additions: firmware catalogue and per-instance skin. MIT; see desktop/README.md.\n" + text
    text = text.replace("mod editor;", "mod editor;\npub mod catalogue;\nmod skins;\nmod controls;\nmod online;")
    text = text.replace('#[persist = "flash"]', '#[persist = "skin"]\n    pub skin: Mutex<String>,\n    #[persist = "flash"]')
    text = text.replace("firmware: Mutex::new(Firmware::default()),", 'firmware: Mutex::new(Firmware::default()),\n            skin: Mutex::new("graphite".into()),')
    text = replace_once(text, '#[id = "master"]', '#[nested]\n    pub controls: controls::PanelParams,\n    #[id = "master"]')
    text = replace_once(text, 'master: FloatParam::new(', 'controls: controls::PanelParams::default(),\n            master: FloatParam::new(')
    text = replace_once(text, '        let link = self.start();\n        self.ratio = RATE / cfg.sample_rate as f64;',
                        '        self.params.controls.resync();\n        let link = self.start();\n        self.ratio = RATE / cfg.sample_rate as f64;')
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
                        '    let mut controls = crate::controls::Routing::new(params.controls.snapshot());\n    let mut m: Option<Machine> = None;')
    text = replace_once(text, '                    active_pkg = m.as_ref().map(|_| pkg);',
                        '                    controls = crate::controls::Routing::new(params.controls.snapshot());\n                    active_pkg = m.as_ref().map(|_| pkg);')
    text = replace_once(text, 'Cmd::Key { id, down } => apply(&mut m, Action::Key { id, down }),',
                        '''Cmd::Key { id, down } => {
                    if m.as_ref().is_some_and(|m| m.soc.board == Board::Fm1) {
                        controls.manual_key(id, down, |e| panel_event(&mut m, e));
                    } else { apply(&mut m, Action::Key { id, down }); }
                }''')
    text = replace_once(text, '                    m = Some(next);',
                        '                    controls = crate::controls::Routing::new(params.controls.snapshot());\n                    m = Some(next);')
    text = replace_once(text, '        snapshots(&mut pending, m.as_ref(), &params, Instant::now());', '''        if m.as_ref().is_some_and(|m| m.soc.board == Board::Fm1) {
            controls.poll(params.controls.snapshot(), Instant::now(), |e| panel_event(&mut m, e));
        } else {
            // FM-1 control IDs must never be applied to an imported SMK-37 firmware.
            controls = crate::controls::Routing::new(params.controls.snapshot());
        }
        snapshots(&mut pending, m.as_ref(), &params, Instant::now());''')
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
    # Only this integration's plugin version changes; dependency pins remain intact.
    manifest = plugin / "Cargo.toml"
    manifest.write_text(replace_once(manifest.read_text(), 'version = "0.2.0"', 'version = "0.2.1"'))
    lock = plugin / "Cargo.lock"
    lock.write_text(replace_once(lock.read_text(), 'name = "fm1-plugin"\nversion = "0.2.0"',
                                'name = "fm1-plugin"\nversion = "0.2.1"'))
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
