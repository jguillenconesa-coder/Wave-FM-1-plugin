# Contributing

Use a branch and a pull request for subsequent changes. Explain the user-visible
problem, the resulting behavior, validation and remaining limitations. Keep
changes focused and do not commit build caches, credentials or firmware.

The initial repository is a Windows packaging prototype around third-party code.
Preserve upstream copyright notices and cite sources for any reused work.
Updating a dependency requires reviewing its changes, security advisories and
licences. Do not silently change the pinned emulator or edit its downloaded
checkout; propose a deliberate update or a documented source patch instead.

Local checks:

```sh
python3 -m unittest discover -s tests -v
python3 tools/bootstrap.py
cargo test --manifest-path .deps/fm1-emu/Cargo.toml --locked --release -p fm1-core -p fm1-soc -j 2
```

On Windows, run `tools/build-windows.ps1` and `tools/package-windows.ps1` in
PowerShell 7 with the documented prerequisites. Installation tests are confined
to disposable CI machines. Real Ableton testing is still necessary.

Report security issues using SECURITY.md instead of including an exploit in a PR.
