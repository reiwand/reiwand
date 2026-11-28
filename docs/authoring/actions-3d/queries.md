# Queries

### 3D scene: models and materials

These queries read the scene and do not change it. The examples use `figure.glb` and `model.glb`, already imported. Pixel positions are canvas ones (with the origin at the top left and the center at `[960, 540]` on the normal canvas). The actions that go with them are in [`scene3d`](3d-scene-how-it-works.md#scene3d).

#### `model_info`

Says what a model file brings: triangles, materials, images, bones, animations, size and whether it fits at each machine tier. It is what the **3D model** panel shows when you choose a model.

| Argument | Type | What it means |
|---|---|---|
| `asset` | text | Id of a model already imported into the scene. |
| `file` | text | Instead, path of a `.glb` in the scene folder (it does not need to be imported). |

The answer has `triangles`, `vertices`, `size` (width, height and depth in units), `middle` (the center), `materials` (a list with `name`, `color`, `metallic`, `roughness`, `unlit`, `double_sided` and `maps`, the image number of each slot or `null`), `textures` (`size` and `bytes`), `bones`, `clips` (the file's animations with `name` and `length` in seconds), `parts`, `shapes` (blend shapes), `tiers` (for each level `t0` to `t3`: `fits`, `reasons` if it does not fit, the maximum `triangles` and `texture`, and the `memory` it would use) and `notes`. With both arguments absent it answers `null`; if the file cannot be read, `{ "error": ... }`.

```sh
reiwand-ctl scene act my-scene import '{"file":"figure.glb"}'
reiwand-ctl scene act my-scene import '{"file":"model.glb"}'
reiwand-ctl scene ask my-scene model_info '{"asset":"figure"}'
reiwand-ctl scene ask my-scene model_info '{"file":"model.glb"}'
```

#### `materials3d`

Lists the materials of a model layer with everything needed to edit them: what the file says, what you have changed and a thumbnail. It is the **Materials** panel.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text | Id of the model layer. |

The answer is `null` if the layer is not a model. Otherwise it has `layer`, `radius` (the model's radius), `env` (the environment image), `shaders` (all the shaders that can be chosen), `packages` (the scene's material shaders), `params` (the limits of the fur, foliage and chroma numbers) and `materials`. Each material has `name`, `file` (color, metal, roughness, own glow, alpha and faces as they come in the file), `set` (what you have changed, or `null`), `maps` (for each slot `base`, `orm`, `normal`, `occlusion` and `emissive`: the file's `texture` and a `thumb` thumbnail), `chosen` (the scene images chosen for each slot), `sphere` (a sample sphere, PNG image) and `pointer` (where it lives in the scene).

```sh
reiwand-ctl scene act my-scene import '{"file":"model.glb"}'
reiwand-ctl scene act my-scene scene3d '{"model":{"add":{"asset":"model","id":"doll"}}}'
reiwand-ctl scene ask my-scene materials3d '{"layer":"doll"}'
```

#### `material_check`

Checks the code of a scene material shader (the one `new_material` creates) and says whether it compiles. It is the "it compiles" line, or the error with line and column, in the **New material shader** panel.

| Argument | Type | What it means |
|---|---|---|
| `name` | text | Name of the shader. |

The answer has `name`, `ok` (yes/no), `material` (whether the descriptor is a material shader's), `error` (the message, or `null`), `line` and `column` of the failure (or `null`) and `source` (the full code). If the files are missing, `ok` is false and `error` says so.

```sh
reiwand-ctl scene act my-scene scene3d '{"new_material":{"name":"glow"}}'
reiwand-ctl scene ask my-scene material_check '{"name":"glow"}'
```

```text
{"name":"glow","ok":true,"material":true,"error":null,"line":null,"column":null,"source":"..."}
```

#### `depth3d`

Says how a layer in a 3D group hides behind nearer models. It is the note under **Hide behind nearer models**.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text (required) | Id of the layer. |

The answer is `null` if there is no such layer. If there is: `layer`, `inside` (whether it is inside a 3D group), `depth_test` (whether the depth test is on), `particles` (whether it is a particle layer), `way` and `reason`. `way` is `one_by_one` when each particle is tested on its own against the models, and `plane` when the whole layer is tested as a plane. `reason` says why some particles go in their plane: `perspective` (the system has no perspective); it is `null` on other layers. `caveat` is `refract_edges` on particles that refract: their distortion hides behind nearer models, but next to the edge of one it can drag some of its pixels along, because the image it distorts does not keep depth; otherwise `null`. `lowest` is a reminder that at the lowest quality every layer goes in its plane.

```sh
reiwand-ctl scene act my-scene scene3d '{"group":{"id":"world"}}'
reiwand-ctl scene act my-scene add_layer '{"kind":"solid","id":"background","into":"world"}'
reiwand-ctl scene act my-scene scene3d '{"depth_test":{"layer":"background","on":true}}'
reiwand-ctl scene ask my-scene depth3d '{"layer":"background"}'
```

#### `surface3d`

Says which part of the model is under a point of the authoring view. It is for knowing which material you are touching before painting it, combing it or taking its color.

| Argument | Type | What it means |
|---|---|---|
| `at` | list of 2 numbers | Canvas point in pixels. |
| `time` | number | Second of the scene. Default 0. |

The answer is `null` if there is no model there. If there is: `layer`, `material`, `uv` (the place on the material's image, from 0 to 1), `point` (the 3D point), `normal` (where the surface faces), `unit` (how large one canvas pixel is at that point, in model units) and `color` (the surface color as `#rrggbb`). It follows the pose of the animations at that second, but not the blend shapes.

```sh
reiwand-ctl scene act my-scene import '{"file":"model.glb"}'
reiwand-ctl scene act my-scene scene3d '{"model":{"add":{"asset":"model","id":"doll"}}}'
reiwand-ctl scene ask my-scene surface3d '{"at":[960,540]}'
```

```text
{"layer":"doll","material":"body","uv":[0.125,1.0],"point":[0.0,0.0,0.25],"normal":[0.0,0.0,1.0],"unit":0.0051,"color":"#ffffff"}
```

### 3D scene: stage, cost and shadows

#### `stage3d`

Describes the 3D stage as the authoring view sees it at a given second: where each model is, its handles, the camera and the lights, all in canvas pixels. It is the query the app uses to draw the handles, and it is useful for knowing which points to give to [`grab`](3d-scene-camera-paths.md#variant-grab-drag-a-handle).

| Argument | Type | What it means |
|---|---|---|
| `time` | number | Second of the scene. Default 0. |

It is `null` if the scene cannot be read. The answer has these parts:

- `size`: the canvas size; `stages`: how many 3D stages there are (a standalone model or each 3D group is one).
- `view`: the authoring view (`yaw`, `pitch`, `distance`, `target`, `through`) and `through`, whether you are now looking through the camera.
- `camera`: the scene's camera (`eye`, `center`, `up`, `fov`, `near`, `far`).
- `models`: for each model, `id`, `pointer` (where it lives in the scene), `stage`, `at` (its center in pixels), `arms` (the ends of the x, y and z arrows), `box` (its on-screen box `[x1, y1, x2, y2]`), `depth` and `length`, `position`, `rotation` and `scale` (as written, or `null`), `now` (the current values), `held` (for each one, `fixed` if it is a value, `keyed` if it has keyframes and `bound` if it is bound to a property) and `turns` (the rotation rings and `grips`, the points where they are grabbed, plus `deep`, the depth handle). If the model has bones, each model also brings `bones` with the position of each bone, and data about its animations and colliders.
- `frustum` and `handles`: with the free view, the lines of what the camera sees and the points of its handles (`eye`, `center`, `up` and `rig` with `orbit`, `dolly` and `roll`). When looking through the camera they are empty or `null`.
- `flat`: the flat layers of 3D groups, with whether they have `depth_test`.
- `lights`: the lights, with `id`, `light` (type), `space` (whether they are in 3D), `at`, `aim`, `held` and `aim_held`.

```sh
reiwand-ctl scene act my-scene import '{"file":"figure.glb"}'
reiwand-ctl scene act my-scene scene3d '{"model":{"add":{"asset":"figure","id":"cube"}}}'
reiwand-ctl scene act my-scene scene3d '{"view":{"orbit":[30,10]}}'
reiwand-ctl scene ask my-scene stage3d '{"time":0}'
```

#### `cost3d`

Computes how much the 3D stage spends at each quality level and which machines it fits on. It is the cost warning in the **3D model** panel.

| Argument | Type | What it means |
|---|---|---|
| `quality` | text | `q0`, `q1`, `q2` or `q3`. It is repeated in the `quality` field of the answer; the figures for all levels always come. |

It is `null` if the scene cannot be read. The answer has:

- `models`: the visible models, with `id`, `triangles` and `textures`.
- `levels`: one entry per level (`q0` to `q3`) with the model's `triangles`, the `drawn` ones that are drawn, the canvas `pixels` at that quality, `scene` (the total cost), `shadow_texels`, `fur_shells`, `fur_triangles`, `skinned`, `colliders` and `left_out` (notices of what that level leaves out).
- `tiers`: one entry per machine tier (`t0` to `t3`) with `fits` (whether the scene fits), `poster` (whether it would stay as a still image), `busy`, `reasons`, `memory`, the maximums of `triangles` and `texture`, the `quality` it uses and the `shadows` it can draw.
- `phone`: the limits of a phone; `groups`: how many 3D groups there are.

```sh
reiwand-ctl scene act my-scene import '{"file":"model.glb"}'
reiwand-ctl scene act my-scene scene3d '{"model":{"add":{"asset":"model","id":"doll"}}}'
reiwand-ctl scene ask my-scene cost3d '{"quality":"q2"}'
```

#### `shadows3d`

Says which lights cast shadows and at which machine tiers they are drawn. It is the information in the **Shadows** panel.

| Argument | Type | What it means |
|---|---|---|
| `time` | number | Second of the scene. Default 0. |

The answer has `lights`, `caps`, `most` and `debug`. `most` is the maximum number of shadow-casting lights in a scene. `caps` says how many shadow-casting lights each machine tier draws (`t0` to `t3`). `debug` is the chosen debug view, or `null`. Each light in `lights` brings `id`, `light` (type), `resolution` (the chosen size, or `null` if automatic), `bias`, `tiers` (for each machine tier, `side` the side of the map in pixels, and also `phone`, `low` at the lowest quality and `shown`, whether that tier draws it) and `frustum` (the segments, in pixels, with which the authoring view draws the shadow volume).

```sh
reiwand-ctl scene act my-scene import '{"file":"figure.glb"}'
reiwand-ctl scene act my-scene scene3d '{"model":{"add":{"asset":"figure","id":"cube"}}}'
reiwand-ctl scene act my-scene add_layer '{"kind":"light","id":"lamp"}'
reiwand-ctl scene act my-scene scene3d '{"shadows":{"layer":"lamp","on":true,"resolution":1024}}'
reiwand-ctl scene ask my-scene shadows3d '{"time":0}'
```

#### `paths3d`

Describes the camera paths: their keys, the line they draw and what is happening at a given second. It is what the **Camera paths** panel draws.

| Argument | Type | What it means |
|---|---|---|
| `time` | number | Second of the scene at which it is looked at. Default 0. |

The answer has `paths`, `order`, `fade`, `total`, `time` and `playing`. `total` is the duration of all the paths together and `fade` is the fade (`to` and `duration`), or `null`. `playing` is the camera at that second (`eye`, `center`, `fov`, `fade`, `cross`, `moving`, `at`), or `null` if no path is playing. Each path brings `id`, `loop`, `ease`, `period` (its duration), `keys` (for each key: `t`, `eye`, `center`, `fov` and their on-screen points `at` and `aim`), `spline` (the camera's line in pixels) and `aims` (the line of the point it looks at). The on-screen points are `null` if there is no model to look at.

```sh
reiwand-ctl scene act my-scene import '{"file":"figure.glb"}'
reiwand-ctl scene act my-scene scene3d '{"model":{"add":{"asset":"figure","id":"cube"}}}'
reiwand-ctl scene act my-scene scene3d '{"path":{"add":{"id":"flight"}}}'
reiwand-ctl scene ask my-scene paths3d '{"time":1}'
```

### 3D scene: a model's bones

These two queries are the ones the app uses when you drag a bone with the mouse. They need a model with bones and an animation set on its layer (the `clip` action), because without an animation the model has no live skeleton. In `model.glb` the bones are called `root` and `tip`.

#### `bone3d`

Turns a gesture on a bone into the values to save as a keyframe: how many degrees to rotate it (`angles`) or how far to slide it (`shift`). The gestures are given in degrees or in view pixels.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text | Id of the model layer. |
| `bone` | text | Name of the bone. |
| `turn` | number | Rotation in degrees around the line of sight. |
| `tilt` | list of 2 numbers | Tilt `[x, y]` in degrees. |
| `slide` | list of 2 numbers | Slide `[x, y]` in pixels. |
| `time` | number | Second of the scene. Default 0. |
| `clip` | text | Id of the animation whose values are taken as the starting point. |
| `clip_time` | number | Second within that animation. Default `time`. |
| `from` | object | Starting values `{ "angles": [x, y, z], "shift": [x, y, z] }`; it takes precedence over `clip`. |

The answer is `null` if the layer, the bone or the live animation is missing. Otherwise `{ "angles": [x, y, z], "shift": [x, y, z] }`, the values that would be given to the bone for that gesture.

```sh
reiwand-ctl scene act my-scene import '{"file":"model.glb"}'
reiwand-ctl scene act my-scene scene3d '{"model":{"add":{"asset":"model","id":"doll"}}}'
reiwand-ctl scene act my-scene clip '{"layer":"doll","id":"step","animation":"walk","add":true,"play":true}'
reiwand-ctl scene ask my-scene bone3d '{"layer":"doll","bone":"tip","turn":10}'
reiwand-ctl scene ask my-scene bone3d '{"layer":"doll","bone":"tip","slide":[20,0],"clip":"step","clip_time":0.5}'
```

```text
{"angles":[0.0,-0.0,10.0],"shift":[0.0,0.0,0.0]}
```

#### `bound3d`

Says which bone rotation angle is closest to a point of the view. It is for dragging the rotation limits of a body with sway.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text | Id of the model layer. |
| `bone` | text | Name of the bone. |
| `point` | list of 2 numbers | Canvas point in pixels. |
| `time` | number | Second of the scene. Default 0. |

The answer is `null` if the layer, the bone or the live animation is missing. Otherwise `{ "degrees": N }`, with N between −180 and 180.

```sh
reiwand-ctl scene act my-scene import '{"file":"model.glb"}'
reiwand-ctl scene act my-scene scene3d '{"model":{"add":{"asset":"model","id":"doll"}}}'
reiwand-ctl scene act my-scene clip '{"layer":"doll","id":"step","animation":"walk","add":true,"play":true}'
reiwand-ctl scene ask my-scene bound3d '{"layer":"doll","bone":"tip","point":[1000,300]}'
```

```text
{"degrees":37.0}
```
