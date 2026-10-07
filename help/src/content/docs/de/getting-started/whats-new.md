---
title: "Moonpool-Versionshinweise und aktuelle Änderungen"
description: "Sehen Sie, was sich in den letzten Moonpool-Versionen geändert hat, welche Voraussetzungen zum Ausführen gelten und wo die vollständigen Versionshinweise auf GitHub stehen."
---

Die vollständigen Hinweise zu jeder Version stehen auf der
[Releases-Seite](https://github.com/kjustinkeener/Moonpool/releases) des Projekts. Diese Hilfe
wird in Moonpool mitgeliefert und beschreibt daher immer die Version, die Sie ausführen. Moonpool
aktualisiert sich selbst; siehe [Aktualisieren](/de/data/updating/).

## 0.3.17

- **Hilfe in 14 Sprachen.** Die Hilfe öffnet sich in der Sprache von Moonpool: Englisch,
  Deutsch, Spanisch, Französisch, Italienisch, Niederländisch, Polnisch, brasilianisches
  Portugiesisch, Russisch, Türkisch, Japanisch, Koreanisch sowie vereinfachtes und traditionelles
  Chinesisch.
- **Neue Anleitungen und Supportseiten** zu Entwicklungsservern, Ports, Start bei der Anmeldung,
  MCP-Agenten, Python-Skripten und häufigen Fehlermeldungen.
- **`mcpProcessName`.** Ein Platzhaltermuster für den Prozessnamen des MCP-Servers einer App,
  für Server, die unter einem anderen Namen laufen. Siehe [mcpProcessName](/de/apps/fields/#mcpprocessname).
- **macOS wird nicht mehr unterstützt.** Es gibt keine macOS-Builds mehr. Für Windows und Linux
  ändert sich nichts.

## 0.3.16

- **Mehrere Moonpools gleichzeitig.** Das installierte Moonpool und beliebig viele portable Kopien
  können nebeneinander laufen, eine pro Ordner, jeweils mit eigenen Apps, eigenem Symbol im
  Infobereich und eigenem Steuerkanal. Siehe [Portabler Modus](/de/data/portable-mode/#mehrere-kopien-gleichzeitig).
- **Design-Browser.** 68 Designs, jedes in seinen eigenen Farben vorab angezeigt. Siehe
  [Designs, Sprache und Transparenz](/de/using/themes-and-language/).
- **Lauffähige Beispiele.** Eine frische `apps.json` enthält Beispiel-Apps, die alle unverändert
  laufen. Die Beispiel-Dashboards liegen jetzt in einem App-eigenen Ordner `dashboards/examples`,
  der mit Moonpool aktualisiert wird. Siehe [Beispiel-Dashboards](/de/getting-started/example-dashboards/).
- **apps.json-Fehler werden angezeigt.** Ein Banner über der Seitenleiste zeigt den Fehler, und ein
  fehlgeschlagenes Neuladen behält die zuletzt geladene Liste. Siehe
  [Wenn apps.json einen Fehler enthält](/de/using/hub-window/#wenn-appsjson-einen-fehler-enthält).
- **Steuerkanal unter Linux** über einen Unix-Socket, plus das Verb `list`. Siehe
  [Steuerverben](/de/automation/control-verbs/).
- „Über“ und der App-Editor folgen Änderungen von Design und Sprache sofort. Der Menüpunkt
  **Moonpool installieren…** ist außerhalb von Windows ausgeblendet.

## 0.3.15

- Eine neu gestartete App behält ihre frühere Ausgabe, mit einer datierten Trennlinie „restarted“.
  Siehe [Terminal-Tabs](/de/using/terminal-tabs/#neu-starten).
- Jede App hat ihren eigenen Ordner `cli-output`, sodass das Bereinigen von Protokollen nie die
  Protokolle einer anderen App berührt.
- `killMode` und `stopCommand` sind im App-Editor verfügbar. Siehe
  [Stoppen und neu starten](/de/apps/stop-and-restart/).
- Gestartete Apps erben nicht mehr das eigene WebView2-Profil von Moonpool.

## 0.3.14

- Sitzungsprotokolle können zwischen Sitzungen behalten werden, mit einer Größenobergrenze pro App.
  Siehe [Protokolle](/de/data/logs/).
- Schaltflächen zum Öffnen und Kopieren der Protokollordner in den Einstellungen.
- Korrekturen an der Titelleiste des Hilfefensters.

## Voraussetzungen

- Windows 10 oder 11 mit WebView2 (siehe [Windows](/de/platforms/windows/)).
- Linux mit WebKitGTK 4.1 und einer AppIndicator-Bibliothek (siehe [Linux](/de/platforms/linux/)).
