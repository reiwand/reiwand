# Lights and ambient light

## `light`

Adds, changes, mutes or removes the scene's lights. A light is a light-type layer: it draws nothing, but it lights the layers that have lighting (see `lighting`). A scene accepts up to 12 lights; on phones and older machines only the first 8 light. In the app, the **Lights** panel, with the handles on the canvas. Each call uses a single variant.

### variant `add`

Adds a light at the center of the canvas with the values of its type: point (radius 400, height 120), spot (radius 900, height 150, intensity 2, cone from 25 to 45 degrees, tilt 45), tube (length 400, radius 240, height 60) or directional (intensity 0.8, tilt 50).

| Argument | Type | What it means |
|---|---|---|
| `light` | text | `point` (default), `spot`, `tube` or `directional`. |
| `id` | text | Id of the light layer. With no value, it uses the type's name. |
| `into` | text | Id of a group it goes into. |
| `at` | list of numbers | Place `[x, y]` of the light inside its group, in pixels, and it is saved as the `position` of its transform. In the main list it matches the canvas pixels; inside a group that has been moved, the light ends up at the canvas place that results from adding the group's to it. With no value, the center of the canvas. To leave a light at a canvas point, use `at` of the `set` variant. |

```sh
reiwand-ctl scene act my-scene light '{"add":{"light":"spot","id":"spot","at":[960,300]}}'
reiwand-ctl scene act my-scene light '{"add":{}}'
```

### variant `set`

Changes a light. Only what you name changes. `color`, `intensity`, `radius`, `z`, `tilt`, `at` and `aim` are written as a key at second `time` if that value already has keys, and are rejected if the value follows a sound or a property, except with `replace`.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text | (required) Id of the light layer. |
| `light` | text | New type: `point`, `spot`, `tube` or `directional`. It removes what the new type does not use (cone, length, projected image, air). |
| `color` | any JSON value | Color of the light, `"#rrggbb"` or `"#rrggbbaa"`. |
| `intensity` | number | Intensity, from 0 to 100. |
| `radius` | number | Reach radius in pixels, from 1 to 100000. |
| `z` | number | Height above the canvas in pixels, from −100000 to 100000. |
| `cone` | list of numbers | Spots only: `[inner, outer]` in degrees, from 0 to 180; the outer one cannot be smaller than the inner one. |
| `length` | number | Tubes only: length in pixels, from 1 to 100000. |
| `tilt` | number | Tilt toward the canvas in degrees, from −89 to 89. |
| `at` | list of numbers | Canvas point the light is moved to. |
| `aim` | list of numbers | Canvas point it aims at (spots, tubes and directional lights). |
| `time` | number | Second at which the key is written (0 or more). Default 0. |
| `replace` | yes/no | If `true`, replaces the keys with a fixed value. Default `false`. |

```sh
reiwand-ctl scene act my-scene light '{"add":{"light":"spot","id":"spot"}}'
reiwand-ctl scene act my-scene light '{"set":{"layer":"spot","color":"#ffcc88","intensity":3,"radius":700,"z":200,"cone":[20,40],"tilt":30,"at":[500,300],"aim":[960,800]}}'
reiwand-ctl scene act my-scene light '{"set":{"layer":"spot","light":"point"}}'
```

### variant `remove`

Removes a light.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text | (required) Id of the light layer. |

```sh
reiwand-ctl scene act my-scene light '{"add":{"light":"point","id":"point"}}'
reiwand-ctl scene act my-scene light '{"remove":{"layer":"point"}}'
```

### variant `solo`

Makes the preview be lit only by that light. It does not change the scene that is published. Without `layer`, the solo ends.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text | Id of the light layer. With no value, the solo ends. |

```sh
reiwand-ctl scene act my-scene light '{"add":{"light":"point","id":"point"}}'
reiwand-ctl scene act my-scene light '{"solo":{"layer":"point"}}'
reiwand-ctl scene act my-scene light '{"solo":{}}'
```

### variant `mute`

Leaves a light out of the preview, or puts it back. It does not change the scene that is published.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text | (required) Id of the light layer. |
| `muted` | yes/no | `true` mutes it; `false` (default) puts it back. |

```sh
reiwand-ctl scene act my-scene light '{"add":{"light":"point","id":"point"}}'
reiwand-ctl scene act my-scene light '{"mute":{"layer":"point","muted":true}}'
reiwand-ctl scene act my-scene light '{"mute":{"layer":"point"}}'
```

### variant `volumetric`

Makes a point or spot light light the air around it, like fog with beams of light. Only what you name changes; it starts with density 0.2, 16 samples and shafts 0.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text | (required) Id of the light (point or spot). |
| `density` | number | Density, from 0 to 1. |
| `samples` | integer | Samples per pixel, from 4 to 64; more samples, more cost. |
| `shafts` | number | From 0 to 1: how much the dark parts of the layers below slow the glow, so they draw beams. |
| `off` | yes/no | If `true`, removes the air. Default `false`. |

```sh
reiwand-ctl scene act my-scene light '{"add":{"light":"spot","id":"spot"}}'
reiwand-ctl scene act my-scene light '{"volumetric":{"layer":"spot","density":0.3,"samples":24,"shafts":0.5}}'
reiwand-ctl scene act my-scene light '{"volumetric":{"layer":"spot","off":true}}'
```

### variant `projection`

Makes a spot project an image, a video or a layer, like a projector.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text | (required) Id of the spot. |
| `asset` | text | Id of a package image or video. |
| `from` | text | Id of a layer that is projected; it cannot receive light or read a layer that does. |
| `off` | yes/no | If `true`, removes the projection. Default `false`. |

You have to give `asset` or `from`, only one.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png"}'
reiwand-ctl scene act my-scene light '{"add":{"light":"spot","id":"spot"}}'
reiwand-ctl scene act my-scene light '{"projection":{"layer":"spot","asset":"picture"}}'
reiwand-ctl scene act my-scene light '{"projection":{"layer":"spot","off":true}}'
```

## `lighting`

Sets how an image, video, text, mesh, model or particle layer receives light: relief, roughness, reflection and own light. A layer without this does not receive light from the light layers. In the app, the layer's **Lighting** section. Only what you name changes. Particle layers accept only `strength` and `emissive`.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text | Id of the layer. Required in practice: without it, it is rejected. |
| `normal` | text | Id of an image that serves as a normal map (relief). Empty text removes it. |
| `roughness` | number | Roughness, from 0 (smooth, sharp highlight) to 1 (rough, diffuse highlight). |
| `roughness_map` | text | Id of a gray image that multiplies the roughness. Empty text removes it. |
| `reflection` | number | How much it reflects, from 0 to 1. |
| `env` | text | Id of an image that is reflected as if it were the environment. |
| `emissive` | number | Own light, from 0 to 16. It can be seen even in the dark. |
| `emissive_map` | text | Id of a gray image that multiplies the own light. Empty text removes it. |
| `strength` | number | From 0 to 1: how much of the lit color replaces the normal color. |
| `off` | yes/no | If `true`, removes all the layer's lighting. Default `false`. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png"}'
reiwand-ctl scene act my-scene lighting '{"layer":"image","roughness":0.3,"reflection":0.2,"emissive":1,"strength":0.9}'
reiwand-ctl scene act my-scene lighting '{"layer":"image","env":"picture","reflection":0.5}'
reiwand-ctl scene act my-scene lighting '{"layer":"image","off":true}'
```

## `normals`

Makes a normal map (a relief) from the brightness edges of an image and saves it as a package image. With `layer`, it also sets it as that layer's normal map. In the app, **Generate normals…** in the **Lighting** section, with **Strength** and **Blur**. The `normals` query shows the result before saving it.

| Argument | Type | What it means |
|---|---|---|
| `asset` | text | Id of a package image. |
| `layer` | text | Id of an image layer (uses its image), a video layer (uses a frame) or a text layer (uses its letters); it also receives the map. |
| `strength` | number | Strength of the relief, from 0 to 16. Default 2. |
| `blur` | number | Prior blur in pixels, from 0 to 32. Default 1. |
| `id` | text | Id of the image that is created. Only a new id or that of a map made earlier by this action is valid. With no value, it uses the image's plus `_normals`. |
| `time` | number | With a video layer, the second of the frame. Default 0. |

You have to give `asset` or `layer`.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png"}'
reiwand-ctl scene act my-scene normals '{"layer":"image","strength":3,"blur":2,"id":"relief"}'
reiwand-ctl scene act my-scene normals '{"asset":"picture"}'
```

## `paint_map`

Paints with a stroke a layer's roughness map or own-light map. If the layer has no map yet, it starts a new one with the size of its image (up to 512 on the long side), white for roughness and black for own light. In the app, **Paint roughness on the preview** and **Paint glow on the preview**, with **Brush size**, **Hardness** and **Gray**.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text | (required) Id of an image, video, text, mesh or model layer. |
| `map` | text | (required) `roughness` or `emissive` (own light). |
| `points` | list of lists of numbers | (required) Points of the stroke, `[[x, y], ...]`, as fractions of the layer (from 0 to 1). From 1 to 4096 points. |
| `radius` | number | (required) Brush radius as a fraction of the shorter side, between 0.001 and 0.5. |
| `hardness` | number | Hardness of the edge, from 0 to 0.99. Default 0.5. |
| `value` | number | Gray that is painted, from 0 to 1. Default 1. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png"}'
reiwand-ctl scene act my-scene paint_map '{"layer":"image","map":"roughness","points":[[0.2,0.2],[0.8,0.5]],"radius":0.1,"value":0.2}'
reiwand-ctl scene act my-scene paint_map '{"layer":"image","map":"emissive","points":[[0.5,0.5]],"radius":0.2,"hardness":1}'
```

## `ambient`

Sets the scene's surrounding light and the sky light, which are added to the lights on the layers with lighting: ambient light falls on what faces down and sky light on what faces up. With neither, it is black. In the app, **Light around** and **Light from above**. The `hemisphere` query shows the sphere they form.

| Argument | Type | What it means |
|---|---|---|
| `ambient` | any JSON value | Color of the surrounding light, `"#rrggbb"` or `"#rrggbbaa"`. |
| `sky` | any JSON value | Color of the sky light; if missing, it is the same as the ambient light. |
| `clear` | yes/no | If `true`, removes both. Default `false`. |

```sh
reiwand-ctl scene act my-scene ambient '{"ambient":"#202040","sky":"#8090ff"}'
reiwand-ctl scene act my-scene ambient '{"clear":true}'
```
