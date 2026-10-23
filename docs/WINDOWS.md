# Installation and Ableton acceptance (Windows x64)

This is an experimental build, not a validated release. Close Ableton before installation.
The wrapper requires its matching CLAP module, although Live does not load CLAP directly.
Keep both files from the same build and retain the accompanying licence notices.

## Installer

After building, install Inno Setup 6.3+ and run `./tools/package-windows.ps1`
in PowerShell 7 or later. The output is
`dist/installer/Wave-FM-1-0.1.0-Windows-x64-Setup.exe`, accompanied by a SHA-256 file.
The GitHub Actions workflow also packages it and tests installation, VST3 factory
loading and uninstallation on a disposable Windows runner. That workflow has not
yet been executed here; no Windows binary or installer is currently validated.

Run the installer with Ableton closed. It requests administrator access and
installs both modules into the system Common Files VST3 and CLAP folders,
plus documentation, licence notices and an entry in Windows Installed apps.
It bundles Microsoft's Visual C++ x64 runtime (downloaded over HTTPS and checked
for a valid Microsoft Authenticode signature during packaging).
Uninstallation retains the shared runtime and your projects/firmware files.
The experimental installer is unsigned; Windows may show an unknown publisher.
It refuses to replace a pre-existing installation it does not own. Remove or
back up other FM-1 builds, including those in custom plugin folders, first.
For upgrades, the installed module hashes must match the installer's ownership
record. Uninstallation preserves modules replaced outside this installer.

No firmware is bundled. After installation, continue at step 3 below.

## Manual installation alternative

1. Copy `CLAP/FM-1.clap` from the build output to
   `%LOCALAPPDATA%\Programs\Common\CLAP\FM-1.clap` (create the directory if needed).
2. Copy `VST3/FM-1.vst3` to `C:\Program Files\Common Files\VST3\FM-1.vst3`.
   This destination usually requires administrator permission. Alternatively, use
   a writable custom VST3 folder and select it in Live's plug-in preferences.
3. Start Live, enable the relevant VST3 folder in Preferences > Plug-Ins and rescan.
   The instrument retains upstream's name **FM-1**, vendor **fm1-emu**.
   Avoid installing alongside another build of the same plugin identity.
4. Add it to a MIDI track, open its window and use **Load firmware / save…**
   to select a `.fwsc` from the firmware author's release.
5. Enable track monitoring and send notes on the firmware's expected MIDI channel.
   Upstream documents channel 2 for the stock firmware; the plugin can remap channels.

## Required acceptance checks — still pending

- Load firmware A, send MIDI and confirm non-silent output without stuck notes.
- Create a second instance with firmware B; confirm both sound independently.
- Change one instance's controls and verify the other remains unchanged.
- Save patches/settings using each firmware's own Save operation. Allow its flash
  write and upstream's periodic state refresh to complete before saving Live's set.
- Close Live, reopen the set and verify each instance's firmware and saved patches.
- Exercise 44.1 and 48 kHz and buffers of 128, 256 and 512 samples. Record CPU,
  underruns and reported latency; do not assume heavy firmware runs at every setting.
- Render/export a short MIDI clip and verify the output has the expected notes
  and timing; compare with real-time playback.
- Close/reopen plugin windows, replace a firmware while stopped, remove instances,
  and confirm Live remains stable.

Also run Steinberg's VST3 validator against the Windows binary before describing
it as a validated build. Upstream's macOS validator results do not validate this port.

If Live does not list the plugin, first verify the CLAP location and identical
`FM-1` basenames, then check x64 architecture and build/runtime dependency errors.

## Downloading a successful GitHub build

Open the repository's **Actions** tab, select **Windows VST3 prototype** and then
a successful run (green check). Under **Artifacts**, download
`Wave-FM-1-Windows-x64-experimental` and extract it. The installer is under
`installer/` inside the download. A failed or pending run is not a usable release.
GitHub normally requires signing in to download workflow artifacts, and they
expire after 14 days. The project does not publish stable releases automatically.
