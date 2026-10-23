"""Prepare our MIT desktop UI fork beside the unchanged, pinned emulator."""
import hashlib
import json
from pathlib import Path
import shutil

from bootstrap import ROOT, SOURCE, main as verify_upstream

DEST = ROOT / ".deps/fm1-desktop"
BASE_HASHES = {
    "crates/fm1-plugin/src/lib.rs": "bacecec087841b1a589f09a0751fd51f336746217d37218c5bcb95c25cc1fbb6",
    "crates/fm1-plugin/src/editor.rs": "cee9c6c3036c9ab62505ec5bbe36217c9e932c0a4d14bd46bc871642b16e2df3",
    "crates/fm1-live/src/panel.rs": "8ea12e3383cca39ed8ace90bb04f465e6bff174e010a23899fec0e8e5985f931",
}


def main():
    verify_upstream()
    for name, expected in BASE_HASHES.items():
        if hashlib.sha256((SOURCE / name).read_bytes()).hexdigest() != expected:
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
    for name in ("editor.rs", "catalogue.rs", "skins.rs"):
        shutil.copyfile(ROOT / "desktop/src" / name, plugin / "src" / name)
    shutil.copyfile(ROOT / "desktop/src/panel.rs", DEST / "crates/fm1-live/src/panel.rs")
    shutil.copyfile(ROOT / "desktop/tests/catalogue.rs", plugin / "tests/catalogue.rs")
    lib = plugin / "src/lib.rs"
    text = lib.read_text()
    text = "// Desktop additions: firmware catalogue and per-instance skin. MIT; see desktop/README.md.\n" + text
    text = text.replace("mod editor;", "mod editor;\npub mod catalogue;\nmod skins;")
    text = text.replace('#[persist = "flash"]', '#[persist = "skin"]\n    pub skin: Mutex<String>,\n    #[persist = "flash"]')
    text = text.replace("firmware: Mutex::new(Firmware::default()),", 'firmware: Mutex::new(Firmware::default()),\n            skin: Mutex::new("graphite".into()),')
    lib.write_text(text)
    entries = json.loads((ROOT / "firmware/manifest.json").read_text())["firmwares"]
    generated = "// Generated from firmware/manifest.json. Do not edit.\npub static ENTRIES: &[Entry] = &[\n"
    for e in entries:
        fields = ("id", "name", "version", "author", "license", "description", "file", "sha256")
        generated += "    Entry { " + ", ".join(f"{k}: {json.dumps(e[k], ensure_ascii=False)}" for k in fields)
        generated += f", size: {e['size']} }},\n"
    (plugin / "src/catalogue_entries.rs").write_text(generated + "];\n")
    print("Prepared desktop fork; pinned emulator sources remain unchanged")


if __name__ == "__main__":
    main()
