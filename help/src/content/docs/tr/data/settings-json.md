---
title: "settings.json dosyasını anlama ve bozuksa onarma"
description: "Moonpool'un settings.json dosyasının biçimini, Moonpool'un sizin için hangi anahtarları yazdığını ve dosya bozulduğunda nasıl okunup onarılacağını görün."
---

Uygulama genelindeki ayarlar, yapılandırma klasöründeki `settings.json` dosyasında bulunur (bkz.
[Yapılandırmanın bulunduğu yer](/tr/apps/apps-json/#yapılandırmanın-bulunduğu-yer)). Bunları, her ayarı JSON anahtarı ve
varsayılanıyla listeleyen [Ayarlar penceresinden](/tr/using/settings/) değiştirin. Günlükler ve saklamaları
[Günlükler](/tr/data/logs/) sayfasındadır.

## Biçim

Tek bir JSON nesnesi. Atladığınız anahtarlar varsayılanlarını alır:

```json title="settings.json"
{
  "closeToTray": true,
  "transparency": 20,
  "debugLogging": true,
  "cliLogging": true,
  "logRetentionMb": 25
}
```

| Anahtar | Varsayılan |
| --- | --- |
| `closeToTray` | `false` |
| `minimizeToTray` | `true` |
| `showInTray` | `true` |
| `showInTaskbar` | `true` |
| `alwaysOnTop` | `false` |
| `transparency` | `0` (0 ile 90 arası) |
| `showStatusbar` | `true` |
| `showMcpProcesses` | `true` |
| `checkOnStartup` | `true` |
| `locale` | `"auto"` |
| `debugLogging` | `false` |
| `cliLogging` | `false` |
| `logRetentionMb` | `10` (en az 1) |

## Sizin için yazılan anahtarlar

Moonpool ayrıca arayüz yakınlaştırmasını (`uiScale`, 0.5 ile 3.0 arası) ve çözümlenen dili
(`localeResolved`) bu dosyada saklar. İkisini de ayarlamanız gerekmez. Tema burada değildir: web
görünümünün depolamasında tutulur (bkz. [Temalar, dil ve saydamlık](/tr/using/themes-and-language/)).

## Okuma ve onarım

Moonpool dosyayı başlangıçta okur. Çalışırken yapılan düzenlemeler algılanmaz; önce çıkın.

Dosya hatalı biçimlendirilmişse Moonpool varsayılanlarla başlar ve ayarları değiştirmeyi reddeder.
Hata `Repair settings.json and restart Moonpool before changing settings` ile biter.
Dosyayı düzeltin ya da tüm ayarları sıfırlamak için silin, ardından Moonpool'u yeniden başlatın.
