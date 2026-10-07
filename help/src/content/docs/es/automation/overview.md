---
title: "Automatiza Moonpool con scripts y agentes de IA"
description: "Las tres formas de controlar un Moonpool en ejecución desde scripts y agentes de IA (MCP, línea de comandos y verbos de control) y qué puede cambiar cada una."
---

Moonpool se puede controlar sin tocar su ventana. Hay tres superficies, todas atendidas por el mismo
Moonpool residente (la instancia de la bandeja, llamada aquí el hub).

Cada copia de Moonpool es su propio hub: el instalado y cada copia portable se ejecutan de forma
independiente, cada una con su propio canal de control. Una superficie siempre llega a la copia cuyo
`moonpool.exe` usa. Consulta [Modo portable](/es/data/portable-mode/#varias-copias-a-la-vez).

| Superficie | Qué es | Referencia |
| --- | --- | --- |
| Servidor MCP | `moonpool.exe mcp`, un servidor [MCP](https://modelcontextprotocol.io) stdio que inicia un host de IA. | [Configuración de MCP](/es/automation/mcp-setup/), [Herramientas MCP](/es/automation/mcp-tools/) |
| Línea de comandos | `moonpool.exe <verb> [args]`. Una segunda ejecución de la misma copia entrega el verbo a su hub a través del canal de control y termina. | [Línea de comandos](/es/automation/command-line/) |
| Canal de control | Una canalización con nombre, `\\.\pipe\moonpool` (`\\.\pipe\moonpool-<id>` para una copia portable), en Windows y un socket Unix en Linux, que atiende una solicitud JSON por línea. | [Verbos de control](/es/automation/control-verbs/) |

## Cómo se relacionan

- El hub es el dueño de todo: iniciar apps, los registros de sesión, `apps.json`.
- El servidor MCP es un cliente del hub, no una segunda copia de él. La mayoría de las llamadas a
  herramientas se reenvían al hub a través del canal de control, y la respuesta vuelve como resultado de la
  herramienta. Las excepciones: `moonpool_bootup_launcher` inicia `moonpool.exe` por sí mismo;
  `moonpool_app_output` y las herramientas de configuración piden al hub que escriba un archivo y luego lo
  leen; `moonpool_launcher_paths` añade las rutas del propio proceso MCP a las del hub.
- Si hay un hub en ejecución se decide haciendo ping a ese canal, no buscando un proceso. Un hub que
  responde está en ejecución; una canalización o un socket inexistente significa que no lo está.
- Todas las superficies ejecutan los mismos controladores que la ventana, así que un verbo hace lo mismo que
  el clic correspondiente.
- Si no hay ningún hub en ejecución, las herramientas que actúan sobre él, incluida `moonpool_list_apps`,
  se niegan con "Moonpool is not running" (Moonpool no está en ejecución). No hay una lista obsoleta.
  `moonpool_bootup_launcher` lo inicia. Si algo ocupa el canal pero no responde en unos segundos, el error
  dice que un proceso de Moonpool puede estar colgado.
- El servidor MCP ya no recurre a controlar un hub anterior al canal de control. Actualiza esa copia, o
  sal de ella y vuelve a iniciarla.

## Qué puede cambiar cosas

| Puede cambiar | Superficies |
| --- | --- |
| Iniciar, detener o reiniciar una app | MCP, línea de comandos, canalización |
| Reescribir `apps.json` | MCP (`moonpool_write_config`, `moonpool_restore_config`), línea de comandos, canalización |
| Salir de Moonpool | MCP (`moonpool_shutdown_launcher`), línea de comandos (`quit`), canalización |
| Terminar el proceso ayudante MCP de una app | MCP (`moonpool_stop_mcp_server`), canalización (`stop-mcp`) |
| Recargar `apps.json`, volver a obtener los iconos, mostrar la ventana | MCP (`moonpool_reload_config`, `moonpool_refresh_app_icons`, `moonpool_raise_launcher`), línea de comandos (`reload`, `refresh-icons`, `show`), canalización |
| Abrir una ventana o una pestaña de terminal | canalización (`open-window`) |
| Borrar los avistamientos recordados de ayudantes MCP | MCP (`moonpool_reset_mcp_seen`), canalización (`reset-mcp-seen`) |

Herramientas de solo lectura: `moonpool_list_apps`, `moonpool_app_output`, `moonpool_read_config`,
`moonpool_launcher_paths`, `moonpool_window_state`, `moonpool_screenshot`.

## Propiedades de seguridad

- **Las escrituras de configuración están protegidas.** Una escritura debe llevar el token de versión de
  la última lectura, un token obsoleto se rechaza, y el nuevo `apps.json` se valida antes de escribir
  nada. Una escritura rechazada deja `apps.json` intacto. Consulta [Herramientas MCP](/es/automation/mcp-tools/#configuración).
- **Los ids de app están restringidos.** El servidor MCP solo acepta letras, dígitos, `.`, `_` y `-`, y
  nunca un `-` inicial, de modo que un id no se pueda interpretar como una opción de línea de comandos.
- **Las capturas de pantalla son solo de Moonpool.** `moonpool_screenshot` captura una de las seis ventanas
  propias de Moonpool (`main`, `settings`, `about`, `installer`, `editor`, `help`, `themes`), nunca la
  pantalla ni otra app. El PNG se construye en memoria y se devuelve en línea; Moonpool no lo guarda en un
  archivo.
- **Sin autenticación en el canal.** Moonpool no añade inicio de sesión ni token a la canalización o al
  socket de control. Cualquier proceso que pueda abrirlo puede enviar verbos. En Linux, el archivo
  del socket se crea con el modo `0600`, de modo que solo tu propio usuario puede.
- **Se detectan los hosts aislados.** Si el servidor MCP detecta que se ejecuta dentro de un entorno aislado
  empaquetado (Store/MSIX), donde vería una copia privada de los archivos de Moonpool, las herramientas que
  leen o escriben archivos (`moonpool_app_output`, `moonpool_read_config`,
  `moonpool_write_config`, `moonpool_restore_config`) devuelven un error que explica por qué, en lugar de
  datos obsoletos. Las herramientas que solo usan el canal de control no se bloquean. Consulta
  [Configuración de MCP](/es/automation/mcp-setup/#hosts-aislados).

## Plataforma

El canal de control existe en todas las plataformas: una canalización con nombre en Windows, un socket
Unix en Linux (ubicación en [Verbos de control](/es/automation/control-verbs/#dónde-escucha)).
Solo `screenshot` (y por tanto `moonpool_screenshot`) es exclusivo de Windows; en Linux devuelve
"not supported on this platform" (no compatible con esta plataforma). Los verbos de la línea de comandos
funcionan en todas las plataformas.

## Véase también

- [Agentes de IA: inicio rápido](/es/automation/quick-start/)
- [Configuración de MCP](/es/automation/mcp-setup/)
