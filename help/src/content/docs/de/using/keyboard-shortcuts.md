---
title: "Moonpool: Tastenkürzel, Mausaktionen und Zoom"
description: "Alle Tastatur- und Mauskürzel im Moonpool-Hub und wie Sie die Oberfläche vergrößern und verkleinern, damit sich der Text angenehm lesen lässt."
---

## Tastatur

| Tasten | Wo | Funktion |
| --- | --- | --- |
| F5, Strg+R, Cmd+R | Hub | Lädt `apps.json` von der Festplatte neu, genau wie **Neu laden** im Menü. Die Seite selbst wird nicht aktualisiert. |
| Esc | Kontextmenü | Schließt es. |
| Esc | Beim Umbenennen einer App | Bricht das Umbenennen ab. |
| Esc | Fenster Einstellungen, Über und App-Editor | Schließt das Fenster (der Editor fragt vor dem Verwerfen von Änderungen nach). |
| Enter | Beim Umbenennen einer App | Speichert den neuen Namen. |

## Maus

| Aktion | Wo | Funktion |
| --- | --- | --- |
| Strg + Mausrad | Hub | Zoomt die Oberfläche. |
| Text markieren | Terminal | Kopiert und hebt die Markierung auf. |
| Mittelklick | Terminal | Fügt ein. |
| Rechtsklick | Seitenleistenzeile | Öffnet das Zeilenmenü. Siehe [Seitenleiste und Menüs](/de/using/sidebar-and-menus/). |

## Zoom

Halten Sie Strg gedrückt und drehen Sie das Mausrad über dem Hub, um zu zoomen. Nach oben scrollen vergrößert, nach unten verkleinert, in Schritten von etwa 10 Prozent pro Mausradschritt.

```text
Ctrl + wheel up      zoom in
Ctrl + wheel down    zoom out
```

- Der Bereich reicht von 0,5x bis 3x.
- Das Fenster ändert seine Größe um denselben Faktor, sodass das Layout bei 2x so kompakt bleibt wie bei 1x. Ist die Grenze erreicht, wächst das Fenster nicht weiter.
- Der Faktor wird als `uiScale` in `settings.json` gespeichert und beim nächsten Start angewendet. Die gespeicherte Fenstergröße ist bereits die gezoomte Größe und wird daher nicht erneut skaliert. Siehe [settings.json](/de/data/settings-json/).

- Der Zoom gilt nur für das Hub-Fenster. Einstellungen, Über, der App-Editor und die Hilfe behalten
  ihre eigene Größe.

Es gibt in den Einstellungen kein Bedienelement für `uiScale` und keine Taste zum Zurücksetzen. Um zur normalen Größe zurückzukehren,
scrollen Sie dieselbe Anzahl Rastungen zurück, oder beenden Sie Moonpool, setzen Sie `uiScale` in
`settings.json` auf `1` (oder entfernen Sie den Schlüssel) und starten Sie es erneut.

## Siehe auch

- [Designs, Sprache und Transparenz](/de/using/themes-and-language/)
- [Seitenleiste und Menüs](/de/using/sidebar-and-menus/)
