# Valores explícitos: paso A de la conversión

`reiwand-upgrade` escribe en cada dato guardado todo valor que el formato deja omitido, con el valor que el
motor usaba hasta ahora para él. El dato se lee exactamente igual antes y después.

- Esquemas congelados: `crates/reiwand-upgrade/frozen/*.json` (escena, manifiesto, `config.toml`, ediciones de
  imagen y archivo de exportación) y `effects.json` (parámetros de los efectos incluidos). Son datos, no se
  regeneran con el código: se rehacen con `cargo test -p reiwand-upgrade --test freeze -- --ignored` solo
  mientras el paso A no haya salido.
- Tabla campo por campo: `crates/reiwand-upgrade/table/fields.tsv` (formato, `Tipo.campo`, forma, obligatorio,
  clase, valor oculto, línea de la especificación). Clase 1: constante; clase 2: depende del contexto (palabra
  explícita); clase 3: ausencia (`null`, `[]`, `{}`). Se rehace con `cargo test -p reiwand-upgrade --test table
  -- --ignored write_table`.
- `table/render-defaults.tsv`: los números que el código de dibujo pone cuando un campo opcional falta.
- `reiwand-upgrade file DIR` recorre las escenas con este motor; un paquete `.wpak` lo desempaqueta,
  completa y empaqueta su autor. `config.toml` conserva sus comentarios (`toml_edit`) y no puede escribir
  `null`: TOML no lo tiene.
- Pruebas: `tests/identical.rs` (lectura idéntica sobre ejemplos y fixtures; con `REIWAND_CORPUS=DIR` suma
  otras escenas), `tests/formats.rs` (pares original y completado de config, edición y exportación),
  `tests/table.rs` (la tabla cubre toda propiedad opcional del esquema vivo y toda constante escrita con
  `unwrap_or` en el código de dibujo).

- Sonido: el paso A quita `audio.gain` del bloque `audio` de la escena (`fixed` escribe `absolute` en cada enlace
  al sonido y `auto` deja `relative`) y escribe `kind` (`relative`) y `unit` (`db`) en cada enlace al sonido,
  en cada regla `remap` de entrada `audio:` y en cada `turbulent` con `audio`; la tabla los marca clase 2
  porque solo se escriben donde hay sonido. `depth_parallax` recibe `reach` 0 (círculo) del descriptor.

## Mecanismo común del cargador (`reiwand_core::explicit`)

Lo usan los cargadores de todos los formatos; ningún formato escribe su propia lectura con ruta.

- `read(archivo, &Value)` y `read_str(archivo, texto)` leen un tipo y, si falta una clave, devuelven
  `ReadError { file, path, fault }` con la ruta completa (`layers[3].effects[1].params.reach`), también a
  través de `#[serde(flatten)]` y de enumeraciones sin etiqueta. `ReadError::said()` pasa por `said` y por el
  catálogo traducido: «falta "ruta". Reiwand no completa valores: escríbelo, o ábrelo en el editor, que lo
  prellena.»
- Una `Option` se escribe siempre: `#[serde(deserialize_with = "explicit::null")]` más
  `#[schemars(required)]`. Sin el atributo serde acepta la omisión; con él, falta = error. La ausencia es `null`.
- Excepción declarada: una opción de un efecto de paquete que declara `when_bound` (la variante depende de que una textura esté ligada) acepta `null` en `params` de su uso, y `null` significa «la elige la ligadura, como se declaró»: con la textura ligada toma el valor `when_bound` y sin ligar toma su `prefill`. Cualquier otra opción rechaza `null` (`Issue::Type` en `/params/<opción>`). El editor escribe `null` en esas opciones.
- Enumeración sin etiqueta: se implementa `Deserialize` con `untagged(d, &[(&["clave"], variante), …])`; la
  clave de pista elige de qué variante se informa el error con su ruta exacta.
- Esquema: `annotate(&mut esquema, &Prefill(&[("Tipo.campo", "valor JSON")]))` marca todo campo como
  obligatorio y pone el valor prellenado como `default` (solo anotación; el cargador nunca lo lee) y devuelve
  las filas desconocidas y los campos sin fila. Una tabla por formato, junto a su crate.
- Las cuatro guardas, como ayuda de pruebas: `all_required(&esquema)` (lista lo que no es obligatorio),
  `scan(&[carpetas], &[Rule…])` (`serde(default`, `skip_serializing_if`, `unwrap_or`, `Default`, `Option` sin
  `explicit::null`; se detiene en `#[cfg(test)]`), `omission(&ejemplo, exento, carga)` (quita cada clave y exige
  el error con esa ruta) y `round_trip(texto, relee)` (mismos bytes).
- Tipo de muestra y pruebas: `crates/reiwand-core/tests/explicit.rs`.

## Ajustes (`config.toml`, formato 3)

- El archivo lleva `format = 3` y todos los campos. En el primer arranque el demonio escribe el archivo
  completo desde la plantilla (`Config::template()`, igual que `Config::default()`); nada se omite.
- TOML no tiene `null`: cada opción se escribe como una palabra. `""` para imagen, lista, preajuste y perfil
  sin elegir; `"general"` para ajuste, fondo, clics, filtro, calidad, HDR y transición de una pantalla que no
  fija nada (en `[default]` significa que manda el paquete o el valor de fábrica); `"none"` en `blank`;
  `"auto"` en `surfaces.layer_namespace`. Los módulos están en `reiwand_core::explicit::{empty, general, none}`.
- Un `config.toml` sin `format` o con uno menor se rechaza al arrancar: el demonio dice
  `reiwand-upgrade client` y que no se sobrescribió nada, y no arranca; uno mayor pide actualizar Reiwand.
  `update_config` también se niega a escribir sobre un archivo de otro formato.
- La conversión de única vez es `reiwand-upgrade client [--config DIR]` (paso A con los valores de hoy, más
  `format`, las palabras y las tablas que faltaban); conserva los comentarios y deja el original en
  `upgrade-copias/<segundos>/config.toml`. Lo que resuelve cada pantalla es idéntico antes y después
  (`crates/reiwand-upgrade/tests/settings.rs`).
- Tabla de prellenado: `crates/reiwand-core/prefill/config.tsv`. Cada `Option` está marcada `bloque` (su
  ausencia significa «función sin usar»: `blank`, `filter`) o `valor` (su `null` tiene sentido): 2 y 14.
- Pruebas: `crates/reiwand-core/tests/config_explicit.rs` (primer arranque, rechazo sin tocar el archivo,
  esquema obligatorio, cada clave quitada, mismos bytes, fuentes sin valores ocultos).

## Ediciones de imagen (`edit.json`, formato 3)

- Cada edición lleva `version = 3` y todos los campos: operaciones, máscaras, recorte, ajuste, relieve y
  paralaje. Lo que no se usa se escribe `null` (clase `bloque`: `span`, `depth`, `parallax`, `crop`,
  `opacity`, `cuts`) o una palabra (clase `valor`: `fit` `"general"`, `mask` `""`, `fill` y `layers` `"auto"`).
  Se añadió el módulo `reiwand_core::explicit::auto`.
- `Edit::parse` rechaza un archivo de otra versión con el mensaje `edit.format` y nombra el campo que falta;
  un archivo viejo no se sobrescribe. La caché de deshacer y rehacer (`history`) se lee con tolerancia: si es
  vieja se descarta.
- La conversión de única vez es el paso A de `reiwand-upgrade client` sobre cada `edit.json`
  (`crates/reiwand-upgrade/src/edits.rs`); deja el original en `upgrade-copias/<segundos>/`. Es idempotente y
  la edición resuelve lo mismo antes y después (`crates/reiwand-upgrade/tests/edits.rs`).
- Tabla de prellenado: `crates/reiwand-core/prefill/edit.tsv`. Pruebas:
  `crates/reiwand-core/tests/edit_explicit.rs`.
- Límite: la ruta del campo que falta dentro de un trazo de pincel no lleva el índice del trazo.

## Permisos de entradas y cuentas (`inputs.json`, `inputs-declined.json`, `accounts.json`, formato 3)

- Cada archivo guarda `{"format": 3, "content": ...}` (`reiwand_core::stored`). Ningún campo es opcional:
  en `accounts.json` el `repo_id` de un enlace es explícito (`null` si aún no existe) y se quitó `serde(default)`
  de `Rec`, `Link` y `File`. No hay tabla de prellenado: estos archivos no tienen campos con valor sugerido.
- El demonio se niega a arrancar si un archivo es de otro formato o está dañado, nombra el archivo y manda
  correr `reiwand-upgrade client`; ya no se descarta en silencio un archivo ilegible. Nunca se sobrescribe.
- La conversión de única vez es el paso A de `reiwand-upgrade client`
  (`crates/reiwand-upgrade/src/stored.rs`): envuelve el contenido, deja el original en
  `upgrade-copias/<segundos>/` y es idempotente. Pruebas: `crates/reiwand-upgrade/tests/stored.rs`,
  `apps/reiwandd/src/inputs/tests.rs`.

## Biblioteca (`library.db`, esquema 9) y estado de rotación

- Los bloques JSON de la base ya no aceptan omisiones: `playlists.rules` (`Rules`: orden, intervalo, horas,
  días, `synced`, `resume`, `advance_while_paused`, `skip_videos_on_battery`, `transition_ms` explícito o
  `null`), `profiles.outputs` (cada salida lleva `item`, `path`, `playlist`, `fit`, `background`, `preset` y
  `filter`, `null` si no se usa), `profiles.auto` (`outputs` y `on_battery`), `presets.value` (`fit` y
  `background`, o `package` y `values` en los ajustes de paquete) y `rotation.state` (`anchor`, `since`,
  `behind` y `ahead` siempre escritos, también `anchor` en cada elemento de `behind`/`ahead`).
- El demonio escribe estos bloques completos (los `null` ya no se quitan). Una fila a la que le falte un valor
  se rechaza al leerla nombrando el campo y `reiwand-upgrade client`.
- La base lleva un número de esquema nuevo; una base con el número anterior se rechaza y el mensaje nombra
  `reiwand-upgrade client`.
- La conversión de única vez trabaja sobre una copia de la base, completa los bloques con los valores de hoy,
  fija la versión 9, deja la original en `upgrade-copias/<segundos>/` y pone la copia en su lugar.
- Horarios: la caja de horarios no guarda archivos propios; lo guardado es el estado de rotación y las reglas de
  cada lista, ya cubiertos arriba.
- Límite: la base de Android usa el mismo esquema; su conversión la hace el receptor de actualización de esa
  aplicación.

## Archivos internos de instalación (`installed.json`, `anchors.json`, `repo-pins.json`)

- Los tres llevan `{"format": 3, "content": ...}`; en `installed.json` `signer` y `revoked` se escriben
  siempre (`null` si no hay). `state.json` de cada repositorio ya exigía todos sus campos y no cambia.
- Un archivo de otro formato o dañado hace que el demonio no arranque (`reiwand-upgrade client`) y nunca se
  sobrescribe. Conversión de única vez: `crates/reiwand-upgrade/src/stored.rs`; prueba en
  `crates/reiwand-upgrade/tests/stored.rs`.

## Bloques opcionales ausentes (6.1)

Una `Option` cuyo `None` significa «función no usada» (máscara, enlace al sonido, estela, `camera3d`, lista de
efectos de una capa, textos de ayuda, referencias a otra capa…) es un **bloque**: puede faltar en el archivo. Si
está, todos sus campos son obligatorios. Una `Option` cuyo `null` tiene sentido propio (`Playback.end` nulo =
hasta el final; `Variant.when_bound`; `Material.blend` y los demás valores que eligen un comportamiento
automático) es un **valor** y se escribe siempre.

- Ausente y `null` explícito significan lo mismo para un bloque: el cargador acepta ambos, así que los archivos
  de la 6.0 se leen igual. Quien escribe (editor, `reiwand-upgrade`, `scene new`, plantillas) deja de escribir
  los bloques nulos.
- Declaración: `#[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "explicit::block")]`
  más `#[schemars(extend("x-null" = true, "x-block" = true))]`. Un valor sigue con `explicit::null` y
  `#[schemars(required)]`.
- Clasificación: `crates/reiwand-scene/src/classes.tsv` (253 filas `Tipo.campo<TAB>block|value`: 221 bloques y 32
  valores). Lo comprueba `tests/classes.rs` contra el código fuente y contra los `x-block` del esquema; se rehace
  con `REIWAND_WRITE_CLASSES=1` (dos pasadas, la tabla se incrusta al compilar). `reiwand-upgrade` lleva su
  copia congelada en `frozen/blocks.tsv` y una prueba exige que sea idéntica.
- Guardas: `all_required` acepta que falte un bloque; `omission` permite quitar un bloque entero (`block_at`) y
  sigue exigiendo error al quitar un campo dentro de un bloque presente; `round_trip` exige además que el
  resultado no tenga bloques nulos (`null_blocks`). `prune_blocks` los quita de un documento.
- Escritura: `reiwand-upgrade` no inserta un valor oculto nulo en un bloque; el editor trabaja sobre el JSON
  completo y `Project::write_files` y la huella de cambios pasan por `plain` (`prune_blocks`); quitar un bloque
  en el editor borra la clave.
- Cuando el significado de un bloque depende del modo (`Binding.loop` con `keys`), la comprobación semántica
  sigue pidiéndolo; el editor lo escribe (`contextual::fill`).
- Alcance: escenas, descriptores de efectos y, desde la 7.0, el manifiesto, los ajustes, las ediciones, la biblioteca, la exportación y las credenciales de la copia (ver abajo).
- Tamaño: las escenas de `scenes/` pasan de 166193 a 105339 bytes (−37 %) y las escenas, ejemplos y efectos
  del repositorio de 524270 a 390665 bytes.

### Bloques de los demás formatos (7.0)

Los mismos criterios se aplican a los formatos de la parte 2. Cada campo clasificado `bloque` en las tablas
`crates/reiwand-core/prefill/{model,edit,config}.tsv`, `crates/reiwand-format/src/prefill.tsv` y
`crates/reiwand-carry/src/prefill.rs` se declara con `explicit::block`: puede faltar, y si está lleva todos sus
campos. Quien escribe no emite bloques nulos; quien lee acepta ausente y `null`. `reiwand-upgrade` guarda la copia
congelada `frozen/blocks-formats.tsv` y no rellena un bloque nulo.

- Guarda: `reiwand_core::explicit::declared` lee la clase que declara el código fuente (bloque o valor) y las
  pruebas `classes` de `reiwand-core`, `reiwand-format`, `reiwand-carry` y `reiwand-android` exigen que coincida
  con la tabla.

## Acciones del editor y comando `scene` sin valores inventados

- La tabla `crates/reiwand-scene/src/prefill.json` lleva, además de las filas `Tipo.campo`, dos familias que no
  anotan el esquema: `Action.<acción>.<ruta>` (valor sugerido de un argumento escalar de una acción) y
  `Offer.<nombre>` (objeto parcial que el editor ofrece al crear algo: cámara, estela, perspectiva, lienzo,
  capas de texto o forma, luces, propiedades). El código del editor las lee con `prefill::arg`, `prefill::offer`
  y `prefill::canvas`; no quedan constantes en las acciones de crear.
- `reiwand-ctl scene actions NOMBRE` muestra cada argumento opcional como `(optional, prefill X)`.
- Toda acción de crear completa el archivo con `completing`; `apps/reiwand-ctl/tests/scene/examples.rs` carga
  de forma estricta `manifest.json` y `scene.json` tras cada ejemplo de `scene act`.
- Pruebas: `crates/reiwand-editor/tests/offers.rs` y `manual/tests.rs`.
- Límite: el QML del editor aún lleva alternativas literales (`?? 16`, `?? [1920, 1080]` y similares) que
  repiten filas de la tabla; se sustituyen cuando el editor entregue la tabla a la interfaz.
