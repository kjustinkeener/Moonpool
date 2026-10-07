---
title: "Moonpool-Probleme beheben: Tray, Apps starten nicht, Updates"
description: "Häufige Moonpool-Probleme nach Symptom beheben: fehlendes Tray-Symbol, Apps, die nicht starten oder stoppen, falsche Statuspunkte, Update- und MCP-Fehler."
---

Suchen Sie das Symptom und folgen Sie der Lösung. Zitierter Text ist das, was Moonpool
anzeigt. Um eine genaue Meldung nachzuschlagen, siehe
[Fehlermeldungen erklärt](/de/support/error-messages/).

## Ich sehe das Tray-Symbol nicht

- **Windows.** Das Symbol befindet sich möglicherweise im Bereich der ausgeblendeten Symbole.
  Klicken Sie rechts in der Taskleiste auf den Pfeil **^**. Ziehen Sie das Symbol auf die
  Taskleiste, damit es sichtbar bleibt.
- **Linux mit Standard-GNOME.** GNOME zeigt ohne die AppIndicator-Erweiterung keine
  Tray-Symbole an. Siehe [Linux](/de/platforms/linux/#infobereich-unter-gnome).
- **Einstellungen.** **Im Infobereich anzeigen** ist möglicherweise ausgeschaltet. Öffnen Sie
  den Hub über die Taskleiste oder das Startmenü und schalten Sie die Option in den
  [Einstellungen](/de/using/settings/) wieder ein.

## Das Installationsprogramm zeigt einen Fehler

| Meldung | Was zu tun ist |
| --- | --- |
| `Install failed: <error>` | Der Text nach dem Doppelpunkt nennt den fehlgeschlagenen Schritt, zum Beispiel `copy exe: ...`. Wenn eine Datei in Benutzung ist, beenden Sie jedes Moonpool, das aus `%USERPROFILE%\.moonpool` läuft, und versuchen Sie es erneut. |
| `target folder does not exist` | Der für eine portable Kopie gewählte Ordner existiert nicht mehr. Wählen Sie einen vorhandenen Ordner. |
| `that folder already has a .moonpool with an exe of this name that isn't a portable Moonpool - pick an empty folder` | Wählen Sie einen leeren Ordner, oder entfernen Sie zuerst den Ordner `.moonpool` darin. |

## „Der Computer wurde durch Windows geschützt“ erscheint beim Ausführen des Installationsprogramms

Das ist Windows SmartScreen, weil `moonpool.exe` nicht codesigniert ist. Klicken Sie auf
**Weitere Informationen** und dann auf **Trotzdem ausführen**. Siehe
[Der Computer wurde durch Windows geschützt](/de/support/windows-protected-your-pc/).

## Das Moonpool-Fenster ist leer oder öffnet sich unter Windows nie

Möglicherweise fehlt die Microsoft Edge WebView2 Runtime. Siehe
[WebView2-Runtime fehlt](/de/support/webview2-runtime-missing/).

## Eine App startet nicht

1. Klicken Sie auf den Namen der App, um ihren Terminal-Tab zu öffnen und die Ausgabe zu
   lesen. Ein Agent kann denselben Text mit `moonpool_app_output` lesen.
2. Prüfen Sie `cwd`. Ein fehlender Ordner oder ein relativer Pfad ohne `./` ist die
   häufigste Ursache. Siehe [Pfade und Umgebung](/de/apps/paths-and-environment/).
3. Prüfen Sie `command`. Führen Sie ihn von Hand in einem Terminal in `cwd` aus. Vermeiden
   Sie unter Windows verschachtelte doppelte Anführungszeichen; `cmd /c` verstümmelt sie.
4. Schalten Sie in den Einstellungen **Debug-Infos in eine Datei schreiben** ein und starten
   Sie die App erneut. `moonpool.log` hält den genauen Befehl und den Ordner fest. Siehe
   [Protokolle](/de/data/logs/).

| Meldung | Bedeutung |
| --- | --- |
| `already running` | Moonpool hat für diese App bereits ein Terminal. Stoppen Sie sie zuerst oder nutzen Sie „Neu starten“. |
| `stopped during launch` | „Stoppen“ wurde gedrückt, während der Start noch lief. |
| `did not reach running in time` | Aus einem Skript oder von einem Agenten: Die App wurde innerhalb von 25 Sekunden nicht als „läuft“ erkannt. Prüfen Sie ihren `port` oder `processName` sowie ihre Ausgabe. |

## Der Statuspunkt ist falsch

Moonpool entscheidet über „läuft“ anhand von `port`, dann `processName` und dann danach, ob
das eigene Terminal noch lebt. Siehe
[Wie „läuft“ bestimmt wird](/de/apps/types/#wie-läuft-festgestellt-wird).

- **Wird nie ausgefüllt.** Der `port` einer `web`-App antwortet nicht, oder der
  `processName` einer `desktop`-App passt nicht. Unter Linux darf `processName` höchstens
  15 Zeichen lang sein.
- **Wird gleich nach dem Start grau.** Eine `cli`-App läuft nicht mehr, sobald ihr Befehl
  endet. Verwenden Sie eine Shell mit `-NoExit`, wenn sie geöffnet bleiben soll.
- **Eine `static`-App zeigt nie „läuft“.** Das ist bei einem Eintrag nur mit `url` zu
  erwarten.
- **Zeigt „läuft“, obwohl Sie die App nicht gestartet haben.** Etwas anderes belegt diesen
  Port oder Prozessnamen. Moonpool zeigt sie als laufend, aber nicht als „managed by
  Moonpool“.

## Error: listen EADDRINUSE oder „Port 5173 is in use“

Etwas anderes lauscht bereits auf dem Port, den Ihr Server verwenden will. Finden und
beenden Sie es, oder setzen Sie `port` bei der App, damit „Stoppen“ ihn freigibt. Siehe
[EADDRINUSE und „Port 5173 is in use“ beheben](/de/support/port-already-in-use/) und
[Den Prozess finden und beenden, der einen Port belegt](/de/guides/find-and-kill-process-using-port-windows/).

## Zwei Apps verwenden denselben Port

Unten im Menü **...** erscheint eine Warnzeile, zum Beispiel `Port 3000: App A / App B`.
Ändern Sie den `port` einer App (und ihr `env`, falls sie `PORT` liest). Siehe
[Warnung bei Port-Konflikt](/de/using/hub-window/#warnung-bei-port-konflikt).

## Die App läuft nach „Stoppen“ weiter

Aus einem Skript oder von einem Agenten lautet der Fehler `still running after stop` (nach
15 Sekunden).

- Die App überlebt ihr Terminal. Setzen Sie `killMode` auf `port` oder `processName`. Siehe
  [Stoppen und Neustarten](/de/apps/stop-and-restart/).
- Eine Docker-App unter Windows: Verwenden Sie `killMode` `command` mit einem `stopCommand`
  wie `docker compose stop app`. Niemals `port`.

## apps.json enthält einen Fehler

In der Seitenleiste erscheint ein Banner: „apps.json enthält einen Fehler, angezeigt wird die
zuletzt geladene Liste.“ oder beim Start „apps.json enthält einen Fehler, daher sind keine
Apps geladen.“ Das Speichern aus Moonpool ist pausiert, bis die Datei wieder geladen wird.

Typische Fehler:

```text
apps.json entry 2 (site) requires a command
apps.json entry 3 has invalid id "my app"; use letters, digits, '.', '_', and '-' without a leading '-'
duplicate app id "site"
apps.json entry 4 (api) has invalid port 0
```

1. Wählen Sie im Banner **apps.json bearbeiten**, korrigieren Sie den Eintrag, speichern Sie
   und wählen Sie dann **Neu laden** (F5).
2. Oder stellen Sie eine aktuelle gute Kopie wieder her. Siehe
   [Sicherung und Wiederherstellung](/de/data/backup-and-recovery/#appsjson-zurückrollen).

Die vollständige Regelliste steht unter [Validierung](/de/apps/apps-json/#prüfung).

Wenn sich eine Einstellung nicht ändern lässt und die Meldung mit `Repair settings.json and
restart Moonpool before changing settings` endet, korrigieren oder löschen Sie
`settings.json` im Konfigurationsordner und starten Sie Moonpool erneut. Das Löschen setzt
jede Einstellung auf ihren Standardwert zurück.

## Meine Änderung wirkt nicht

- Handgemachte Änderungen brauchen **Neu laden** (oder F5). Moonpool überwacht die Datei
  nicht.
- „Neu laden“ startet laufende Apps nicht neu. Starten Sie die App neu, damit ein geänderter
  `command`, `cwd` oder `env` verwendet wird.
- Ein Agent bearbeitet möglicherweise eine andere `apps.json`. Bitten Sie ihn,
  `moonpool_launcher_paths` aufzurufen und den Ordner des Hubs mit seinem eigenen zu
  vergleichen. Prüfen Sie bei mehreren Moonpool-Kopien, welche Kopie Sie bearbeiten.

## Die Beispiel-Apps fehlen

Beispiele werden nur geschrieben, wenn keine `apps.json` existiert. Um sie zurückzuholen,
siehe [Auf die Beispiele zurücksetzen](/de/data/backup-and-recovery/#auf-die-beispiele-zurücksetzen),
oder kopieren Sie die Einträge aus
[Beispiel-Dashboards](/de/getting-started/example-dashboards/#beispiel-apps-erscheinen-nur-beim-ersten-start).

## Ein Update ist fehlgeschlagen

Das Banner zeigt `Update failed: <error>` (Update fehlgeschlagen: `<error>`). Siehe
[Wenn ein Update fehlschlägt](/de/data/updating/#wenn-ein-update-fehlschlägt).

## Ein Weblink lässt sich nicht öffnen

`refusing to open non-web url: <url>` bedeutet, dass die `url` nicht mit `http://`,
`https://`, `mailto:` oder `file://` beginnt. Korrigieren Sie die `url`.

## MCP- und Skriptfehler

| Meldung | Was zu tun ist |
| --- | --- |
| `Moonpool is not running - call moonpool_bootup_launcher first` | Starten Sie Moonpool, oder lassen Sie den Agenten `moonpool_bootup_launcher` aufrufen. |
| `frontend not loaded` | Das Hub-Fenster ist noch nicht fertig geladen. Warten Sie einen Moment und versuchen Sie es erneut. |
| `stale token: ...` | `apps.json` hat sich geändert, seit der Agent sie gelesen hat. Lesen Sie sie erneut und schreiben Sie dann. |
| `rejected invalid manifest: ...` | Die neue `apps.json` hat die Validierung nicht bestanden. Die Datei wurde nicht geändert. |
| `... A Moonpool process may be hung ...` | Etwas hält den Steuerkanal, ohne zu antworten. Beenden Sie Moonpool über den Tray oder beenden Sie den Prozess und starten Sie es erneut. |

Mehr unter [MCP-Einrichtung](/de/automation/mcp-setup/#hinweise) und
[MCP-Tools](/de/automation/mcp-tools/).

## Fensterprobleme

- **Außerhalb des Bildschirms.** Moonpool ignoriert eine gespeicherte Position, die auf
  keinem angeschlossenen Display liegt. Wenn das Fenster trotzdem verschwunden bleibt,
  beenden Sie Moonpool und löschen Sie `window-state.json` im Konfigurationsordner.
- **Zoom bleibt zu groß oder zu klein.** Strg + Mausrad über dem Hub ändert ihn. Siehe
  [Tastenkürzel und Zoom](/de/using/keyboard-shortcuts/#zoom).
- **Die Einstellungen öffnen sich hinter dem Hub.** Schalten Sie in den Einstellungen
  **Immer im Vordergrund** aus oder ein. Die Option gilt für jedes Moonpool-Fenster, sodass
  alle in derselben Ebene bleiben.

## Wo sind die Protokolle?

Siehe [Protokolle](/de/data/logs/).

## Sichern, zurücksetzen oder deinstallieren

Siehe [Sicherung und Wiederherstellung](/de/data/backup-and-recovery/) und
[Deinstallieren](/de/getting-started/install/#deinstallieren).

## FAQ

**Beendet das Schließen des Fensters meine Apps?**
Standardmäßig beendet das Schließen Moonpool, und unter Windows stoppt das Beenden die Apps,
die es gestartet hat. Schalten Sie **Beim Schließen in den Infobereich** ein, damit Moonpool
beim Schließen des Fensters weiterläuft. Siehe
[Tray, Schließen und Minimieren](/de/using/tray-and-closing/).

**Kann ich Moonpool zweimal ausführen?**
Eine Instanz pro Ordner. Wenn Sie dieselbe Kopie erneut starten, kommt ihr Fenster wieder
nach vorn. Die installierte Kopie und portable Kopien können nebeneinander laufen. Siehe
[Portabler Modus](/de/data/portable-mode/#mehrere-kopien-gleichzeitig).

**Telefoniert Moonpool nach Hause?**
Nur um nach Updates zu suchen: Es lädt beim Start (wenn **Beim Start nach Updates suchen**
eingeschaltet ist) und beim Drücken von **Nach Updates suchen** die Release-Datei
(`update.json`) von GitHub. Jeder Download wird vor der Verwendung anhand des
Signaturschlüssels von Moonpool geprüft.

**Welche Shell führt meine Befehle aus?**
`cmd /c` unter Windows, `$SHELL -c` unter Linux.

**Wohin gehören Geheimnisse?**
`env`-Werte werden im Klartext in `apps.json` gespeichert. Bevorzugen Sie eine Datei, die
Ihre App selbst liest, oder eine Variable, die bereits in Ihrer Benutzerumgebung gesetzt
ist und die gestartete Apps erben.

**Ist der Steuerkanal geschützt?**
Er hat weder Anmeldung noch Token. Jeder Prozess, der unter Ihrem Konto läuft, kann ihm
Befehle senden. Unter Linux ist der Socket nur für Ihren Benutzer lesbar. Siehe
[Sicherheitseigenschaften](/de/automation/overview/#sicherheitseigenschaften).
