# Blend shapes and expressions

A **blend shape** is a stored deformation of the mesh (a smile, a closed eye). Its weight goes from 0 (no effect) to 1 (the whole deformation). An **expression** is a set of shapes with their weights, stored under a name.

## `shape_sculpt`

Sculpts a mesh's blend shape with a brush. With a new name, the first stroke creates the shape. In the app, **Sculpt a blend shape on the preview**: **Shape** (or **New shape…**), the **Brush** (**Push away**, **Pull in**, **Smooth**, **Erase**), **Strength**, **Brush size** and **Soft edge**. The preview shows the shape at full weight while you sculpt.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text (required) | Mesh layer (a model is not valid). |
| `shape` | text (required) | Name of the shape, 1 to 64 characters without `/` or `#`. A name the mesh does not have creates the shape. Up to 64 per mesh. |
| `points` | list of `[x, y]` (required) | The stroke, from 1 to 1024 points, in fractions of the layer. |
| `mode` | text (required) | `push` moves the vertices under the brush away from the stroke. `pull` brings them closer. `smooth` blends each vertex's shape with its neighbors'. `erase` turns it back into nothing. |
| `by` | number (required) | With `push` and `pull`, pixels from 0 to 1000 at the center of the brush. With `smooth` and `erase`, from 0 to 1. |
| `radius` | number | Brush size as a fraction of the longer side, from 0 (exclusive) to 1. Default 0.1. |
| `falloff` | number | How much of the brush is soft edge, from 0 to 1. Default 0.5. |

It only changes the displacements of that shape.

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene shape_sculpt '{"layer":"mesh","shape":"smile","points":[[0.4,0.5],[0.6,0.5]],"mode":"push","by":20,"radius":0.2}'
```

Smooth and erase:

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene shape_sculpt '{"layer":"mesh","shape":"smile","points":[[0.5,0.5]],"mode":"pull","by":20}'
reiwand-ctl scene act my-scene shape_sculpt '{"layer":"mesh","shape":"smile","points":[[0.5,0.5]],"mode":"smooth","by":0.5}'
reiwand-ctl scene act my-scene shape_sculpt '{"layer":"mesh","shape":"smile","points":[[0.5,0.5]],"mode":"erase","by":1}'
```

## `expression`

Creates, changes, renames or removes an expression: a name that is bound to a distribution of blend-shape weights. In the app, the **Expressions** list: **Keep the sliders in this expression**, **Save the sliders**, **Rename** and **Remove**.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text (required) | Mesh or model layer. |
| `name` | text (required) | Name of the expression, 1 to 64 characters. If it does not exist, it is created. |
| `shapes` | object | Weights of the shapes, such as `{"smile": 1}`: each shape must exist and its weight goes from -1 to 1. It replaces that expression's previous distribution. |
| `to` | text | New name. |
| `remove` | yes/no | `true` removes the expression (and its weight in the layer's `shapes`). |

A layer accepts up to 64 expressions. An expression is then handled like one more shape: you give it a weight with `animate` (`morph`) or with keys.

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene shape_sculpt '{"layer":"mesh","shape":"smile","points":[[0.4,0.5],[0.6,0.5]],"mode":"push","by":20,"radius":0.2}'
reiwand-ctl scene act my-scene expression '{"layer":"mesh","name":"happy","shapes":{"smile":1}}'
```

Rename and remove:

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene shape_sculpt '{"layer":"mesh","shape":"smile","points":[[0.4,0.5],[0.6,0.5]],"mode":"push","by":20,"radius":0.2}'
reiwand-ctl scene act my-scene expression '{"layer":"mesh","name":"happy","shapes":{"smile":1}}'
reiwand-ctl scene act my-scene expression '{"layer":"mesh","name":"happy","to":"cheerful"}'
reiwand-ctl scene act my-scene expression '{"layer":"mesh","name":"cheerful","remove":true}'
```
