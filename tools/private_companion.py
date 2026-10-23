"""Pin-checked wrapper patch: load only the private CLAP in this VST3 bundle."""
import hashlib
from pathlib import Path
import sys

EXPECTED = "f8b5680281e4958389ad56683460fde113dcf8b3d734bc858d8317ed2c1989c7"
START = "  auto parentfolder = os::getParentFolderName();"
END = "  return false;\n}\n\n/*"
LEGACY_START = """#if SMTG_OS_WINDOWS
  // Wave FM-1: private companion only. Never load a globally installed CLAP.
  const auto companion = os::getPluginPath().parent_path() / pluginfilename;
  return fs::exists(companion) && lib.load(companion);
#else
""" + START
PRIVATE_START = LEGACY_START.replace("#else\n", """#elif SMTG_OS_MACOS
  // The CLAP is a private bundle, independent of the VST3's installed name.
  const auto companion = os::getPluginPath() / "Contents" / "PlugIns" / "FM-1.clap";
  return fs::exists(companion) && lib.load(companion);
#else
""")
PRIVATE_END = "  return false;\n#endif\n}\n\n/*"


def patch(data):
    text = data.decode("utf-8").replace("\r\n", "\n")
    original = text.replace(PRIVATE_START, START).replace(LEGACY_START, START).replace(PRIVATE_END, END)
    if hashlib.sha256(original.encode("utf-8")).hexdigest() != EXPECTED:
        raise ValueError("Wrapper source hash mismatch; review the upstream update before patching.")
    if original.count(START) != 1 or original.count(END) != 1:
        raise ValueError("Wrapper patch anchors are ambiguous.")
    return original.replace(START, PRIVATE_START).replace(END, PRIVATE_END).encode("utf-8")


if __name__ == "__main__":
    path = Path(sys.argv[1]) / "src/wrapasvst3_entry.cpp"
    path.write_bytes(patch(path.read_bytes()))
    print("Verified wrapper revision and enabled private Windows/macOS companion loading")
