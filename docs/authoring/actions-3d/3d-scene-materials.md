# Materials

On this page: 3D scene: materials.

## 3D scene: materials

A model brings its materials (each with a name, which appears in [`model_info`](queries.md#model_info)). These variants change how a layer's materials look without touching the file.

### Variant material: change a material

Changes the material `name` of the layer `layer`. Only what you set is saved. It matches the app's **Materials** panel. With `clear: true` all the changes are erased and the file's material comes back.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text (required in practice) | Id of the model layer. |
| `name` | text (required in practice) | Name of the material as it is in the model file. |
| `shader` | text | `pbr` (physical, the normal one), `unlit` (no light), `fur`, `foliage` (vegetation), `chroma` (chroma key) or the id of a package material shader (see `new_material`). |
| `lit` | yes/no | `false` draws it without light. |
| `double_sided` | yes/no | Draws both faces of each triangle. |
| `cast_shadows` | yes/no | Whether it casts shadows (default yes). |
| `receive_shadows` | yes/no | Whether it receives shadows (default yes). |
| `color` | text | Tint, as `"#rrggbb"` or `"#rrggbbaa"`. `null` removes it. |
| `emissive` | text or list | Own glow, same format as `color`. `null` removes it. |
| `params` | object of numbers | Shader numbers. `pbr`: `metallic`, `roughness`, `occlusion` (0 to 1) and `normal_scale` (0 to 10). `fur`: `layers` (1 to 32), `length` (0 to 100), `density` (0 to 1), `comb_x`, `comb_y`, `comb_z` (−1 to 1). `foliage`: `strength` and `speed` (0 to 10). `chroma`: `threshold` and `softness` (0 to 1). |
| `values` | object | Parameters of a package material shader, checked against its descriptor. Images are given as the id of a scene image or as `white`, `black`, `noise` or `normal_flat`. `null` removes a value. |
| `key` | text or list | Color that the `chroma` material makes transparent. Only valid with `chroma`. |
| `maps` | object | For each slot (`base`, `orm`, `normal`, `occlusion`, `emissive`), the id of a scene image. `null` brings back the file's map. |
| `files` | object | For each slot, an image file from the scene folder that is imported as a new image and chosen in the same step. The `orm`, `normal` and `occlusion` slots are read as data, not as colors. |
| `clear` | yes/no | `true` erases all the material's changes. Default `false`. |

```sh
reiwand-ctl scene act my-scene import '{"file":"figure.glb"}'
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene scene3d '{"model":{"add":{"asset":"figure","id":"cube"}}}'
reiwand-ctl scene act my-scene scene3d '{"material":{"layer":"cube","name":"paint","color":"#ff8040","params":{"metallic":0.8,"roughness":0.3},"double_sided":true}}'
reiwand-ctl scene act my-scene scene3d '{"material":{"layer":"cube","name":"paint","maps":{"base":"picture"}}}'
reiwand-ctl scene act my-scene scene3d '{"material":{"layer":"cube","name":"paint","files":{"emissive":"picture.png"}}}'
reiwand-ctl scene act my-scene scene3d '{"material":{"layer":"cube","name":"paint","maps":{"base":null}}}'
reiwand-ctl scene act my-scene scene3d '{"material":{"layer":"cube","name":"paint","clear":true}}'
```

With fur, foliage or chroma key:

```sh
reiwand-ctl scene act my-scene import '{"file":"model.glb"}'
reiwand-ctl scene act my-scene scene3d '{"model":{"add":{"asset":"model","id":"doll"}}}'
reiwand-ctl scene act my-scene scene3d '{"material":{"layer":"doll","name":"body","shader":"fur","params":{"layers":12,"length":3,"density":0.7}}}'
reiwand-ctl scene act my-scene scene3d '{"material":{"layer":"doll","name":"body","shader":"chroma","key":"#00ff00","params":{"threshold":0.3,"softness":0.1}}}'
```

### Variant new_material: create a new material shader

Writes a material shader into the scene from a template: the files `effects/NAME.json` (the description) and `effects/NAME.wgsl` (the code), and adds it as an effect. Afterwards you use it with `material` by putting its name in `shader`. In the app it is **New material shader**. The template brings three parameters: `tint` (color), `glow` (number from 0 to 4) and `picture` (image).

| Argument | Type | What it means |
|---|---|---|
| `name` | text (required) | Name of the shader: 1 to 32 lowercase letters, digits or `_`, and not already in use. |

```sh
reiwand-ctl scene act my-scene import '{"file":"figure.glb"}'
reiwand-ctl scene act my-scene scene3d '{"model":{"add":{"asset":"figure","id":"cube"}}}'
reiwand-ctl scene act my-scene scene3d '{"new_material":{"name":"glow"}}'
reiwand-ctl scene act my-scene scene3d '{"material":{"layer":"cube","name":"paint","shader":"glow","values":{"tint":"#ffcc00","glow":2,"picture":"noise"}}}'
```

The [`material_check`](queries.md#material_check) query says whether the code compiles.

### Variant paint: paint a material map

Paints a stroke directly on a material map (for example the base color) and saves it as a new image in `maps/`. If the material already has an image chosen in that slot, it paints on top of it; if not, it starts from the file's map and, if there is none either, from a white 512-pixel image (or a flat normal for `normal`). In the app it is **Paint this map…** in a map's selector.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text (required) | Id of the model layer. |
| `name` | text (required) | Name of the material. |
| `slot` | text (required) | `base`, `orm`, `normal`, `occlusion` or `emissive`. |
| `color` | list of 3 numbers (required) | Red, green and blue from 0 to 1. |
| `radius` | number (required) | Brush radius as a fraction of the image width (0.1 is a tenth). |
| `hardness` | number | Hardness of the edge, from 0 (soft) to 1. Default 0.5. |
| `points` | list of pairs (required) | From 1 to 4096 points `[u, v]` from 0 to 1 on the image, with the origin at the top left. |

```sh
reiwand-ctl scene act my-scene import '{"file":"model.glb"}'
reiwand-ctl scene act my-scene scene3d '{"model":{"add":{"asset":"model","id":"doll"}}}'
reiwand-ctl scene act my-scene scene3d '{"paint":{"layer":"doll","name":"body","slot":"base","color":[1,0,0],"radius":0.1,"points":[[0.2,0.2],[0.5,0.5]]}}'
```

### Variant comb: comb fur with the brush

Combs the fur of a `fur` material. The stroke is given in canvas pixels over the model as it appears in the authoring view; each segment between two points paints the direction of the drag onto the surface in a comb map (`comb`) that is saved in `maps/`. The points of a segment must fall on that material and be close to each other (long segments that jump from one part of the texture to another are ignored). If no segment touches the material, it fails. In the app it is **Comb on the model**.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text (required) | Id of the model layer. |
| `name` | text (required) | Name of the material, which must have `shader: fur`. |
| `points` | list of pairs (required) | From 2 to 4096 canvas points in pixels. |
| `radius` | number (required) | Brush radius in pixels. |
| `strength` | number | Strength from 0 to 1. Default 1. |
| `hardness` | number | Hardness of the edge. Default 0.5. |
| `erase` | yes/no | `true` undoes the combing where it passes. Default `false`. |
| `time` | number | Second of the scene at which the model is looked at. Default 0. |

```sh
reiwand-ctl scene act my-scene import '{"file":"model.glb"}'
reiwand-ctl scene act my-scene scene3d '{"model":{"add":{"asset":"model","id":"doll"}}}'
reiwand-ctl scene act my-scene scene3d '{"material":{"layer":"doll","name":"body","shader":"fur"}}'
reiwand-ctl scene act my-scene scene3d '{"comb":{"layer":"doll","name":"body","points":[[955,480],[960,520],[965,560]],"radius":30}}'
reiwand-ctl scene act my-scene scene3d '{"comb":{"layer":"doll","name":"body","points":[[955,480],[960,520],[965,560]],"radius":30,"erase":true}}'
```

### Variant key: pick the model's key color

Sets as the key color of a `chroma` material the color of its surface under a point of the view (the color of the map, the file and the tint, before the color is removed). In the app it is **Pick the key color from the model**.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text (required) | Id of the model layer. |
| `name` | text (required) | Name of the material, which must be `chroma`. |
| `at` | list of 2 numbers (required) | Canvas point in pixels. It must fall on that material. |
| `time` | number | Second of the scene. Default 0. |

```sh
reiwand-ctl scene act my-scene import '{"file":"model.glb"}'
reiwand-ctl scene act my-scene scene3d '{"model":{"add":{"asset":"model","id":"doll"}}}'
reiwand-ctl scene act my-scene scene3d '{"material":{"layer":"doll","name":"body","shader":"chroma"}}'
reiwand-ctl scene act my-scene scene3d '{"key":{"layer":"doll","name":"body","at":[960,540]}}'
```

### Variant debug: see masks and shadows in the view

Changes what the authoring view draws in place of the light: `mask` (the sway mask of the vertex colors), `matte` (the matte, the alpha after the key color is removed) or `shadows` (where the shadows fall). Only the person editing sees it: the finished wallpaper never draws it. Without `show` the normal light comes back. In the app it is **Debug view**.

| Argument | Type | What it means |
|---|---|---|
| `show` | text | `mask`, `matte` or `shadows`. Without it, it is turned off. |
| `layer` | text | With `material`, limits the view to that material of that layer. |
| `material` | text | Name of the material. Given together with `layer` or not given. |

```sh
reiwand-ctl scene act my-scene import '{"file":"model.glb"}'
reiwand-ctl scene act my-scene scene3d '{"model":{"add":{"asset":"model","id":"doll"}}}'
reiwand-ctl scene act my-scene scene3d '{"debug":{"show":"shadows"}}'
reiwand-ctl scene act my-scene scene3d '{"debug":{"show":"matte","layer":"doll","material":"body"}}'
reiwand-ctl scene act my-scene scene3d '{"debug":{}}'
```

### Variant sculpt: sculpt the model

Sculpts the surface with a brush and saves the result as a named shape (a blend shape), in a `.wpsh` file next to the model. The shape is set on the layer with weight 1. Repeating the name keeps sculpting that shape. Only the parts that face the camera are touched. In the app it is the sculpt brush of the model's bone tools, with the modes **Push**, **Pull** and **Smooth**.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text (required) | Id of the model layer. |
| `shape` | text (required) | Name of the shape: 1 to 64 letters, digits, `_` or `-`. It cannot match a shape the file already brings. A model accepts up to 64. |
| `mode` | text (required) | `push` (push outward), `pull` (pull inward) or `smooth`. |
| `points` | list of pairs (required) | From 1 to 256 canvas points in pixels. |
| `radius` | number (required) | Brush radius in pixels, from 1 to 4096. |
| `strength` | number (required) | Strength, greater than 0 and up to 10 (up to 1 with `smooth`). |
| `softness` | number | Softness of the edge from 0 to 1. Default 0.5. |
| `time` | number | Second of the scene at which the model is looked at. Default 0. |

```sh
reiwand-ctl scene act my-scene import '{"file":"figure.glb"}'
reiwand-ctl scene act my-scene scene3d '{"model":{"add":{"asset":"figure","id":"cube"}}}'
reiwand-ctl scene act my-scene scene3d '{"sculpt":{"layer":"cube","shape":"belly","mode":"push","points":[[960,540]],"radius":100,"strength":1}}'
reiwand-ctl scene act my-scene scene3d '{"sculpt":{"layer":"cube","shape":"belly","mode":"smooth","points":[[940,540],[980,540]],"radius":60,"strength":0.5,"softness":0.8}}'
```
