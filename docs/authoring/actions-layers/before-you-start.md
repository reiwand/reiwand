# Before you start

On this page: Before you start, Changing the document at a low level.

## Actions

## Before you start

All the actions in this section are called the same way: `reiwand-ctl scene act FOLDER NAME 'JSON'`. `FOLDER` is the scene folder (in the examples, `my-scene`) and `JSON` holds the arguments. Each action is a single **Undo** step. The files that are named (`picture.png`, `sound.ogg`...) are looked up from the folder you run the command in.

**The document.** A scene stores two parts. `/manifest` is the package's data sheet (title, license, user properties, presets). `/scene` is the scene: `/scene/layers` (layers), `/scene/assets` (assets), `/scene/canvas` (canvas), `/scene/timelines` (timelines), `/scene/effects` (package effects), `/scene/scripts`, `/scene/templates`, `/scene/pools` and `/scene/particles`. To see any part: `reiwand-ctl scene show my-scene PATH`. Paths (JSON Pointer) start with `/` and separate the levels with `/`:

| Path | What it is |
|------|--------|
| `/scene/layers/0` | The first layer in the list, the bottom one (the one that is drawn first). |
| `/scene/layers/0/opacity` | A field of that layer. |
| `/scene/layers/1/transform/position` | The position of the second layer. |
| `/scene/layers/2/layers/0` | The first layer inside layer 2, which is a group. |
| `/scene/assets/picture/path` | The file of the asset `picture`. |
| `/scene/canvas/size` | The canvas size. |
| `/manifest/title` | The package title. |
| `/manifest/properties/glow` | The user property `glow`. |

**Ids.** Each layer, asset, particle system and timeline has an id of 1 to 32 lowercase letters, digits or `_`. Layers, assets and systems share a single set of names: there cannot be a layer and an asset with the same id.

**Coordinates.** Unless stated otherwise, a point `[x, y]` is in canvas pixels, with `[0, 0]` at the top left, `x` to the right and `y` downward. Angles are degrees, clockwise. Time is in seconds.

**Animated values.** Several layer and canvas actions have `at` (a second) and `replace`. If the field already has keys, `at` writes a key at that second. If the field follows another source (a property, the audio...), the action is rejected unless you set `"replace": true`, which replaces it with a fixed value.

**Copying layers.** Copying is not a `scene ask` query on its own. It is done with `reiwand-ctl scene ask FOLDER copy`, which prints the text that `paste` later receives.

## Changing the document at a low level

These four actions edit any part of the document by its path. They do not check ranges or field names: to know whether the result is valid, run `reiwand-ctl pack check my-scene`. The app's windows do the same with their fields.

### `set`

Writes a value at a path. If objects are missing along the way, it creates them. If the path already exists, it replaces the value.

A value of a different kind than the existing one is rejected (text where a number went, for example). A value of the right kind that breaks a rule, such as an opacity of 7 or an asset that does not exist, is accepted, just as in the editor while a change is half done. The action's reply says so: `reiwand-ctl` prints a `problem: PATH: MESSAGE` line (or `warning:`) to the error output for each problem that change adds, the same ones the editor's problem list shows, and with `--json` it returns them in `problems`. With no lines, the change added none.

| Argument | Type | What it means |
|-----------|------|---------------|
| `pointer` | text (required) | Path of the value, for example `/scene/canvas/background`. |
| `value` | any JSON value (required) | The new value: number, text, list or object. |

```sh
reiwand-ctl scene act my-scene set '{"pointer":"/scene/canvas/background","value":"#112233"}'
```

### `add`

Inserts a value into a list or an object. In a list, the last level of the path is the position (0 is the first) or `-` to put it at the end. In an object, it is the name of the new field (if it already existed, it replaces it). It accepts and warns about problems like [`set`](#set).

| Argument | Type | What it means |
|-----------|------|---------------|
| `pointer` | text (required) | Path to insert at, for example `/scene/layers/-`. |
| `value` | any JSON value (required) | What is inserted. |

```sh
reiwand-ctl scene act my-scene add '{"pointer":"/scene/layers/-","value":{ "blend_space": "canvas", "id":"background","type":"solid","color":"#102030"}}'
```

### `remove`

Removes the value at a path. It fails if the path does not exist.

| Argument | Type | What it means |
|-----------|------|---------------|
| `pointer` | text (required) | Path of what is removed. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"solid","id":"background"}'
reiwand-ctl scene act my-scene remove '{"pointer":"/scene/layers/0/color"}'
```

### `move`

Moves a value from one path to another, for example a layer to another position in the list. A value cannot be moved inside itself.

| Argument | Type | What it means |
|-----------|------|---------------|
| `from` | text (required) | Source path. |
| `to` | text (required) | Destination path. In a list, the last level is the position. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"solid","id":"background"}'
reiwand-ctl scene act my-scene add_layer '{"kind":"text","id":"title"}'
reiwand-ctl scene act my-scene move '{"from":"/scene/layers/1","to":"/scene/layers/0"}'
```
