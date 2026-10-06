---
title: "Bir yapay zekâ ajanını MCP üzerinden Moonpool'a bağlama"
description: "moonpool.exe mcp komutunu ana bilgisayarınıza stdio MCP sunucusu olarak kaydedin (kurulu veya taşınabilir) ve Moonpool'un bir uygulamanın kendi MCP yardımcısını nasıl izlediğini öğrenin."
---

Moonpool'un çalıştırılabilir dosyası kendi MCP sunucusudur. Onu ana bilgisayara, `moonpool.exe` dosyasını
tek bir `mcp` bağımsız değişkeniyle çalıştıran bir stdio sunucusu olarak kaydedin.

## Sunucuyu kaydetme

Kurulu sürümde program `%USERPROFILE%\.moonpool\moonpool.exe` dosyasıdır. Taşınabilir sürümde `.moonpool\` klasörünüzün içindeki `moonpool.exe` dosyasıdır. Bu tam yolu
`command` olarak kullanın. `.mcp.json` okuyan bir ana bilgisayar için:

```json title=".mcp.json" {5}
{
  "mcpServers": {
    "moonpool": {
      "type": "stdio",
      "command": "C:\\Users\\you\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    }
  }
}
```

Bir JSON dosyasında ters eğik çizgiler yukarıdaki gibi çiftlenmelidir. Claude Code gibi komut satırından
kayıt destekleyen bir ana bilgisayar bunu tek adımda ekleyebilir:

```powershell frame="terminal"
claude mcp add moonpool -- "$env:USERPROFILE\.moonpool\moonpool.exe" mcp
```

Sunucu kendini
`moonpool` olarak tanıtır, MCP protokol revizyonu `2025-06-18` ile konuşur ve yalnızca araçlar sunar
(kaynak veya istem listelemez). Araçlar ajana `moonpool_*` olarak görünür; bkz.
[MCP araçları](/tr/automation/mcp-tools/).

## Birden fazla Moonpool

Kurulu Moonpool ve her taşınabilir kopya ayrı başlatıcılardır; her birinin kendi uygulamaları vardır ve hepsi
aynı anda çalışabilir. Bir kopyanın `moonpool.exe mcp` komutu her zaman o kopyayı yönetir. Bir ajanın birkaçını
kullanabilmesi için her birini, o kopyanın exe dosyasına işaret eden ayrı bir adla kaydedin:

```json title=".mcp.json"
{
  "mcpServers": {
    "moonpool": {
      "type": "stdio",
      "command": "C:\\Users\\you\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    },
    "moonpool-work": {
      "type": "stdio",
      "command": "D:\\Work\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    }
  }
}
```

```powershell frame="terminal"
claude mcp add moonpool-work -- "D:\Work\.moonpool\moonpool.exe" mcp
```

İki kopyayı aynı adla kaydetmek, çoğu ana bilgisayarda birinin diğerinin yerini almasına yol açar. Araç adları her
kopya için aynıdır; bu yüzden ana bilgisayar onları sizin kaydettiğiniz adla ayırt eder. Taşınabilir bir kopya
ayrıca kendini `moonpool (<folder>)` olarak tanıtır ve sunucu yönergeleri klasörün adını verir; böylece ajan hangi
kopyayla konuştuğunu görebilir.

## Notlar

- `moonpool.exe mcp` asla pencere açmaz ve yükleyiciyi asla başlatmaz. Ana bilgisayar girişini kapattığında çıkar.
- Başlatıldığı exe'nin yapılandırma klasörünü ve denetim kanalını kullanır; yani taşınabilir bir exe taşınabilir
  klasörün verilerini okur ve o taşınabilir kopyayı yönetir. Bir exe yalnızca yanında `moonpool.portable`
  bulunduğu sürece taşınabilir sayılır. Başka herhangi bir
  `moonpool.exe`, nerede olursa olsun, kurulu Moonpool'un klasörünü
  (`%USERPROFILE%\.moonpool\moonpool-config\`) kullanır ve kurulu Moonpool'u yönetir.
- Çoğu araç çalışan bir Moonpool gerektirir. Çalışmıyorsa ajan önce
  `moonpool_bootup_launcher` aracını çağırabilir.
- `moonpool_launcher_paths`, merkezin kullandığı klasörleri MCP sürecinin çözümlediği klasörlerin yanında gösterir.
  Bir fark, ajanın merkezden farklı bir `apps.json` dosyasına baktığı anlamına gelir.

## Korumalı alandaki ana bilgisayarlar

Bazı ana bilgisayarlar araçlarını, AppData'yı paket başına özel bir kopyaya yönlendiren paketlenmiş (Store/MSIX)
bir korumalı alanın içinde çalıştırır. Moonpool, yapılandırma klasörü veya exe dosyası
`...\Packages\<package>\LocalCache\...` gibi bir yolun altında çözümlendiğinde bunu algılar.

Denetim kanalı yanıt verdiği halde `state.json` okunamadığında da algılar. Dosya okuyan veya yazan araçlar
(`moonpool_app_output`, `moonpool_read_config`, `moonpool_write_config`, `moonpool_restore_config`) o zaman boş
veya eski veri yerine nedeni belirten bir hata döndürür. `moonpool_list_apps` gibi yalnızca denetim kanalını
kullanan araçlar, kanala ulaşılabildiği sürece engellenmez. Korumalı alan kanalı da gizliyorsa araçlar
"Moonpool is not running" yerine korumalı alanı bildirir.
Bunun yerine korumalı alanın dışındaki bir kabuktan [komut satırını](/tr/automation/command-line/) kullanın.

## Kendi MCP sunucusu olan uygulamalar

Moonpool'daki birçok uygulamaya bir MCP ana bilgisayarı `<exe> mcp` yardımcı süreci üzerinden ulaşır. Moonpool,
adı uygulamanın `processName` değeriyle eşleşen ve ilk bağımsız değişkeni `mcp` olan bir süreç arar; örneğin
`notes-app.exe mcp`. Sunucu başka bir adla çalışıyorsa (yeniden adlandırılmış bir kopya gibi) uygulamanın
`mcpProcessName` joker karakterini ayarlayın (bkz. [Alanlar](/tr/apps/fields/#mcpprocessname)); onunla eşleşen
bir süreç `mcp` bağımsız değişkeni olmadan da sayılır.

- Bir yardımcı bağlıyken uygulamanın kenar çubuğunda bir MCP alt satırı çalışıyor olarak görünür ve
  `moonpool_list_apps`, uygulamanın satırına `[mcp: running]` ekler. Yardımcı uygulamanın kendisinin
  çalıştığı anlamına gelmez.
- Bir yardımcı bir kez görüldüğünde Moonpool onu hatırlar (yapılandırma klasöründeki `mcp_seen.json` içinde);
  böylece yardımcı çıktıktan sonra MCP alt satırı durdurulmuş olarak görünür kalır ve `moonpool_list_apps`
  `[mcp: stopped]` gösterir.
- MCP alt satırı `showMcpProcesses` ayarıyla denetlenir
  ([Ayarlar penceresi](/tr/using/settings/)).
- `moonpool_stop_mcp_server` yardımcıyı sonlandırır ve uygulamaya dokunmaz. Bir başlatma karşılığı yoktur:
  yardımcıya sahip olan ana bilgisayar onu bir sonraki araç çağrısında yeniden başlatır.

## Araçlar çalışmıyorsa

- **Ana bilgisayar hiç `moonpool_*` aracı göstermiyor.** `command` değerinin `moonpool.exe` dosyasının tam yolu
  ve `args` değerinin `["mcp"]` olduğunu denetleyin, ardından ana bilgisayarı yeniden başlatın.
- **Her araç Moonpool'un çalışmadığını söylüyor.** Moonpool'u başlatın ya da
  `moonpool_bootup_launcher` aracını çağırın. Kayıtlı exe'nin çalıştırdığınız kopya olduğundan emin olun.
- **Bir düzenleme görünmüyor.** `moonpool_launcher_paths` aracını çağırın ve merkezin klasörlerini MCP sürecininkilerle
  karşılaştırın. Bkz. [Korumalı alandaki ana bilgisayarlar](#korumalı-alandaki-ana-bilgisayarlar).

Daha fazlası [Sorun giderme](/tr/support/troubleshooting/#mcp-ve-betik-hataları) sayfasındadır.

## Ayrıca bakın

- [Bir yapay zekâ ajanına (Claude Code, Codex, Cursor) yerel uygulamaları başlatıp durdurmak için MCP sunucusu verin](/tr/guides/mcp-server-for-ai-agent-to-start-stop-local-apps/)
- [MCP araçları](/tr/automation/mcp-tools/)
- [Yapay zekâ ajanları: hızlı başlangıç](/tr/automation/quick-start/)
