# Export and import file (`.reiwand`)

The `reiwand-carry` crate defines the file that moves a device's configuration to another device. It is a pure library (no D-Bus, no network) used by the daemon, the CLI and the phone. Exporting and importing is always a file the user moves as they like; there is no account, server or QR involved.

## Container

A `.reiwand` file is a zip archive:

- `carry.json`: the manifest. Fields: `format` (1), `program`, `version`, `created` (text given by the caller), `sections` (names of the sections held), `encrypted`, and `sums` (SHA-256 of every file under `files/`).
- `files/content.json`: the data of every section.
- `files/own/<id>`: one file per own wallpaper (stored, not recompressed). `<id>` only has letters, digits, `.`, `-` and `_`.

Unencrypted files carry no signature: integrity comes from the sums in the manifest. They catch accidents and tampering with one file, not a deliberate rewrite of the whole archive. A file that asks for secrets is always encrypted.

## Sections

- `repositories`: address, `repo_id` anchor and server origin of each added repository.
- `providers`: photo providers (kind, name, configuration) without secrets.
- `lists`: name and items (`repo_id` optional, `id`).
- `favourites`, `ratings`, `tags`: per item.
- `installed`: repository wallpapers as references `(repo_id, id, version)`. They are downloaded again on import and never carried.
- `settings`: portable settings only.
- Optional `own`: the files of the user's own wallpapers.
- Optional `secrets`: account sessions per repository and provider keys.

## Settings of this device

These keys are never exported and are dropped on import even if a file holds them: playback volume and mute, `playback.software_parallax`, `playback.max_fps`, the thumbnail cache size, the model download permission, `render.gpu`, `playback.quality`, the layer namespace, remote covers, the private network switch, battery pause (`pause.on_battery`), the HDR choice (`playback.hdr`), the bezel size (`layout.bezel_mm`) and everything that names a screen of this machine (`outputs.`, `screens.`, `device.`, `layout.`, `rgb.` prefixes and the per-screen `playback.quality.<output>`, `playback.hdr.<output>` and `transition.kind.<output>`). The list is `DEVICE_KEYS` and `is_device_local` in the crate; `portable` filters a settings map.

## Encryption

When the user ticks secrets, the whole zip is encrypted with a password chosen at export. Key: Argon2id (default 64 MiB, 3 passes; the cost is stored in the header and bounded on read) over a random 16-byte salt. Cipher: XChaCha20-Poly1305 in 64 KiB segments. The nonce is a 19-byte random prefix, the segment number and a flag on the last segment, so reordering and truncation are detected; the header is authenticated with every segment. A file starts with `REIWANDE` and a header version byte. Errors are distinct: needs a password, wrong password or damaged, truncated.

## Hostile input

Reading refuses: more than 100,000 entries, an entry above 8 GiB, a total above 16 GiB, an expansion ratio above 200 for entries above 1 MiB, a JSON file above 32 MiB, names with `..`, absolute paths, backslashes or `:`, symbolic links, own wallpaper ids that are not plain names, and a `format` above the one the program knows (error asking to update). Sizes are enforced while reading, not only from the declared values. `Limits` makes them adjustable.

## Merge rules

`plan(content, local, pick)` is a pure function that returns the changes to apply; `preview(content, local)` returns, per section, how many items the file holds, how many are new, skipped and in conflict, for the import checklist. Importing adds and never replaces:

- A repository with a `repo_id` already present is skipped; the same for a provider with the same kind and name.
- Favourites and tags are added; a rating is set only when the item has none.
- Installed references already installed are skipped.
- A list with the same name (ignoring case and spaces) is joined (only missing items are added) or kept as a second list, which gets the suffix passed by the caller (the crate has no interface texts) and a number if that name is also taken.
- Settings only when `pick.settings` is set, and never device-local ones.
- Own wallpapers already present by id are skipped; secrets are not added for a repository or provider key that already has one.

## Limits

An encrypted file is built and opened in memory, so very large own wallpapers in an encrypted export need that much memory. Plain files stream from and to disk.

## En el daemon (`Carry1`)

El daemon (`apps/reiwandd/src/carry`) llena el contenido con el estado real: repositorios añadidos (dirección, `repo_id` y origen; el repositorio de ejemplos no viaja), proveedores, listas, favoritos, valoraciones, etiquetas, paquetes instalados como referencias (`repo_id`, `id`), los ajustes portables y, si se marca, los fondos propios con su archivo. Las listas inteligentes viajan con su búsqueda y sus reglas (sin elementos). Los repositorios llevan la huella del certificado anclada para su origen. Las claves de los proveedores y las sesiones de cuenta salen solo con `secrets` y siempre cifradas. Al importar, cada equipo obtiene su propia sesión con `POST /api/v1/auth/session/copy` (la sesión del archivo no se reutiliza); si el servidor no responde o la rechaza, la cuenta queda como pendiente «sign in again» y el resto se importa.

Importar solo agrega y sigue este orden: repositorios, proveedores y claves, paquetes, fondos propios, biblioteca (listas, favoritos, valoraciones, etiquetas) y ajustes. Un repositorio que pide confirmación queda como pendiente y no hace fallar el resto. Lo que se refiere a un fondo que no está aquí se cuenta como no resuelto. Con `lists = join` una segunda importación no duplica nada; con `both` cada importación crea otra copia con el sufijo (` (importada)`, ` (importada) 2`…).

`reiwand-ctl export ARCHIVO [--own] [--secrets] [--password-file ARCHIVO]` y `reiwand-ctl import ARCHIVO [--list] [--take SECCIÓN]… [--lists join|both] [--suffix TEXTO]`; `--list` solo muestra lo que trae. Ver `dbus.md`, `Carry1`.

## En la app de escritorio

Ajustes > Equipo > «Mover mi configuración» ofrece «Exportar mi configuración…» e «Importar una configuración…». Exportar permite marcar los fondos propios (muestra el tamaño que suman) y las contraseñas y claves (pide la contraseña dos veces y cifra todo el archivo), y ejecuta `Carry1.Export` como tarea. Importar abre el archivo, pide la contraseña si está cifrado, muestra la vista previa por sección (nuevos, ya presentes, conflictos de nombre; todo marcado salvo los ajustes), deja elegir «Unir» o «Tener las dos» en las listas con el mismo nombre y ejecuta `ImportChosen` como tarea. El resultado lista lo añadido y los pendientes con botones hacia la página que corresponde.

## En el teléfono

`crates/reiwand-android` (`carry`) arma el mismo contenido que el daemon y aplica el mismo orden y el mismo informe. El archivo se elige con el selector de archivos del sistema: Kotlin copia el documento a la caché de la app (o desde ella) y Rust trabaja sobre esa ruta. Operaciones de `Native.account`: `carry_preview`, `carry_export` (`own`, `secrets`, `password`) y `carry_import` (`take`, `lists`, `suffix`). Un archivo exportado en el escritorio se importa en el teléfono y al revés; las claves de los proveedores y las sesiones siguen las mismas reglas. El teléfono no tiene subcomando de terminal: la paridad es la de los dos programas sobre el mismo archivo.
