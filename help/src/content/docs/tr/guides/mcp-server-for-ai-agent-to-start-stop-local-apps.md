---
title: "Bir yapay zeka aracısına (Claude Code, Codex, Cursor) yerel uygulamaları başlatıp durdurması için MCP sunucusu verin"
description: "Moonpool'u MCP sunucusu olarak kaydedin; Claude Code, Codex veya Cursor geliştirme sunucularınızı fazladan kopya açmadan başlatsın, durdursun, yeniden başlatsın ve çıktısını okusun."
---

Bir yapay zeka kodlama aracısı genellikle geliştirme sunucunuzu kendi kabuğuna `npm run dev`
yazarak çalıştırır. Bu, aracıyı bloke edebilir, bağlantı noktasını tutan yetim bir süreç
bırakabilir ya da zaten çalışan bir şeyin ikinci bir kopyasını başlatabilir. Bir MCP sunucusu,
aracının komut satırını yeniden kurmak yerine, önceden yapılandırdığınız uygulamayı başlatıp
durdurmak için araçları çağırmasına izin verir.

## Moonpool yöntemi

Moonpool'un çalıştırılabilir dosyası kendi MCP sunucusudur: `moonpool.exe` dosyasını tek
bağımsız değişkeni `mcp` olan bir stdio sunucusu olarak kaydedin. Uygulama `apps.json`
içinde olduktan sonra aracı onu kimliğiyle başlatır.

```json title="apps.json"
{
  "id": "site",
  "name": "Site",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\site",
  "command": "npm run dev",
  "port": 5173,
  "url": "http://localhost:5173"
}
```

Sunucuyu kaydedin. Claude Code'da tek bir komutla (kurulu Moonpool; kendi exe dosyanızın tam
yolunu kullanın):

```powershell frame="terminal"
claude mcp add moonpool -- "$env:USERPROFILE\.moonpool\moonpool.exe" mcp
```

MCP sunucularının JSON dosyasını okuyan ana bilgisayarlar, örneğin Cursor'ın `mcp.json`
dosyası, aynı biçimi alır (ters eğik çizgiler iki kat):

```json title="mcp.json"
{
  "mcpServers": {
    "moonpool": {
      "command": "C:\\Users\\you\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    }
  }
}
```

Codex için yapılandırmasına (`~/.codex/config.toml`) aynı komut ve `mcp` bağımsız
değişkeniyle bir sunucu ekleyin:

```toml title="config.toml"
[mcp_servers.moonpool]
command = 'C:\Users\you\.moonpool\moonpool.exe'
args = ["mcp"]
```

Dosyanın ve anahtar adlarının kesin biçimi her ana bilgisayara aittir; sürümünüz farklıysa
onun MCP belgelerine bakın. Moonpool'un ihtiyaç duyduğu tek şey `moonpool.exe` dosyasının tam
yolu ve bağımsız değişken olarak `mcp`'dir. Sonrasında ana bilgisayarı yeniden başlatın.

## Aracı neler yapabilir

Araçlar `moonpool_*` olarak görünür. Günlük iş için olanlar:

| Araç | Kullanım |
| --- | --- |
| `moonpool_list_apps` | Bir uygulamanın kimliğini bulun ve çalışıp çalışmadığını görün. |
| `moonpool_start_app` | Bir uygulamayı kimliğiyle başlatın ve terminal sekmesini açın. |
| `moonpool_stop_app` | Alt süreçleri dahil durdurun. |
| `moonpool_restart_app` | Durdurun, bağlantı noktasının boşalmasını bekleyin, başlatın. Kod değişikliğinden sonra kullanın. |
| `moonpool_app_output` | Uygulamanın yazdırdıklarını okuyun; sınırlamak için `tail_lines` kullanın. |
| `moonpool_bootup_launcher` | Çalışmıyorsa Moonpool'un kendisini başlatın. |

Tipik bir döngü `moonpool_restart_app`, ardından `moonpool_app_output` şeklindedir. Diğer
araçlar (`apps.json` okuma ve yazma, ekran görüntüleri) [MCP araçları](/tr/automation/mcp-tools/)
sayfasındadır.

## Çalışmazsa

Her aracın `Moonpool is not running - call moonpool_bootup_launcher first` demesi, Moonpool'un
henüz başlatılmadığı anlamına gelir. Görünmeyen bir düzenleme genellikle aracının farklı bir
`apps.json` dosyasına baktığını gösterir: `moonpool_launcher_paths` komutunu çağırın. Bkz.
[Araçlar çalışmazsa](/tr/automation/mcp-setup/#araçlar-çalışmıyorsa).

## Ayrıca bakın

- [MCP kurulumu](/tr/automation/mcp-setup/)
- [MCP araçları](/tr/automation/mcp-tools/)
- [Yapay zeka aracıları: hızlı başlangıç](/tr/automation/quick-start/)
- [Windows'ta bir npm geliştirme sunucusunu arka planda çalıştırın](/tr/guides/run-npm-dev-server-in-background-windows/)
