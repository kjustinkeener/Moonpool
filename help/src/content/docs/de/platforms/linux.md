---
title: "Moonpool unter Linux installieren und verwenden"
description: "Installieren Sie Moonpool unter Linux, umgehen Sie die Einschränkung beim GNOME-Infobereich, lernen Sie, wie Updates funktionieren, und sehen Sie die Unterschiede zu Windows."
---

Moonpool läuft unter Linux über WebKitGTK. Es wird hauptsächlich unter Windows entwickelt, daher
wird Linux unterstützt, ist aber weniger erprobt. Unter Linux gibt es weder eine Installationskarte
noch eine Auswahl für den portablen Modus, und das Menü „...“ hat keinen Eintrag **Moonpool installieren…**.

## Installation

Laden Sie ein Paket von der Releases-Seite des Projekts herunter.

| Paket | Updates |
| --- | --- |
| AppImage | Moonpool aktualisiert sich selbst |
| `.deb` | Ihre Paketverwaltung |
| RPM (mit dem RPM-Werkzeug Ihrer Distribution installieren) | Ihre Paketverwaltung |

```bash title="AppImage" frame="terminal"
chmod +x Moonpool_*.AppImage
./Moonpool_*.AppImage
```

```bash title=".deb" frame="terminal"
sudo apt install ./Moonpool_*_amd64.deb
```

```bash title="RPM" frame="terminal"
sudo dnf install ./Moonpool-*.x86_64.rpm
```

Das `.deb` zieht seine Laufzeitabhängigkeiten mit. Beim AppImage müssen die Bibliotheken WebKitGTK
und AppIndicator vorhanden sein, zum Beispiel unter Debian oder Ubuntu:

```bash frame="terminal"
sudo apt-get install -y libwebkit2gtk-4.1-0 libayatana-appindicator3-1
```

Unter Fedora oder Arch nutzen Sie die Entsprechungen:

```bash title="Fedora" frame="terminal"
sudo dnf install webkit2gtk4.1 libayatana-appindicator-gtk3
```

```bash title="Arch" frame="terminal"
sudo pacman -S webkit2gtk-4.1 libayatana-appindicator
```

## Infobereich unter GNOME

Reines GNOME zeigt keine Symbole im Infobereich, daher erscheint das Moonpool-Symbol erst, wenn die
AppIndicator-Erweiterung installiert und aktiviert ist:

```bash frame="terminal"
sudo apt-get install -y gnome-shell-extension-appindicator
gnome-extensions enable ubuntu-appindicators@ubuntu.com
```

Melden Sie sich danach ab und wieder an. Das Hub-Fenster und die eingebetteten Terminals
funktionieren auch ohne sie. KDE, Cinnamon, XFCE und MATE zeigen den Infobereich sofort an.

## Updates

Nur das AppImage aktualisiert sich selbst. Es liest `linux-update.json` aus den GitHub Releases,
prüft die minisign-Signatur und ersetzt die AppImage-Datei an Ort und Stelle, legen Sie sie also in
einem Ordner ab, in den Sie schreiben können. Installationen per `.deb` und RPM werden von Moonpool
nie überschrieben: Die Update-Suche kann weiterhin eine neuere Version melden, aber die Installation
aus Moonpool schlägt mit dem Hinweis fehl, Ihre Paketverwaltung zu nutzen. Siehe
[Aktualisieren](/de/data/updating/).

## Konfigurationsort

```text
~/.config/Moonpool/              (or $XDG_CONFIG_HOME/Moonpool/)
~/.config/Moonpool/apps.json
~/.config/Moonpool/dashboards/examples/   (example dashboards)
```

`apps.json` wird beim ersten Start aus dem Beispiel angelegt. Siehe
[Konfigurationsübersicht](/de/apps/apps-json/).

## Unterschiede zu Windows

- Startbefehle laufen über `$SHELL -c <command>` (`/bin/sh`, wenn `SHELL` nicht gesetzt ist), nutzen Sie also eine Syntax, die Ihre Shell versteht.
- Stoppen beendet die Prozessgruppe und führt dann die zusätzliche Bereinigung aus, die `killMode` festlegt. Das Freigeben eines Ports unter `killMode: "port"` nutzt `lsof` und weicht auf `fuser` aus; installieren Sie `lsof`, wenn Ihre Distribution es nicht mitbringt. Siehe [Stoppen und neu starten](/de/apps/stop-and-restart/).
- Der `processName` einer `desktop`-App darf höchstens 15 Zeichen lang sein. Linux kürzt einen Prozessnamen auf 15 Zeichen, daher wird ein längerer Name nie als laufend erkannt und lässt sich nicht über den Namen stoppen. `web`-Apps werden über ihren Port erkannt und sind nicht betroffen.
- Symbole werden aus `src-tauri/icons/`, `public/favicon.*`, `icon.png` einer App oder ihrem aktuellen Favicon gefunden. Das Extrahieren eines Symbols aus einer Binärdatei gibt es nur unter Windows.
- Die Schaltflächen zum Anzeigen öffnen den übergeordneten Ordner, statt die Datei auszuwählen.
- Konfigurationsdateien öffnen sich im Standard-Texteditor (aus der Zuordnung für `text/plain` ermittelt).
- Der Windows-Installer, die Verknüpfungen und der Eintrag unter „Apps hinzufügen/entfernen“ gelten nicht.

## Siehe auch

- [Windows](/de/platforms/windows/#unterschiede-je-plattform): eine Tabelle der Unterschiede je Plattform.
- [Aktualisieren](/de/data/updating/#linux)
