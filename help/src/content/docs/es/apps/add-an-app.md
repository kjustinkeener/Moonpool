---
title: "Añade una app o un servidor de desarrollo a Moonpool"
description: "Registra una app local o un servidor de desarrollo con un comando de inicio, una carpeta de trabajo y un entorno para que Moonpool lo inicie y lo vigile por ti."
---

Cada app de Moonpool es una entrada con un comando de inicio, una carpeta de trabajo y un entorno
opcional. Moonpool ejecuta el comando en su propia terminal gestionada.

## Añadir una app

1. Abre el menú **...** en la parte superior de la barra lateral y elige **Añadir app**.
2. Escribe un **name** y elige un **group**.
3. Elige el **type**: `web` (servidor en un puerto), `desktop` (app nativa), `static` (una página) o `cli` (un comando).
4. Define el **command** y el **cwd** en el que se ejecuta.
5. Rellena lo que necesita el tipo: **port** y **url** para web, **processName** para desktop,
   **url** para static. Una app `static` con solo una `url` no necesita **command** ni **cwd**.
6. Guarda. La app aparece en la barra lateral. Usa su control **Iniciar** para ponerla en marcha.

![El selector de type (1) y el campo port (2) en el editor de apps, con cwd y command entre ambos](../../../../assets/screenshots/edit-app-type-and-port.png)

1. El selector de **type**; su ayuda explica cómo se ejecuta ese tipo.
2. El campo **port**, que usan las apps `web`.

El resultado es una entrada en `apps.json`, por ejemplo:

```json title="apps.json"
{
  "id": "my-api",
  "name": "My API",
  "group": "Dev",
  "type": "web",
  "command": "npm run dev",
  "cwd": "C:\\code\\my-api",
  "port": 3000,
  "url": "http://localhost:3000"
}
```

Al hacer clic en el nombre de una app solo se abre su pestaña de terminal; consulta [Estados de una app](/es/support/glossary/#estados-de-una-app).

## El editor de apps

- **Group.** Elige un grupo de la lista, o elige **+ Nuevo grupo...** y escribe un nombre.
  **volver a la lista** vuelve a la lista. Un grupo vacío se guarda como `Apps`.
- Los **campos atenuados** no los usa el tipo seleccionado. Aun así se guardan.
- **Guardar sin nombre** muestra `el nombre es obligatorio.`
- **Esc** o cerrar el editor con cambios sin guardar pregunta "¿Descartar los cambios?".
- Para cambiar una app más adelante, usa el lápiz de su fila, o haz clic derecho en ella y elige **Editar**.

## Editar a mano

Elige **Editar apps.json** en el mismo menú, guarda el archivo y luego elige **Recargar**. El
formato, las reglas de validación y las opciones de recuperación están en el
[Resumen de la configuración](/es/apps/apps-json/).

## Adónde ir después

- [Campos de las apps](/es/apps/fields/): cada clave y qué hace.
- [Tipos de app](/es/apps/types/): cómo se inicia cada tipo y cómo muestra "en ejecución".
- [Detener y reiniciar](/es/apps/stop-and-restart/): qué definir cuando Detener deja algo en ejecución y por qué las apps de Docker requieren cuidado.
- [Rutas y entorno](/es/apps/paths-and-environment/): `{MP_HOME}`, rutas `./` y `env`.
- [Ejemplos](/es/apps/examples/): entradas completas para copiar.
- [Guías prácticas](/es/guides/run-npm-dev-server-in-background-windows/): servidores de desarrollo en segundo plano, scripts de Python, puertos.
- [Modo portable](/es/data/portable-mode/)
- [Actualización](/es/data/updating/)
