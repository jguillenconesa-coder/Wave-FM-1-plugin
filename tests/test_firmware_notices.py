import io
from pathlib import Path
import sys
import tarfile
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "tools"))
import firmware


class FirmwareNoticeTests(unittest.TestCase):
    def test_missing_nested_licence_and_changed_credit_are_rejected(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            source = root / "source.tar.gz"
            notices = {"LICENSE": b"GPL notice", "LICENSES/MIT-DSP.txt": b"DSP author and MIT text",
                       "assets/samples/CREDITS.txt": b"Sample author and terms"}
            with tarfile.open(source, "w:gz") as archive:
                for name, contents in notices.items():
                    member = tarfile.TarInfo("upstream-version/" + name)
                    member.size = len(contents)
                    archive.addfile(member, io.BytesIO(contents))
            destination = root / "notices"
            for name in ("LICENSE", "assets/samples/CREDITS.txt"):
                path = destination / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_bytes(notices[name])
            with self.assertRaisesRegex(ValueError, "MIT-DSP"):
                firmware.verify_notices(source, destination)
            path = destination / "LICENSES/MIT-DSP.txt"
            path.parent.mkdir()
            path.write_bytes(notices["LICENSES/MIT-DSP.txt"])
            firmware.verify_notices(source, destination)
            (destination / "assets/samples/CREDITS.txt").write_bytes(b"different credit")
            with self.assertRaisesRegex(ValueError, "CREDITS"):
                firmware.verify_notices(source, destination)
