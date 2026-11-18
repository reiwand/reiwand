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
