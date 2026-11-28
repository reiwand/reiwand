# Shaders, undo and redo

On this page: Shaders, Undo and redo.

## Shaders

A package shader is an effect (or a material, or a particle shader) that you write yourself: a JSON descriptor that says what settings it has, and a WGSL file with the code. They live in `effects/ID.json` and `effects/ID.wgsl`, and are declared in `/scene/effects`. A layer uses them like any effect (see [`effects`](../effects.md)). For the code and the shape of the descriptor see [effects](../effects.md).

### `shader`

Writes a package effect or material: create it from a template, write its code or its descriptor, or edit its settings, passes and fields. It is a single action with five variants. To read the code and check it, see the [`shader_source`](queries.md#shader_source) and [`shader_check`](queries.md#shader_check) queries.

#### variant `new`

Creates an effect with its descriptor and its starter WGSL, ready to use.

| Argument | Type | What it means |
|-----------|------|---------------|
| `name` | text (required) | Id of the effect. Unused and not reserved by the engine. |
| `template` | text | `color` (changes the color, the normal choice if you put nothing), `distort` (deforms the image), `passes` (a multi-pass effect, like a blur), `material` (material of a 3D model) or `particle` (for particles). |

```sh
reiwand-ctl scene act my-scene shader '{"new":{"name":"tone"}}'
reiwand-ctl scene act my-scene shader '{"new":{"name":"swirl","template":"distort"}}'
reiwand-ctl scene act my-scene shader '{"new":{"name":"soften","template":"passes"}}'
reiwand-ctl scene act my-scene shader '{"new":{"name":"glass","template":"material"}}'
reiwand-ctl scene act my-scene shader '{"new":{"name":"spark","template":"particle"}}'
```

#### variant `write`

Saves the WGSL code, the descriptor, or both. Each file must be valid: the descriptor is checked before saving (a shader is up to 64 KiB and all of them together up to 256 KiB).

| Argument | Type | What it means |
|-----------|------|---------------|
| `effect` | text (required) | Id of a package effect. |
| `wgsl` | text | The complete code. |
| `descriptor` | object | The complete descriptor; its `id` must be the effect's. |

```sh
reiwand-ctl scene act my-scene shader '{"new":{"name":"tone"}}'
reiwand-ctl scene act my-scene shader '{"write":{"effect":"tone","wgsl":"fn color_tone(color: vec4<f32>, uv: vec2<f32>) -> vec4<f32> {\n    return color;\n}\n"}}'
reiwand-ctl scene act my-scene shader '{"write":{"effect":"tone","descriptor":{"id":"tone","category":"color","fuse":"color","cost":{"taps":1,"passes":1,"scale":1},"params":[{"id":"amount","type":"float","prefill":1,"min":0,"max":1}]}}}'
```

#### variant `param`

Creates, changes, renames, moves or removes a setting of the effect. If it does not exist, it creates it (of type `float` if you say no other). When renaming, the layers that use it are updated.

| Argument | Type | What it means |
|-----------|------|---------------|
| `effect` | text (required) | Id of the effect. |
| `id` | text (required) | Id of the setting. |
| `set` | object | Fields to set: `type` (`float`, `vec2`, `color`, `bool`, `texture`, `lut`, `int`, `enum`, `blend`, `position`, `mask`, `emitters`), `default`, `min`, `max`, `unit`, `label` and `help` (texts per language, such as `{"en": "Strength"}`), `values`, `channels`, `paint`, `uses`, `yields_to`, `waits_for`, `tools`. |
| `clear` | list of texts | Fields that are removed (the same ones, except `type` and `default`). |
| `to` | text | New id of the setting. |
| `at` | integer | Place of the setting in the list (0 is the first). |
| `remove` | yes/no | `true` removes the setting. Default `false`. |

```sh
reiwand-ctl scene act my-scene shader '{"new":{"name":"tone"}}'
reiwand-ctl scene act my-scene shader '{"param":{"effect":"tone","id":"strength","set":{"type":"float","prefill":0.5,"min":0,"max":2,"label":{"en":"Strength"}}}}'
reiwand-ctl scene act my-scene shader '{"param":{"effect":"tone","id":"strength","to":"energy","at":0}}'
reiwand-ctl scene act my-scene shader '{"param":{"effect":"tone","id":"energy","clear":["label"]}}'
reiwand-ctl scene act my-scene shader '{"param":{"effect":"tone","id":"energy","remove":true}}'
```

#### variant `pass`

Edits the passes of a multi-pass effect. Without `index`, it adds a new one at the end.

| Argument | Type | What it means |
|-----------|------|---------------|
| `effect` | text (required) | Id of the effect. |
| `index` | integer | Number of the pass (0 is the first). |
| `set` | object | Fields to set: `entry`, `scale`, `inputs`, `output`, `target`, `copy`, `blend`, `vertex`, `grid`, `swap`. |
| `clear` | list of texts | Fields that are removed. |
| `remove` | yes/no | `true` removes the pass `index`. Default `false`. |

```sh
reiwand-ctl scene act my-scene shader '{"new":{"name":"soften","template":"passes"}}'
reiwand-ctl scene act my-scene shader '{"pass":{"effect":"soften","index":0,"set":{"scale":0.25}}}'
reiwand-ctl scene act my-scene shader '{"pass":{"effect":"soften","set":{"entry":"fs_soften_extra"}}}'
reiwand-ctl scene act my-scene shader '{"pass":{"effect":"soften","index":0,"clear":["scale"]}}'
reiwand-ctl scene act my-scene shader '{"pass":{"effect":"soften","index":2,"remove":true}}'
```

#### variant `field`

Changes a descriptor field that is neither a setting nor a pass.

| Argument | Type | What it means |
|-----------|------|---------------|
| `effect` | text (required) | Id of the effect. |
| `field` | text (required) | One of: `name`, `category`, `description`, `uses`, `mask_mode`, `cost`, `fuse`, `reach`, `bounded`, `min_fps`, `velocity`, `state`, `solve`, `pace`, `quality`, `targets`, `options`, `compose`. |
| `value` | any JSON value | The new value. `name` and `description` are texts per language, such as `{"en": "Tone"}`. |

```sh
reiwand-ctl scene act my-scene shader '{"new":{"name":"tone"}}'
reiwand-ctl scene act my-scene shader '{"field":{"effect":"tone","field":"name","value":{"en":"Tone"}}}'
reiwand-ctl scene act my-scene shader '{"field":{"effect":"tone","field":"cost","value":{"taps":2,"passes":1,"scale":1}}}'
```

### `import_effect`

Brings in an effect written outside the scene: it checks the descriptor and the WGSL the way whoever opens the package will check them, copies them to `effects/ID.json` and `effects/ID.wgsl` and declares it (if it already existed, it replaces it).

| Argument | Type | What it means |
|-----------|------|---------------|
| `descriptor` | text (required) | `.json` file of the descriptor. |
| `wgsl` | text | `.wgsl` file. Without it, the one next to the descriptor with the same name. |

```sh
reiwand-ctl scene new origin
reiwand-ctl scene act origin shader '{"new":{"name":"tone"}}'
reiwand-ctl scene new my-scene
reiwand-ctl scene act my-scene import_effect '{"descriptor":"origin/effects/tone.json"}'
reiwand-ctl scene act my-scene import_effect '{"descriptor":"origin/effects/tone.json","wgsl":"origin/effects/tone.wgsl"}'
```

### `remove_effect`

Removes the declaration of a package effect (its files stay in `effects/`). It is rejected while a layer has it in its effects or a model uses it as a material.

| Argument | Type | What it means |
|-----------|------|---------------|
| `effect` | text (required) | Id of the effect. |

```sh
reiwand-ctl scene act my-scene shader '{"new":{"name":"tone"}}'
reiwand-ctl scene act my-scene remove_effect '{"effect":"tone"}'
```

## Undo and redo

### `undo`

Undoes the last change. Each action is a step. In the app, **Undo**. To see the steps: `reiwand-ctl scene history my-scene`.

It takes no arguments: pass `'{}'`.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"solid","id":"background"}'
reiwand-ctl scene act my-scene undo '{}'
```

### `redo`

Redoes the last undone change. In the app, **Redo**. It takes no arguments.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"solid","id":"background"}'
reiwand-ctl scene act my-scene undo '{}'
reiwand-ctl scene act my-scene redo '{}'
```
