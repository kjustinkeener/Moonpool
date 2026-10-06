---
title: "Moonpool-Design, Sprache und Transparenz ändern"
description: "Wählen Sie ein Farbdesign und die Oberflächensprache, stellen Sie Hintergrundtransparenz und UI-Skalierung ein und sehen Sie sie sofort in jedem Moonpool-Fenster."
---

Design, Sprache und Transparenz stellen Sie im [Einstellungsfenster](/de/using/settings/) ein.
Alle drei wirken sofort in jedem geöffneten Moonpool-Fenster.

![Auswahl für Sprache (1) und Design (2) oben in den Einstellungen](../../../../assets/screenshots/settings-language-theme.png)

1. Sprachauswahl.
2. Design-Schaltfläche. Sie zeigt den Namen des aktuellen Designs und öffnet den Design-Browser.

## Designs

Der Design-Browser ist ein eigenes Fenster. Er enthält eine Vorschaukarte pro Design, jeweils in den
eigenen Farben dieses Designs gezeichnet (Text, Panel, Eingabefeld, Schaltfläche, Statuspunkte, der Verlauf der Anzeige und der
Terminal-Satz mit 16 Farben), gruppiert als Grundlegend, Neon, Warm, Kühl, Grün, Neutral, Hell, Rosé,
Leuchtend, Helle Pastelltöne und Pastell. Klicken Sie auf eine Karte, um sie anzuwenden: Jedes geöffnete Moonpool-Fenster
ändert sich sofort, und die Wahl wird gespeichert. Das Fenster bleibt zum Vergleichen geöffnet; mit
Esc schließen Sie es.

Es gibt 68 Designs plus Automatisch, und etwa die Hälfte davon ist hell. Designnamen sind Eigennamen
und werden nicht übersetzt; nur Automatisch (System), Dunkel und Hell sind übersetzt.

**Automatisch (System)** folgt der Hell- oder Dunkel-Einstellung des Betriebssystems und wechselt live,
wenn das Betriebssystem wechselt. Jede andere Wahl ist fest. Auch die 16 ANSI-Farben des Terminals folgen dem
Design.

Haben Sie in einer früheren Version ein Design gespeichert, bleibt es erhalten. Ein gespeicherter Name, den Moonpool nicht mehr
kennt, fällt auf Automatisch zurück. Einige Bezeichnungen weichen von früher ab (zum Beispiel heißt Matrix jetzt
Terminal, Nord heißt Arctic, Dracula heißt Nocturne, Gruvbox heißt Retro und Solarized heißt
Solar); die gespeicherte Wahl selbst bleibt unverändert.

Das Design wird im `localStorage` der WebView gespeichert, nicht in `settings.json`. Ist der Speicher
nicht verfügbar, fällt es auf Automatisch zurück.

```text
localStorage key: moonpool.theme
```

## Sprachen

Automatisch folgt der Sprache des Betriebssystems. Andernfalls wählen Sie eine von 14, jeweils in ihrer eigenen Sprache angezeigt:

```text
English, Deutsch, Español, Français, Italiano, 日本語, 한국어, Nederlands, Polski,
Português (Brasil), Русский, Türkçe, 简体中文, 繁體中文
```

Die Auswahl wirkt sofort im Hub und in den anderen Fenstern. Die Wahl wird als
`locale` in `settings.json` gespeichert.

## Transparenz

**Hintergrundtransparenz** macht den Fensterhintergrund durchscheinend, von 0 % (deckend) bis
90 %.

- Fahren Sie mit dem Zeiger über ein Fenster, wird es sofort vollständig deckend. Verlässt der Zeiger es, blendet es in etwa 2 Sekunden wieder auf Ihre Einstellung zurück.
- Terminals folgen derselben Tönung, statt eine eigene hinzuzufügen.
- Jedes Fenster (Hub, Einstellungen, Über, der App-Editor und der Design-Browser) wendet die Einstellung selbst an, und die Einstellungen aktualisieren die anderen live, während Sie den Regler ziehen.

## UI-Skalierung

Zoomen Sie die gesamte Oberfläche mit Strg + Mausrad. Es gibt keinen Tastaturzoom. Siehe
[Tastenkürzel und Zoom](/de/using/keyboard-shortcuts/).

## Siehe auch

- [Einstellungsfenster](/de/using/settings/)
- [Tastenkürzel und Zoom](/de/using/keyboard-shortcuts/)
