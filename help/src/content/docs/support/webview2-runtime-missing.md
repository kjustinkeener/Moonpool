---
title: "WebView2 runtime missing: fix a blank or missing Moonpool window on Windows"
description: "If Moonpool's window never opens or stays blank on Windows, the Microsoft Edge WebView2 Runtime may be missing. How to check for it and install it."
---

If Moonpool's window never opens, or opens and stays blank, on Windows, the likely cause is
a missing Microsoft Edge WebView2 Runtime. Moonpool is a Tauri app, and its windows are web
pages drawn by WebView2.

WebView2 ships with Windows 11 and with current Windows 10, so most PCs already have it. It
can be missing on an older or stripped-down Windows 10, or a PC where it was removed.
Moonpool's own source does not show a dedicated message for this case, so no error text is
quoted here: the symptom is the window not appearing or being empty.

## Check whether it is installed

In PowerShell, look for the runtime's version in the registry (the first path is the
system-wide install, the second a per-user one):

```powershell frame="terminal"
Get-ItemProperty "HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -Name pv
Get-ItemProperty "HKCU:\Software\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -Name pv
```

A version number such as `120.0.2210.91` means it is installed. An error for both means it
is not.

## Install it

Download the **Evergreen** WebView2 Runtime from Microsoft's WebView2 page (search for
"WebView2 Runtime download"), run the installer, then start Moonpool again. The Evergreen
runtime updates itself.

## If it is installed and the window is still blank

- Quit every Moonpool from the tray (or end `moonpool.exe` in Task Manager) and start it
  again.
- Turn on **Log debug info to a file** in [Settings](/using/settings/) if you can reach it,
  and check `moonpool.log`. See [Logs](/data/logs/).
- If the window opens but is off screen, see
  [Window problems](/support/troubleshooting/#window-problems).

## See also

- [Windows](/platforms/windows/#before-you-run-it)
- [Installing](/getting-started/install/)
- [Troubleshooting](/support/troubleshooting/)
