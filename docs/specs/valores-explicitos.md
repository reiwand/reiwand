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
- `reiwand-upgrade scenes DIR` recorre las escenas con este motor; un paquete `.wpak` lo desempaqueta,
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
- Alcance: escenas y descriptores de efectos. Manifiesto, ajustes, ediciones y exportación quedan como estaban.
- Tamaño: las escenas de `scenes/` pasan de 166193 a 105339 bytes (−37 %) y las escenas, ejemplos y efectos
  del repositorio de 524270 a 390665 bytes.
