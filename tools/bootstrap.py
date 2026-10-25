"""Fetch the exact upstream emulator without modifying its sources."""
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / ".deps" / "fm1-emu"
REVISION = "ba11e2e68f3934a2c5c98a0eb55607387e262a49"
URL = "https://github.com/joakimeriksson/fm1-emu.git"


def run(*args):
    subprocess.run(args, check=True)


def main():
    if SOURCE.is_symlink():
        raise SystemExit("Dependency checkout must not be a symlink.")
    if not SOURCE.exists():
        SOURCE.parent.mkdir(parents=True, exist_ok=True)
        run("git", "clone", "--no-checkout", URL, str(SOURCE))
        run("git", "-C", str(SOURCE), "checkout", "--detach", REVISION)
    head = subprocess.check_output(
        ["git", "-C", str(SOURCE), "rev-parse", "HEAD"], text=True
    ).strip()
    if head != REVISION:
        raise SystemExit("Upstream checkout differs from the pinned revision; preserve your work before replacing it.")
    changes = subprocess.check_output(
        ["git", "-C", str(SOURCE), "status", "--porcelain", "--untracked-files=all"], text=True
    )
    if changes:
        raise SystemExit("Upstream has local changes or extra source files; refusing to build a modified dependency.")
    print(f"Verified fm1-emu {head}")


if __name__ == "__main__":
    main()
