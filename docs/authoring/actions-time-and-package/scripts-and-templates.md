# Scripts and templates

A script is a JavaScript module of the package (`scripts/NAME.js`) that the scene lists in `/scene/scripts`. Each script declares which events it responds to (`events`) and which parts of the engine it uses (`uses`), so the package asks for just the right permissions. The app's **Assets** panel shows them with **New script…**, their code and **What the scripts declare**.

## `import_script`

Copies a `.js` module to `scripts/` and declares it, or creates a new one from a starter template. It detects by itself the events it handles and the parts of the engine it uses, and prints one line for each change; it never proposes the `params` (the script's settings), which are written by hand. It is rejected if the scene would go over 1 MiB of scripts or 64 modules.

| Argument | Type | What it means |
|-----------|------|---------------|
| `file` | text | Without a template (required): the `.js` file that is copied (path from where you run the command). With a template: the name of the new module, which must end in `.js`; if missing, it is named like the template (`empty.js`, `timer.js`...). If the file already exists, the action fails. |
| `template` | text | Starter template: `empty` (empty, with `init` and `update`), `media` (handles the media player's state), `touch` (responds when the pointer is over the output) or `timer` (counts seconds). |

```sh
reiwand-ctl scene act my-scene import_script '{"template":"empty"}'
reiwand-ctl scene act my-scene import_script '{"file":"clock.js","template":"timer"}'
reiwand-ctl scene act my-scene import_script '{"file":"music.js","template":"media"}'
reiwand-ctl scene act my-scene import_script '{"file":"touch.js","template":"touch"}'
```

With a module you wrote yourself:

```sh
printf 'on("update", () => {});\nconst c = scene.camera;\n' > mine.js
reiwand-ctl scene act my-scene import_script '{"file":"mine.js"}'
```

## `script`

Manages the scene's scripts. It is a single action with several variants. All of them ask for `path`, the module's path inside the package, such as `scripts/mine.js`.

### variant `add`

Declares a `.js` module that is already in the package folder and that the scene does not list yet. It declares it like `import_script`.

| Argument | Type | What it means |
|-----------|------|---------------|
| `path` | text (required) | Path of the module. |

```sh
reiwand-ctl scene act my-scene import_script '{"template":"empty"}'
reiwand-ctl scene act my-scene script '{"remove":{"path":"scripts/empty.js"}}'
reiwand-ctl scene act my-scene script '{"add":{"path":"scripts/empty.js"}}'
```

### variant `remove`

Removes the module from the list (the file stays in the folder). It is rejected, saying where, while a value follows the module.

| Argument | Type | What it means |
|-----------|------|---------------|
| `path` | text (required) | Path of the module. |

```sh
reiwand-ctl scene act my-scene import_script '{"template":"empty"}'
reiwand-ctl scene act my-scene script '{"remove":{"path":"scripts/empty.js"}}'
```

### variant `move`

Changes the module's place in the list, which is the order they run in.

| Argument | Type | What it means |
|-----------|------|---------------|
| `path` | text (required) | Path of the module. |
| `to` | integer (required) | New position (0 is the first). |

```sh
reiwand-ctl scene act my-scene import_script '{"template":"empty"}'
reiwand-ctl scene act my-scene import_script '{"file":"clock.js","template":"timer"}'
reiwand-ctl scene act my-scene script '{"move":{"path":"scripts/clock.js","to":0}}'
```

### variant `declare`

Puts in `events` and `uses` what the module's code really uses. It keeps the `uses` that the code cannot show (`clicks`, `geometry`, `animations`, `shortcuts`, `hierarchy`) and the `params`.

| Argument | Type | What it means |
|-----------|------|---------------|
| `path` | text (required) | Path of the module. |
| `params` | yes/no | `true` also adds, as numbers, the settings the code reads as `params.name` and the module does not declare. Default `false`. |

```sh
reiwand-ctl scene act my-scene import_script '{"template":"empty"}'
reiwand-ctl scene act my-scene script '{"write":{"path":"scripts/empty.js","text":"export function update() { const x = params.strength; }\n"}}'
reiwand-ctl scene act my-scene script '{"declare":{"path":"scripts/empty.js","params":true}}'
```

### variant `write`

Saves the code of a module that the scene already lists. It does not change its declaration: the scene check says what changed. It is an **Undo** step: undoing and redoing write the version of that step again.

| Argument | Type | What it means |
|-----------|------|---------------|
| `path` | text (required) | Path of the module. |
| `text` | text (required) | The complete code. |

```sh
reiwand-ctl scene act my-scene import_script '{"template":"empty"}'
reiwand-ctl scene act my-scene script '{"write":{"path":"scripts/empty.js","text":"export function update() {}\n"}}'
```

### variant `param`

Creates, changes, renames or removes a setting (`params`) of the module, which the user or the scene can give to the script. Up to 32 per module. A new setting starts with the default value of its type; if you change the type, `min` and `max` are lost; the default value must fit in the range.

| Argument | Type | What it means |
|-----------|------|---------------|
| `path` | text (required) | Path of the module. |
| `id` | text (required) | Name of the setting. |
| `type` | text | `number`, `bool`, `text`, `color` or `vec2` (two numbers). |
| `default` | any JSON value | Default value. |
| `min`, `max` | number | Range, only for numbers. |
| `to` | text | New name; the values that followed the setting follow it. |
| `remove` | yes/no | `true` removes the setting. Default `false`. |

```sh
reiwand-ctl scene act my-scene import_script '{"template":"empty"}'
reiwand-ctl scene act my-scene script '{"param":{"path":"scripts/empty.js","id":"speed","type":"number","default":2,"min":0,"max":10}}'
reiwand-ctl scene act my-scene script '{"param":{"path":"scripts/empty.js","id":"speed","to":"rhythm"}}'
reiwand-ctl scene act my-scene script '{"param":{"path":"scripts/empty.js","id":"rhythm","remove":true}}'
```

## `uses`

Declares which parts of the engine a script uses, or takes from its code the ones that are missing. The parts that can be declared are: `audio`, `pointer`, `clicks`, `media`, `timers`, `storage`, `layers`, `geometry`, `effects`, `particles`, `sounds`, `animations`, `camera`, `bloom`, `math`, `shortcuts`, `console`, `shared`, `hierarchy` and `folders`. Some ask for a package input (`audio`, `pointer`...): if it is missing, the action says so and you have to declare it first with the `inputs` variant of [`manifest`](package.md#manifest). A script that uses `geometry` in a scene without shape pools also creates a `shapes` pool. To see what each script declares and what it lacks, see the `uses` query (under Queries). In the app it is **Declare what it uses**.

| Argument | Type | What it means |
|-----------|------|---------------|
| `path` | text | Path of a scene script. Without it, all of them. |
| `add` | list of texts | Parts that are added to what is declared. |
| `remove` | list of texts | Parts that are removed. |

Without `add` or `remove`, it declares the parts the code uses and the declaration lacks.

```sh
reiwand-ctl scene act my-scene import_script '{"template":"empty"}'
reiwand-ctl scene act my-scene uses '{"path":"scripts/empty.js","add":["timers","math"]}'
reiwand-ctl scene act my-scene uses '{"path":"scripts/empty.js","remove":["math"]}'
reiwand-ctl scene act my-scene script '{"write":{"path":"scripts/empty.js","text":"export function update() { scene.bloom; }\n"}}'
reiwand-ctl scene act my-scene uses '{}'
```

## `template`

Manages the layer templates that scripts repeat (for example, to fire many copies of a layer). A template is a layer saved in `/scene/templates`. Three variants. From the app: **Make a template** on a layer; there is also [`duplicate_layer`](../actions-layers/layers.md#duplicate_layer) with `template`.

### variant `make`

Saves one or more layers as a template. The layers stay in the scene. Image, text, shape, solid, spectrum, mesh and group are valid.

| Argument | Type | What it means |
|-----------|------|---------------|
| `layers` | list of texts (required) | Ids of the layers. With several, the template is a group with copies of all of them. |
| `name` | text | Name of the template. Default `ID_template`. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"text","id":"a"}'
reiwand-ctl scene act my-scene add_layer '{"kind":"shape","id":"b"}'
reiwand-ctl scene act my-scene template '{"make":{"layers":["a","b"],"name":"pair"}}'
```

### variant `rename`

Changes a template's name.

| Argument | Type | What it means |
|-----------|------|---------------|
| `name` | text (required) | Current name. |
| `to` | text (required) | New name. |
| `rewrite` | yes/no | `true` also changes, in the scene's scripts, the exact texts `"name"`, `'name'` and `` `name` ``, in the same step. The other lines that name it are not touched. Default `false`. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"text","id":"a"}'
reiwand-ctl scene act my-scene template '{"make":{"layers":["a"],"name":"pair"}}'
reiwand-ctl scene act my-scene template '{"rename":{"name":"pair","to":"couple","rewrite":true}}'
```

### variant `remove`

Removes a template.

| Argument | Type | What it means |
|-----------|------|---------------|
| `name` | text (required) | Name of the template. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"text","id":"a"}'
reiwand-ctl scene act my-scene template '{"make":{"layers":["a"],"name":"pair"}}'
reiwand-ctl scene act my-scene template '{"remove":{"name":"pair"}}'
```

## `pool`

Adds, changes or removes a shape pool: the set of shapes that a script builds on each frame and that `shapes` layers draw. In the app, **Shape pools** in the **Assets** panel. Up to 16 per scene.

| Argument | Type | What it means |
|-----------|------|---------------|
| `id` | text (required) | Id of the pool: 1 to 32 lowercase letters, digits or `_`. |
| `add` | yes/no | `true` creates a new one, with 16 shapes of 1024 vertices if you give no other figures. Default `false`. |
| `remove` | yes/no | `true` removes it. It is rejected while a layer draws it (and it says which). Default `false`. |
| `max` | integer | How many shapes fit at once, from 1 to 256. |
| `max_vertices` | integer | Vertices per shape, from 3 to 65536. |
| `texture` | text | Id of a scene image to paint the shapes with. |
| `size` | list of 2 integers | The pool's own size in pixels, from 1 to 4096 per side. |
| `clear` | list of texts | What is removed: `texture`, `size`. |

```sh
reiwand-ctl scene act my-scene pool '{"id":"shapes","add":true}'
reiwand-ctl scene act my-scene pool '{"id":"shapes","max":32,"max_vertices":2048}'
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene pool '{"id":"shapes","texture":"picture","size":[512,512]}'
reiwand-ctl scene act my-scene pool '{"id":"shapes","clear":["texture","size"]}'
reiwand-ctl scene act my-scene add_layer '{"kind":"shapes","source":"shapes","id":"drawing"}'
reiwand-ctl scene act my-scene remove_layer '{"id":"drawing"}'
reiwand-ctl scene act my-scene pool '{"id":"shapes","remove":true}'
```
