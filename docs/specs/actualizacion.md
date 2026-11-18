# Conversión de datos guardados (`reiwand-upgrade`)

Programa aparte, de única vez. Ninguna aplicación contiene ni llama código de conversión: si encuentran datos
viejos se niegan a arrancar y nombran la orden exacta que corre la conversión.

## Órdenes

- `reiwand-upgrade client [--data DIR] [--config DIR] [--also DIR]... [--skip-download] [--check] [--then PROGRAMA ...]`:
  toda la carpeta de datos de la persona: `config.toml` (de la carpeta de configuración), ediciones de imágenes,
  permisos de entradas, cuentas, base de la biblioteca, registros de repositorios, paquetes instalados
  (se reescribe el `.wpak` y se actualizan `sha256` y `size` en `installed.json`; la firma del repositorio ya no
  corresponde al archivo reescrito, la confianza queda en el registro local) y las escenas y paquetes de las carpetas
  de la biblioteca (`--also` suma carpetas de proyectos del editor). Al terminar escribe la marca `format-version`.
- `reiwand-upgrade file RUTA... [--password-file ARCHIVO] [--copies DIR] [--check]`: una carpeta de escena o de
  paquete (todo lo que hay debajo), un `.wpak`, un `.reiwand` (si está cifrado sigue cifrado), una edición
  (`nombre.ext.reiwand.json`) o un descriptor de efecto. Es la orden de quien crea, antes de volver a firmar.
- `reiwand-upgrade server --data DIR [--check] [--then PROGRAMA ...]`: carpeta de datos y base del servidor.

## Garantías

- `--check` dice qué cambiaría y no escribe nada.
- Cada archivo que cambia se copia antes en `upgrade-copias/<tiempo>/` con un `index.txt` (`número`, ruta original).
- Cada formato lleva su marca de versión (`format`, `schema`, esquema de la base): lo que ya está en la versión
  nueva se deja, así que repetir la orden no cambia nada.
- El recorrido de escenas y paquetes es uno solo (`scenes::walk_in`): `client` y `file` lo usan.
- Un paquete firmado que se reescribe pierde su firma de autor (el archivo ya no es el que firmó); quien lo publicó
  lo firma de nuevo con `file` sobre su carpeta de trabajo.
