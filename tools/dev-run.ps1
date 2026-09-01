# Moonpool dev launcher.
# Kills a running Moonpool first (it holds a lock on the binary and the tray), rebuilds
# the full PATH (shortcut-launched processes get a thin PATH, and Moonpool's child apps
# inherit it), pre-builds Rust to surface stale-cache linker errors early, frees the Vite
# port, then runs `tauri dev` and blocks until the window closes.
$ErrorActionPreference = 'Continue'
# Repo root is the parent of this script's tools\ folder.
Set-Location (Split-Path -Parent $PSScriptRoot)

# Rebuild full machine + user PATH so node/cargo/python/pwsh resolve for Moonpool AND the
# apps it launches (they inherit this process's environment).
$machine = [Environment]::GetEnvironmentVariable('Path', 'Machine')
$user    = [Environment]::GetEnvironmentVariable('Path', 'User')
$env:Path = "$machine;$user"

# Kill a running Moonpool first - a live .exe locks the binary so cargo can't relink.
$running = Get-Process -Name 'moonpool' -ErrorAction SilentlyContinue
if ($running) {
    Write-Host '[dev-run] Stopping running Moonpool...' -ForegroundColor Yellow
    $running | Stop-Process -Force -ErrorAction SilentlyContinue
    Start-Sleep -Milliseconds 300
}

# Force cargo to see the current source. Some editors/tools don't bump file mtimes the
# way cargo's fingerprint detects, so cargo can skip recompiling and `tauri dev` launches
# a STALE binary (old compiled-in behaviour). Touching the sources guarantees a rebuild.
Get-ChildItem 'src-tauri\src' -Filter *.rs -Recurse | ForEach-Object { $_.LastWriteTime = Get-Date }
if (Test-Path 'src-tauri\resources\apps.example.json') { (Get-Item 'src-tauri\resources\apps.example.json').LastWriteTime = Get-Date }

Write-Host '[dev-run] Pre-build (Rust)...' -ForegroundColor Cyan
cargo build --manifest-path src-tauri/Cargo.toml 2>&1 | Tee-Object -Variable buildOut
$buildRc = $LASTEXITCODE
if ($buildRc -ne 0 -and ($buildOut -match 'LNK2019|LNK1120|anon\..*\.llvm')) {
    Write-Host '[dev-run] Stale linker cache detected - cleaning and rebuilding...' -ForegroundColor Yellow
    cargo clean --manifest-path src-tauri/Cargo.toml
    cargo build --manifest-path src-tauri/Cargo.toml 2>&1 | Tee-Object -Variable buildOut
    $buildRc = $LASTEXITCODE
}
if ($buildRc -ne 0) {
    Write-Host '[dev-run] Pre-build failing - starting dev server anyway; watcher will rebuild.' -ForegroundColor Yellow
}

# Free the Vite dev port if a previous run's server is still holding it.
$devPort = 1460
$listeners = Get-NetTCPConnection -LocalPort $devPort -State Listen -ErrorAction SilentlyContinue
if ($listeners) {
    foreach ($procId in ($listeners.OwningProcess | Sort-Object -Unique)) {
        Write-Host "[dev-run] Freeing port $devPort (PID $procId)..." -ForegroundColor Yellow
        taskkill /PID $procId /T /F 2>$null | Out-Null
    }
    Start-Sleep -Milliseconds 400
}

Write-Host '[dev-run] Starting dev server (npm run tauri dev)...' -ForegroundColor Cyan
$dev = Start-Process -FilePath 'cmd.exe' -ArgumentList '/c', 'npm run tauri dev' -NoNewWindow -PassThru

# Wait for the app window, then block until the user closes it.
$app = $null
$deadline = (Get-Date).AddMinutes(10)
while (-not $app -and -not $dev.HasExited -and (Get-Date) -lt $deadline) {
    Start-Sleep -Milliseconds 500
    $app = Get-Process -Name 'moonpool' -ErrorAction SilentlyContinue | Select-Object -First 1
}
if ($app) { $app.WaitForExit() }

# Tree-kill the dev-server subtree, then free the port in case Vite lingers.
if (-not $dev.HasExited) { taskkill /PID $dev.Id /T /F 2>$null | Out-Null }
$leftover = Get-NetTCPConnection -LocalPort $devPort -State Listen -ErrorAction SilentlyContinue
if ($leftover) {
    foreach ($procId in ($leftover.OwningProcess | Sort-Object -Unique)) { taskkill /PID $procId /T /F 2>$null | Out-Null }
}
exit
