---
title: "Bir yapay zekâ ajanının Moonpool'u kurup yönetmesi: hızlı başlangıç"
description: "Bir yapay zekâ ajanının veya betiğin Moonpool'u kurup yönetmesinin üç yolu, ajanınız için hangisini seçeceğiniz ve aynı eylemin her birindeki karşılığı."
---

Üç giriş yolu vardır. Ajanınızın yapabildiklerine göre seçin.

| İstediğiniz | Kullanın | Buradan başlayın |
| --- | --- | --- |
| Bir ajanın uygulamalarınızı bulup bir kerede eklemesi | Merkezin boş ekranındaki **İstemi kopyala** | Aşağıda |
| Bir ajanın uygulamaları araç çağrılarıyla başlatması, durdurması ve okuması | MCP sunucusu, `moonpool.exe mcp` | [MCP kurulumu](/tr/automation/mcp-setup/) |
| Bir betik ya da MCP olmayan bir ajan | Komut satırı komutları | [Komut satırı](/tr/automation/command-line/) |

## İstemi kopyala

Hiçbir sekme açık değilken CLI paneli hazır bir istem gösterir ("Yeni mi başladınız? Uygulamalarınızı ayarlaması
için bunu bir yapay zekâ aracısına verin:"). **İstemi kopyala** onu panoya koyar. Ajanınıza yapıştırın. Ajanı,
yapılandırma klasörünüzdeki `AI-README.md` ve `apps.json` dosyalarına yönlendirir ve uygulamalarınızı bulup
kaydetmesini ister. İşi bittiğinde **Yeniden yükle** seçeneğini seçin.

Moonpool, `AI-README.md` dosyasını her açılışta `apps.json` yanında yeniden yazar; böylece her zaman
çalıştırdığınız sürümle eşleşir. Kendi düzenlemelerinizi onun içinde tutmayın.

## Aynı eylem üç yolla

| Eylem | Komut satırı | Denetim kanalı komutu | MCP aracı |
| --- | --- | --- | --- |
| Bir uygulamayı başlat | `moonpool.exe launch <id>` | `launch` | `moonpool_start_app` |
| Bir uygulamayı durdur | `moonpool.exe stop <id>` | `stop` | `moonpool_stop_app` |
| Bir uygulamayı yeniden başlat | `moonpool.exe restart <id>` | `restart` | `moonpool_restart_app` |
| Bir uygulamanın çıktısını oku | `moonpool.exe dump <id> [out-path]` | `dump` | `moonpool_app_output` |
| Uygulamaları ve durumu listele | `state.json` dosyasını oku | `list` | `moonpool_list_apps` |
| `apps.json` dosyasını yeniden oku | `moonpool.exe reload` | `reload` | `moonpool_reload_config` |
| `apps.json` dosyasını oku | `moonpool.exe read-config` | `read-config` | `moonpool_read_config` |
| `apps.json` dosyasını değiştir | `moonpool.exe write-config <file> [token]` | `write-config` | `moonpool_write_config` |
| `apps.json` dosyasını geri al | `moonpool.exe restore-config [n]` | `restore-config` | `moonpool_restore_config` |
| Pencereyi göster | `moonpool.exe show` | `show` | `moonpool_raise_launcher` |
| Moonpool'u başlat | `moonpool.exe` | yok | `moonpool_bootup_launcher` |
| Moonpool'dan çık | `moonpool.exe quit` | `quit` | `moonpool_shutdown_launcher` |
| Kullanımdaki klasörleri göster | `moonpool.exe paths` | `paths` | `moonpool_launcher_paths` |

Komut satırı hiçbir şey yazdırmaz; sonucu bir `--ticket` ile okuyun (bkz.
[Sonucu okuma](/tr/automation/command-line/#sonucu-okuma)). Kanal ve MCP doğrudan yanıt verir.

## Bir ajanın araçları başarısız olduğunda

- `Moonpool is not running - call moonpool_bootup_launcher first`: Moonpool'u başlatın ya da
  ajanın o aracı çağırmasına izin verin.
- Bir düzenleme "etkili olmadı": ajandan `moonpool_launcher_paths` çıktısını isteyin. Merkez ve MCP klasörleri
  farklıysa ajan farklı bir `apps.json` dosyasını okuyordur. Bkz.
  [Korumalı alandaki ana bilgisayarlar](/tr/automation/mcp-setup/#korumalı-alandaki-ana-bilgisayarlar).
- Birkaç Moonpool kopyası: her birini kendi adıyla kaydedin. Bkz.
  [Birden fazla Moonpool](/tr/automation/mcp-setup/#birden-fazla-moonpool).

Claude Code, Codex ve Cursor için çalışan bir örnek
[Bir yapay zekâ ajanına yerel uygulamaları başlatıp durdurmak için MCP sunucusu verin](/tr/guides/mcp-server-for-ai-agent-to-start-stop-local-apps/)
sayfasındadır.

Daha fazla belirti [Sorun giderme](/tr/support/troubleshooting/#mcp-ve-betik-hataları) sayfasındadır.
