import importlib.util
import json
from pathlib import Path
import struct
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "tools"))
import artifacts


def pe(machine=0x8664):
    data = bytearray(160)
    data[:2] = b"MZ"
    struct.pack_into("<I", data, 60, 64)
    data[64:68] = b"PE\0\0"
    struct.pack_into("<H", data, 68, machine)
    struct.pack_into("<H", data, 88, 0x20B)
    return data


class BuildIntegrityTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        for name in artifacts.FILES:
            file = self.root / name
            file.parent.mkdir(parents=True)
            file.write_bytes(pe())

    def record(self):
        with patch.object(artifacts.subprocess, "check_output", return_value="a" * 40):
            artifacts.process(self.root, record=True)

    def test_recorded_pair_verifies_but_changed_binary_is_rejected(self):
        self.record()
        artifacts.process(self.root)
        file = self.root / artifacts.FILES[1]
        file.write_bytes(pe() + b"replaced")
        with self.assertRaisesRegex(ValueError, "mismatch"):
            artifacts.process(self.root)

    def test_32_bit_binary_is_rejected(self):
        (self.root / artifacts.FILES[0]).write_bytes(pe(0x14C))
        with self.assertRaisesRegex(ValueError, "x64"):
            artifacts.process(self.root, record=True)

    def test_missing_companion_is_rejected(self):
        (self.root / artifacts.FILES[1]).unlink()
        with self.assertRaisesRegex(ValueError, "Missing"):
            artifacts.process(self.root, record=True)

    def test_truncated_binary_is_rejected(self):
        (self.root / artifacts.FILES[0]).write_bytes(b"MZ")
        with self.assertRaisesRegex(ValueError, "PE binary"):
            artifacts.process(self.root, record=True)

    def test_manifest_cannot_substitute_another_emulator_revision(self):
        self.record()
        path = self.root / "build-info.json"
        data = json.loads(path.read_text())
        data["emulator_commit"] = "0" * 40
        path.write_text(json.dumps(data))
        with self.assertRaisesRegex(ValueError, "mismatch"):
            artifacts.process(self.root)
