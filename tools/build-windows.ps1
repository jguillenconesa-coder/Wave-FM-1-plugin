#requires -Version 7.0
param([int]$Jobs = 2)
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
if ($Jobs -lt 1) { throw 'Jobs must be positive.' }
$Root = Split-Path $PSScriptRoot -Parent
$PreviousRustFlags = $env:RUSTFLAGS
$env:RUSTFLAGS = "$PreviousRustFlags -C target-feature=+crt-static"
function Invoke-Checked {
    param([string]$Program, [string[]]$Arguments)
    & $Program @Arguments
    if ($LASTEXITCODE -ne 0) { throw "$Program failed with exit code $LASTEXITCODE" }
}
Push-Location $Root
try {
    # A failed rebuild must not leave a previous successful manifest packageable.
    if (Test-Path 'dist/build-info.json') { Remove-Item 'dist/build-info.json' }
    Invoke-Checked -Program python -Arguments @('tools/bootstrap.py')
    $Manifest = Join-Path $Root '.deps/fm1-emu/crates/fm1-plugin/Cargo.toml'
    Invoke-Checked -Program cargo -Arguments @('build', '--manifest-path', $Manifest, '--release', '--locked', '--target', 'x86_64-pc-windows-msvc', '-j', "$Jobs")
    Invoke-Checked -Program cmake -Arguments @('-S', '.', '-B', 'build/windows', '-G', 'Visual Studio 17 2022', '-A', 'x64')
    Invoke-Checked -Program cmake -Arguments @('--build', 'build/windows', '--config', 'Release', '--target', 'wave_fm1', '--parallel', "$Jobs")
    Invoke-Checked -Program cmake -Arguments @('--install', 'build/windows', '--config', 'Release', '--prefix', "$Root/dist")
    New-Item -ItemType Directory -Force 'dist/VST3/FM-1.vst3/Contents/x86_64-win', 'dist/licenses' | Out-Null
    Copy-Item '.deps/fm1-emu/crates/fm1-plugin/target/x86_64-pc-windows-msvc/release/fm1_plugin.dll' 'dist/VST3/FM-1.vst3/Contents/x86_64-win/FM-1.clap'
    Copy-Item '.deps/fm1-emu/LICENSE' 'dist/licenses/fm1-emu-MIT.txt'
    Copy-Item 'LICENSE' 'dist/licenses/packaging-MIT.txt'
    Copy-Item 'THIRD_PARTY_NOTICES.md' 'dist/licenses/THIRD_PARTY_NOTICES.md'
    Invoke-Checked -Program python -Arguments @('tools/collect-rust-licenses.py')
    # Preserve the wrapper and SDK licence notices beside the binary distribution.
    $BuildRoot = Join-Path $Root 'build/windows'
    Get-ChildItem $BuildRoot -Recurse -File | Where-Object {
        $_.Name -match '^(LICENSE|LICENCE|COPYING|NOTICE)(\..*)?$'
    } | ForEach-Object {
        $Relative = [IO.Path]::GetRelativePath($BuildRoot, $_.FullName)
        $Destination = Join-Path "$Root/dist/licenses/third-party" $Relative
        New-Item -ItemType Directory -Force (Split-Path $Destination -Parent) | Out-Null
        Copy-Item $_.FullName $Destination
    }
    Copy-Item 'docs/WINDOWS.md' 'dist/INSTALL.md'
    if (-not (Test-Path 'dist/VST3/FM-1.vst3/Contents/x86_64-win/FM-1.vst3' -PathType Leaf)) { throw 'Missing VST3 output.' }
    Invoke-Checked -Program python -Arguments @('tools/artifacts.py', 'record')
    Write-Host 'Built the private VST3/CLAP pair in dist/VST3. See dist/INSTALL.md.'
} finally { $env:RUSTFLAGS = $PreviousRustFlags; Pop-Location }
