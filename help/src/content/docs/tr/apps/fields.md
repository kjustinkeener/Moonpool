---
title: "Tüm apps.json alanları: tür, varsayılan değer ve işlevi"
description: "Bir apps.json girdisinin her anahtarını türü, varsayılan değeri ve hangi uygulama türlerinin kullandığıyla arayın; adlar Uygulamayı düzenle iletişim kutusuyla aynıdır."
---

Uygulamayı düzenle iletişim kutusu aynı alanları aynı adlarla gösterir. Seçili türe uygulanmayan alanlar
iletişim kutusunda soluk gösterilir ama yine de kaydedilir; tek istisna:
`stopCommand` yalnızca `killMode` değeri `command` iken kaydedilir.

![Uygulamayı düzenle iletişim kutusu, addan stopCommand alanına kadar; killMode seçimi çerçevelenmiş, processName ve stopCommand gibi kullanılmayan alanlar soluk](../../../../assets/screenshots/edit-app-dialog.png)

1. `killMode` seçimi. Kullanmadığı alanlar soluk kalır.

| Alan | Tür | Zorunlu | Kullanan | Ne yapar |
| --- | --- | --- | --- | --- |
| `id` | string | evet | tümü | Benzersiz anahtar. Harfler, rakamlar, `.`, `_`, `-`; `-` ile başlayamaz. Bkz. [Genel bakış](/tr/apps/apps-json/#id). |
| `name` | string | evet | tümü | Kenar çubuğundaki etiket. Boş olamaz. |
| `group` | string | evet | tümü | Uygulamanın kenar çubuğunda altında listelendiği başlık. Elle düzenlemede boş olamaz; iletişim kutusu boş grubu `Apps` olarak kaydeder. Herhangi bir metin; yeni bir ad yeni bir grup oluşturur. |
| `type` | string | evet | tümü | `web`, `desktop`, `static` veya `cli`. Bkz. [Uygulama türleri](/tr/apps/types/). |
| `command` | string | `static` dışındakiler için | tümü | Uygulamayı başlatmak için bir terminalde çalıştırılır; Windows'ta `cmd /c`, diğer sistemlerde `$SHELL -c` ile (`SHELL` tanımlı değilse `/bin/sh`). `static` için isteğe bağlıdır. |
| `cwd` | string | hayır | `command` içeren tümü | Komutun çalıştığı klasör. Varsayılan olarak Moonpool'un kendi çalışma klasörüdür. Belirteçleri ve `./` yollarını destekler. Bkz. [Yollar ve ortam](/tr/apps/paths-and-environment/). |
| `port` | integer, 1 ile 65535 arası | hayır | herhangi biri | Localhost üzerinde (IPv4 veya IPv6) bu bağlantı noktasında bir şey yanıt verdiği sürece Çalışıyor. `killMode` `port` tarafından okunur. |
| `processName` | string | hayır | herhangi biri, çoğunlukla `desktop` | Bu ada sahip bir süreç var olduğu sürece Çalışıyor. Büyük/küçük harf duyarsızdır, `.exe` ile veya `.exe` olmadan yazılabilir; yani `my-app`, `my-app.exe` ile eşleşir. Linux'ta en fazla 15 karakter. `killMode` `processName` tarafından okunur. |
| `mcpProcessName` | string | hayır | `processName` içeren herhangi biri | Bu uygulamanın MCP sunucusunun süreç adı için joker karakterli desen. `*` herhangi bir karakter dizisiyle, `?` tek bir karakterle eşleşir. Büyük/küçük harf duyarsızdır, adın tamamıyla eşleştirilir ve `.exe` isteğe bağlıdır. Eşleşen bir süreç uygulamanın MCP sunucusu sayılır (kenar çubuğundaki MCP alt satırı) ve ilk bağımsız değişken olarak `mcp` gerektirmez. Bkz. [mcpProcessName](#mcpprocessname). |
| `url` | string | yalnızca `static` | `web`, `static` | Açılacak sayfa. Yalnızca `http://`, `https://`, `mailto:` ve `file://` URL'leri açılır. |
| `openBrowser` | boolean, varsayılan `false` | hayır | `url` içeren her tür (iletişim kutusu bunu `desktop` ve `cli` için soluk gösterir) | Moonpool uygulamanın ayağa kalktığını algılar algılamaz `url` adresini otomatik açar (aşağıya bakın). |
| `killMode` | string | hayır | tümü | Durdur ve Yeniden başlat sırasında ek temizlik: `processName`, `port`, `command` veya `none`. Bkz. [Durdurma ve yeniden başlatma](/tr/apps/stop-and-restart/). |
| `stopCommand` | string | hayır | `killMode` `command` | Durdur sırasında çalıştırılan komut. Diğer tüm kiplerde yok sayılır. |
| `env` | dizelerden oluşan nesne | hayır | tümü | Ek ortam değişkenleri. İletişim kutusu bunu satır başına bir `KEY=VALUE` olarak düzenler. |
| `icon` | string | hayır | tümü | Kenar çubuğu görseli: bir dosya yolu, bir `http(s)` URL'si veya bir `data:` URI'si. Uygulamanın bağlam menüsündeki **Simge seç...** ile ya da elle ayarlanır. |
| `note` | string | hayır | tümü | Kenar çubuğunda uygulamanın üzerine gelince görünen ipucu. |

`env` ve `killMode` kullanan bir girdi:

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "command": "npm start",
  "port": 3000,
  "env": { "PORT": "3000", "NODE_ENV": "development" },
  "killMode": "port"
}
```

`port` ve `killMode` değerlerinin birlikte nasıl çalıştığı için bkz.
[Bir bağlantı noktasını kullanan işlemi bulup sonlandırma](/tr/guides/find-and-kill-process-using-port-windows/).

## mcpProcessName

Varsayılan olarak Moonpool, adı `processName` ile eşleşen ve ilk bağımsız değişkeni `mcp` olan bir süreci
(örneğin `notes-app.exe mcp`) uygulamanın MCP sunucusu sayar. Sunucu farklı bir adla çalışıyorsa
`mcpProcessName` değerini ayarlayın: bir exe'yi izleyen ama MCP sunucusu başka olan bir uygulama
(`mog.exe mcp`) ya da sunucunun yeniden adlandırılmış bir kopyası gibi.

Değer, joker karakterli bir desendir. `*` herhangi bir karakter dizisiyle (boş olabilir), `?` tam olarak tek
karakterle eşleşir. Süreç adının tamamıyla büyük/küçük harf duyarsız karşılaştırılır; `.exe` içermeyen bir
desen, `.exe` ile biten adla da eşleşir. Boş değer, ayarlanmamış sayılır.

```json
{
  "id": "destiny",
  "name": "Destiny",
  "group": "Desktop apps",
  "type": "desktop",
  "processName": "destiny",
  "mcpProcessName": "destiny-mcp-*"
}
```

Bu, `destiny-mcp-2706210170.exe` gibi yeniden adlandırılmış bir kopyayla eşleşir. `mcpProcessName` ile eşleşen bir süreç,
`mcp` ile başlatılmış olsun olmasın sunucudur ve uygulamanın kendisinin çalıştığı anlamına asla gelmez. Desen
`processName` değerinin kendisiyle de eşleşiyorsa (örneğin `destiny*`), Moonpool yine de `mcp` bağımsız değişkenini
arar; böylece gerçek uygulama MCP sunucusuyla asla karıştırılmaz. Bkz. [MCP kurulumu](/tr/automation/mcp-setup/#kendi-mcp-sunucusu-olan-uygulamalar).

## openBrowser

Moonpool'un başlattığı bir uygulama ilk kez Çalışıyor olarak göründüğünde `url` adresini bir kez açar. Bunun
algılanması için bir `port` ya da `processName` gerekir. İkisi de yoksa Çalışıyor yalnızca terminal sürecinin
canlı olduğu anlamına gelir ve tarayıcı otomatik açılmaz. Komutunuz tarayıcıyı kendisi açıyorsa `openBrowser`
değerini kapatın. Komutsuz bir `static` girdi, `openBrowser` ne olursa olsun Başlat düğmesine her
bastığınızda `url` adresini açar.

Aynı `port` ile yapılandırılmış iki uygulama kenar çubuğunda işaretlenir.

## Simgeler

Bir uygulamanın simgesi, bunlardan var olan ilkidir:

1. `icon` alanı.
2. Yapılandırma klasöründeki `icons\<id>.<ext>`, örneğin `icons\site.png`.
3. Uygulamanın kendi klasöründeki bir simge dosyası (`cwd` klasörü ya da `file:///` biçimli bir `url` içeren klasör).
4. `desktop` için, derlenmiş veya çalışan `.exe` dosyasının simgesi.
5. `web` ve `static` için, sunucu ayağa kalktıktan sonra sitenin `/favicon.ico` dosyası.
6. Tür için bir glif.

Çoğu uygulamanın simge ayarına ihtiyacı yoktur.
