#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
version=0.2.5
python3 tools/macos-artifacts.py verify
codesign --verify --deep --strict 'dist/macos/VST3/Wave FM-1.vst3'
mkdir -p dist/macos-downloads
ditto -c -k --sequesterRsrc --keepParent dist/macos "dist/macos-downloads/Wave-FM-1-$version-macOS-Universal.zip"
mkdir -p build/macos-pkg-root/Library/Audio/Plug-Ins/VST3
ditto 'dist/macos/VST3/Wave FM-1.vst3' 'build/macos-pkg-root/Library/Audio/Plug-Ins/VST3/Wave FM-1.vst3'
# No pre/postinstall scripts, no host changes, no network, no shared runtimes.
pkgbuild --root build/macos-pkg-root --install-location / \
    --identifier io.github.jguillenconesa.wave-fm1.preview --version "$version" \
    "dist/macos-downloads/Wave-FM-1-$version-macOS-Universal.pkg"
cp dist/macos/build-info.json dist/macos-downloads/build-info-macos.json
cd dist/macos-downloads
shasum -a 256 ./*.zip ./*.pkg > SHA256SUMS-macos
