---
title: "Moonpool'u Linux'ta kurun ve kullanın"
description: "Moonpool'u Linux'a kurun, GNOME sistem tepsisi sorununu aşın, güncellemelerin nasıl çalıştığını öğrenin ve Windows sürümünden hangi özelliklerin farklı olduğunu görün."
---

Moonpool, Linux'ta WebKitGTK üzerinden çalışır. Esas olarak Windows'ta geliştirilir; bu
yüzden Linux desteklenir ancak daha az sınanmıştır. Linux'ta kurulum kartı ya da taşınabilir
mod seçicisi yoktur ve "..." menüsünde **Moonpool'u kur…** öğesi bulunmaz.

## Kurulum

Projenin Sürümler sayfasından bir paket indirin.

| Paket | Güncellemeler |
| --- | --- |
| AppImage | Moonpool kendini günceller |
| `.deb` | Paket yöneticiniz |
| RPM (dağıtımınızın RPM aracıyla kurun) | Paket yöneticiniz |

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

`.deb` çalışma zamanı bağımlılıklarını kendisi getirir. AppImage için WebKitGTK ve
AppIndicator kitaplıklarının kurulu olması gerekir; örneğin Debian veya Ubuntu'da:

```bash frame="terminal"
sudo apt-get install -y libwebkit2gtk-4.1-0 libayatana-appindicator3-1
```

Fedora veya Arch'ta karşılıklarını kullanın:

```bash title="Fedora" frame="terminal"
sudo dnf install webkit2gtk4.1 libayatana-appindicator-gtk3
```

```bash title="Arch" frame="terminal"
sudo pacman -S webkit2gtk-4.1 libayatana-appindicator
```

## GNOME'da sistem tepsisi

Standart GNOME sistem tepsisi simgelerini göstermez; bu yüzden AppIndicator eklentisi kurulup
etkinleştirilene kadar Moonpool'un sistem tepsisi simgesi görünmez:

```bash frame="terminal"
sudo apt-get install -y gnome-shell-extension-appindicator
gnome-extensions enable ubuntu-appindicators@ubuntu.com
```

Ardından oturumu kapatıp yeniden açın. Merkez penceresi ve gömülü terminaller bu olmadan da
çalışır. KDE, Cinnamon, XFCE ve MATE sistem tepsisini kutudan çıktığı gibi gösterir.

## Güncellemeler

Yalnızca AppImage kendini günceller. GitHub Releases'tan `linux-update.json` dosyasını okur,
minisign imzasını doğrular ve AppImage dosyasını yerinde değiştirir; bu yüzden onu yazabildiğiniz
bir klasörde tutun. `.deb` ve RPM kurulumlarının üzerine Moonpool asla yazmaz: güncelleme
denetimi yine de daha yeni bir sürüm bildirebilir, ancak Moonpool'dan kurmak, paket yöneticinizi
kullanmanızı söyleyen bir iletiyle başarısız olur. Bkz. [Güncelleme](/tr/data/updating/).

## Yapılandırma konumu

```text
~/.config/Moonpool/              (or $XDG_CONFIG_HOME/Moonpool/)
~/.config/Moonpool/apps.json
~/.config/Moonpool/dashboards/examples/   (example dashboards)
```

`apps.json`, ilk çalıştırmada örnekten oluşturulur. Bkz.
[Yapılandırmaya genel bakış](/tr/apps/apps-json/).

## Windows'tan farklar

- Başlatma komutları `$SHELL -c <command>` üzerinden çalışır (`SHELL` ayarlı değilse `/bin/sh`), bu yüzden kabuğunuzun anladığı sözdizimini kullanın.
- Durdur, süreç grubunu sonlandırır, ardından `killMode` tarafından seçilen ek temizliği yapar. `killMode: "port"` altında bir bağlantı noktasını serbest bırakmak `lsof` kullanır, olmazsa `fuser`'a döner; dağıtımınız `lsof` ile gelmiyorsa onu kurun. Bkz. [Durdurma ve yeniden başlatma](/tr/apps/stop-and-restart/).
- Bir `desktop` uygulamasının `processName` değeri 15 karakter veya daha kısa olmalıdır. Linux bir süreç adını 15 karaktere kısaltır; bu yüzden daha uzun bir ad asla çalışıyor olarak algılanmaz ve ada göre durdurulamaz. `web` uygulamaları bağlantı noktalarıyla eşleşir ve bundan etkilenmez.
- Simgeler, uygulamanın `src-tauri/icons/`, `public/favicon.*`, `icon.png` dosyalarından ya da canlı favicon'undan bulunur. Bir ikili dosyadan simge çıkarmak yalnızca Windows'a özgüdür.
- Göster düğmeleri dosyayı seçmek yerine onu içeren klasörü açar.
- Yapılandırma dosyaları varsayılan metin düzenleyicinizde açılır (`text/plain` ilişkilendirmesinden çözülür).
- Windows kurulum programı, kısayollar ve Program Ekle/Kaldır girdisi geçerli değildir.

## Ayrıca bakın

- [Windows](/tr/platforms/windows/#platforma-göre-farklar): platforma göre nelerin farklı olduğunu gösteren bir tablo.
- [Güncelleme](/tr/data/updating/#linux)
