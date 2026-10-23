# Wave FM-1 Windows Preview 0.1.0

Experimental Windows x64 VST3 around Joakim Eriksson's fm1-emu. Not an official
M-VAVE or Ableton product. No firmware binaries are included.

The installer is per-user and needs no administrator permission. It installs only
under `%LOCALAPPDATA%\Wave FM-1 Preview` and registers an uninstaller in Windows
Installed apps / Add or remove programs. It does not change Ableton's files,
preferences, projects, scan folders or any system plugin folder. The VST3 loads
its private CLAP only. Runtime libraries are statically linked. This preview has
its own VST3 class ID, so it does not replace upstream FM-1 project references.

First test in an empty Set with important projects closed and backed up. If you
already use a custom VST3 folder, do not replace that setting; use a separate
Windows test profile to preserve your preferences. No plugin can promise zero
risk when loaded in a DAW. The preview is unsigned; do not disable antivirus or
SmartScreen. See docs/WINDOWS.md for installation and complete removal.

Validation so far: 47 official VST3 validator tests pass under Wine 10; Windows
firmware/MIDI/audio and portable-state tests pass; packaging integrity tests pass.
Native Ableton Live acceptance on the user's Windows system remains outstanding.
Open dependency-maintenance warnings are recorded in docs/SECURITY_REVIEW.md.

Downloads include an installer, a portable ZIP and SHA-256 hashes. For portable
testing, keep the entire `FM-1.vst3` directory together. The ZIP has no installer
registration; deleting that extracted preview removes the portable copy.
