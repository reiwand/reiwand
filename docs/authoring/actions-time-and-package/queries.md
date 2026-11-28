# Queries

### Before you start

Queries are made with `reiwand-ctl scene ask FOLDER NAME 'JSON'` and answer with JSON; they change nothing. Some ask for an object with arguments, others a single value (such as an asset's id in quotes). The rules for ids, paths and coordinates are those of [the actions](#before-you-start). The layers that are copied are asked for with [`copy`](../actions-layers/queries.md#copy); from the command line there is also `reiwand-ctl scene ask FOLDER copy`.

### Package texts, credits and notices

#### `strings`

Answers with the package's text table: which languages there are, which keys, in which languages each one is missing, where the texts live and the texts the engine already brings. It is for knowing what is left to translate before publishing. It is written with the `text` variant of [`manifest`](package.md#manifest).

It takes no arguments: pass `{}`.

The answer has: `default` (the base language), `languages` (all the languages), `places` (per language, `manifest` if its texts are in the data sheet or `file` if they are in `locale/LANGUAGE.json`), `keys` (for each key: `key`, `texts` with the text of each language, `missing` with the languages it lacks and `used`, which says whether any text in the data sheet or the scene names it as `@key`), `engine` (the texts the engine already knows how to say, with their `key`, `en` and `es`) and `reads_files` (whether the package declares that it reads language files).

```sh
reiwand-ctl scene act my-scene manifest '{"text":{"key":"greeting","language":"en","text":"Hello"}}'
reiwand-ctl scene act my-scene manifest '{"text":{"key":"greeting","language":"de","text":"Hallo"}}'
reiwand-ctl scene ask my-scene strings '{}'
```

#### `credits`

Answers with the package files that can be credited, the credits there are and the files that have none. It is for not publishing other people's files without giving their author. It is written with the `credit` and `uncredit` variants of [`manifest`](package.md#manifest).

It takes no arguments: pass `{}`.

The answer has: `assets` (each file with its `path`, its `kind`, its `size` in bytes and `credited`, whether any credit covers it), `credits` (each with `paths`, `license`, `attribution` and `url`) and `missing` (the paths of the files with no credit).

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene import '{"file":"sound.ogg"}'
reiwand-ctl scene act my-scene manifest '{"credit":{"paths":["assets/picture.png"],"license":"CC-BY-4.0","attribution":"Alex Morgan"}}'
reiwand-ctl scene ask my-scene credits '{}'
```

The last line gives `"missing": ["assets/sound.ogg"]`.

#### `preset_steps`

Answers what a preset package still lacks before it is published. It is for knowing whether it can already be packaged, signed and added to a repository. On a package that is not a preset it answers `"preset": false`.

It takes no arguments: pass `{}`.

The answer has: `preset` (whether the package is a preset), `ready` (whether all the steps are done), `errors` (how many errors the check has) and `steps`, with an `id` and a `done` per step: `base` (the base package is chosen), `values` (the preset gives some value or file), `title`, `license`, `preview` (it has a thumbnail) and `check` (the check has no errors).

```sh
reiwand-ctl scene new my-scene --from preset
reiwand-ctl scene ask my-scene preset_steps '{}'
```

The last line gives `"ready": false`: a freshly created preset already has a title and a license, and lacks `base`, `values`, `preview` and `check`.

#### `flashes`

Answers with the last result of measuring how much the scene flashes, or with a new measurement if you ask for `run`. It is for deciding whether the package should be marked as sensitive to flashes. The result is stored with the scene's state as long as the scene does not change, and any other `reiwand-ctl` call sees it: there is no need to ask for `run` again after `shot` with `flashes`.

| Argument | Type | What it means |
|-----------|------|---------------|
| `run` | yes/no | `true` measures now and answers with the result. If missing, it answers with the last one or with `null`. |
| `from` | number | Second where it starts. Default 0. |
| `seconds` | number | How long it measures, from 0.1 to 60. Default 8. |
| `fps` | integer | Frames per second of the measurement, from 1 to 30. Default 15. |

The answer has: `flashing` (whether it went over the safe limit of 3 flashes per second), `rate` (the maximum flashes per second), `at` (the second of the first dangerous flash, or `null`), and `from`, `seconds` and `fps` of the measurement. If it fails, `{"error": ...}`.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"solid"}'
reiwand-ctl scene ask my-scene flashes '{"run":true,"from":0,"seconds":1,"fps":5}'
reiwand-ctl scene ask my-scene flashes '{}'
```

The last line gives the measurement the previous one made; if the scene changes afterwards, it gives `null`.

### Keys, curves and triggers

#### `curves`

Answers with the keys of an animated value, with their handles and with the curve already computed as points, to draw it or to review an easing.

| Argument | Type | What it means |
|-----------|------|---------------|
| `pointer` | text (required) | Path of a value with keys, such as `/scene/layers/0/opacity`. |
| `samples` | integer | How many points to compute, from 2 to 512. Default 96. |
| `from` | number | Second where the curve starts. Default, that of the first key. |
| `to` | number | Second where it ends. Default, that of the last key (at least 1 s after `from`). |
| `picked` | list of numbers | Seconds of the picked keys, to compute the selection's bounds. |

The answer has: `pointer`, `kind` (the value's type, for example `float`), `unit`, `range` (from which second to which it was computed), `keys` (per key: `t`, `value`, `parts`, `ease`, and `in` and `out`, the handles, with their `dt` in seconds, their value `dv` and whether the user set them: `set`), `tracks` (per component of the value, `name`, `part` and `points`, the list of `[second, value]` pairs) and `bounds` (the bounds of all the values and of the picked ones). If the value has no keys, `{"error": ...}`.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png","id":"sky"}'
reiwand-ctl scene act my-scene keyframe '{"pointer":"/scene/layers/0/opacity","time":0,"value":0}'
reiwand-ctl scene act my-scene keyframe '{"pointer":"/scene/layers/0/opacity","time":2,"value":1}'
reiwand-ctl scene ask my-scene curves '{"pointer":"/scene/layers/0/opacity","samples":3}'
reiwand-ctl scene ask my-scene curves '{"pointer":"/scene/layers/0/opacity","samples":3,"picked":[2]}'
```

The first of those queries gives, among other things, `"points": [[0.0, 0.0], [1.0, 0.5], [2.0, 1.0]]`.

#### `key_clip`

Turns keys into text that can be copied and pasted, or reads such a text and says which keys it carries. The text starts with `reiwand-keys:`.

| Argument | Type | What it means |
|-----------|------|---------------|
| `copy` | list of objects | What to copy: each with `pointer` (path of a value with keys) and `times` (the seconds of the keys). |
| `read` | text | A clipboard text. If you give it, the query reads it instead of copying. |

With `copy`, the answer has `picks` (each value with its `keys`) and `text`, the text for the clipboard. With `read`, the answer has `picks` with the keys the text carries, or `null` if the text is not a keys text. If a path has no keys, `{"error": ...}`. To paste keys there is a variant of the `edit_keys` action (see [actions](../actions.md)).

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png","id":"sky"}'
reiwand-ctl scene act my-scene keyframe '{"pointer":"/scene/layers/0/opacity","time":0,"value":0.2}'
reiwand-ctl scene act my-scene keyframe '{"pointer":"/scene/layers/0/opacity","time":1,"value":1}'
reiwand-ctl scene ask my-scene key_clip '{"copy":[{"pointer":"/scene/layers/0/opacity","times":[0,1]}]}'
reiwand-ctl scene ask my-scene key_clip '{"read":"reiwand-keys:{\"picks\":[{\"pointer\":\"/scene/layers/0/opacity\",\"keys\":[[0,0.2]]}]}"}'
reiwand-ctl scene ask my-scene key_clip '{"read":"hello"}'
```

The last query gives `{"picks": null}`.

#### `fires`

Answers with the timelines and sound layers that a trigger starts. It is for knowing what happens when an event sounds or something is pressed. The trigger is written as in `trigger` of the [`timeline`](timeline-and-keys.md#timeline) action: `start`, `event:NAME`, `click:LAYER`, `prop:ID`...

The argument is the trigger, as a JSON text in quotes.

The answer has: `trigger`, `timelines` (ids of the timelines that have it as `trigger`) and `sounds` (ids of the sound layers whose `play_on` is that trigger).

```sh
reiwand-ctl scene act my-scene timeline '{"id":"one","add":true,"add_events":[{"t":1,"name":"boom"}]}'
reiwand-ctl scene act my-scene timeline '{"id":"two","add":true,"trigger":"event:boom"}'
reiwand-ctl scene act my-scene add_layer '{"kind":"sound","file":"sound.ogg","id":"voice"}'
reiwand-ctl scene act my-scene set '{"pointer":"/scene/layers/0/play_on","value":"event:boom"}'
reiwand-ctl scene ask my-scene fires '"event:boom"'
```

The last query gives `{"sounds": ["voice"], "timelines": ["two"], "trigger": "event:boom"}`.

#### `cues`

Answers with every trigger that the timelines and sound layers of the scene start on, each as the `fires` query answers it, in the order of their names. The Inputs panel of the editor lists them with a button to fire each.

The argument is an empty object.

```sh
reiwand-ctl scene act my-scene timeline '{"id":"one","add":true,"add_events":[{"t":1,"name":"boom"}]}'
reiwand-ctl scene act my-scene timeline '{"id":"two","add":true,"trigger":"event:boom"}'
reiwand-ctl scene ask my-scene cues '{}'
```

The last query gives `[{"sounds": [], "timelines": ["two"], "trigger": "event:boom"}]`.

### Assets and media

#### `palette`

Answers with an image's main colors, to get a palette that matches it.

The argument is the id of an image asset, as a JSON text in quotes.

The answer is a list of 5 `#rrggbb` colors, or `null` if the asset does not exist or is not a still image.

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene ask my-scene palette '"picture"'
```

#### `clocks`

Answers with the video layers that have their own clock (`playback.own_clock`) and where each clock is at a second of the scene. It is for knowing which frame of its video each one would be at. It only counts the videos that have `own_clock` set to `true`.

| Argument | Type | What it means |
|-----------|------|---------------|
| `time` | number | Second of the scene. Default 0. |
| `strip` | integer | How many thumbnails to ask for from each video, spread over its length. Default 0, none. |

The answer has `time` and `clocks`, and each clock brings: `layer`, `pointer`, `videos`, `length` (the video's length in seconds), `start`, `end`, `rate`, `loop`, `playing`, `at` (the second of the video it is at) and `keyed` (whether the speed has keys). With `strip`, each clock also brings `strip`, a list of `{at, file}` with the thumbnail of each instant.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"video","file":"video.webm","id":"clip"}'
reiwand-ctl scene act my-scene set '{"pointer":"/scene/layers/0/playback","value":{"own_clock":true,"rate":2}}'
reiwand-ctl scene ask my-scene clocks '{"time":0.25}'
```

#### `pools`

Answers with the scene's shape pools, with the layers that draw them and the scripts that fill them. It is for checking that each pool has something that draws it and something that builds it. They are edited with the [`pool`](scripts-and-templates.md#pool) action.

It takes no arguments: pass `{}` (or nothing).

The answer has `limit` (how many pools fit, 16) and `pools`, and each pool brings: `id`, `pointer`, `max` (shapes), `max_vertices`, `texture`, `size`, `layers` (ids of the layers that draw it) and `scripts` (those that declare `geometry`).

```sh
reiwand-ctl scene act my-scene pool '{"id":"shapes","add":true}'
reiwand-ctl scene act my-scene add_layer '{"kind":"shapes","source":"shapes","id":"drawing"}'
reiwand-ctl scene ask my-scene pools '{}'
```

### Scripts and shaders

#### `script_uses`

Says on which lines of the scene's scripts an id appears in quotes (single, double or template). It is for knowing which scripts still name a layer, a particle system or a template before renaming or removing it: the `rename` action does not rewrite scripts.

| Argument | Type | What it means |
|---|---|---|
| `name` | text | (required) The id that is searched for. |

The answer is a list of `{path, line}`: the script and the line number (from 1). It is an empty list if no script names it.

```sh
printf 'layers.show("sky");\n' > one.js
reiwand-ctl scene act my-scene import_script '{"file":"one.js"}'
reiwand-ctl scene ask my-scene script_uses '{"name":"sky"}'
```

#### `uses`

Answers, for each script in the scene, which parts of the engine it declares, which it detects in its code and which it still lacks declaring. It is for checking that the package asks for the permissions its scripts need. What is missing is fixed with the [`uses`](scripts-and-templates.md#uses) action.

It takes no arguments: pass `{}`.

The answer has `scripts`, and each script brings `path`, `pointer`, `declared`, `detected` and `missing`.

```sh
reiwand-ctl scene act my-scene import_script '{"template":"empty"}'
reiwand-ctl scene act my-scene script '{"write":{"path":"scripts/empty.js","text":"export function update() { scene.bloom; }\n"}}'
reiwand-ctl scene ask my-scene uses '{}'
```

The last query gives, for that script, `"declared": []`, `"detected": ["bloom"]` and `"missing": ["bloom"]`.

#### `shader_source`

Answers with the descriptor and the WGSL code of a package effect or material, and with what the editor needs to build its form. It is for reading what someone else wrote or what was created with [`shader`](shaders.md#shader).

| Argument | Type | What it means |
|-----------|------|---------------|
| `effect` | text (required) | Id of a package effect. |

The answer has: `effect`, `descriptor_path` and `wgsl_path` (where the files live), `descriptor` (the descriptor's text) and `parsed` (the descriptor already read), `wgsl` (the code), `stage` (`layer`, `material` or `particle`), `kinds` (the setting types that exist), `fields` (the fields that the `shader` action with `field` can change), `templates` (the templates of `new`) and `tools` (per setting, the editing tools it offers). If the effect is not the package's, `null`.

```sh
reiwand-ctl scene act my-scene shader '{"new":{"name":"tone"}}'
reiwand-ctl scene ask my-scene shader_source '{"effect":"tone"}'
```

#### `shader_check`

Checks a package effect the way whoever opens the package checks it, with its saved text or with new text you have not saved yet. It says whether it is valid and, if not, at which line and column the error is.

| Argument | Type | What it means |
|-----------|------|---------------|
| `effect` | text (required) | Id of a package effect. |
| `wgsl` | text | WGSL code to test instead of the saved one. |
| `descriptor` | text | Descriptor, written as JSON text, to test instead of the saved one. |

The answer has: `effect`, `ok` (whether it is valid), `error` (the message, or `null`), `line` and `column` (where, or `null`), `stage`, `units` (the relative cost), `uses` (the parts of the engine it uses) and `handles` (the handles the canvas will offer).

```sh
reiwand-ctl scene act my-scene shader '{"new":{"name":"tone"}}'
reiwand-ctl scene ask my-scene shader_check '{"effect":"tone"}'
reiwand-ctl scene ask my-scene shader_check '{"effect":"tone","wgsl":"fn color_tone(color: vec4<f32>, uv: vec2<f32>) -> vec4<f32> {\n    return colr;\n}\n"}'
```

The last query gives `"ok": false`, `"line": 2` and `"column": 12`, with the notice that `colr` does not exist.

### Package review and signature

#### `review`

Answers with the package review as the editor sees it before publishing: the problems, what the package needs, on which platforms it plays and whether it is signed. It is the editor's problem list, with the same checks as `pack check`, plus the platform notices.

It takes no arguments: pass `{}`.

The answer has `findings` (each with `level`: `error`, `warning` or `note`; `pointer`, `file` and `message`), `needs` and `build_adds` (what the package needs and what the build adds to it), `platforms` (per platform, `refused` with the reason and `without` with what it lacks), `facets` and `signature`.

```sh
reiwand-ctl scene ask my-scene review '{}'
```

#### `signature`

Says whether a package folder or a `.wpak` file is signed, and whether the signature is valid for some keys.

| Argument | Type | What it means |
|---|---|---|
| `keys` | list of texts | Trusted public keys. Without them, the signature is only reported, not verified. |
| `path` | text | Folder or `.wpak` file. Default, the scene's folder. |

The answer has `state`: `unsigned` (no signature), `unchecked` (there is a signature, but no keys were given; it brings `blocks`), `valid` (the signature is valid; it brings `signers`) or `invalid` (it brings `error`).

```sh
reiwand-ctl scene ask my-scene signature '{}'
```

### Properties and presets

#### `tried`

Answers with the user properties form as it looks when trying a preset or some values on top of the factory ones. It is for seeing which fields are shown and with which values without changing the scene. In the app, trying a preset on the preview.

| Argument | Type | What it means |
|---|---|---|
| `preset` | text | Id of a package preset. |
| `values` | object | Values that are tried on top, by property id. |
| `languages` | list of texts | Languages of the labels. |

The answer has `fields` (each field with its `key`, `kind`, `label`, the limits, `value`, `visible`, `enabled`...), `presets` (the presets' names) and `values` (the resulting values, already fitted to each property's limits). Secret fields, shortcuts and buttons are not listed.

```sh
reiwand-ctl scene act my-scene add_property '{"kind":"slider","id":"glow"}'
reiwand-ctl scene ask my-scene tried '{"values":{"glow":40}}'
```

#### `levels`

Says what value a property takes at each quality level and at which ones it has a value of its own. A level's own value goes in the property's `quality` field (`/manifest/properties/ID/quality/q1`) and is written with [`set`](../actions-layers/before-you-start.md#set).

| Argument | Type | What it means |
|---|---|---|
| `property` | text | (required) Id of the property. |

The answer has `property`, `default` and `levels`, a list with a `{quality, value, own}` per level (`own` says whether the level sets the value itself). If the property does not exist, it is `null`.

```sh
reiwand-ctl scene act my-scene add_property '{"kind":"slider","id":"glow"}'
reiwand-ctl scene act my-scene set '{"pointer":"/manifest/properties/glow/quality/q1","value":20}'
reiwand-ctl scene ask my-scene levels '{"property":"glow"}'
```

#### `base`

For a preset package: answers with the properties of the package it applies to and the values the preset gives them. In the app, the preset package's form.

| Argument | Type | What it means |
|---|---|---|
| `file` | text | (required) Folder or `.wpak` file of the base package. |

The answer has `id`, `version`, `fields` (each property with its `key`, `title`, `range`, `default`, the preset's `value`, `set`, `pointer` and `widget`) and `files`. If the base package cannot be read, it is `null`.

```sh
reiwand-ctl scene new origin
reiwand-ctl scene act origin add_property '{"kind":"slider","id":"glow"}'
reiwand-ctl scene new my-scene --from preset
reiwand-ctl scene ask my-scene base '{"file":"origin"}'
```

#### `media`

For an image or video package: answers with its file, its size and, if it is a video, its length. In the app, the media package's page.

It takes no arguments: pass `{}`.

The answer has `kind` (`image` or `video`), `entry` (the file that is expected), `present` (whether it exists), `bytes`, `file`, `size`, `length` and `aspect`. With another package type it is `null`.

```sh
reiwand-ctl scene new my-scene --from image
reiwand-ctl scene ask my-scene media '{}'
```

### Clocks

#### `clock_sample`

Writes a clock pattern with the time of an instant and says when the text changes. It is for testing a pattern of a text layer with a clock.

| Argument | Type | What it means |
|---|---|---|
| `pattern` | text | The pattern, for example `HH:mm`. See `clock_help`. |
| `dates` | yes/no | `true` if the pattern is a date one. Default `false`. |
| `lang` | text | Language of the day and month names. Default `en`. |
| `wall` | number | Instant in seconds since 1970. Default, now. |

The answer has `text` (the pattern already written out) and `next` (the seconds left until it changes).

```sh
reiwand-ctl scene ask my-scene clock_sample '{"pattern":"HH:mm","wall":0}'
```

#### `clock_help`

Lists the pieces that can be used in a clock pattern, with an example of each, and the numbers a clock gives.

| Argument | Type | What it means |
|---|---|---|
| `lang` | text | Language of the examples. Default `en`. |
| `wall` | number | Instant of the examples, in seconds since 1970. Default, now. |

The answer has `tokens` (each piece with `token`, `dates` and `sample`), `numbers` (the values `day`, `hour`, `minute`, `second`, `weekday` and `month` of the instant) and `quote` (the character that quotes literal text).

```sh
reiwand-ctl scene ask my-scene clock_help '{"lang":"en","wall":0}'
```

### Change history

#### `history`

Answers with the steps that `undo` undoes and those that `redo` redoes. It is the history that the app's **Undo** and **Redo** hints show, and the one `scene history` lists.

It takes no arguments: pass `{}`.

The answer has `done` (the steps done, from oldest to newest), `undone` (the undone ones, the one `redo` redoes first), `undo` and `redo` (the next step of each, or `null`) and `dirty` (whether there are unsaved changes). Each step brings `label` (the action's name) and `text` (the name the editing person sees). The history is saved separately when the scene is saved, so another `reiwand-ctl` call sees it.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"solid","id":"background"}'
reiwand-ctl scene ask my-scene history '{}'
```
