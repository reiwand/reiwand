# How 3D works, models and groups

On this page: 3D scene: how it works, 3D scene: models and groups.

## Actions

## 3D scene: how it works

### `scene3d`

Edits everything to do with 3D models: adding and moving models, materials, camera, shadows, camera paths and handle drags. Each call takes a single variant, which is the top-level key of the JSON (`model`, `material`, `camera`...). The variants are explained by topic in the following sections. The app's editor does the same with the **3D model** panel, the **Materials** panel, the **3D camera** panel, **Camera paths**, **Lights in 3D**, **Shadows** and **Tone map**, and with the handles it draws on the authoring view.

Before you start, four common ideas:

- A model is a `.glb` file that you first import with the `import` action. Its id is the file name without the extension (`figure`, `model`). Each time you put it in the scene a model layer is created, and you choose that layer's id with `id`. If you do not give one, one is made up from the model's name, so the examples always give it.
- The 3D positions `[x, y, z]` are in the model's units: x to the right, y up and z toward the viewer. The camera starts at `[0, 0, 5]` looking at the center `[0, 0, 0]`. Rotations are in degrees and are applied first around x, then y, then z.
- The pixel positions `[x, y]` (strokes, view points, drags) are canvas pixels, with the origin at the top left. With the normal canvas, the center is `[960, 540]`. The [`stage3d`](queries.md#stage3d) query tells you where each handle and each model is, in pixels.
- A model with bones must have at least one animation set (the `clip` action) for its bones to answer the bone queries.

The examples use `figure.glb` (a simple cube with the material `paint`) and `model.glb` (a doll with bones `root` and `tip`, material `body` and animations `bend`, `stroll` and `walk`).

## 3D scene: models and groups

### Variant model: add, change or remove a model

Three forms, each inside `model`.

#### model → add

Puts an already imported model in the scene as a new layer. In the app it is the **3D model from a file** button of the **3D model** panel.

| Argument | Type | What it means |
|---|---|---|
| `asset` | text (required) | Id of the imported model. |
| `id` | text | Id of the new layer. One is made up by default. |
| `into` | text | Id of a group to put the layer in. By default it goes at the root. |
| `at` | list of 3 numbers | Initial position `[x, y, z]` in model units. Default `[0, 0, 0]`. |

```sh
reiwand-ctl scene act my-scene import '{"file":"figure.glb"}'
reiwand-ctl scene act my-scene scene3d '{"model":{"add":{"asset":"figure","id":"cube","at":[1,0,0]}}}'
```

#### model → set

Changes the position, rotation, scale or environment of a model layer. If the position, rotation or scale already has keyframes or is bound to a property, this action refuses: use `keyframe` or change the binding (or drag with [`grab`](3d-scene-camera-paths.md#variant-grab-drag-a-handle)).

| Argument | Type | What it means |
|---|---|---|
| `layer` | text | Id of the model layer. |
| `position` | list of 3 numbers | Position, from −1,000,000 to 1,000,000. |
| `rotation` | list of 3 numbers | Rotation in degrees around x, y and z, from −3600 to 3600. |
| `scale` | list of 3 numbers | Scale in x, y and z. None of them can be 0. |
| `size` | list of 2 numbers | Size in pixels of the box where a standalone model is drawn (width and height, from 1 to 100,000). |
| `env` | text | Id of an image that the materials reflect (a panorama). An empty string `""` removes it. |
| `asset` | text | Id of another imported model, to change the layer's model. |

```sh
reiwand-ctl scene act my-scene import '{"file":"figure.glb"}'
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene scene3d '{"model":{"add":{"asset":"figure","id":"cube"}}}'
reiwand-ctl scene act my-scene scene3d '{"model":{"set":{"layer":"cube","rotation":[0,45,0],"scale":[2,2,2]}}}'
reiwand-ctl scene act my-scene scene3d '{"model":{"set":{"layer":"cube","env":"picture"}}}'
reiwand-ctl scene act my-scene scene3d '{"model":{"set":{"layer":"cube","size":[800,600]}}}'
```

#### model → remove

Removes the model layer. The imported model stays in the scene.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text (required) | Id of the model layer. |

```sh
reiwand-ctl scene act my-scene import '{"file":"figure.glb"}'
reiwand-ctl scene act my-scene scene3d '{"model":{"add":{"asset":"figure","id":"cube"}}}'
reiwand-ctl scene act my-scene scene3d '{"model":{"remove":{"layer":"cube"}}}'
```

### Variant group: create a 3D group

Creates a 3D group. The models, lights and flat layers of a 3D group are drawn together, with a shared depth. Without `layers` the group starts empty. With `layers` it gathers those layers inside the new group.

| Argument | Type | What it means |
|---|---|---|
| `id` | text | Id of the group. Default `world` (with a number if it already exists). |
| `into` | text | Id of a group to put the new group in (only if it starts empty). |
| `layers` | list of texts | Ids of the layers that are gathered into the group. None by default. |

```sh
reiwand-ctl scene act my-scene import '{"file":"figure.glb"}'
reiwand-ctl scene act my-scene scene3d '{"model":{"add":{"asset":"figure","id":"cube"}}}'
reiwand-ctl scene act my-scene scene3d '{"group":{"id":"world","layers":["cube"]}}'
reiwand-ctl scene act my-scene scene3d '{"group":{"id":"other"}}'
```

### Variant space: turn a group into 3D and back

Turns a group into a 3D group (`deep: true`) or back into a flat group (`false`). It only works on groups.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text (required) | Id of the group. |
| `deep` | yes/no (required) | `true` makes it 3D, `false` makes it flat. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"group","id":"box"}'
reiwand-ctl scene act my-scene scene3d '{"space":{"layer":"box","deep":true}}'
reiwand-ctl scene act my-scene scene3d '{"space":{"layer":"box","deep":false}}'
```

### Variant depth_test: hide a flat layer behind models

Makes a flat layer that is inside a 3D group hidden when a nearer model covers it. In the app it is **Hide behind nearer models**. It does not work on models, lights or sounds, and turning it on requires the layer to be inside a 3D group.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text (required) | Id of the flat layer. |
| `on` | yes/no (required) | `true` turns the depth test on, `false` removes it. |

```sh
reiwand-ctl scene act my-scene scene3d '{"group":{"id":"world"}}'
reiwand-ctl scene act my-scene add_layer '{"kind":"solid","id":"background","into":"world"}'
reiwand-ctl scene act my-scene scene3d '{"depth_test":{"layer":"background","on":true}}'
```
