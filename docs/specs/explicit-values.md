# Explicit values

Every Reiwand format stores all its values: a missing field is rejected naming its path, and no
program fills it in or converts a file from another version. Only the editor and the templates
prefill values.

- Test helper: `crates/reiwand-fill` (no application uses it) writes the omitted fields of the short
  documents of the tests (scene, manifest, image edit and effect descriptor) with the frozen schemas
  `frozen/*.json` and `effects.json` (parameters of the built-in effects). The integration scripts
  use it through the example `apps/reiwandd/examples/complete.rs` (`complete PATH...`).

## Common mechanism of the loader (`reiwand_core::explicit`)

It is used by the loaders of all formats; no format writes its own path-aware reading.

- `read(file, &Value)` and `read_str(file, text)` read a type and, if a key is missing, return
  `ReadError { file, path, fault }` with the full path (`layers[3].effects[1].params.reach`), also
  through `#[serde(flatten)]` and untagged enumerations. `ReadError::said()` goes through `said` and
  through the translated catalog: ""path" is missing. Reiwand does not fill in values: write it."
- An `Option` is always written: `#[serde(deserialize_with = "explicit::null")]` plus
  `#[schemars(required)]`. Without the serde attribute it accepts omission; with it, missing = error.
  Absence is `null`.
- Declared exception: an option of a package effect that declares `when_bound` (the variant depends on
  a texture being bound) accepts `null` in `params` of its use, and `null` means "the binding
  chooses it, as declared": with the texture bound it takes the `when_bound` value and unbound it
  takes its `prefill`. Any other option rejects `null` (`Issue::Type` at `/params/<option>`). The
  editor writes `null` in those options.
- Untagged enumeration: `Deserialize` is implemented with `untagged(d, &[(&["key"], variant), …])`;
  the hint key chooses which variant the error is reported for, with its exact path.
- Schema: `annotate(&mut schema, &Prefill(&[("Type.field", "JSON value")]))` marks every field as
  required and sets the prefilled value as `default` (annotation only; the loader never reads it) and
  returns the unknown rows and the fields without a row. One table per format, next to its crate.
- The four guards, as a test helper: `all_required(&schema)` (lists what is not required),
  `scan(&[folders], &[Rule…])` (`serde(default`, `skip_serializing_if`, `unwrap_or`, `Default`,
  `Option` without `explicit::null`; stops at `#[cfg(test)]`), `omission(&example, exempt, load)`
  (removes each key and demands the error with that path) and `round_trip(text, reread)` (same
  bytes).
- Sample type and tests: `crates/reiwand-core/tests/explicit.rs`.

## Settings (`config.toml`, format 3)

- The file carries `format = 3` and all fields. On first start the daemon writes the complete file
  from the template (`Config::template()`, the same as `Config::default()`); nothing is omitted.
- TOML has no `null`: each option is written as a word. `""` for image, list, preset and profile not
  chosen; `"general"` for fit, wallpaper, clicks, filter, quality, HDR and transition of a screen
  that sets nothing (in `[default]` it means the package or the factory value rules); `"none"` in
  `blank`; `"auto"` in `surfaces.layer_namespace`. The modules are in
  `reiwand_core::explicit::{empty, general, none}`.
- A `config.toml` without `format` or with a lower one is rejected on start: the daemon says it was
  made by an earlier version, that this one does not read it and that nothing was overwritten, and
  does not start; a higher one asks to update Reiwand. `update_config` also refuses to write over a
  file of another format.
- Prefill table: `crates/reiwand-core/prefill/config.tsv`. Each `Option` is marked `block` ("block":
  its absence means "feature unused": `blank`, `filter`, `lock`, `updates` and the `desktop` of one
  lock screen) or `value` ("value": its `null` has meaning): 5 and 18.
- `[lock]` (the wallpaper of the lock screen, `SetLockWallpaper` in `dbus.md`) is a block: absent,
  every lock screen follows the desktop, so a file without it is read as it is and the format stays
  3. Present, it carries `image` and `playlist` (`""` when not chosen) for every screen and
  `outputs`, a table per screen with the same two fields and, only when written, `desktop = true`
  (that screen shows its desktop while every lock screen has a choice; a file without it reads as
  before). The daemon removes the block when nothing is chosen any more.
- `[updates]` (the daily check for a new version of the AppImage, `updates.auto` in `dbus.md`) is a
  block too: absent, the check is on.
- Tests: `crates/reiwand-core/tests/config_explicit.rs` (first start, rejection without touching the
  file, required schema, each key removed, same bytes, sources without hidden values).

## Image edits (`edit.json`, format 3)

- Each edit carries `version = 3` and all fields: operations, masks, crop, fit, relief and parallax.
  What is not used is written `null` (class `block`: `span`, `depth`, `parallax`, `crop`, `opacity`,
  `cuts`) or a word (class `value`: `fit` `"general"`, `mask` `""`, `fill` and `layers` `"auto"`). The
  module `reiwand_core::explicit::auto` was added.
- `Edit::parse` rejects a file of another version with the message `edit.format` and names the field
  that is missing; an old file is not overwritten. The undo and redo cache (`history`) is read
  tolerantly: if it is old it is discarded.
- Prefill table: `crates/reiwand-core/prefill/edit.tsv`. Tests:
  `crates/reiwand-core/tests/edit_explicit.rs`.
- Limit: the path of the missing field inside a brush stroke does not carry the stroke's index.

## Input permissions and accounts (`inputs.json`, `inputs-declined.json`, `accounts.json`, format 3)

- Each file stores `{"format": 3, "content": ...}` (`reiwand_core::stored`). No field is optional: in
  `accounts.json` the `repo_id` of a link is explicit (`null` if it does not exist yet) and
  `serde(default)` was removed from `Rec`, `Link` and `File`. There is no prefill table: these files
  have no fields with a suggested value.
- The daemon refuses to start if a file is of another format or damaged and names the file and the
  format it reads; an unreadable file is no longer silently discarded. It is never overwritten.
  Tests: `apps/reiwandd/src/inputs/tests.rs`.

## Library (`library.db`, schema 9) and rotation state

- The JSON blocks of the database no longer accept omissions: `playlists.rules` (`Rules`: order,
  interval, hours, days, `synced`, `resume`, `advance_while_paused`, `skip_videos_on_battery`,
  `transition_ms` explicit or `null`), `profiles.outputs` (each output carries `item`, `path`,
  `playlist`, `fit`, `background`, `preset` and `filter`, `null` if unused), `profiles.auto`
  (`outputs` and `on_battery`), `presets.value` (`fit` and `background`, or `package` and `values`
  in package settings) and `rotation.state` (`anchor`, `since`, `behind` and `ahead` always written,
  also `anchor` in each element of `behind`/`ahead`).
- The daemon writes these blocks complete (the `null`s are no longer removed). A row missing a value
  is rejected when read, naming the field.
- The database carries schema 9; a database with another number is rejected without touching it.
- Schedules: the schedule box stores no files of its own; what is stored is the rotation state and
  the rules of each list, already covered above.
- The Android database uses the same schema and the same rule.

## Internal install files (`installed.json`, `anchors.json`, `repo-pins.json`)

- All three carry `{"format": 3, "content": ...}`; in `installed.json` `signer` and `revoked` are
  always written (`null` if none); `author`, the id of the author that signed it, is left out when it
was installed unsigned or before the field existed. The `state.json` of each repository already required all its
  fields and does not change.
- A file of another format or damaged makes the daemon not start and is never overwritten.
- `uploads-abandoned.json` (`{"format": 3, "content": {server: [upload ids]}}`) lists the uploads this
  computer opened and could not give back while the server was unreachable (`Publish1.UploadAs`);
  each is given back before the next upload to that server and then leaves the list. It is not read
  at start: a damaged one, or of another format, is left as it is and not used.

## Absent optional blocks (6.1)

An `Option` whose `None` means "feature unused" (mask, sound link, trail, `camera3d`, effect list of a
layer, help texts, references to another layer…) is a **block**: it may be missing from the file. If
present, all its fields are required. An `Option` whose `null` has a meaning of its own
(`Variant.when_bound`; `Material.blend` and the other values that choose an automatic behavior)
is a **value** and is always written. A field that follows its context (`Camera.center`,
`Transform.origin`, `Playback.end` and the `size` of image, video, solid, adjustment, mesh and model
layers) is written as a number or as its word (`canvas`, `anchor`, `end`, `source`), never as
`null`; the frozen schema of `reiwand-fill` takes the same words (`tests/explicit_scene.rs`).

- Absent and explicit `null` mean the same for a block: the loader accepts both. Whoever writes
  (editor, `scene new`, templates) does not write null blocks.
- Declaration: `#[serde(default, skip_serializing_if = "Option::is_none", deserialize_with =
  "explicit::block")]` plus `#[schemars(extend("x-null" = true, "x-block" = true))]`. A value keeps
  `explicit::null` and `#[schemars(required)]`.
- Classification: `crates/reiwand-scene/src/classes.tsv` (253 rows `Type.field<TAB>block|value`: 221
  blocks and 32 values). It is checked by `tests/classes.rs` against the source code and against the
  `x-block` of the schema; it is rebuilt with `REIWAND_WRITE_CLASSES=1` (two passes, the table is
  embedded at compile time). `reiwand-fill` carries a frozen copy in `frozen/blocks.tsv` so as not
  to fill blocks.
- Guards: `all_required` accepts a missing block; `omission` allows removing a whole block
  (`block_at`) and still demands an error when removing a field inside a present block; `round_trip`
  also demands that the result have no null blocks (`null_blocks`). `prune_blocks` removes them from
  a document.
- Writing: the editor works on the complete JSON and `Project::write_files` and the change
  fingerprint go through `plain` (`prune_blocks`); removing a block in the editor deletes the key.
- When the meaning of a block depends on the mode (`Binding.loop` with `keys`), the semantic check
  still requires it; the editor writes it (`contextual::fill`).
- Scope: scenes, effect descriptors and, since 7.0, the manifest, settings, edits, library, export
  and the credentials of the copy (see below).
- Size: the scenes of `scenes/` go from 166193 to 105339 bytes (−37 %) and the scenes, examples and
  effects of the repository from 524270 to 390665 bytes.

### Blocks of the other formats (7.0)

The same criteria apply to the other formats. Each field classified `block` in the tables
`crates/reiwand-core/prefill/{model,edit,config}.tsv`, `crates/reiwand-format/src/prefill.tsv` and
`crates/reiwand-carry/src/prefill.rs` is declared with `explicit::block`: it may be missing, and if
present it carries all its fields. Whoever writes emits no null blocks; whoever reads accepts absent
and `null`. `reiwand-fill` keeps the frozen copy `frozen/blocks-formats.tsv` and does not fill a null
block.

- Guard: `reiwand_core::explicit::declared` reads the class that the source code declares (block or
  value) and the `classes` tests of `reiwand-core`, `reiwand-format`, `reiwand-carry` and
  `reiwand-android` require it to match the table.

## Editor actions and `scene` command without invented values

- The table `crates/reiwand-scene/src/prefill.json` carries, besides the `Type.field` rows, two
  families that do not annotate the schema: `Action.<action>.<path>` (suggested value of a scalar
  argument of an action) and `Offer.<name>` (partial object that the editor offers when creating
  something: camera, trail, perspective, canvas, text or shape layers, lights, properties). The
  editor code reads them with `prefill::arg`, `prefill::offer` and `prefill::canvas`; no constants
  remain in the create actions.
- `reiwand-ctl scene actions NAME` shows each optional argument as `(optional, prefill X)`.
- Every create action completes the file with `completing`; `apps/reiwand-ctl/tests/scene/examples.rs`
  strictly loads `manifest.json` and `scene.json` after each example of `scene act`.
- Tests: `crates/reiwand-editor/tests/offers.rs` and `manual/tests.rs`.
- Limit: the editor's QML still carries literal alternatives (`?? 16`, `?? [1920, 1080]` and the like)
  that repeat rows of the table; they are replaced when the editor hands the table to the interface.
