#requires -Version 7.0
param([string]$Version = '0.1.0', [string]$Iscc = '')
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
if ($Version -notmatch '^\d+\.\d+\.\d+$') { throw 'Version must be numeric major.minor.patch.' }
$Root = Split-Path $PSScriptRoot -Parent
Push-Location $Root
try {
    foreach ($File in @('dist/VST3/FM-1.vst3', 'dist/CLAP/FM-1.clap', 'dist/build-info.json', 'dist/INSTALL.md', 'dist/licenses/fm1-emu-MIT.txt', 'dist/licenses/rust/inventory.json', 'dist/licenses/THIRD_PARTY_NOTICES.md')) {
        if (-not (Test-Path $File -PathType Leaf)) { throw "Missing build input: $File. Run tools/build-windows.ps1 first." }
    }
    & python 'tools/artifacts.py' verify
    if ($LASTEXITCODE -ne 0) { throw 'Build integrity verification failed.' }
    if (-not $Iscc) {
        $Command = Get-Command ISCC.exe -ErrorAction SilentlyContinue
        if ($Command) { $Iscc = $Command.Source }
        else { $Iscc = Join-Path ${env:ProgramFiles(x86)} 'Inno Setup 6/ISCC.exe' }
    }
    if (-not (Test-Path $Iscc -PathType Leaf)) { throw 'Install Inno Setup 6.3+ or pass -Iscc with the path to ISCC.exe.' }
    New-Item -ItemType Directory -Force 'build/prerequisites' | Out-Null
    $Runtime = Join-Path $Root 'build/prerequisites/VC_redist.x64.exe'
    Invoke-WebRequest 'https://aka.ms/vs/17/release/vc_redist.x64.exe' -OutFile $Runtime
    $Signature = Get-AuthenticodeSignature $Runtime
    if ($Signature.Status -ne 'Valid' -or $Signature.SignerCertificate.Subject -notmatch 'O=Microsoft Corporation(?:,|$)') {
        Remove-Item $Runtime
        throw 'The Visual C++ runtime did not have a valid Microsoft signature.'
    }
    & $Iscc "/DAppVersion=$Version" 'installer/windows.iss'
    if ($LASTEXITCODE -ne 0) { throw "Inno Setup failed: $LASTEXITCODE" }
    $Output = Join-Path $Root "dist/installer/Wave-FM-1-$Version-Windows-x64-Setup.exe"
    if (-not (Test-Path $Output -PathType Leaf)) { throw 'Installer output not found.' }
    $Hash = (Get-FileHash $Output -Algorithm SHA256).Hash.ToLowerInvariant()
    "$Hash  $(Split-Path $Output -Leaf)" | Set-Content "$Output.sha256" -Encoding ascii
    Write-Host "Created $Output"
} finally { Pop-Location }
