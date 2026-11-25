# Run after successful Windows x64 release build. Do not bundle voice models.
$ErrorActionPreference = 'Stop'
Push-Location $PSScriptRoot
try {
    $Release = Join-Path $PSScriptRoot 'target/x86_64-pc-windows-msvc/release'
    $Dist = Join-Path $PSScriptRoot 'dist'
    $Exe = Join-Path $Release 'zundamon-voicevox-app.exe'
    if (-not (Test-Path -LiteralPath $Exe -PathType Leaf)) { throw "Missing executable: $Exe" }
    if (Test-Path -LiteralPath $Dist) { Remove-Item -LiteralPath $Dist -Recurse -Force }
    New-Item -ItemType Directory -Path $Dist | Out-Null
    $excluded = @('build', 'deps', 'examples', 'incremental', '.fingerprint', 'output', 'voicevox_core', '.cargo-lock')
    Get-ChildItem -LiteralPath $Release -Force | Where-Object {
        $_.Name -notin $excluded -and ($_.PSIsContainer -or $_.Extension -notin @('.d', '.pdb', '.rlib', '.rmeta', '.lib', '.exp'))
    } | ForEach-Object { Copy-Item -LiteralPath $_.FullName -Destination $Dist -Recurse -Force }
    if (-not (Test-Path -LiteralPath (Join-Path $Dist 'zundamon-voicevox-app.exe'))) { throw 'Executable missing in package' }
    if (-not (Get-ChildItem -LiteralPath $Dist -Filter '*.dll' -File)) { throw 'No staged runtime DLLs found' }
    # Bundle only the official downloader executable, not any downloaded assets.
    $Downloader = Join-Path $Dist 'download.exe'
    Invoke-WebRequest -Uri 'https://github.com/VOICEVOX/voicevox_core/releases/download/0.17.1/download-windows-x64.exe' -OutFile $Downloader
    $stream = [System.IO.File]::OpenRead($Downloader)
    try {
        if ($stream.ReadByte() -ne 0x4D -or $stream.ReadByte() -ne 0x5A) { throw 'Downloader is not a Windows executable' }
    } finally { $stream.Dispose() }
    if ((Test-Path (Join-Path $Dist 'voicevox_core')) -or @(Get-ChildItem -LiteralPath $Dist -Recurse -File -Filter '*.vvm').Count -gt 0) {
        throw 'Voice models or downloaded assets must not be distributed'
    }
    foreach ($name in @('USER_GUIDE.md', 'README.md', 'FIRST_RUN_TESTS.md', 'setup-assets.ps1', 'setup-assets.cmd', 'VOICEVOX-DOWNLOADER-LICENSE.txt')) {
        Copy-Item -LiteralPath (Join-Path $PSScriptRoot $name) -Destination $Dist
    }
    $hashes = Get-ChildItem -LiteralPath $Dist -Recurse -File | Sort-Object FullName | ForEach-Object {
        $relative = $_.FullName.Substring($Dist.Length + 1).Replace('\','/')
        '{0}  {1}' -f (Get-FileHash -LiteralPath $_.FullName -Algorithm SHA256).Hash.ToLowerInvariant(), $relative
    }
    $hashes | Set-Content -LiteralPath (Join-Path $Dist 'SHA256SUMS.txt') -Encoding utf8
    Write-Host "Packaged application: $Dist"
} finally { Pop-Location }
