# Build and verify on a Windows x64 machine with Rust/MSVC and CMake installed.
param([switch]$Launch)
$ErrorActionPreference = 'Stop'
Push-Location $PSScriptRoot
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
    Write-Host 'Build and unit tests completed. Model setup, synthesis and GUI require manual testing.'
    if ($Launch) {
        & cargo run --target x86_64-pc-windows-msvc
        if ($LASTEXITCODE -ne 0) { throw 'cargo run failed' }
    }
} finally {
    $env:PATH = $oldPath
    Pop-Location
}
