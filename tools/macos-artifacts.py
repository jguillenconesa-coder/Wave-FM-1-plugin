"""Record/verify the complete macOS bundle, its architectures, sources and licences."""
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import firmware
from bootstrap import ROOT, REVISION

directory = ROOT / "dist/macos"
bundle = directory / "VST3/Wave FM-1.vst3"
firmware.verify_bundle(bundle, "macos")
for name in ("Contents/MacOS/FM-1", "Contents/PlugIns/FM-1.clap/Contents/MacOS/FM-1"):
    subprocess.run(["lipo", str(bundle / name), "-verify_arch", "arm64", "x86_64"], check=True)
hashes = {p.relative_to(bundle).as_posix(): firmware.sha(p) for p in sorted(bundle.rglob("*")) if p.is_file()}
manifest = directory / "build-info.json"
if sys.argv[1] == "record":
    data = {"schema": 1, "version": "0.2.1", "architectures": ["arm64", "x86_64"],
            "minimum_macos": "11.0", "emulator_commit": REVISION,
            "source_commit": subprocess.check_output(["git", "rev-parse", "HEAD"], text=True).strip(), "sha256": hashes}
    manifest.write_text(json.dumps(data, indent=2) + "\n")
else:
    data = json.loads(manifest.read_text())
    if data["sha256"] != hashes or data["emulator_commit"] != REVISION:
        raise SystemExit("macOS bundle changed after verification")
print("macOS bundle, firmware sources, notices and architectures verified")
