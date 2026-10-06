---
title: "Comprendre settings.json et réparer un fichier endommagé"
description: "Voyez la structure du fichier settings.json de Moonpool, les clés que Moonpool écrit pour vous, et comment le lire et le réparer lorsqu'il est endommagé."
---

Les paramètres généraux de l'app se trouvent dans `settings.json` dans le dossier de configuration (voir
[Où se trouve la configuration](/fr/apps/apps-json/#où-se-trouve-la-configuration)). Modifiez-les dans la
[fenêtre Paramètres](/fr/using/settings/), qui liste chaque paramètre avec sa clé JSON et sa valeur par
défaut. Les journaux et leur conservation sont décrits sur la page [Journaux](/fr/data/logs/).

## Structure

Un seul objet JSON. Les clés omises prennent leur valeur par défaut :

```json title="settings.json"
{
  "closeToTray": true,
  "transparency": 20,
  "debugLogging": true,
  "cliLogging": true,
  "logRetentionMb": 25
}
```

| Clé | Valeur par défaut |
| --- | --- |
| `closeToTray` | `false` |
| `minimizeToTray` | `true` |
| `showInTray` | `true` |
| `showInTaskbar` | `true` |
| `alwaysOnTop` | `false` |
| `transparency` | `0` (0 à 90) |
| `showStatusbar` | `true` |
| `showMcpProcesses` | `true` |
| `checkOnStartup` | `true` |
| `locale` | `"auto"` |
| `debugLogging` | `false` |
| `cliLogging` | `false` |
| `logRetentionMb` | `10` (minimum 1) |

## Clés écrites pour vous

Moonpool stocke aussi dans ce fichier le zoom de l'interface (`uiScale`, de 0,5 à 3,0) et la langue résolue
(`localeResolved`). Vous n'avez besoin de définir ni l'un ni l'autre. Le thème n'y figure pas : il est
conservé dans le stockage du webview (voir [Thèmes, langue et transparence](/fr/using/themes-and-language/)).

## Lecture et réparation

Moonpool lit le fichier au démarrage. Les modifications faites pendant son exécution ne sont pas prises en compte ; quittez d'abord.

Si le fichier est mal formé, Moonpool démarre avec les valeurs par défaut et refuse de modifier les paramètres.
L'erreur se termine par `Repair settings.json and restart Moonpool before changing settings`.
Corrigez le fichier, ou supprimez-le pour réinitialiser tous les paramètres, puis redémarrez Moonpool.
