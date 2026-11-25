# Download VOICEVOX 0.17.1 assets; run on Windows x64.
param([switch]$Force)
$ErrorActionPreference = 'Stop'
Push-Location $PSScriptRoot
try {
    $Version = '0.17.1'
    $Base = Join-Path $PSScriptRoot 'voicevox_core'
    $Downloader = Join-Path $PSScriptRoot 'download.exe'
    function Test-Assets {
        $models = @(Get-ChildItem (Join-Path $Base 'models/vvms') -Filter '*.vvm' -File -ErrorAction SilentlyContinue)
        if ($models.Count -eq 0) {
            $models = @(Get-ChildItem (Join-Path $Base 'models') -Filter '*.vvm' -File -ErrorAction SilentlyContinue)
        }
        $dict = (Test-Path (Join-Path $Base 'dict/sys.dic')) -or (Test-Path (Join-Path $Base 'dict/open_jtalk_dic_utf_8-1.11/sys.dic')) -or (Test-Path (Join-Path $Base 'open_jtalk_dic_utf_8-1.11/sys.dic'))
        $ort = @(Get-ChildItem (Join-Path $Base 'onnxruntime/lib') -Filter '*onnxruntime*.dll' -File -ErrorAction SilentlyContinue)
        return ($models.Count -gt 0 -and $dict -and $ort.Count -gt 0)
    }
    if ($Force -or -not (Test-Assets)) {
        $Url = "https://github.com/VOICEVOX/voicevox_core/releases/download/$Version/download-windows-x64.exe"
        Invoke-WebRequest -Uri $Url -OutFile $Downloader
        & $Downloader -o $Base --exclude c-api
        if ($LASTEXITCODE -ne 0) { throw "VOICEVOX downloader failed: $LASTEXITCODE" }
    }
    if (-not (Test-Assets)) { throw "VOICEVOX assets missing or incomplete: $Base" }
    Write-Host "VOICEVOX assets ready: $Base"
} finally {
    Pop-Location
}
