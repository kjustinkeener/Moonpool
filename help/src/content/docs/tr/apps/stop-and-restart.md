---
title: "Bir geliştirme sunucusunu ve başlattığı her şeyi durdurma"
description: "killMode ve stopCommand ile Durdur ve Yeniden başlat işlemlerinin bir uygulamayı ve alt süreçlerini temiz biçimde sonlandırmasını sağlayın; tür başına varsayılanlar ve Windows'ta Docker dahil."
---

Durdur her zaman önce şunu yapar: Moonpool, uygulama için başlattığı terminali, o terminalin
başlattığı her şey dahil sonlandırır. Birçok uygulama için gereken tek şey budur.

Bazı uygulamalar o terminalden daha uzun yaşar (bir masaüstü penceresi onu başlatan geliştirme
sunucusundan ayrılır ya da bir sunucu alt süreci bağlantı noktasını tutmaya devam eder). **`killMode`**,
ardından çalışacak tek bir ek adımı seçer.

| `killMode` | Durdur sırasında ek adım | Okuduğu | Varsayılan olduğu tür |
| --- | --- | --- | --- |
| `processName` | O ada sahip her süreci zorla sonlandırır. Windows'ta alt süreçlerini de (`taskkill /IM <name>.exe /T /F`). Diğer sistemlerde `pkill -KILL -x <name>`: tam ve büyük/küçük harfe duyarlı bir ad eşleşmesi, alt süreçler dahil değil. | `processName` | `desktop` |
| `port` | `port` üzerinde dinleyen süreci zorla sonlandırır. | `port` | `web` |
| `command` | `stopCommand` komutunu `cwd` içinde çalıştırır ve bitmesini bekler. | `stopCommand`, `cwd`, `env` | hiçbiri |
| `none` | Hiçbir şey. | hiçbir şey | `static`, `cli` |

Uygulamanın türü için varsayılanı almak üzere `killMode` değerini atlayın ve yalnızca Durdur bir şeyi
çalışır halde bıraktığında ayarlayın.

![Uygulamayı düzenle iletişim kutusunda "varsayılan (türe göre)" olarak ayarlı killMode seçimi; ipucu satırı her türün varsayılan olarak ne yaptığını listeler](../../../../assets/screenshots/edit-app-killmode.png)

1. `killMode` seçimi. "varsayılan (türe göre)", anahtarı atlamakla aynıdır.

- Kipin ihtiyaç duyduğu alan boşsa (örneğin `port` kipinde `port` yoksa) ek adım atlanır. Bu bir hata
  değildir.
- `killMode`, `type` değerinden bağımsızdır: `port` bir `cli` uygulamasında, `processName` bir
  `web` uygulamasında çalışır.
- Boş bir dize ya da tanınmayan bir değer fazladan hiçbir şey yapmaz. Tür varsayılanına geri dönmez.

Bir masaüstü uygulaması için `processName` kipi, şuna eşdeğerini çalıştırır:

```powershell frame="terminal"
taskkill /IM notes-app.exe /T /F
```

## Birkaç Moonpool veya kendi süreçleriniz

`processName` ve `port`, bir süreci kimin başlattığını bilmez. `processName` o ada sahip her süreci
sonlandırır; `port` ise bağlantı noktasını dinleyen her şeyi sonlandırır; buna başka bir Moonpool kopyasının
başlattığı süreç (kurulu olan ve taşınabilir kopyalar birbirinden bağımsız çalışır; bkz.
[Taşınabilir mod](/tr/data/portable-mode/#aynı-anda-birkaç-kopya)) ve kendi başlattığınız süreç de dahildir.
Bu kipleri yalnızca bu şekilde çakışmayacak uygulamalarda kullanın: makinedeki başka hiçbir şeyin kullanmadığı
bir ad ya da bağlantı noktası. İki kopya aynı uygulamayı kaydediyorsa veya uygulamayı elle de
çalıştırıyorsanız, ona `killMode` `none` verin ya da yalnızca kendi örneğini durduran bir `command` kullanın.

## stopCommand

Yalnızca `killMode` değeri `command` iken kullanılır. Windows'ta `cmd /c`, diğer sistemlerde `$SHELL -c` ile,
`cwd` içinde ve `env` değerleriniz eklenerek çalışır. İçinde `{MP_HOME}` ve `{MP_DATA}` kullanılabilir. Moonpool
başka bir şey yapmadan önce bitmesini bekler; yani Yeniden başlat, o hâlâ çalışırken asla yeniden başlatmaz.
Çıkış kodu yok sayılır. 60 saniye sonra hâlâ çalışıyorsa Moonpool onu ve alt süreçlerini sonlandırır ve devam eder.

## Yeniden başlatma

Yeniden başlat, Durdur'un ardından aynı `command` için Başlat'tır. Moonpool, yeniden başlatmadan önce eski
örneğin durmuş görünmesi için (böylece bağlantı noktası serbest kalır) en fazla 4 saniye bekler. Yalnızca
`url` içeren bir `static` girdide durdurulacak bir şey yoktur: Yeniden başlat sayfayı yalnızca yeniden açar.

## Windows'ta Docker uygulamaları

`none` kullanın ya da `docker compose stop app` gibi gerçek bir durdurma komutuyla `command` kullanın.
`port` kullanmayın.

Docker Desktop, her kapsayıcının bağlantı noktasını paylaşılan tek bir arka plan süreci üzerinden yayımlar.
Windows'ta "bağlantı noktasını dinleyen her şey" o paylaşılan süreçtir; bu yüzden `port` kipi Docker Desktop'ı
zorla sonlandırır ve yalnızca bu uygulamayı değil, tüm kapsayıcıları kapatırdı. Son güvence olarak Moonpool,
paylaşılan Windows süreçlerinden oluşan sabit bir listeyi bağlantı noktasına göre sonlandırmayı reddeder:
Docker Desktop'ın arka uç, proxy ve hizmet süreçleri, `dockerd`, `vpnkit`, WSL ana bilgisayar süreçleri ve
`svchost` gibi çekirdek sistem süreçleri. Bu, doğru kipi seçmenin yerini tutmaz.

`command` değeriniz kapsayıcıyı zaten yeniden oluşturuyorsa (`docker compose up -d --build`), `none`
doğrudur: Yeniden başlat onu yalnızca yeniden çalıştırır.

Ayrıca bkz. [Bir bağlantı noktasını kullanan işlemi bulup sonlandırma](/tr/guides/find-and-kill-process-using-port-windows/)
ve [EADDRINUSE ve "Port 5173 is in use" hatalarını giderme](/tr/support/port-already-in-use/).

## Örnekler

Zaman zaman bağlantı noktasını tutan bir node süreci bırakan bir geliştirme sunucusu (bu, `web` için
varsayılandır, burada açıkça gösterilmiştir):

```json title="apps.json"
{ "id": "site", "name": "Site", "group": "Web apps", "type": "web",
  "cwd": "C:\\code\\site", "command": "npm run dev", "port": 5173,
  "killMode": "port" }
```

Bir Docker Compose uygulaması:

```json title="apps.json"
{ "id": "api", "name": "API", "group": "Web apps", "type": "web",
  "cwd": "C:\\code\\api", "command": "docker compose up -d --build", "port": 8080,
  "killMode": "command", "stopCommand": "docker compose stop app" }
```
