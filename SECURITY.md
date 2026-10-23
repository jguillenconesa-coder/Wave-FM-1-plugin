# Security policy

This is an experimental, unaudited plugin port. There is no supported stable
release yet. The current development revision is the only maintained version.

## Trust boundaries

- Load only firmware, shared saves and Live projects from sources you trust.
  The emulator runs in the DAW's process with the DAW's permissions. It is not
  a sandbox. A parser or native JIT bug can crash or compromise that process.
- Upstream's Windows JIT uses executable, writable memory. Firmware hashes
  detect corruption; they do not prove who authored a firmware or that it is safe.
- Use Ableton as an ordinary user. Only the machine-wide installer needs elevation.
  Back up projects before testing. Multiple heavy instances may exhaust CPU.
- Build only trusted source checkouts on disposable runners. Cargo build scripts,
  procedural macros, CMake dependencies and installer compilers execute code.
- Do not add tokens, signing keys, `.env` files or local firmware to the repository.
  Do not disable TLS verification, signature checks or dependency checksums.
- An unsigned installer may show an unknown publisher. A SHA-256 sidecar proves
  integrity against a separately trusted digest, not publisher identity.

## Reporting a vulnerability

Use GitHub's **Security > Advisories > Report a vulnerability** if private
reporting is available for this repository. If it is unavailable, open an issue
asking the maintainer for a private reporting channel, without exploit details,
personal data, credentials or sensitive project files. Do not send secrets in issues.
Problems in the emulator or other dependencies should also be reported privately
to their upstream maintainers using their policies.

## Maintenance and release expectations

Run `cargo audit` against both upstream lockfiles before distributing a new
build. Review informational findings as well as hard errors; a clean advisory
scan is not proof that a dependency has no flaws. Keep immutable dependency and
Actions pins current, review licence changes, and run the Windows installer
smoke test plus a VST3 validator and the Ableton acceptance checklist.
Do not publish a stable installer until those checks have completed.

See [the recorded review](docs/SECURITY_REVIEW.md) for the actual scope and
remaining limitations. Repository security features such as private reporting,
secret scanning and branch protection require separate GitHub settings; this
file does not turn them on.
