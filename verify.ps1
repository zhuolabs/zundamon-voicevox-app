# Build and verify on a Windows x64 machine with Rust/MSVC and CMake installed.
param([switch]$SkipAssets, [switch]$Launch)
$ErrorActionPreference = 'Stop'
Push-Location $PSScriptRoot
$oldAssetDir = $env:VOICEVOX_ASSET_DIR
$oldPath = $env:PATH
try {
    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) { throw 'Install Rust (MSVC toolchain) first.' }
    & rustc -V
    if ($LASTEXITCODE -ne 0) { throw 'rustc failed' }
    & cargo build --target x86_64-pc-windows-msvc
    if ($LASTEXITCODE -ne 0) { throw 'cargo build failed' }
    $env:PATH = (Join-Path $PSScriptRoot 'target/x86_64-pc-windows-msvc/debug') + ';' + $env:PATH
    & cargo test --locked --target x86_64-pc-windows-msvc
    if ($LASTEXITCODE -ne 0) { throw 'cargo test failed' }
    if (-not $SkipAssets) {
        & (Join-Path $PSScriptRoot 'setup-assets.ps1')
        $env:VOICEVOX_ASSET_DIR = Join-Path $PSScriptRoot 'voicevox_core'
        & cargo test --locked --target x86_64-pc-windows-msvc voicevox_smoke -- --ignored --nocapture
        if ($LASTEXITCODE -ne 0) { throw 'VOICEVOX smoke test failed' }
    }
    Write-Host 'Requested build/tests completed. GUI and physical audio playback still require manual testing.'
    if ($Launch) {
        & cargo run --target x86_64-pc-windows-msvc
        if ($LASTEXITCODE -ne 0) { throw 'cargo run failed' }
    }
} finally {
    $env:VOICEVOX_ASSET_DIR = $oldAssetDir
    $env:PATH = $oldPath
    Pop-Location
}
