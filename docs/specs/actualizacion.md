# Conversión de datos guardados (`reiwand-upgrade`)

Programa aparte, de única vez. Ninguna aplicación contiene ni llama código de conversión: si encuentran datos
viejos se niegan a arrancar y nombran la orden exacta que corre la conversión.

## Órdenes

- `reiwand-upgrade client [--data DIR] [--config DIR] [--also DIR]... [--skip-download] [--check] [--then PROGRAMA ...]`:
  toda la carpeta de datos de la persona: `config.toml` (de la carpeta de configuración), ediciones de imágenes,
  permisos de entradas, cuentas, base de la biblioteca, registros de repositorios, paquetes instalados
  (se reescribe el `.wpak` y se actualizan `sha256` y `size` en `installed.json`; la firma del repositorio ya no
  corresponde al archivo reescrito, la confianza queda en el registro local) y las escenas y paquetes de las carpetas
  de la biblioteca (`--also` suma carpetas de proyectos del editor). El repositorio «Example wallpapers»
  (`examples-repo`) cuya raíz ya no carga en el formato actual se rehace vacío con claves
  nuevas (el daemon vuelve a sumar los ejemplos), el nuevo `repo_id` se escribe en `examples-repo.json` (conservando `declined`) y en `installed.json`, la copia
  vieja del cliente en `repos/<id>` se aparta con las copias y el servicio vuelve a añadirlo sano al arrancar
  (en el teléfono el registro recibe `stamp` 0 para que la app lo ofrezca de nuevo).
  También convierte `examples-repo.json` si no lleva el envoltorio de formato, los `.wpak` de la biblioteca que la base
  marca con error (los importados a `files/`: se reescriben y la fila recibe tamaño, fecha y hash nuevos y se vuelve a
  leer) y los `*.reiwand.json` junto a imágenes importadas o de proveedores. Al terminar escribe la marca `format-version`.

## Una vez y marcada

`client` corre solo si la marca es menor que la versión de datos actual (4); con la marca al día no lee ningún paquete ni
carpeta. De una marca menor que 3 hace todo; de 3 a 4 hace solo lo agregado después: el repositorio de ejemplos, los
paquetes importados y las ediciones sueltas. La marca se escribe en un temporal, con `fsync`, y se renombra; una marca
vacía cuenta como ausente. Los programas que usan los datos se niegan a arrancar con una marca vieja y nombran la orden.
Una conversión nueva de los datos guardados sube esa versión en uno y corre desde la marca anterior: con la marca al
día no corre nunca, así que una conversión agregada sin subirla no llegaría a quien ya tiene la marca. Una versión
anterior del programa se niega a arrancar con datos de una versión de datos mayor.

Un archivo dañado (un `.wpak` roto, un `scene.json` ilegible, un `state.json` vacío) no frena la conversión: se
informa «not converted RUTA: motivo» y se guarda la lista en `upgrade-copias/<tiempo>/not-converted.txt`; la marca se
escribe igual, porque la versión nueva tampoco leería ese archivo. `installed.json` se guarda después de cada paquete
reescrito, y si la corrida falla las firmas ya perdidas se nombran en el error. El recorrido no sigue enlaces simbólicos
a carpetas. El repositorio de ejemplos se rehace en `examples-repo.upgrading` y se intercambia al final; si se corta
entre medias, la corrida siguiente termina el intercambio. `save` conserva los permisos y escribe a través de enlaces.
La base de la biblioteca se reemplaza con un solo renombrado, sin ventana sin `library.db`.
- `reiwand-upgrade file RUTA... [--password-file ARCHIVO] [--copies DIR] [--check]`: una carpeta de escena o de
  paquete (todo lo que hay debajo), un `.wpak`, un `.reiwand` (si está cifrado sigue cifrado), una edición
  (`nombre.ext.reiwand.json`) o un descriptor de efecto. Es la orden de quien crea, antes de volver a firmar.
- `reiwand-upgrade server --data DIR [--check] [--then PROGRAMA ...]`: carpeta de datos y base del servidor.

## Garantías

- `--check` dice qué cambiaría y no escribe en los datos ni abre el llavero ni la base en escritura (para ver si un
  paquete cambia lo desempaqueta en una carpeta temporal del sistema).
- Cada archivo que cambia se copia antes en `upgrade-copias/<tiempo>/` con un `index.txt` (`número`, ruta original).
- Cada formato lleva su marca de versión (`format`, `schema`, esquema de la base): lo que ya está en la versión
  nueva se deja, así que repetir la orden no cambia nada.
- El recorrido de escenas y paquetes es uno solo (`scenes::walk_in`): `client` y `file` lo usan.
- Un paquete firmado que se reescribe pierde su firma de autor (el archivo ya no es el que firmó); quien lo publicó
  lo firma de nuevo con `file` sobre su carpeta de trabajo.
  En `client`, el paquete instalado que estaba firmado pasa a mostrarse «Local, sin firma verificada» (como uno importado):
  se le quita la firma, `signer` queda en `null` en `installed.json` y el informe nombra cada paquete («lost its signature:
  id versión») para que su autor lo publique de nuevo.
