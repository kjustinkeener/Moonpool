---
title: "Windows'ta bir bağlantı noktasını kullanan süreci bulun ve sonlandırın (3000, 5173, 8080)"
description: "Windows'ta 3000 veya 5173 bağlantı noktasını hangi sürecin tuttuğunu netstat ya da PowerShell ile bulun, taskkill ile sonlandırın; Moonpool bir uygulamayı durdurunca portu serbest bıraksın."
---

Bir geliştirme sunucusu "bağlantı noktası zaten kullanımda" hatasıyla başarısız olduğunda,
başka bir şey o bağlantı noktasını dinliyordur. Komut İstemi'nde dinleyenleri sahibi olan süreç
kimliğiyle listeleyin, sonra sonlandırın:

```text frame="terminal"
netstat -ano | findstr :3000
taskkill /PID 12345 /F
```

`LISTENING` satırının son sütunu PID'dir (`findstr :3000` ayrıca `:30001` ile de eşleşir, bu
yüzden yerel adrese bakın). `tasklist /FI "PID eq 12345"` bunun hangi program olduğunu
gösterir. PowerShell'de aynı arama şöyledir:

```powershell frame="terminal"
Get-NetTCPConnection -LocalPort 3000 -State Listen | Select-Object LocalPort, OwningProcess
Get-Process -Id 12345
Stop-Process -Id 12345 -Force
```

Sürecin alt süreçlerini de sonlandırmak için `taskkill` komutuna `/T` ekleyin. Başka bir
kullanıcıya ya da sisteme ait süreçler yükseltilmiş (yönetici) bir pencere gerektirebilir.

## Moonpool yöntemi

Moonpool üzerinden çalıştırdığınız bir uygulama için PID'yi aramazsınız. Uygulamaya bir
`port` verin, Durdur onu serbest bırakır. Bir `web` uygulaması için bu varsayılan `killMode`
değeridir, burada açıkça yazılmıştır:

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\api",
  "command": "npm start",
  "port": 3000,
  "env": { "PORT": "3000" },
  "killMode": "port"
}
```

Durdur önce Moonpool'un başlattığı terminali sonlandırır, ardından `port` üzerinde hâlâ
dinleyen her şeyi zorla sonlandırır. Windows'ta bu, yukarıdakiyle aynı aramadır
(`Get-NetTCPConnection -LocalPort <port> -State Listen`) ve ardından her sahip için
`taskkill /PID <pid> /T /F` çalışır.

- Sizin başlatmadığınız bir şey bağlantı noktasını tutuyorsa Moonpool uygulamayı çalışıyor ama
  "Moonpool tarafından yönetilmiyor" olarak gösterir. Üzerinde **Durdur**'a basın:
  `port` adımı yine de çalışır.
- Moonpool, ortak Windows süreçlerinden sabit bir listeyi bağlantı noktası üzerinden
  sonlandırmayı reddeder; örneğin Docker Desktop'ın arka ucu, `svchost` ve WSL ana bilgisayarı.
  Bir Docker uygulaması için `killMode` değeri olarak `command` ya da `none` kullanın, asla
  `port` kullanmayın. Bkz.
  [Windows'ta Docker uygulamaları](/tr/apps/stop-and-restart/#windowsta-docker-uygulamaları).
- Bu yalnızca `apps.json` içinde listelenen uygulamaların bağlantı noktaları için çalışır.
  Başka herhangi bir bağlantı noktası için en üstteki komutları kullanın.
- `port` modu, elle başlattığınız bir kopya dahil dinleyen her şeyi sonlandırır; bu yüzden
  yalnızca makinedeki başka hiçbir şeyin ihtiyaç duymadığı bağlantı noktaları için kullanın.

## Ayrıca bakın

- [EADDRINUSE ve "Port 5173 is in use" hatalarını düzeltin](/tr/support/port-already-in-use/)
- [Durdurma ve yeniden başlatma](/tr/apps/stop-and-restart/)
- [Uygulama alanları](/tr/apps/fields/): `port` ve `killMode`.
- [İki uygulama aynı bağlantı noktasını kullanıyor](/tr/support/troubleshooting/#i̇ki-uygulama-aynı-bağlantı-noktasını-kullanıyor)
