# Masks, painted maps and own transitions

On this page: Masks and painted maps, Own transition.

## Masks and painted maps

These actions paint an image with a brush stroke and put it in an effect parameter. Each stroke saves a new image in the package folder (`masks/`, `maps/`, `ramps/` or `normals/`) and is undone as a single step.

### `paint_mask`

Paints a stroke on the image used by an effect parameter (a mask or a grayscale map). If the parameter has no image yet, it starts a 512 by 512 one with the parameter's fill color (black if it does not declare one). In the app it is the **Paint** button next to the parameter, with **Erase**, **Brush size**, **Hardness** and **Value**.

| Argument | Type | What it means |
|---|---|---|
| `pointer` | text | (required) Path of the parameter, `/scene/layers/N/effects/M/params/NAME`, with N the position of the layer and M that of the effect. |
| `points` | list of lists of numbers | (required) Points of the stroke, `[[x, y], ...]`, as fractions of the layer (from 0 to 1). From 1 to 4096 points. |
| `radius` | number | (required) Brush radius as a fraction of the shorter side, between 0.001 and 0.5. |
| `hardness` | number | Hardness of the edge, from 0 (all soft) to 0.99. Default 0.5. |
| `value` | number | Gray that is painted, from 0 to 1; 0 erases. Default 1. |
| `channel` | integer | Channel that is painted: 0 red, 1 green, 2 blue, 3 alpha. With no value, it paints red, green and blue at once. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png"}'
reiwand-ctl scene act my-scene effects '{"add":{"layer":"image","effect":"displace"}}'
reiwand-ctl scene act my-scene paint_mask '{"pointer":"/scene/layers/0/effects/0/params/map","points":[[0.2,0.3],[0.8,0.6]],"radius":0.1}'
reiwand-ctl scene act my-scene paint_mask '{"pointer":"/scene/layers/0/effects/0/params/map","points":[[0.5,0.5]],"radius":0.2,"hardness":1,"value":0,"channel":0}'
```

### `comb_flow`

Combs a flow map: each point the stroke touches points in the direction it was dragged. It is for effects that move the image according to a direction map, such as `flow` or `shake`. It starts from a still map. In the app it is **Paint the direction**, with **Flatten** to leave it still. The arguments are those of `paint_mask`, but `value` is the strength of the movement (0 leaves it still) and `channel` is not used. It needs at least two distinct points to have a direction.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png"}'
reiwand-ctl scene act my-scene effects '{"add":{"layer":"image","effect":"flow"}}'
reiwand-ctl scene act my-scene comb_flow '{"pointer":"/scene/layers/0/effects/0/params/map","points":[[0.2,0.3],[0.8,0.6]],"radius":0.1,"value":0.8}'
```

### `ramp`

Makes a gradient from color stops and puts it in an effect parameter. With the `png` format it saves a one-row image that is read from left to right (the `gradient` parameters of effects such as `fire` use it). With `cube` it saves a color table that tints each tone according to its brightness (`color_lut` uses it). The stops are saved so they can be edited again. In the app, **Gradient stops**.

| Argument | Type | What it means |
|---|---|---|
| `stops` | list of objects | (required) From 1 to 16 stops `{"at": 0 to 1, "color": "#rrggbb"}`, in any order. |
| `pointer` | text | Path of the parameter that receives the result, `/scene/layers/N/effects/M/params/NAME`. With no value, only the image is created. |
| `asset` | text | Id of a gradient made earlier with this action, to redo it with other stops. Any other id is rejected. |
| `format` | text | `png` (default) or `cube`. |
| `width` | integer | Width in pixels, from 16 to 1024. Default 256. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png"}'
reiwand-ctl scene act my-scene effects '{"add":{"layer":"image","effect":"fire"}}'
reiwand-ctl scene act my-scene ramp '{"stops":[{"at":0,"color":"#000000"},{"at":0.5,"color":"#ff6a00"},{"at":1,"color":"#ffff99"}],"pointer":"/scene/layers/0/effects/0/params/gradient"}'
reiwand-ctl scene act my-scene ramp '{"asset":"gradient","stops":[{"at":0,"color":"#200000"},{"at":1,"color":"#ffffff"}],"width":64}'
reiwand-ctl scene act my-scene effects '{"add":{"layer":"image","effect":"color_lut"}}'
reiwand-ctl scene act my-scene ramp '{"stops":[{"at":0,"color":"#000020"},{"at":1,"color":"#ffe0a0"}],"format":"cube","pointer":"/scene/layers/0/effects/1/params/lut"}'
```

### `depth_map`

Computes a gray depth map of a package image (white is near) with the app's depth model, and puts it in an effect parameter, for example the `depth` of `depth_parallax`. It needs the model and its library installed; if they are missing it is rejected with the command that installs them (the `depth_model` query tells you beforehand). In the app, **Make a depth map**.

| Argument | Type | What it means |
|---|---|---|
| `asset` | text | (required) Id of a package image. |
| `id` | text | Id of the image that is created. Default: the image's id plus `_depth`. |
| `pointer` | text | Path of the parameter that receives the map, `/scene/layers/N/effects/M/params/NAME`. |
| `input` | integer | Input size of the model: a multiple of 14 between 196 and 1036. Default 518. |

Example (not tested here because it requires the model to be installed):

```text
reiwand-ctl scene act my-scene depth_map '{"asset":"picture","pointer":"/scene/layers/0/effects/0/params/depth"}'
```

## Own transition

### `transition`

Creates, changes or removes the background's own transition for when it enters (`"way": "in"`) or when it leaves (`"way": "out"`). It checks the same things as scene loading: what the check rejects is not saved and the scene stays as it was. In the app, **Options › Transitions**. Guide: [transitions](../transitions.md).

| Argument | Type | What it means |
|---|---|---|
| `way` | text | `in` (on entering, default) or `out` (on leaving). |
| `make` | yes/no | `true` creates it if missing: it copies the effect of the other direction or writes the starting one `transitions/opening.json`. |
| `effect` | text | Package descriptor with `"stage": "transition"`; its `.wgsl` goes next to it. It clears the values. |
| `duration_ms` | integer | Duration, from 100 to 10000 ms. |
| `curve` | text or list | Named curve (`out_cubic`...) or Bézier `[x1, y1, x2, y2]`. |
| `params` | object | Effect values that change; `null` removes one. |
| `remove` | yes/no | `true` removes that direction. |

```sh
reiwand-ctl scene new my-scene
reiwand-ctl scene act my-scene transition '{"way":"in","make":true}'
reiwand-ctl scene act my-scene transition '{"way":"in","duration_ms":1800,"curve":[0.2,0.8,0.4,1],"params":{"center":[0.3,0.6]}}'
reiwand-ctl scene act my-scene transition '{"way":"in","remove":true}'
```
