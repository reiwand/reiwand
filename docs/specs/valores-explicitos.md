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
