"""Create a complete Windows portable ZIP and checksums after build verification."""
import hashlib
from pathlib import Path
import shutil
import zipfile
import artifacts

ROOT = Path(__file__).resolve().parents[1]
DIST = ROOT / "dist"
VERSION = "0.2.5"
artifacts.process(DIST)
output = DIST / "installer"
setup = output / f"Wave-FM-1-{VERSION}-Windows-x64-Setup.exe"
if not setup.is_file():
    raise SystemExit("Build the installer first")
portable = output / f"Wave-FM-1-{VERSION}-Windows-x64-Portable.zip"
with zipfile.ZipFile(portable, "w", compression=zipfile.ZIP_DEFLATED, compresslevel=6) as archive:
    for folder in (DIST / "VST3", DIST / "licenses"):
        for path in sorted(folder.rglob("*")):
            if path.is_file():
                if path.is_symlink():
                    raise SystemExit("Refusing linked package input")
                archive.write(path, path.relative_to(DIST).as_posix())
    for path in (DIST / "INSTALL.md", DIST / "build-info.json", DIST / "REALTIME_X0X.txt"):
        archive.write(path, path.name)
checksums = []
for path in (setup, portable):
    with path.open("rb") as stream:
        checksums.append(f"{hashlib.file_digest(stream, 'sha256').hexdigest()}  {path.name}")
(output / "SHA256SUMS").write_text("\n".join(checksums) + "\n", encoding="ascii")
shutil.copyfile(DIST / "build-info.json", output / "build-info-windows.json")
print("Created complete portable ZIP and release checksums")
