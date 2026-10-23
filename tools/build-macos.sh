#!/usr/bin/env bash
# Native macOS only: Xcode command-line tools, CMake >=3.27, Ninja, Python 3,
# and Rust 1.99.1. Produces a Universal 2 VST3 with a private Universal 2 CLAP.
set -euo pipefail
cd "$(dirname "$0")/.."
[[ "$(uname -s)" == Darwin ]] || { echo 'Run this script on macOS.' >&2; exit 1; }
export MACOSX_DEPLOYMENT_TARGET=11.0
export CARGO_TARGET_DIR="$PWD/build/macos-rust"
jobs=${FM1_BUILD_JOBS:-3}
python3 tools/prepare-desktop.py
rustup target add aarch64-apple-darwin x86_64-apple-darwin
for target in aarch64-apple-darwin x86_64-apple-darwin; do
    cargo build --manifest-path .deps/fm1-desktop/crates/fm1-plugin/Cargo.toml \
        --release --locked --target "$target" -j "$jobs"
done
cmake -S . -B build/macos -G Ninja -DCMAKE_BUILD_TYPE=Release \
    '-DCMAKE_OSX_ARCHITECTURES=arm64;x86_64' -DCMAKE_OSX_DEPLOYMENT_TARGET=11.0
cmake --build build/macos --target wave_fm1 --parallel "$jobs"
out="$PWD/dist/macos"
mkdir -p "$out/VST3" "$out/licenses"
bundle="$out/VST3/Wave FM-1.vst3"
if [[ -d "$bundle" ]]; then rm -rf "$bundle"; fi
ditto build/macos/bundle/FM-1.vst3 "$bundle"
clap="$bundle/Contents/PlugIns/FM-1.clap"
mkdir -p "$clap/Contents/MacOS"
lipo -create "$CARGO_TARGET_DIR/aarch64-apple-darwin/release/libfm1_plugin.dylib" \
    "$CARGO_TARGET_DIR/x86_64-apple-darwin/release/libfm1_plugin.dylib" -output "$clap/Contents/MacOS/FM-1"
python3 - "$clap" <<'PY'
import plistlib, sys
from pathlib import Path
plist = {'CFBundleExecutable': 'FM-1', 'CFBundleIdentifier': 'io.github.jguillenconesa.wave-fm1.clap',
         'CFBundleName': 'FM-1', 'CFBundlePackageType': 'BNDL', 'CFBundleVersion': '0.2.5',
         'CFBundleShortVersionString': '0.2.5', 'LSMinimumSystemVersion': '11.0'}
with (Path(sys.argv[1]) / 'Contents/Info.plist').open('wb') as f: plistlib.dump(plist, f)
PY
python3 tools/firmware.py --platform macos --bundle "$bundle"
cp LICENSE THIRD_PARTY_NOTICES.md AUTHORS.md "$out/licenses/"
cp .deps/fm1-emu/LICENSE "$out/licenses/fm1-emu-MIT.txt"
python3 tools/collect-rust-licenses.py --target aarch64-apple-darwin --output "$out/licenses/rust-arm64"
python3 tools/collect-rust-licenses.py --target x86_64-apple-darwin --output "$out/licenses/rust-x86_64"
python3 - <<'PY'
from pathlib import Path
import re, shutil
base = Path('build/macos')
for p in base.rglob('*'):
    if p.is_file() and re.match(r'^(LICENSE|LICENCE|COPYING|NOTICE)(\..*)?$', p.name, re.I):
        dst = Path('dist/macos/licenses/third-party') / p.relative_to(base)
        dst.parent.mkdir(parents=True, exist_ok=True); shutil.copyfile(p, dst)
PY
ditto "$out/licenses" "$bundle/Contents/Resources/licenses"
cp docs/MACOS.md "$out/INSTALL.md"
# Local ad-hoc integrity signatures, never described as Developer ID/notarization.
codesign --force --sign - "$clap"
codesign --force --sign - "$bundle"
codesign --verify --deep --strict "$bundle"
lipo "$bundle/Contents/MacOS/FM-1" -verify_arch arm64 x86_64
lipo "$clap/Contents/MacOS/FM-1" -verify_arch arm64 x86_64
python3 tools/macos-artifacts.py record
echo 'Built Universal 2 VST3. Native validation must pass before packaging.'
