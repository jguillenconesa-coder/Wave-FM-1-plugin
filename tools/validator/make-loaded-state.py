"""Create an isolated NIH-plug state fixture from an unchanged bundled firmware.

For testing the actual VST3 callback, not a user preset migration or distribution.
The original firmware's authorship/licence remains in firmware/manifest.json.
"""
import argparse
import base64
import hashlib
import json
import struct
from pathlib import Path

root = Path(__file__).resolve().parents[2]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("firmware", choices=("dub", "dub13", "x0x"))
parser.add_argument("output", type=Path)
args = parser.parse_args()
entry = next(e for e in json.loads((root / "firmware/manifest.json").read_text())["firmwares"]
             if e["id"] == args.firmware)
package = (root / "firmware/catalogue" / entry["file"]).read_bytes()
if len(package) != entry["size"] or hashlib.sha256(package).hexdigest() != entry["sha256"]:
    raise SystemExit("Bundled firmware integrity mismatch")
state = {
    "version": "0.2.5",
    "params": {"master": {"f32": 900 / 1023}, "clock": {"i32": 0},
               "channel": {"i32": 0}, "transport_sync": {"i32": 0}},
    "fields": {
        "firmware": json.dumps({"name": f"{entry['name']} {entry['version']}",
                                "pkg": base64.b64encode(package).decode("ascii")}),
        "flash": "[]", "skin": '"graphite"',
    },
}
args.output.parent.mkdir(parents=True, exist_ok=True)
encoded = json.dumps(state).encode("utf-8")
# NIH-plug's CLAP state stream prefixes JSON with its 64-bit little-endian size.
args.output.write_bytes(struct.pack("<Q", len(encoded)) + encoded)
print(f"Created isolated state for {entry['name']} {entry['version']}; package hash verified")
