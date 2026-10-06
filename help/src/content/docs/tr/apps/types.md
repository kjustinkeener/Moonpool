---
title: "Uygulama türü seçme: web, desktop, static veya cli"
description: "web, desktop, static ve cli uygulamalarının Moonpool'da nasıl başlatıldığını, her biri için Çalışıyor durumunun nasıl algılandığını ve Durdur düğmesinin varsayılan işlevini öğrenin."
---

`type`, hangi alanların önemli olduğunu ve Durdur'un varsayılan olarak ne yaptığını belirler.

| | `web` | `desktop` | `static` | `cli` |
| --- | --- | --- | --- | --- |
| Gerektirir | `command` | `command` | `url` | `command` |
| Genellikle ayrıca | `port`, `url` | `processName` | kendi kendine sunuyorsa `command` ve `port` | `cwd` |
| Başlat | `command` komutunu bir terminal sekmesinde çalıştırır | `command` komutunu bir terminal sekmesinde çalıştırır | `command` yoksa: `url` adresini tarayıcıda açar. Varsa: onu bir terminal sekmesinde çalıştırır | `command` komutunu bir terminal sekmesinde çalıştırır |
| Varsayılan `killMode` | `port` | `processName` | `none` | `none` |

![Bir web uygulaması için Uygulamayı düzenle iletişim kutusu: tür web olarak ayarlı, tek satırlık açıklamayla; port alanı dolu](../../../../assets/screenshots/edit-app-type-and-port.png)

1. `type` seçimi. İpucu satırı o türün ne yaptığını açıklar.
2. `port` alanı. Bir `web` uygulaması için Çalışıyor durumu, bu bağlantı noktasının yanıt verip vermediğine bağlıdır.

## Çalışıyor durumu nasıl belirlenir

Moonpool birkaç saniyede bir denetler. Türü ne olursa olsun, bir uygulama şunlardan herhangi biri geçerliyse
Çalışıyor sayılır:

- `processName` ayarlıdır ve bu ada sahip bir süreç vardır. Moonpool'un kendi `<exe> mcp`
  yardımcı süreçleri sayılmaz.
- `port` ayarlıdır ve localhost üzerinde yanıt verir.
- Moonpool onu başlatmıştır, ne `port` ne de `processName` vardır ve terminalin süreci hâlâ canlıdır.

Yani bir `cli` uygulaması komutu çalıştığı sürece Çalışıyor görünür; `port` içermeyen bir `web` uygulaması da
aynı şekilde davranır. Yalnızca `url` içeren bir `static` girdinin izlenecek bir şeyi yoktur ve hiçbir zaman
Çalışıyor görünmez.

## web

Yerel bir sunucu. Çalışıyor durumunun sunucunun yanıt verip vermediğini yansıtması için `port`, ayağa kalktığında
açılması için `url` ve `openBrowser` ayarlayın.

## desktop

Yerel bir uygulama. Çalışıyor durumunun, pencere onu başlatan komuttan ayrıldıktan sonra da sürmesi için
`processName` değerini çalıştırılabilir dosya adına ayarlayın. Varsayılan Durdur, bu ada sahip her süreci
sonlandırır.

## static

Bir sayfa. Yalnızca bir `url` ile Başlat ve Yeniden başlat onu tarayıcınızda açar, Durdur ise hiçbir şey yapmaz.
`http://`, `https://`, `mailto:` ve `file://` URL'leri açılır; dolayısıyla yerel bir sayfa da çalışır:

```json title="apps.json"
{ "id": "csv", "name": "CSV dashboard", "group": "Docs", "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/csv/index.html" }
```

Bir sunucu gerektiren sayfalar (PHP ya da yerel dosyaları getiren herhangi bir şey) için, birini başlatan bir
`command` ve onu izlemek için bir `port` gerekir. Bkz. [örnekler](/tr/apps/examples/).

## cli

Bir araç. `command`, `cwd` içinde bir terminal sekmesinde çalışır ve komut çıktığında uygulama Çalışıyor
olmaktan çıkar. Açık kalan bir kabuk için komutu bir kabuk yapın; örneğin bu `command`:

```text title="command"
pwsh -NoLogo -NoProfile -NoExit -Command python run.py --flag
```

`command` içinde iç içe çift tırnaktan kaçının: `cmd /c` sarmalayıcısı onları bozar.

![Bir cli uygulamasının terminal sekmesi: bir PowerShell komutunun çıktısı ve altında açık bir istem](../../../../assets/screenshots/terminal-cli-output.png)

## Tıklamak ne yapar

Bir uygulamanın adına tıklamak yalnızca terminal sekmesini açar. Çalıştırmak için Başlat, Durdur ve Yeniden başlat
denetimlerini kullanın. Bkz. [Uygulama durumları](/tr/support/glossary/#uygulama-durumları).
