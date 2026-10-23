#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
export CARGO_HOME=/workspace/.fm1-tools/cargo
export RUSTUP_HOME=/workspace/.fm1-tools/rustup
export PATH="$CARGO_HOME/bin:$PATH"
if [ ! -x "$CARGO_HOME/bin/rustup" ]; then
    installer=$(mktemp /tmp/fm1-rustup.XXXXXX)
    trap 'rm -f "$installer"' EXIT
    curl --proto '=https' --tlsv1.2 --fail --silent --show-error https://sh.rustup.rs -o "$installer"
    sh "$installer" -y --no-modify-path --profile minimal --default-toolchain 1.99.1
fi
rustup toolchain install 1.99.1 --profile minimal
python3 tools/bootstrap.py
cargo test --manifest-path .deps/fm1-emu/Cargo.toml --locked --release -p fm1-core -p fm1-soc -j 2
