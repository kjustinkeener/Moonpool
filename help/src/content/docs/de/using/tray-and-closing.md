---
title: "Moonpool im Tray halten: Verhalten beim Schließen, Minimieren und Beenden"
description: "Steuern Sie, was Tray-Symbol, Schließen-Schaltfläche, Minimieren und Beenden tun, halten Sie das Fenster immer im Vordergrund und vermeiden Sie, Tray und Taskleiste zugleich auszublenden."
---

## Tray-Symbol

| Aktion | Ergebnis |
| --- | --- |
| Linksklick | Zeigt das Hub-Fenster (stellt es wieder her, wenn es minimiert oder ausgeblendet ist). |
| Rechtsklick | Menü mit nur **Moonpool anzeigen** und **Beenden** (in Ihrer Sprache). |

Laufen mehrere Moonpool-Kopien, hat jede ihr eigenes Tray-Symbol. Der Tooltip nennt, um welche Kopie
es sich handelt. Siehe [Portabler Modus](/de/data/portable-mode/#mehrere-kopien-gleichzeitig).

## Beenden

**Beenden** schließt Moonpool und stoppt unter Windows jede App, die Moonpool gestartet hat, einschließlich ihrer
Kindprozesse. Apps, die bereits liefen, bevor Moonpool sie sah (als laufend angezeigt, aber
ohne „managed by Moonpool“), bleiben unberührt. Unter Linux und macOS stoppt das Beenden gestartete Apps nicht zuverlässig.

## Schließen und Minimieren

Die Schließen-Schaltfläche beendet Moonpool standardmäßig (`closeToTray` ist `false`). Schalten Sie in den Einstellungen **Beim Schließen in den Infobereich** ein, dann blendet das Schließen das Fenster stattdessen in den Tray aus. Moonpool läuft weiter,
und das Tray-Symbol oder **Moonpool anzeigen** holt es zurück.

**Beim Minimieren in den Infobereich** (`minimizeToTray`, standardmäßig an) blendet das Fenster beim
Minimieren in den Tray aus, und es verschwindet aus der Taskleiste. Schalten Sie es aus, um wie üblich in die Taskleiste zu minimieren.

![Einstellungen: Beim Schließen in den Infobereich und Beim Minimieren in den Infobereich (1) sowie der Regler Hintergrundtransparenz (2)](../../../../assets/screenshots/settings-tray-and-transparency.png)

1. **Beim Schließen in den Infobereich** und **Beim Minimieren in den Infobereich**.
2. **Hintergrundtransparenz**. Siehe [Designs, Sprache und Transparenz](/de/using/themes-and-language/#transparenz).

## Sperre von Tray und Taskleiste

**Im Infobereich anzeigen** und **In der Taskleiste anzeigen** legen fest, ob das Tray-Symbol und die Taskleisten-Schaltfläche
sichtbar sind. Mindestens eines muss aktiv bleiben, sonst gäbe es für ein ausgeblendetes Fenster keinen Weg zurück.
Ist nur eines aktiv, ist sein Kontrollkästchen deaktiviert, bis Sie das andere wieder einschalten.

## Immer im Vordergrund

**Immer im Vordergrund** in den Einstellungen hält jedes Moonpool-Fenster (das Hub, Einstellungen, Über, den App-Editor, den Design-Browser, den Installer und die Hilfe) über anderen Fenstern. Standardmäßig ist es
aus.

## Siehe auch

- [Einen npm-Entwicklungsserver unter Windows im Hintergrund ausführen](/de/guides/run-npm-dev-server-in-background-windows/)
- [Ein Skript oder einen Entwicklungsserver automatisch bei der Windows-Anmeldung starten](/de/guides/start-app-at-windows-login/)
- [Einstellungsfenster](/de/using/settings/)
- [Das Hub-Fenster](/de/using/hub-window/)
