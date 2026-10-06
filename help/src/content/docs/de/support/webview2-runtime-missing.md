---
title: "WebView2-Runtime fehlt: leeres oder fehlendes Moonpool-Fenster unter Windows beheben"
description: "Öffnet sich das Moonpool-Fenster unter Windows nicht oder bleibt leer, fehlt womöglich die Edge WebView2 Runtime. So prüfen und installieren Sie sie."
---

Wenn sich das Moonpool-Fenster unter Windows nie öffnet oder sich öffnet und leer bleibt, ist die wahrscheinliche Ursache
eine fehlende Microsoft Edge WebView2 Runtime. Moonpool ist eine Tauri-App, und ihre Fenster sind Webseiten,
die von WebView2 gezeichnet werden.

WebView2 wird mit Windows 11 und mit aktuellem Windows 10 ausgeliefert, daher haben es die meisten PCs bereits. Es
kann auf einem älteren oder abgespeckten Windows 10 fehlen oder auf einem PC, auf dem es entfernt wurde.
Der Quellcode von Moonpool zeigt für diesen Fall keine eigene Meldung, daher wird hier kein Fehlertext
zitiert: Das Symptom ist, dass das Fenster nicht erscheint oder leer ist.

## Prüfen, ob sie installiert ist

Suchen Sie in PowerShell in der Registrierung nach der Version der Runtime (der erste Pfad ist die
systemweite Installation, der zweite eine pro Benutzer):

```powershell frame="terminal"
Get-ItemProperty "HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -Name pv
Get-ItemProperty "HKCU:\Software\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -Name pv
```

Eine Versionsnummer wie `120.0.2210.91` bedeutet, dass sie installiert ist. Ein Fehler bei beiden bedeutet, dass sie
nicht installiert ist.

## Installieren

Laden Sie die **Evergreen**-WebView2-Runtime von der WebView2-Seite von Microsoft herunter (suchen Sie nach
„WebView2 Runtime download“), führen Sie das Installationsprogramm aus und starten Sie dann Moonpool erneut. Die Evergreen-
Runtime aktualisiert sich selbst.

## Wenn sie installiert ist und das Fenster trotzdem leer bleibt

- Beenden Sie jedes Moonpool über den Tray (oder beenden Sie `moonpool.exe` im Task-Manager) und starten Sie es
  erneut.
- Schalten Sie **Debug-Infos in eine Datei schreiben** in den [Einstellungen](/de/using/settings/) ein, falls Sie sie erreichen,
  und prüfen Sie `moonpool.log`. Siehe [Protokolle](/de/data/logs/).
- Öffnet sich das Fenster, liegt aber außerhalb des Bildschirms, siehe
  [Fensterprobleme](/de/support/troubleshooting/#fensterprobleme).

## Siehe auch

- [Windows](/de/platforms/windows/#vor-dem-start)
- [Installation](/de/getting-started/install/)
- [Fehlerbehebung](/de/support/troubleshooting/)
