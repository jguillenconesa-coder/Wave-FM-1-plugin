"""Collect source notices and SPDX metadata for the selected Cargo dependency graph."""
import json
import argparse
from pathlib import Path
import re
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[1]
PATTERN = re.compile(r"^(licen[cs]e|copying|copyright|notice|ofl|ufl)([._-]|$)", re.I)
SUPPLEMENTAL = {"ecolor": "egui-MIT.txt", "egui": "egui-MIT.txt", "egui_glow": "egui-MIT.txt",
    "emath": "egui-MIT.txt", "epaint": "egui-MIT.txt", "profiling": "profiling-MIT.txt",
    "clipboard-win": "clipboard-win-BSL-1.0.txt", "winapi-x86_64-pc-windows-gnu": "winapi-MIT.txt"}


def main(target="x86_64-pc-windows-msvc", output=None):
    manifest = ROOT / ".deps/fm1-emu/crates/fm1-plugin/Cargo.toml"
    metadata = json.loads(subprocess.check_output(["cargo", "metadata", "--locked",
        "--manifest-path", str(manifest), "--filter-platform", target,
        "--format-version", "1"], text=True))
    out = output or ROOT / "dist/licenses/rust"
    out.mkdir(parents=True, exist_ok=True)
    inventory = []
    for package in sorted(metadata["packages"], key=lambda p: (p["name"], p["version"])):
        if not package["license"]:
            raise ValueError(f"Missing licence declaration: {package['name']}")
        base = Path(package["manifest_path"]).parent
        notices = [p for p in base.rglob("*") if p.is_file() and (PATTERN.match(p.name)
            or ("fonts" in p.relative_to(base).parts and p.suffix.lower() == ".txt"))
            and "target" not in p.relative_to(base).parts and ".git" not in p.relative_to(base).parts]
        # Workspace subcrates (fm1-emu and nih-plug) share the root licence.
        parent = base
        while not notices and parent.parent != parent:
            if (parent / ".git").exists():
                notices = [p for p in parent.iterdir() if p.is_file() and PATTERN.match(p.name)]
                break
            parent = parent.parent
        if not notices and package["name"] in SUPPLEMENTAL:
            notices = [ROOT / "LICENSES" / SUPPLEMENTAL[package["name"]]]
        if not notices:
            raise ValueError(f"Missing original licence text: {package['name']} {package['version']}")
        folder = out / f"{package['name']}-{package['version']}"
        folder.mkdir(parents=True, exist_ok=True)
        for i, path in enumerate(sorted(notices)):
            shutil.copyfile(path, folder / f"{i:02d}-{path.name}")
        inventory.append({k: package[k] for k in ("name", "version", "license", "authors", "repository", "source")})
    (out / "inventory.json").write_text(json.dumps(inventory, indent=2) + "\n", encoding="utf-8")
    print(f"Collected licence notices for {len(inventory)} {target} Rust packages")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--target", choices=("x86_64-pc-windows-msvc", "x86_64-pc-windows-gnu", "aarch64-apple-darwin", "x86_64-apple-darwin"), default="x86_64-pc-windows-msvc")
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    main(args.target, args.output)
