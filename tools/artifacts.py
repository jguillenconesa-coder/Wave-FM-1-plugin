"""Record and verify a coherent pair of Windows x64 plugin binaries."""
import argparse
import hashlib
import json
from pathlib import Path
import struct
import subprocess

from bootstrap import REVISION
import firmware

ROOT = Path(__file__).resolve().parents[1]
FILES = ("VST3/FM-1.vst3/Contents/x86_64-win/FM-1.vst3",
         "VST3/FM-1.vst3/Contents/x86_64-win/FM-1.clap")


def digest(path):
    if path.is_symlink() or not path.is_file():
        raise ValueError(f"Missing or linked build input: {path.name}")
    with path.open("rb") as stream:
        header = stream.read(64)
        if len(header) != 64 or header[:2] != b"MZ":
            raise ValueError(f"Not a Windows PE binary: {path.name}")
        stream.seek(struct.unpack_from("<I", header, 60)[0])
        pe = stream.read(26)
        if (len(pe) != 26 or pe[:4] != b"PE\0\0" or
                struct.unpack_from("<H", pe, 4)[0] != 0x8664 or
                struct.unpack_from("<H", pe, 24)[0] != 0x20B):
            raise ValueError(f"Not a Windows x64 PE32+ binary: {path.name}")
        stream.seek(0)
        return hashlib.file_digest(stream, "sha256").hexdigest()


def process(directory, record=False):
    hashes = {name: digest(directory / name) for name in FILES}
    prefix = "VST3/FM-1.vst3/"
    hashes.update({prefix + name: value for name, value in firmware.verify_bundle(directory / prefix).items()})
    actual = {p.relative_to(directory).as_posix() for p in (directory / prefix).rglob("*") if p.is_file()}
    if actual != set(hashes):
        raise ValueError("Unexpected files in VST3 bundle")
    desktop = hashlib.sha256()
    for path in sorted((ROOT / "desktop").rglob("*.rs")) + [ROOT / "tools/prepare-desktop.py"]:
        desktop.update(path.relative_to(ROOT).as_posix().encode())
        desktop.update(path.read_bytes())
    desktop_hash = desktop.hexdigest()
    manifest = directory / "build-info.json"
    if record:
        commit = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip()
        manifest.write_text(json.dumps({"schema": 2, "source_commit": commit,
            "emulator_commit": REVISION, "desktop_sha256": desktop_hash, "sha256": hashes}, indent=2) + "\n", encoding="utf-8")
    else:
        data = json.loads(manifest.read_text(encoding="utf-8"))
        if data.get("schema") != 2 or data.get("desktop_sha256") != desktop_hash or data.get("emulator_commit") != REVISION or data.get("sha256") != hashes:
            raise ValueError("Build manifest mismatch: rebuild both plugin modules together.")
    print("Windows x64 binary pair and SHA-256 manifest verified")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("action", choices=("record", "verify"))
    parser.add_argument("--directory", type=Path, default=ROOT / "dist")
    args = parser.parse_args()
    process(args.directory, record=args.action == "record")
