---
title: "Der Computer wurde durch Windows geschützt: Moonpool-Installer trotzdem ausführen"
description: "Windows SmartScreen warnt beim Start von moonpool.exe. Warum das passiert, wie Sie Weitere Informationen und Trotzdem ausführen wählen und was Sie vorher prüfen."
---

Wenn Sie die heruntergeladene `moonpool.exe` ausführen, zeigt Windows möglicherweise ein blaues Feld mit dem Titel **Windows
protected your PC** (**Der Computer wurde durch Windows geschützt**) und der Zeile „Microsoft Defender SmartScreen prevented an unrecognized
app from starting. Running this app might put your PC at risk.“ (Microsoft Defender SmartScreen hat den Start einer unbekannten
App verhindert. Das Ausführen dieser App kann ein Risiko für den PC darstellen.)

## Warum sie erscheint

SmartScreen warnt vor Programmen, die neu sind oder die es nicht auf vielen PCs hat laufen sehen.
`moonpool.exe` ist nicht per Code signiert, daher hat Windows keinen Herausgeber, dem es vertrauen kann, und zeigt
beim ersten Mal womöglich die Warnung. Es ist eine Reputationsprüfung, kein Befund, dass die Datei schädlich ist.

## Was zu tun ist

1. Klicken Sie im Feld auf **Weitere Informationen** (**More info**). Als Herausgeber steht „Unbekannter Herausgeber“ („Unknown publisher“).
2. Klicken Sie auf **Trotzdem ausführen** (**Run anyway**). Die Installationskarte öffnet sich. Siehe [Installation](/de/getting-started/install/).

Wenn Sie vorher vorsichtig sein möchten, laden Sie nur von der offiziellen Website von Moonpool oder von seinen GitHub-
Releases herunter und prüfen Sie, dass die Datei `moonpool.exe` heißt.

## Wenn es keine Schaltfläche „Trotzdem ausführen“ gibt

Auf manchen verwalteten PCs schaltet der Administrator die Option ab, und Sie sehen kein **Trotzdem
ausführen** (**Run anyway**). Fragen Sie Ihren Administrator oder nutzen Sie einen PC, den Sie selbst verwalten. Auch eine Datei aus einem heruntergeladenen
ZIP kann eine Sperre tragen: Klicken Sie mit der rechten Maustaste auf die Datei, wählen Sie **Eigenschaften**, setzen Sie
das Häkchen bei **Zulassen** (**Unblock**), falls es erscheint, dann **OK**, und führen Sie sie erneut aus.

## Virenscanner-Warnungen

Eine neue unsignierte EXE, die sich in Ihr Profil kopiert und sich bei einem Update selbst ersetzt, kann
auch Virenschutzsoftware auslösen. Blockiert oder isoliert Ihre `moonpool.exe`, erlauben Sie sie für
den Ordner `.moonpool`. Siehe [Windows](/de/platforms/windows/#vor-dem-start).

## Siehe auch

- [Installation](/de/getting-started/install/)
- [Windows](/de/platforms/windows/)
- [Der Installer zeigt einen Fehler](/de/support/troubleshooting/#das-installationsprogramm-zeigt-einen-fehler)
