#!/usr/bin/env bash
# Linux cross-build: Rust 1.99.1, MinGW-w64 POSIX, CMake >=3.24 and Ninja.
set -euo pipefail
cd "$(dirname "$0")/.."
jobs=${FM1_BUILD_JOBS:-2}
python3 tools/prepare-desktop.py
rm -f dist/build-info.json
rustup target add x86_64-pc-windows-gnu
export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc-posix
export RUSTFLAGS="${RUSTFLAGS:-} -C target-feature=+crt-static"
export CARGO_TARGET_DIR="$PWD/.deps/fm1-emu/crates/fm1-plugin/target"
cargo build --manifest-path .deps/fm1-desktop/crates/fm1-plugin/Cargo.toml \
    --release --locked --target x86_64-pc-windows-gnu -j "$jobs"
cmake -S . -B build/cross-windows -G Ninja \
    -DCMAKE_TOOLCHAIN_FILE=cmake/windows-mingw.cmake -DCMAKE_BUILD_TYPE=Release
cmake --build build/cross-windows --target wave_fm1 --parallel "$jobs"
cmake --install build/cross-windows --prefix "$PWD/dist"
mkdir -p dist/VST3/FM-1.vst3/Contents/x86_64-win dist/licenses
cp .deps/fm1-emu/crates/fm1-plugin/target/x86_64-pc-windows-gnu/release/fm1_plugin.dll dist/VST3/FM-1.vst3/Contents/x86_64-win/FM-1.clap
cp .deps/fm1-emu/LICENSE dist/licenses/fm1-emu-MIT.txt
cp LICENSE dist/licenses/packaging-MIT.txt
cp THIRD_PARTY_NOTICES.md dist/licenses/
mkdir -p dist/licenses/toolchain
cp /usr/share/doc/gcc-mingw-w64-base/copyright dist/licenses/toolchain/GCC-runtime-copyright.txt
cp /usr/share/doc/mingw-w64-common/copyright dist/licenses/toolchain/MinGW-copyright.txt
cp /usr/share/common-licenses/GPL-3 dist/licenses/toolchain/GPL-3.txt
python3 tools/collect-rust-licenses.py --target x86_64-pc-windows-gnu
python3 - <<'PY'
from pathlib import Path
import re, shutil
root = Path('build/cross-windows')
for p in root.rglob('*'):
    if p.is_file() and re.match(r'^(LICENSE|LICENCE|COPYING|NOTICE)(\..*)?$', p.name, re.I):
        dst = Path('dist/licenses/third-party') / p.relative_to(root)
        dst.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(p, dst)
PY
cp docs/WINDOWS.md dist/INSTALL.md
python3 tools/firmware.py --bundle dist/VST3/FM-1.vst3
python3 tools/artifacts.py record
