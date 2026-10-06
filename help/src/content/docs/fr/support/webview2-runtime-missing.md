---
title: "Runtime WebView2 manquant : corriger une fenêtre Moonpool vide ou absente sous Windows"
description: "Si la fenêtre de Moonpool ne s'ouvre pas ou reste vide sous Windows, le runtime Microsoft Edge WebView2 est peut-être absent. Comment le vérifier et l'installer."
---

Si la fenêtre de Moonpool ne s'ouvre jamais, ou s'ouvre et reste vide, sous Windows, la cause
probable est l'absence du runtime Microsoft Edge WebView2. Moonpool est une app Tauri, et ses
fenêtres sont des pages web affichées par WebView2.

WebView2 est fourni avec Windows 11 et avec les versions récentes de Windows 10 ; la plupart des
PC l'ont donc déjà. Il peut manquer sur un Windows 10 ancien ou allégé, ou sur un PC où il a été
supprimé. Le code source de Moonpool ne montre pas de message dédié à ce cas : aucun texte
d'erreur n'est donc cité ici, le symptôme étant une fenêtre qui n'apparaît pas ou qui est vide.

## Vérifier s'il est installé

Dans PowerShell, cherchez la version du runtime dans le registre (le premier chemin correspond à
l'installation pour tout le système, le second à une installation par utilisateur) :

```powershell frame="terminal"
Get-ItemProperty "HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -Name pv
Get-ItemProperty "HKCU:\Software\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -Name pv
```

Un numéro de version comme `120.0.2210.91` signifie qu'il est installé. Une erreur pour les deux
signifie qu'il ne l'est pas.

## L'installer

Téléchargez le runtime WebView2 **Evergreen** depuis la page WebView2 de Microsoft (cherchez
« WebView2 Runtime download »), exécutez l'installateur, puis redémarrez Moonpool. Le runtime
Evergreen se met à jour tout seul.

## S'il est installé et que la fenêtre reste vide

- Quittez tous les Moonpool depuis la zone de notification (ou terminez `moonpool.exe` dans le
  Gestionnaire des tâches) et redémarrez-le.
- Activez **Journaliser les infos de débogage dans un fichier** dans les
  [Paramètres](/fr/using/settings/) si vous pouvez y accéder, et consultez `moonpool.log`. Voir
  [Journaux](/fr/data/logs/).
- Si la fenêtre s'ouvre mais se trouve hors de l'écran, voir
  [Problèmes de fenêtre](/fr/support/troubleshooting/#problèmes-de-fenêtre).

## Voir aussi

- [Windows](/fr/platforms/windows/#avant-de-lexécuter)
- [Installation](/fr/getting-started/install/)
- [Dépannage](/fr/support/troubleshooting/)
