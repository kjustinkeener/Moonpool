---
title: "Moonpool aktualisieren und ein fehlgeschlagenes Update beheben"
description: "Wie Moonpool Updates sucht, herunterlädt und anwendet, was das Update-Banner tut, wie portable und Linux-Kopien sich aktualisieren und was bei einem Fehler zu tun ist."
---

Moonpool aktualisiert sich selbst. Es gibt keinen separaten Installer herunterzuladen und keinen Assistenten, den Sie
durchklicken müssen.

## Wie Updates ankommen

Moonpool lädt `update.json` (unter Linux `linux-update.json`) aus den GitHub Releases des Projekts, vergleicht Versionen und bietet nur
eine echt neuere an. Es sucht:

- beim Start, sofern **Beim Start nach Updates suchen** in den
  [Einstellungen](/de/using/settings/) nicht ausgeschaltet ist;
- immer, wenn Sie im Über-Fenster **Nach Updates suchen** drücken. Diese Schaltfläche installiert eine neuere
  Version sofort und startet Moonpool neu. Andernfalls meldet sie, dass Sie die neueste
  Version haben, oder zeigt den Fehler.

Das Über-Fenster zeigt unter dem Namen die Version an, die Sie ausführen:

![Der obere Teil des Über-Fensters: das Logo, der Name (1) und darunter die Versionszeile](../../../../assets/screenshots/about-header.png)

1. Der Name. Die Zeile darunter zeigt die Version und das Build-Datum.

Jeder Download wird vor dem Anwenden gegen den minisign-Signaturschlüssel von Moonpool geprüft, sodass
ein manipulierter oder beschädigter Download abgelehnt wird. Moonpool installiert nie eine ältere Version.

## Das Update-Banner

Beim Start erscheint ein gefundenes Update als Banner auf dem leeren Bildschirm des Hubs:

```text
Moonpool {version} is available (you have {current}).
```

(In der deutschen Oberfläche lautet der Text „Moonpool {version} ist verfügbar (Sie haben {current}).“)

Das Banner erscheint nur, solange kein App-Tab geöffnet und der CLI-Bereich eingeblendet ist. Bei
eingeklapptem Bereich pulsiert stattdessen der Pfeil neben dem Filterfeld. Bei geöffnetem Tab gibt es gar
kein Zeichen. Um das Banner zu sehen, schließen Sie alle Tabs (und blenden den Bereich ein) oder nutzen **Nach Updates
suchen** im Über-Fenster.

Klicken Sie auf **Herunterladen und installieren**, und Moonpool ersetzt sich selbst und startet neu, oder blenden Sie das
Banner mit dem x aus.

## Portable Kopien

Eine portable Kopie aktualisiert die `moonpool.exe` in ihrem eigenen Ordner `.moonpool\` auf dieselbe Weise.
Jede Kopie sucht und aktualisiert sich selbst. Der Ordner muss beschreibbar sein, daher kann sich eine Kopie auf einem
schreibgeschützten Stick oder Netzlaufwerk nicht selbst aktualisieren; kopieren Sie dann eine neuere `moonpool.exe` von Hand darüber.

## Linux

Nur das AppImage aktualisiert sich selbst. Es ersetzt die AppImage-Datei an Ort und Stelle, legen Sie sie also in einem
Ordner ab, in den Sie schreiben können. Eine Installation per `.deb` oder RPM wird von Ihrem Paketmanager aktualisiert:
Das Installieren aus Moonpool schlägt fehl mit

```text
automatic updates are available for the AppImage only; update the .deb or RPM with your package manager
```

Siehe [Linux](/de/platforms/linux/#updates).

## Wenn ein Update fehlschlägt

Das Banner zeigt den Grund, und die Schaltfläche steht wieder zur Verfügung, damit Sie es erneut versuchen können:

```text
Update failed: <error>
```

(Deutsch: „Update fehlgeschlagen: <error>“.)

| Fehler enthält | Wahrscheinliche Ursache | Was zu tun ist |
| --- | --- | --- |
| `download failed` | Keine Verbindung, ein Proxy oder GitHub begrenzt Anfragen | Warten und erneut versuchen oder von Hand aktualisieren. |
| `signature verification FAILED - refusing to install` | Der Download ist beschädigt oder wurde verändert | Erneut versuchen. Schlägt es weiter fehl, von Hand über die Releases-Seite aktualisieren. |
| `rename self aside` oder `write new exe` | Der Ordner ist schreibgeschützt oder ein Virenscanner hält die Datei fest | Den Ordner beschreibbar machen oder `moonpool.exe` im Virenscanner zulassen, dann erneut versuchen. |
| `refusing to install ... not newer than current` | Die angebotene Version ist nicht neuer | Nichts zu tun. |

### Von Hand aktualisieren

Beenden Sie Moonpool, laden Sie `moonpool.exe` von der
[Releases-Seite](https://github.com/kjustinkeener/Moonpool/releases) des Projekts herunter und kopieren Sie sie über die
alte: `%USERPROFILE%\.moonpool\moonpool.exe` bei der Installation, oder die in Ihrem Ordner `.moonpool\`
bei einer portablen Kopie. Ihr Konfigurationsordner bleibt unberührt. Unter Linux ersetzen Sie das
AppImage oder nutzen Ihren Paketmanager.

## Die Hilfe wird ebenfalls aktualisiert

Diese Hilfe wird mit Moonpool ausgeliefert, daher bringt jedes Programm-Update die passende Hilfe mit.
Die Offline-Kopie passt immer zur Version, die Sie ausführen.

## Siehe auch

- [Was ist neu](/de/getting-started/whats-new/)
- [Einstellungsfenster](/de/using/settings/)
