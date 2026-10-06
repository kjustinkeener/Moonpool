---
title: "Error: listen EADDRINUSE: address already in use :::3000 ve Vite Port 5173 is in use hatalarını düzeltin"
description: "Node'un EADDRINUSE ve Vite'ın Port 5173 is in use hatalarını düzeltin: bağlantı noktasını neyin tuttuğunu bulun, serbest bırakın ve tekrarını önlemek için Moonpool'un port ve killMode alanlarını kullanın."
---

```text
Error: listen EADDRINUSE: address already in use :::3000
```

Bu Node.js hatası, 3000 numaralı bağlantı noktasını zaten başka bir sürecin dinlediği anlamına
gelir (`:::`, "tüm adresler"in IPv6 biçimidir; `127.0.0.1:3000` da görebilirsiniz). Çoğu zaman
bu, daha önce başlattığınız ve hiç durdurmadığınız aynı sunucunun bir kopyasıdır.

Vite aynı durumu farklı ele alır. Varsayılan olarak şunu yazdırır:

```text
Port 5173 is in use, trying another one...
```

ve bir sonraki boş bağlantı noktasında başlar; yani sunucu ayaktadır ama beklediğiniz yerde
değildir. `--strictPort` ile (veya `server.strictPort: true` ile) Vite bunun yerine
`Error: Port 5173 is already in use` ile çıkar.

## Kendiniz düzeltin

1. Bağlantı noktasının sahibi olan süreci bulun ve sonlandırın. Windows'ta:

   ```text frame="terminal"
   netstat -ano | findstr :3000
   taskkill /PID 12345 /F
   ```

   Adım adım, PowerShell sürümüyle birlikte,
   [Bir bağlantı noktasını kullanan süreci bulun ve sonlandırın](/tr/guides/find-and-kill-process-using-port-windows/)
   sayfasındadır.
2. Ya da sunucunuzu başka bir bağlantı noktasında başlatın; örneğin birçok Node sunucusu için
   `PORT=3001`, Vite için `--port 5174`.

## Moonpool nasıl yardımcı olur

Sunucuyu Moonpool üzerinden çalıştırıyorsanız girdisinde `port` ayarlayın. Moonpool o zaman:

- o bağlantı noktasında bir şey yanıt verdiği sürece uygulamayı Çalışıyor olarak gösterir;
  böylece onu tutan artık bir sunucu, çalışıyor ama "Moonpool tarafından yönetilmiyor"
  olarak görünür;
- **Durdur** ve **Yeniden başlat** sırasında, `killMode` değeri `port` olduğunda (`web`
  uygulamaları için varsayılandır) `port` üzerinde hâlâ dinleyen her şeyi sonlandırır; böylece
  bir sonraki Başlat bağlantı noktasını boş bulur;
- aynı `port` ile yapılandırılmış iki uygulamayı işaretler.

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\api",
  "command": "node server.js",
  "port": 3000,
  "env": { "PORT": "3000" },
  "killMode": "port"
}
```

Moonpool, başlatmadan önce bağlantı noktasını denetlemez. Bağlantı noktası hâlâ doluysa komut,
uygulamanın terminal sekmesinde yukarıdaki hatayı yazdırır. **Durdur**'a (bağlantı noktasını
serbest bırakır) basın ve yeniden **Başlat**'a tıklayın.

Vite için `--strictPort` verin ve `port` değerini istediğiniz bağlantı noktasına eşit tutun:

```text title="command"
npm run dev -- --port 5173 --strictPort
```

Bunsuz Vite 5174'e geçebilir, Moonpool ise 5173'ü izlemeye devam eder ve durum noktası hiç
sabitlenmez.

`killMode` `port`, bağlantı noktasındaki her süreci sonlandırır; bu yüzden yalnızca başka hiçbir
şeyin ihtiyaç duymadığı bağlantı noktaları için kullanın. Windows'ta Docker uygulamaları için
asla kullanmayın. Bkz.
[Durdurma ve yeniden başlatma](/tr/apps/stop-and-restart/#windowsta-docker-uygulamaları).

## Ayrıca bakın

- [Uygulama alanları](/tr/apps/fields/): `port`, `killMode`.
- [Durdurma ve yeniden başlatma](/tr/apps/stop-and-restart/)
- [Sorun giderme](/tr/support/troubleshooting/#i̇ki-uygulama-aynı-bağlantı-noktasını-kullanıyor)
- [Windows'ta bir npm geliştirme sunucusunu arka planda çalıştırın](/tr/guides/run-npm-dev-server-in-background-windows/)
