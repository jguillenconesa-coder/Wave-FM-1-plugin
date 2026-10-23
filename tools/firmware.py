"""Verify and package the offline catalogue and its complete source/licence files."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import shutil
import tarfile

ROOT = Path(__file__).resolve().parents[1]


def sha(path):
    if path.is_symlink() or not path.is_file():
        raise ValueError(f"Missing or linked firmware input: {path}")
    return hashlib.sha256(path.read_bytes()).hexdigest()


def verify_notices(source, directory):
    """Retain every source notice, including texts named MIT-*.txt in LICENSES/."""
    with tarfile.open(source) as archive:
        for member in archive:
            path = Path(member.name)
            if not member.isfile() or not (
                "LICENSES" in (part.upper() for part in path.parts)
                or re.search(r"LICENSE|LICENCE|LICENSING|COPYING|COPYRIGHT|NOTICE|CREDITS|AUTHORS|ATTRIBUTION|OFL", path.name, re.I)
            ):
                continue
            if path.is_absolute() or ".." in path.parts or len(path.parts) < 2:
                raise ValueError(f"Invalid source notice path: {member.name}")
            # All pinned archives have a single named root directory. Read only;
            # never extract a source tree or execute an upstream build here.
            relative = Path(*path.parts[1:])
            with archive.extractfile(member) as stream:
                expected = hashlib.file_digest(stream, "sha256").hexdigest()
            if sha(directory / relative) != expected:
                raise ValueError(f"Original source notice mismatch: {relative}")


def verify_source():
    entries = json.loads((ROOT / "firmware/manifest.json").read_text())["firmwares"]
    ids = set()
    for e in entries:
        if e["id"] in ids or Path(e["file"]).name != e["file"]:
            raise ValueError("Duplicate or invalid firmware entry")
        ids.add(e["id"])
        path = ROOT / "firmware/catalogue" / e["file"]
        if sha(path) != e["sha256"] or path.stat().st_size != e["size"]:
            raise ValueError(f"Firmware hash mismatch: {e['id']}")
        if sha(ROOT / "firmware/sources" / e["source"]) != e["source_sha256"]:
            raise ValueError(f"Corresponding source hash mismatch: {e['id']}")
        for name in ("LICENSE", "LICENSING.md"):
            sha(ROOT / "firmware/licenses" / e["id"] / name)
        verify_notices(ROOT / "firmware/sources" / e["source"], ROOT / "firmware/licenses" / e["id"])
    return entries


def package(bundle, platform):
    verify_source()
    module = bundle / "Contents" / ("PlugIns/FM-1.clap/Contents/Resources" if platform == "macos" else "x86_64-win")
    resources = bundle / "Contents/Resources"
    for src, dst in ((ROOT / "firmware/catalogue", module / "firmwares"),
                     (ROOT / "firmware/sources", resources / "firmware-sources"),
                     (ROOT / "firmware/licenses", resources / "firmware-licenses")):
        if dst.exists():
            shutil.rmtree(dst)
        shutil.copytree(src, dst)
    shutil.copyfile(ROOT / "firmware/manifest.json", resources / "firmware-manifest.json")
    shutil.copyfile(ROOT / "firmware/README.md", resources / "FIRMWARES.md")
    shutil.copyfile(ROOT / "firmware/RESEARCH.md", resources / "RESEARCH.md")
    shutil.copyfile(ROOT / "firmware/community-index.json", resources / "COMMUNITY_INDEX.json")
    for name in ("MIDI_LIVE.md", "COMMUNITY_BROWSER.md", "COMMUNITY_COMPATIBILITY.md"):
        shutil.copyfile(ROOT / "docs" / name, resources / name)
    (resources / "validation").mkdir(exist_ok=True)
    shutil.copyfile(ROOT / "docs/validation/community-2026-10-23.json", resources / "validation/community-2026-10-23.json")


def verify_bundle(bundle, platform="windows"):
    verify_source()
    module = "Contents/PlugIns/FM-1.clap/Contents/Resources" if platform == "macos" else "Contents/x86_64-win"
    expected = {}
    for src, dst in ((ROOT / "firmware/catalogue", module + "/firmwares"),
                     (ROOT / "firmware/sources", "Contents/Resources/firmware-sources"),
                     (ROOT / "firmware/licenses", "Contents/Resources/firmware-licenses")):
        for file in src.rglob("*"):
            if file.is_file():
                expected[f"{dst}/{file.relative_to(src).as_posix()}"] = sha(file)
    expected["Contents/Resources/firmware-manifest.json"] = sha(ROOT / "firmware/manifest.json")
    expected["Contents/Resources/FIRMWARES.md"] = sha(ROOT / "firmware/README.md")
    expected["Contents/Resources/RESEARCH.md"] = sha(ROOT / "firmware/RESEARCH.md")
    expected["Contents/Resources/COMMUNITY_INDEX.json"] = sha(ROOT / "firmware/community-index.json")
    for name in ("MIDI_LIVE.md", "COMMUNITY_BROWSER.md", "COMMUNITY_COMPATIBILITY.md", "validation/community-2026-10-23.json"):
        expected["Contents/Resources/" + name] = sha(ROOT / "docs" / name)
    for name, value in expected.items():
        if sha(bundle / name) != value:
            raise ValueError(f"Packaged firmware/source/licence mismatch: {name}")
    return expected


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--bundle", type=Path)
    parser.add_argument("--platform", choices=("windows", "macos"), default="windows")
    args = parser.parse_args()
    verify_source()
    if args.bundle:
        package(args.bundle, args.platform)
    print("Verified catalogue packages, matching sources and licence notices")
