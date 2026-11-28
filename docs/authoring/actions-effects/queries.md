# Queries

### A layer's effects

#### `rules`

Says which parameters an effect has and what each one asks for in order to animate (time, the pointer, sound or clicks). It is also useful for finding out the names of an effect's parameters before using them in `effects`. Built-in effects and package effects both answer.

| Argument | Type | What it means |
|---|---|---|
| `effect` | text | (required) Effect name. |

The answer has `effect`, `own` (`true` if it is a package effect, which can be changed with `effect_rule`), `uses` (inputs the whole effect asks for), `settle` (seconds it takes to come to rest, or `null`) and `params`, a list with `id`, `kind` (the parameter's type: `float`, `vec2`, `color`, `bool`, `texture`...), `uses`, `waits_for` and `yields_to`. If the effect does not exist, the answer is `null`.

```sh
reiwand-ctl scene ask my-scene rules '{"effect":"displace"}'
```

#### `effect_cost`

Says how much an effect costs at each quality level (from `q0`, the lowest, to `q3`, the highest). The cost is measured in units of work: a simple effect is worth 1. A level with `null` means the effect is left out at that level. In the app, the effect's **Cost** section.

| Argument | Type | What it means |
|---|---|---|
| `effect` | text | (required) Effect name. |
| `optional` | yes/no | If `true`, it computes the cost as an optional effect, which is left out sooner. Default `false`. |

The answer has `effect`, `units` (the cost at the highest quality) and `levels`, a list of `{quality, units}`. If the effect does not exist, the answer is `null`.

```sh
reiwand-ctl scene ask my-scene effect_cost '{"effect":"blur"}'
reiwand-ctl scene ask my-scene effect_cost '{"effect":"fluid","optional":true}'
```

### Masks and painted maps

#### `ramp`

Returns the color stops of a gradient made with the `ramp` action, so you can edit it again.

The argument is not an object but the gradient's id, written in quotes.

The answer has `asset` and `stops`, the list of `{at, color}`; `stops` is `null` if that id is not a gradient made with `ramp`.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png"}'
reiwand-ctl scene act my-scene effects '{"add":{"layer":"image","effect":"fire"}}'
reiwand-ctl scene act my-scene ramp '{"stops":[{"at":0,"color":"#000000"},{"at":1,"color":"#ffff99"}],"pointer":"/scene/layers/0/effects/0/params/gradient"}'
reiwand-ctl scene ask my-scene ramp '"gradient"'
```

#### `depth_model`

Says whether the depth model and the library that runs it are installed, that is, whether `depth_map` can work. It takes no arguments.

The answer has `installed` (the model), `runtime` (the library), `ready` (both) and `install`, the command that installs the model.

```sh
reiwand-ctl scene ask my-scene depth_model '{}'
```

#### `normals`

Shows what the normal map made by the `normals` action would look like, without saving anything. It takes the same arguments as the action (`asset` or `layer`, and optional `strength`, `blur` and `time`; `id` is ignored). In the app it is the preview in the **Generate normals…** box.

The answer has `from` (the image it comes from), `size`, and three small images embedded as `data:image/png;base64,...` text: `normals` (the map), `relief` (the relief, lit from the top left) and `sphere` (a sample sphere with that light). If the arguments are not valid, the answer is `null`.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png"}'
reiwand-ctl scene ask my-scene normals '{"layer":"image","strength":3,"blur":2}'
```

#### `map`

Shows a layer's roughness map or emissive (own light) map as a heat map.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text | Layer id. It can also be given with `--layer ID`. |
| `map` | text | `emissive` (own light) or `roughness` (default). |

The answer has `map` (the id of the map's image), `size` and `heat`, an image embedded as `data:image/png;base64,...` text. If the layer does not have that map, the answer is `{"map": null}`.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png"}'
reiwand-ctl scene act my-scene paint_map '{"layer":"image","map":"emissive","points":[[0.5,0.5]],"radius":0.2,"hardness":1}'
reiwand-ctl scene ask my-scene map '{"layer":"image","map":"emissive"}'
```

### Quality and effect handles

#### `level`

Says what a quality level does to the scene: the draw scale, the density, the particles, which effects are left out and how much what remains costs. It is for seeing how the package would look on a more modest machine. In the app, the per-quality-level preview.

| Argument | Type | What it means |
|---|---|---|
| `quality` | text | Level: `q0`, `q1`, `q2` or `q3`. Default `q3`, the highest. |
| `battery` | yes/no | `true` applies the cut that is made on battery. Default `false`. |
| `calm` | yes/no | Returned as is in the answer, for the calm preview. |

The answer has `asked` and `quality` (the level that was asked for and the one that is really used), `render_scale`, `density`, `particles`, `bloom` (`on`, `hdr` and `glare`), `pointer` (with the emitters that follow the pointer), `effects`, `left_out` (how many effects are left out) and `units` (the cost of those that remain). Each effect brings `layer`, `effect`, `optional`, `kept`, `scale`, `units`, `full_units` and `levels`, the cost at each level.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png"}'
reiwand-ctl scene act my-scene effects '{"add":{"layer":"image","effect":"blur","optional":true}}'
reiwand-ctl scene ask my-scene level '{"quality":"q1"}'
```

#### `effect_handles`

Answers with the handles that a layer's effect shows on the canvas: the points, lines and quadrilaterals that are dragged with the `grip` variant of [`effects`](a-layer-s-effects.md#effects).

| Argument | Type | What it means |
|---|---|---|
| `layer` | text | (required) Layer id. |
| `index` | integer | Position of the effect on the layer. Default 0. |
| `time` | number | Second at which the layer's position is measured. Default 0. |

The answer has `effect` (the effect's id), `quad` (whether the effect is dragged by four corners) and `handles`, each with `param`, `kind`, `at` (its place on the canvas), `value`, `range`, `anchor` and `partner`. If the effect has no handles, `handles` comes back empty.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png"}'
reiwand-ctl scene act my-scene effects '{"add":{"layer":"image","effect":"blur_radial"}}'
reiwand-ctl scene ask my-scene effect_handles '{"layer":"image","index":0,"time":0}'
```

#### `mask_view`

Answers with the mask of a layer's effect as it is drawn on the canvas: where it comes from, the area, the soft edge and the handles. It is written with the `mask` and `mask_grip` variants of [`effects`](a-layer-s-effects.md#effects).

| Argument | Type | What it means |
|---|---|---|
| `layer` | text | (required) Layer id. |
| `index` | integer | Position of the effect on the layer. |
| `time` | number | Second at which it is measured. Default 0. |

The answer has `source` (`shape`, `texture` or `layer`), `kind`, `invert`, `channel`, `strength`, `feather`, `box`, `size`, `area` and `area_u` (the outline on the canvas and inside the layer), `outer` and `inner` (the edges of the soft edge) and `holds`, the handles with their `grip` and their place `at`. If the effect has no mask, it is `null`.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png"}'
reiwand-ctl scene act my-scene effects '{"add":{"layer":"image","effect":"blur"}}'
reiwand-ctl scene act my-scene effects '{"mask":{"layer":"image","index":0,"shape":{"ellipse":{"size":[600,400]}},"feather":40}}'
reiwand-ctl scene ask my-scene mask_view '{"layer":"image","index":0,"time":0}'
```

#### `mask_picture`

Prepares an image as a one-channel mask and answers with the file that results. The app uses it to show the texture mask over the layer.

| Argument | Type | What it means |
|---|---|---|
| `asset` | text | (required) Image id. |
| `channel` | text | Channel: `r`, `g`, `b`, `a` or `luminance`. Default `luminance`. |
| `invert` | yes/no | `true` inverts the mask. Default `false`. |
| `strength` | number | Strength, from 0 to 1. Default 1. |
| `tint` | text | Mask color, `#rrggbb`. Default white. |

The answer is `{"path": ...}` with the prepared image, which is saved in the cache. If the asset is not a package image, it is `null`.

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene ask my-scene mask_picture '{"asset":"picture","channel":"luminance"}'
```

### Own transition

#### `transition`

The two directions (`in`, `out`, or `null` if missing) with their sampled curve (`shape`), the name and fields of the effect, whether it plays at each quality level (`plays`) and the check's problem, worded for people (`problem`); also the package's transition effects (`effects`), the named curves (`curves`) and the limits (`limits`).

```sh
reiwand-ctl scene new my-scene
reiwand-ctl scene act my-scene transition '{"way":"out","make":true}'
reiwand-ctl scene ask my-scene transition
```
