# Restore your real Moonpool apps.json after taking demo screenshots.
# Usage (PowerShell):  pwsh -NoProfile -File tools\screenshot-demo\restore.ps1
$ErrorActionPreference = 'Stop'
$cfg   = Join-Path $env:APPDATA 'Moonpool'
$stash = Join-Path $cfg 'apps.beforedemo.json'
$live  = Join-Path $cfg 'apps.json'

# Fall back to the newest timestamped backup if the stash is gone.
if (-not (Test-Path $stash)) {
    $stash = Get-ChildItem -Path $cfg -Filter 'apps.real-backup-*.json' |
             Sort-Object LastWriteTime -Descending | Select-Object -First 1 -ExpandProperty FullName
}

if ($stash -and (Test-Path $stash)) {
    # Moonpool may be running and can race a single write, so verify + retry.
    $want = Get-Content $stash -Raw
    $ok = $false
    for ($i = 0; $i -lt 5; $i++) {
        [System.IO.File]::WriteAllText($live, $want)
        Start-Sleep -Milliseconds 400
        if ((Get-Content $live -Raw) -eq $want) { $ok = $true; break }
    }
    if ($ok) {
        if ($stash -like '*apps.beforedemo.json') { Remove-Item $stash }
        Write-Host "Restored your real apps.json. Click Reload in Moonpool."
    } else {
        Write-Warning "apps.json kept reverting - is Moonpool overwriting it? Close it and re-run."
    }
} else {
    Write-Host "No backup found (apps.beforedemo.json or apps.real-backup-*.json) in $cfg."
}
Write-Host "(Demo icons left in place under $cfg\icons; delete them if you want them gone.)"
