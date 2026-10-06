---
title: "Eine App oder einen Entwicklungsserver zu Moonpool hinzufügen"
description: "Registrieren Sie eine lokale App mit Startbefehl, Arbeitsordner und Umgebung, damit Moonpool sie für Sie starten, stoppen und überwachen kann."
---

Jede App in Moonpool ist ein Eintrag mit einem Startbefehl, einem Arbeitsordner und einer optionalen
Umgebung. Moonpool führt den Befehl in einem eigenen verwalteten Terminal aus.

## App hinzufügen

1. Öffnen Sie das Menü **...** oben in der Seitenleiste und wählen Sie **App hinzufügen**.
2. Geben Sie einen **name** ein und wählen Sie eine **group**.
3. Wählen Sie den **type**: `web` (Server an einem Port), `desktop` (native App), `static` (eine Seite) oder `cli` (ein Befehl).
4. Legen Sie den **command** und das **cwd** fest, in dem er läuft.
5. Füllen Sie aus, was der Typ braucht: **port** und **url** für web, **processName** für desktop,
   **url** für static. Eine `static`-App mit nur einer `url` braucht weder **command** noch **cwd**.
6. Speichern Sie. Die App erscheint in der Seitenleiste. Mit ihrem Bedienelement **Starten** starten Sie sie.

![Die Typauswahl (1) und das Feld port (2) im App-Editor, mit cwd und command dazwischen](../../../../assets/screenshots/edit-app-type-and-port.png)

1. Die Auswahl **type**; ihr Hinweis erklärt, wie dieser Typ läuft.
2. Das Feld **port**, das von `web`-Apps verwendet wird.

Das Ergebnis ist ein Eintrag in `apps.json`, zum Beispiel:

```json title="apps.json"
{
  "id": "my-api",
  "name": "My API",
  "group": "Dev",
  "type": "web",
  "command": "npm run dev",
  "cwd": "C:\\code\\my-api",
  "port": 3000,
  "url": "http://localhost:3000"
}
```

Ein Klick auf den Namen einer App öffnet nur ihren Terminal-Tab; siehe [App-Status](/de/support/glossary/#app-zustände).

## Der App-Editor

- **Gruppe.** Wählen Sie eine Gruppe aus der Liste, oder wählen Sie **+ Neue Gruppe...** und geben Sie einen Namen ein.
  **zurück zur Liste** führt zur Liste zurück. Eine leere Gruppe wird als `Apps` gespeichert.
- **Abgedunkelte Felder** werden vom gewählten Typ nicht verwendet. Sie werden trotzdem gespeichert.
- **Speichern ohne Namen** zeigt `Name ist erforderlich.`
- **Esc** oder das Schließen des Editors mit ungespeicherten Änderungen fragt „Ihre Änderungen verwerfen?“.
- Um eine App später zu ändern, nutzen Sie den Stift in ihrer Zeile oder klicken Sie sie mit der rechten Maustaste an und wählen **Bearbeiten**.

## Von Hand bearbeiten

Wählen Sie im selben Menü **apps.json bearbeiten**, speichern Sie die Datei und wählen Sie dann **Neu laden**. Das
Format, die Prüfregeln und die Wiederherstellungsoptionen stehen in der
[Konfigurationsübersicht](/de/apps/apps-json/).

## Wie es weitergeht

- [App-Felder](/de/apps/fields/): jeder Schlüssel und was er bewirkt.
- [App-Typen](/de/apps/types/): wie jeder Typ startet und wann „läuft“ angezeigt wird.
- [Stoppen und neu starten](/de/apps/stop-and-restart/): was Sie einstellen, wenn Stoppen etwas weiterlaufen lässt, und warum Docker-Apps Vorsicht brauchen.
- [Pfade und Umgebung](/de/apps/paths-and-environment/): `{MP_HOME}`, `./`-Pfade und `env`.
- [Beispiele](/de/apps/examples/): vollständige Einträge zum Kopieren.
- [Anleitungen](/de/guides/run-npm-dev-server-in-background-windows/): Entwicklungsserver im Hintergrund, Python-Skripte, Ports.
- [Portabler Modus](/de/data/portable-mode/)
- [Aktualisieren](/de/data/updating/)
