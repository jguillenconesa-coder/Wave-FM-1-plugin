from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "tools"))
import bootstrap


class DependencyIntegrityTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.source = self.root / "upstream"
        subprocess.run(["git", "init", "-q", str(self.source)], check=True)
        (self.source / "source.txt").write_text("original")
        self.git("add", "source.txt")
        self.git("-c", "user.name=Test", "-c", "user.email=test@example.invalid", "commit", "-qm", "fixture")
        self.revision = self.git("rev-parse", "HEAD").strip()

    def git(self, *args):
        return subprocess.check_output(["git", "-C", str(self.source), *args], text=True)

    def verify(self, revision=None):
        with patch.object(bootstrap, "SOURCE", self.source), patch.object(bootstrap, "REVISION", revision or self.revision):
            bootstrap.main()

    def test_unchanged_source_is_accepted(self):
        self.verify()

    def test_changed_revision_is_rejected(self):
        with self.assertRaisesRegex(SystemExit, "revision"):
            self.verify("0" * 40)

    def test_modified_source_is_preserved_and_rejected(self):
        path = self.source / "source.txt"
        path.write_text("local work")
        with self.assertRaisesRegex(SystemExit, "local changes"):
            self.verify()
        self.assertEqual(path.read_text(), "local work")

    def test_extra_source_is_rejected(self):
        (self.source / "build.rs").write_text("unexpected build script")
        with self.assertRaisesRegex(SystemExit, "extra source"):
            self.verify()
