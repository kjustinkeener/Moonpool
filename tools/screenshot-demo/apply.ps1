# Load the screenshot demo into Moonpool's live config, backing up your real setup first.
# Usage (PowerShell):  pwsh -NoProfile -File tools\screenshot-demo\apply.ps1
$ErrorActionPreference = 'Stop'
$cfg  = Join-Path $env:APPDATA 'Moonpool'
$here = $PSScriptRoot
$icons = Join-Path $cfg 'icons'

New-Item -ItemType Directory -Force -Path $cfg, $icons | Out-Null

# Back up the current apps.json (only if we haven't already stashed one).
$stash = Join-Path $cfg 'apps.beforedemo.json'
$live  = Join-Path $cfg 'apps.json'
if ((Test-Path $live) -and -not (Test-Path $stash)) {
    Copy-Item $live $stash
    Write-Host "Backed up your apps.json -> $stash"
}

# Moonpool may be running and can race a single write, so verify + retry.
function Write-Manifest($src, $dst) {
    $want = (Get-Content $src -Raw)
    for ($i = 0; $i -lt 5; $i++) {
        [System.IO.File]::WriteAllText($dst, $want)
        Start-Sleep -Milliseconds 400
        if ((Get-Content $dst -Raw) -eq $want) { return $true }
    }
    return $false
}
if (-not (Write-Manifest (Join-Path $here 'apps.sample.json') $live)) {
    Write-Warning "apps.json kept reverting - is Moonpool overwriting it? Close it and re-run."
}
Copy-Item (Join-Path $here 'icons\*.svg') $icons -Force
Write-Host "Demo manifest + $((Get-ChildItem $here\icons\*.svg).Count) icons installed."
Write-Host "In Moonpool: click Reload. Then launch 'Metrics Dashboard' for a live terminal + green dot."
Write-Host "Restore your real setup later with:  pwsh -NoProfile -File tools\screenshot-demo\restore.ps1"
