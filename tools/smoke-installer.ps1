#requires -Version 7.0
# For a disposable Windows CI runner only: installs and then uninstalls the preview.
param([string]$Version = '0.2.8')
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
if ($env:GITHUB_ACTIONS -ne 'true' -or $env:RUNNER_OS -ne 'Windows') {
    throw 'Run this installation test only on a disposable Windows GitHub Actions runner.'
}
$Root = Split-Path $PSScriptRoot -Parent
$Installer = Join-Path $Root "dist/installer/Wave-FM-1-$Version-Windows-x64-Setup.exe"
$Process = Start-Process $Installer -ArgumentList '/VERYSILENT /SUPPRESSMSGBOXES /NORESTART /SP-' -Wait -PassThru
if ($Process.ExitCode -notin @(0, 3010)) { throw "Installer failed: $($Process.ExitCode)" }
try {
    $App = Join-Path ${env:CommonProgramFiles} 'VST3/Wave FM-1.vst3'
    $Vst = Join-Path $App 'Contents/x86_64-win/FM-1.vst3'
    $Clap = Join-Path $App 'Contents/x86_64-win/FM-1.clap'
    $Registration = 'HKLM:\Software\Microsoft\Windows\CurrentVersion\Uninstall\{E45CB310-7AAF-4D78-9093-FAC036548235}_is1'
    if ((Get-ItemProperty $Registration).DisplayName -ne 'Wave FM-1 Preview') { throw 'Missing Installed apps registration.' }
    foreach ($File in @($Vst, $Clap)) {
        if (-not (Test-Path $File -PathType Leaf)) { throw "Installer did not create $File" }
    }
    # GetPluginFactory discovers and loads the companion CLAP, checking the
    # wrapper name, search location, exported factory and runtime dependencies.
    @'
import ctypes, os
path = os.path.join(os.environ['CommonProgramFiles'], 'VST3', 'Wave FM-1.vst3', 'Contents', 'x86_64-win', 'FM-1.vst3')
plugin = ctypes.WinDLL(path)
plugin.InitDll.restype = ctypes.c_bool
plugin.InitDll.argtypes = []
plugin.ExitDll.restype = ctypes.c_bool
plugin.ExitDll.argtypes = []
assert plugin.InitDll(), 'VST3 module initialization failed'
factory = plugin.GetPluginFactory
factory.restype = ctypes.c_void_p
factory.argtypes = []
pointer = factory()
assert pointer, 'VST3 factory did not discover the installed CLAP'
vtable = ctypes.cast(pointer, ctypes.POINTER(ctypes.POINTER(ctypes.c_void_p))).contents
release = ctypes.WINFUNCTYPE(ctypes.c_uint32, ctypes.c_void_p)(vtable[2])
release(pointer)
assert plugin.ExitDll(), 'VST3 module shutdown failed'
print('Installed VST3 factory loaded successfully')
'@ | python -
    if ($LASTEXITCODE -ne 0) { throw 'Installed VST3 failed factory smoke check.' }
} finally {
    $Uninstaller = Join-Path $App 'unins000.exe'
    $Process = Start-Process $Uninstaller -ArgumentList '/VERYSILENT /SUPPRESSMSGBOXES /NORESTART' -Wait -PassThru
    if ($Process.ExitCode -ne 0) { throw "Uninstaller failed: $($Process.ExitCode)" }
}
foreach ($File in @($Vst, $Clap)) {
    if (Test-Path $File) { throw "Uninstaller left plugin file: $File" }
}
if (Test-Path $App) { throw 'Uninstaller left the private application directory.' }
if (Test-Path $Registration) { throw 'Uninstaller left its Installed apps registration.' }
