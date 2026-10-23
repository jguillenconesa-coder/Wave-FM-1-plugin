#requires -Version 7.0
# For a disposable Windows CI runner only: installs and then uninstalls the package.
param([string]$Version = '0.1.0')
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
if ($env:GITHUB_ACTIONS -ne 'true' -or $env:RUNNER_OS -ne 'Windows') {
    throw 'Run this machine-wide installation test only on a disposable Windows GitHub Actions runner.'
}
$Root = Split-Path $PSScriptRoot -Parent
$Installer = Join-Path $Root "dist/installer/Wave-FM-1-$Version-Windows-x64-Setup.exe"
$Process = Start-Process $Installer -ArgumentList '/VERYSILENT /SUPPRESSMSGBOXES /NORESTART /SP-' -Wait -PassThru
if ($Process.ExitCode -notin @(0, 3010)) { throw "Installer failed: $($Process.ExitCode)" }
try {
    $Vst = Join-Path $env:CommonProgramFiles 'VST3/FM-1.vst3'
    $Clap = Join-Path $env:CommonProgramFiles 'CLAP/FM-1.clap'
    foreach ($File in @($Vst, $Clap)) {
        if (-not (Test-Path $File -PathType Leaf)) { throw "Installer did not create $File" }
    }
    # GetPluginFactory discovers and loads the companion CLAP, checking the
    # wrapper name, search location, exported factory and runtime dependencies.
    @'
import ctypes, os
path = os.path.join(os.environ['CommonProgramFiles'], 'VST3', 'FM-1.vst3')
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
    $Uninstaller = Join-Path $env:ProgramFiles 'Wave FM-1/unins000.exe'
    $Process = Start-Process $Uninstaller -ArgumentList '/VERYSILENT /SUPPRESSMSGBOXES /NORESTART' -Wait -PassThru
    if ($Process.ExitCode -ne 0) { throw "Uninstaller failed: $($Process.ExitCode)" }
}
foreach ($File in @($Vst, $Clap)) {
    if (Test-Path $File) { throw "Uninstaller left plugin file: $File" }
}
