import hashlib
from pathlib import Path
import sys
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "tools"))
import private_companion as module


class PrivateCompanionTests(unittest.TestCase):
    def setUp(self):
        self.source = ("bool findPlugin() {\n" + module.START + "\n" + module.END).encode()

    def test_reviewed_source_gets_private_windows_branch_and_is_repeatable(self):
        with patch.object(module, "EXPECTED", hashlib.sha256(self.source).hexdigest()):
            result = module.patch(self.source)
            self.assertEqual(module.patch(result), result)
        windows = result.decode().split("#if SMTG_OS_WINDOWS\n")[1].split("#else")[0]
        self.assertIn("os::getPluginPath().parent_path()", windows)
        self.assertNotIn("getValidCLAPSearchPaths", windows)

    def test_unknown_wrapper_source_is_rejected(self):
        with self.assertRaisesRegex(ValueError, "hash mismatch"):
            module.patch(self.source)

    def test_windows_checkout_line_endings_are_normalized_before_verification(self):
        with patch.object(module, "EXPECTED", hashlib.sha256(self.source).hexdigest()):
            self.assertEqual(module.patch(self.source), module.patch(self.source.replace(b"\n", b"\r\n")))

    def test_modified_patch_is_rejected_instead_of_trusted(self):
        with patch.object(module, "EXPECTED", hashlib.sha256(self.source).hexdigest()):
            changed = module.patch(self.source).replace(b"lib.load(companion)", b"true")
            with self.assertRaisesRegex(ValueError, "hash mismatch"):
                module.patch(changed)
