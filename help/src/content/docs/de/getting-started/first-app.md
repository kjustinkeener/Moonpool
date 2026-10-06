---
title: "Ihre erste App in Moonpool hinzufügen und ausführen"
description: "Kommen Sie in wenigen Minuten vom ersten Start zu einer laufenden eigenen App: hinzufügen, starten, stoppen und später den Hub und diese Hilfe wiederfinden."
---

## 1. Moonpool starten

Führen Sie unter Windows `moonpool.exe` aus und klicken Sie auf **Moonpool installieren** (siehe
[Installation](/de/getting-started/install/)). Starten Sie unter Linux das AppImage oder das
installierte Paket.

Beim ersten Start füllt Moonpool die Seitenleiste mit Beispiel-Apps (unter Windows Notepad, eine
Shell, ein kleiner Webserver und die mitgelieferten Dashboards). Sie laufen unverändert (der
Webserver braucht Python), sodass Sie sie ausprobieren und dann bearbeiten oder löschen können.
Außerdem legt Moonpool ein Symbol im Infobereich ab. Wenn Sie das Symbol unter Windows nicht sehen,
klicken Sie rechts in der Taskleiste auf den Pfeil **^**.

## 2. Ihre App hinzufügen

1. Öffnen Sie oben in der Seitenleiste das Menü **...** und wählen Sie **App hinzufügen**.
2. Geben Sie einen **name** ein. Die Gruppe beginnt als `Web apps`; behalten Sie sie oder wählen Sie eine andere.
3. Lassen Sie **type** für einen Entwicklungsserver auf `web`.
4. Setzen Sie **cwd** auf Ihren Projektordner und **command** auf das, was Sie zum Starten
   eingeben, zum Beispiel `npm run dev`.
5. Setzen Sie **port** auf den Port, auf dem sie lauscht, und **url** auf die zu öffnende Seite.
6. Speichern.

Die Einzelheiten zu jedem Feld finden Sie unter [Apps hinzufügen](/de/apps/add-an-app/).

## 3. Starten

Klicken Sie auf die Schaltfläche **Starten** der App (das Wiedergabesymbol in ihrer Zeile). Ihr
Terminal-Tab öffnet sich und zeigt die Ausgabe. Der Statuspunkt pulsiert, solange die App startet,
und wird dauerhaft ausgefüllt, sobald ihr Port antwortet. Ist **openBrowser** aktiv, öffnet sich die Seite.

Ein Klick auf den Namen der App öffnet nur ihren Terminal-Tab. Er startet die App nie.

## 4. Stoppen

Klicken Sie in der Zeile auf die Schaltfläche **Stoppen** (das Quadrat). Der Punkt wird grau.

Wenn nach dem Stoppen etwas weiterläuft, siehe [Stoppen und neu starten](/de/apps/stop-and-restart/).

## Einen Agenten das erledigen lassen

Ohne geöffneten Tab zeigt der CLI-Bereich eine Schaltfläche **Prompt kopieren**. Fügen Sie den
Prompt in einen KI-Agenten ein, und er findet Ihre Apps und fügt sie hinzu. Siehe
[KI-Agenten: Schnellstart](/de/automation/quick-start/).

## Den Hub später wiederfinden

- Klicken Sie mit links auf das Symbol im Infobereich, um den Hub anzuzeigen. Ein Rechtsklick
  öffnet ein Menü mit **Moonpool anzeigen** und **Beenden**.
- Standardmäßig beendet das Schließen des Fensters Moonpool. Aktivieren Sie in den Einstellungen
  **Beim Schließen in den Infobereich**, um es stattdessen in den Infobereich auszublenden und
  weiterlaufen zu lassen. Siehe [Infobereich, Schließen und Minimieren](/de/using/tray-and-closing/).

## Hilfe erhalten

**Hilfe** im Menü **...** oben in der Seitenleiste öffnet diese Hilfe in einem eigenen Fenster.
Sie funktioniert offline und passt immer zur Version, die Sie ausführen.

![Hilfefenster mit der hervorgehobenen Abschnittsnavigation links und einer Seite rechts](../../../../assets/screenshots/help-window.png)

## Weiter

- [Apps hinzufügen](/de/apps/add-an-app/)
- [Fehlerbehebung](/de/support/troubleshooting/)
