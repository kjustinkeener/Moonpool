---
title: "Moonpool unter Windows oder Linux installieren"
description: "Installieren Sie Moonpool mit wenigen Klicks, wählen Sie installierten oder portablen Modus, nutzen Sie den Menüpunkt zum Installieren später und deinstallieren Sie sauber."
---

Diese Seite gilt für Windows. Unter Windows ist Moonpool sein eigener Installer: Der Download ist
eine einzelne `moonpool.exe`. Linux hat weder eine Installationskarte noch eine Auswahl für den
portablen Modus; siehe [Linux](/de/platforms/linux/).

## Installierter Modus

Führen Sie die heruntergeladene `moonpool.exe` aus. Beim ersten Start zeigt sie die
Installationskarte. Sie hat drei Bedienelemente: die Schaltfläche **Moonpool installieren**, ein
Kontrollkästchen **Verknüpfung auf dem Desktop anlegen** (standardmäßig aktiv) und einen Link
**Portabel installieren**.

Die Installation kopiert Moonpool in Ihr Benutzerprofil unter `.moonpool\`, legt eine
Startmenü-Verknüpfung an (und eine auf dem Desktop, wenn das Kästchen aktiviert ist) und trägt einen
Eintrag unter „Apps hinzufügen/entfernen“ ein. Danach startet sie die installierte Kopie und
schließt sich. Die heruntergeladene Datei bleibt, wo sie war; Sie können sie löschen. Starten Sie
Moonpool danach über die Verknüpfung wie jede andere App.

![Die Installationskarte: Schaltfläche „Moonpool installieren“, Kontrollkästchen für die Desktop-Verknüpfung, Link „Portabel installieren“ und der Installationspfad](../../../../assets/screenshots/installer-window.png)

Alles, was Moonpool braucht, liegt in diesem einen Ordner: das Programm, Ihre Konfiguration und
die mitgelieferte Hilfe.

```text title="Installed layout"
%USERPROFILE%\.moonpool\
```

## Moonpool installieren... über das Menü

Unter Windows hat das Menü „...“ in beiden Modi den Eintrag **Moonpool installieren…**. Er öffnet
dieselbe Installationskarte. Aus einer portablen Kopie können Sie Moonpool so richtig installieren.
In einer installierten Kopie ist **Moonpool installieren** deaktiviert („Bereits installiert“),
und **Portabel installieren** bleibt verfügbar.

## Deinstallieren

Nutzen Sie „Apps hinzufügen/entfernen“ von Windows (Installierte Apps) oder führen Sie die
installierte Kopie mit `--uninstall` aus. Sie steht nicht im PATH, geben Sie also den vollen Pfad an:

```powershell frame="terminal"
& "$env:USERPROFILE\.moonpool\moonpool.exe" --uninstall
```

Das entfernt die Verknüpfungen im Startmenü und auf dem Desktop, den Registrierungseintrag und den
gesamten Ordner `%USERPROFILE%\.moonpool`, **einschließlich Ihrer Konfiguration** (`apps.json`,
Einstellungen und Protokolle). Sichern Sie diesen Ordner zuerst, wenn Sie Ihre Konfiguration
behalten möchten:

```text
%USERPROFILE%\.moonpool\moonpool-config
```

Jede laufende Moonpool-Instanz wird im Zuge der Deinstallation beendet.

## Portabler Modus

Lieber einen USB-Stick oder einen verschiebbaren Ordner? Klicken Sie auf der Installationskarte auf
**Portabel installieren** und wählen Sie einen Ordner. Siehe [Portabler Modus](/de/data/portable-mode/).

## Weiter

- [Windows hat Ihren PC geschützt](/de/support/windows-protected-your-pc/): wenn SmartScreen den Installer blockiert.
- [WebView2-Runtime fehlt](/de/support/webview2-runtime-missing/): wenn das Fenster leer bleibt.
- [Ihre erste App](/de/getting-started/first-app/)
