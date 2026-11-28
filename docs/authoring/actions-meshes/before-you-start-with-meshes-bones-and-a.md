# Before you start and creating a mesh

On this page: Before you start with meshes, bones and animation, Create a mesh.

## Actions

## Before you start with meshes, bones and animation

- A **mesh layer** shows an image on a mesh of triangles that deforms with bones. A **model layer** shows a 3D model from a `.glb` file. Almost all the actions in this section ask for the id of one of the two in `layer`. Those that only work for meshes or only for models say so.
- A mesh is created in two ways: with `mesh_from_picture` from an image, or with `add_layer` of kind `mesh` and a `.glb` file (in the app, **Add a layer**).
- The points of a stroke and the places of bones and points are `[x, y]` in fractions of the layer: `[0, 0]` is the top-left corner and `[1, 1]` the bottom-right one. A brush radius is also a fraction, of the layer's longer side.
- A mesh layer created from an image has the same size as the image. The pixel measures in this section (depth, the position of a key, the target of a chain) are pixels of that layer.
- Angles are in degrees. A positive angle turns the bone clockwise, as you see it on screen.
- A **clip** is a named animation of a mesh or model layer. For it to play, the layer needs an **animation layer** that plays it: `animate` with the `layer` variant, or `clip` with `play`.
- Every change can be undone in one step, brush strokes included. Strokes that change the mesh's shape save a new mesh and leave the previous one for undo.

## Create a mesh

### `mesh_from_picture`

Cuts a scene image into triangles and adds a mesh layer that shows it. It is the first step to give an image bones. In the app it is **Add a layer**, **Mesh from a picture…**, with the options **Follow the outline of the picture** and **Cells along the longer side**.

The image must be imported first (`import`). The layer's id is also the mesh's.

| Argument | Type | What it means |
|---|---|---|
| `image` | text (required) | Id of the scene image that is cut. |
| `outline` | yes/no | `false` (default) covers with a grid the cells that have something opaque. `true` follows the edge of the largest opaque part and puts points inside. |
| `cells` | integer | Cells along the longest side, from 2 to 64. Default 16. A value outside that range is adjusted to the nearest one. |
| `id` | text | Id of the new layer. If missing, it is formed from the image's id and `_mesh`. An id already in use is rejected. |

The image cannot exceed 8192 pixels per side.

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
```

With the outline and more cells:

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh","outline":true,"cells":32}'
```

The freshly cut mesh has a single part called `body` and no bones. To see how the edge came out, use the `outline` query.

### `mesh_depth`

Raises or lowers the depth of a mesh's vertices along a stroke. It is for giving the mesh relief: nearer vertices cover farther ones. In the app, **Paint depth on the preview**, with **Pixels towards the viewer** and **Brush size**.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text (required) | Mesh layer. |
| `points` | list of `[x, y]` (required) | The stroke, from 1 to 1024 points, in fractions of the layer. |
| `radius` | number (required) | Brush size as a fraction of the longer side: greater than 0 and up to 1. |
| `by` | number (required) | Pixels toward the viewer at the center of the brush, from -1000 to 1000. It fades to 0 at the brush's edge. A negative value pushes the vertices away. |

Bones, weights and blend shapes are kept.

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene mesh_depth '{"layer":"mesh","points":[[0.3,0.5],[0.7,0.5]],"radius":0.2,"by":40}'
```

For the depth to show, you have to turn it on in the layer with `parts` and its `depth` option.
