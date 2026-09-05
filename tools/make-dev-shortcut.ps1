# Create a Desktop shortcut "Moonpool (dev)" that launches tools\dev-run.ps1.
# Matches the shortcut convention used by the other local apps. This is for
# working from a source checkout; installing a release is the app's own job.
$ErrorActionPreference = 'Stop'

$root = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Definition)
$devRun = Join-Path $root 'tools\dev-run.ps1'
$iconPng = Join-Path $root 'src-tauri\icons\icon.ico'

$desktop = [Environment]::GetFolderPath('Desktop')
$lnkPath = Join-Path $desktop 'Moonpool (dev).lnk'

# Use Windows PowerShell 5.1 (System32), NOT the Store 'pwsh' (MSIX). The Store pwsh
# runs under an app package and redirects child processes' %APPDATA% into a private
# per-package overlay, so Moonpool would read a sandboxed copy of apps.json instead of
# the real one. 5.1 is a normal Win32 process with no such redirection.
$sh = New-Object -ComObject WScript.Shell
$lnk = $sh.CreateShortcut($lnkPath)
$lnk.TargetPath = "$env:WINDIR\System32\WindowsPowerShell\v1.0\powershell.exe"
$lnk.Arguments = "-NoProfile -ExecutionPolicy Bypass -File `"$devRun`""
$lnk.WorkingDirectory = $root
if (Test-Path $iconPng) { $lnk.IconLocation = $iconPng }
$lnk.Description = 'Moonpool - tray launcher hub for local apps'
$lnk.WindowStyle = 7   # minimized
$lnk.Save()

Write-Host "Created shortcut: $lnkPath" -ForegroundColor Green
