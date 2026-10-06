---
title: "Installa e usa Moonpool su Linux"
description: "Installa Moonpool su Linux, aggira il problema dell'area di notifica di GNOME, scopri come funzionano gli aggiornamenti e cosa cambia rispetto a Windows."
---

Moonpool funziona su Linux tramite WebKitGTK. Viene sviluppato soprattutto su Windows, quindi
Linux è supportato ma meno collaudato. Su Linux non c'è la scheda di installazione né la scelta
della modalità portatile, e il menu «...» non ha la voce **Installa Moonpool...**.

## Installazione

Scarica un pacchetto dalla pagina Releases del progetto.

| Pacchetto | Aggiornamenti |
| --- | --- |
| AppImage | Moonpool si aggiorna da solo |
| `.deb` | Il tuo gestore di pacchetti |
| RPM (da installare con lo strumento RPM della tua distribuzione) | Il tuo gestore di pacchetti |

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

Il `.deb` installa da sé le sue dipendenze di runtime. L'AppImage richiede che siano presenti le
librerie WebKitGTK e AppIndicator, ad esempio su Debian o Ubuntu:

```bash frame="terminal"
sudo apt-get install -y libwebkit2gtk-4.1-0 libayatana-appindicator3-1
```

Su Fedora o Arch usa gli equivalenti:

```bash title="Fedora" frame="terminal"
sudo dnf install webkit2gtk4.1 libayatana-appindicator-gtk3
```

```bash title="Arch" frame="terminal"
sudo pacman -S webkit2gtk-4.1 libayatana-appindicator
```

## Area di notifica su GNOME

GNOME standard non mostra le icone dell'area di notifica, quindi l'icona di Moonpool non
comparirà finché non viene installata e abilitata l'estensione AppIndicator:

```bash frame="terminal"
sudo apt-get install -y gnome-shell-extension-appindicator
gnome-extensions enable ubuntu-appindicators@ubuntu.com
```

Poi esci e rientra nella sessione. La finestra hub e i terminali integrati funzionano anche
senza. KDE, Cinnamon, XFCE e MATE mostrano l'area di notifica di serie.

## Aggiornamenti

Solo l'AppImage si aggiorna da solo. Legge `linux-update.json` da GitHub Releases, verifica la
firma minisign e sostituisce il file AppImage sul posto, quindi tienilo in una cartella in cui
puoi scrivere. Le installazioni `.deb` e RPM non vengono mai sovrascritte da Moonpool: il
controllo degli aggiornamenti può comunque segnalare una versione più recente, ma installarla da
Moonpool fallisce con un messaggio che invita a usare il gestore di pacchetti. Vedi
[Aggiornamenti](/it/data/updating/).

## Posizione della configurazione

```text
~/.config/Moonpool/              (or $XDG_CONFIG_HOME/Moonpool/)
~/.config/Moonpool/apps.json
~/.config/Moonpool/dashboards/examples/   (example dashboards)
```

`apps.json` viene creato a partire dall'esempio al primo avvio. Vedi
[Panoramica della configurazione](/it/apps/apps-json/).

## Differenze rispetto a Windows

- I comandi di avvio vengono eseguiti tramite `$SHELL -c <command>` (`/bin/sh` se `SHELL` non è impostata), quindi usa una sintassi che la tua shell capisce.
- Arresta termina il gruppo di processi, poi esegue la pulizia aggiuntiva scelta da `killMode`. Liberare una porta con `killMode: "port"` usa `lsof`, con ripiego su `fuser`; installa `lsof` se la tua distribuzione non lo include. Vedi [Arresto e riavvio](/it/apps/stop-and-restart/).
- Il `processName` di un'app `desktop` deve essere di 15 caratteri o meno. Linux tronca il nome di un processo a 15 caratteri, quindi un nome più lungo non viene mai rilevato come in esecuzione e non può essere arrestato per nome. Le app `web` vengono individuate dalla porta e non sono interessate.
- Le icone vengono trovate da `src-tauri/icons/` dell'app, `public/favicon.*`, `icon.png` o dalla sua favicon attiva. Estrarre un'icona da un binario è possibile solo su Windows.
- I pulsanti di apertura aprono la cartella che contiene il file invece di selezionarlo.
- I file di configurazione si aprono nel tuo editor di testo predefinito (individuato dall'associazione `text/plain`).
- L'installer di Windows, i collegamenti e la voce di Installazione applicazioni non si applicano.

## Vedi anche

- [Windows](/it/platforms/windows/#differenze-tra-piattaforme): una tabella delle differenze tra piattaforme.
- [Aggiornamenti](/it/data/updating/#linux)
