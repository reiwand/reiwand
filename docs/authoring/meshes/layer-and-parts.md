# The mesh layer and its parts

On this page: the mesh layer and its parts.

## The mesh layer

A mesh is a binary glTF file (`.glb`) of flat parts: positions (with an optional depth), texture
coordinates and, to bend, bone weights with a skeleton. A scene can have up to 16 `mesh` and
`model` layers in total.

You do not need to write the `.glb` by hand. The editor cuts a mesh from an image of the scene, by
a grid or by its outline, and places the bones (see [editor.md](../editor.md)). From the terminal:

```
reiwand-ctl scene act MY-FOLDER mesh_from_picture '{"image":"sky","outline":true}'
reiwand-ctl scene ask MY-FOLDER rig '{}' --layer figure
```

`mesh-from-picture` cuts a mesh layer from an image asset (by its outline with `--outline`, or by
a grid of `--cells` cells), ready to receive bones; `scene act FOLDER bone` adds, moves, hangs and
renames bones, and `scene act FOLDER weights` and `scene act FOLDER auto_weights` give and spread
their weights on the vertices (at most 4 bones per vertex).

`bones` prints the bones, parts, blend shapes, expressions, animations and clips of a mesh layer,
and is the way to see which names you have to work with.

```json
    {
      "blend_space": "canvas", "id": "figure", "type": "mesh", "mesh": "doll", "texture": "sky",
      "size": [100, 100],
      "transform": { "scale": [8, 8], "origin": "anchor", "rotation": 0.0, "skew": [0.0, 0.0] },
      "clips": [
        {
          "id": "nod", "loop": "pingpong",
          "bones": {
            "neck": {
              "rotation": [[0, -12], [0.8, 12, "in_out_sine"]], "angles": [], "position": [],
              "scale": [], "shift": []
            }
          },
          "shapes": { "smile": [[0, 0], [0.8, 1]] }, "events": [{ "t": 0.4, "name": "half" }],
          "from": 0.0, "order": {}, "to": null
        }
      ],
      "animations": [
        {
          "clip": "nod", "weight": 1, "rate": 1, "blend_in": 0.3, "additive": false,
          "blend_out": 0.0, "bones": [], "enabled": true
        }
      ],
      "physics": [
        {
          "bone": "neck", "mode": "spring", "stiffness": 0.6, "damping": 0.4, "inertia": 0.5,
          "gravity": 0.0, "wind": 0.0
        }
      ],
      "ik": [
        {
          "chain": ["hip", "neck"], "target": [60, 20], "limits": { "neck": [-30, 30] },
          "weight": 0.3, "bend": "clockwise"
        }
      ],
      "rules": [{ "bone": "neck", "parent": "hip", "weight": 1 }],
      "parts": { "chest": { "order": 1 }, "legs": { "order": 0 } }, "shapes": { "smile": 0.2 },
      "blend": "normal", "cache": false, "colliders": [], "color": "#ffffff", "depth_test": false,
      "effects": [], "expressions": {}, "hit": [], "opacity": 1.0, "parallax": [0, 0],
      "solid": false, "visible": true
    }
```

> In these tables, *Prefill* is the value the editor and the templates write for you. The file still has to contain every field: the loader refuses a missing one and never fills it in (see `../specs/explicit-values.md`).

| Field | Prefill | What it does |
|-------|---------|--------------|
| `mesh` | required | The `.glb` asset. |
| `texture` | those of the file | An image or video asset (or a value) that replaces the textures of the file. Without it, each part shows the image of its material (up to 64 per file; an image that repeats over its part, with coordinates outside 0 to 1, repeats up to 4 times per side, according to the wrap mode of the sampler of its texture in the file: repeat, mirrored repeat or clamp to edge; beyond 4 times the edge shows), or white. |
| `size` | the canvas | The box in pixels that the mesh fills. |
| `color` | white | Color that multiplies the mesh. |
| `parts` | | Per part (the name of a mesh of the file): `order`, `clip_to` and `visible`. |
| `depth` | | `{ "scale", "cull", "sort" }`: depth of the vertices. |
| `clips` | | Up to 64 animation stretches. |
| `animations` | | Up to 16 animation layers. |
| `ik` | | Up to 16 chains that reach a target. |
| `physics` | | Up to 64 bodies that sway. |
| `rules` | | Up to 64 rules that change the parent of a bone. |
| `shapes`, `expressions` | | Blend shapes and expressions. |
| `hit` | | Bones that receive the pointer. |

The positions in the file are in pixels, from the middle of the box that the layer fills, with `x`
to the right and `y` upward, as glTF writes them; texture coordinates start at the top left. The
size of the mesh is that of its coordinates: the figure of the example measures 20 by 40 pixels
and that is why the layer carries `"transform": { "scale": [8, 8] }`. Parts that leave the box at
rest have a margin around it, up to half the box per side, and are not cut; those that a pose takes
farther than that are cut at the edge of the margin, so `size` leaves room for them.

Each node of the file is a joint: the bones of the skeletons, the nodes that carry a mesh without a
skeleton (which move in one piece) and the nodes without a mesh, such as attachment points, which
follow their parent. The rest pose is the transform of each node. Each vertex follows up to 4 bones
whose weights add up to 1 (with a margin of 0.02).

The engine draws the mesh once and only draws it again when the pose, the blend shapes, the order
of the parts or a texture video change. A still skeleton asks for no frames. At the lowest quality
level, a moving skeleton asks for at most 30 frames per second.

### File limits

A file is rejected if it has a bone cycle or a node with two parents, a vertex whose weights do not
add up to 1 or that follows a bone of no skeleton, an index outside the vertices, primitives that
are not triangles (strips and fans are read as triangles), unordered animation keys, a repeated
blend shape, more than 1,024 nodes, 65,536 vertices, 1,048,576 triangle corners, 256 bones and
parts, 64 drawn primitives, 524,288 blend shape vertices or 4,194,304 animation values, or 128 MiB
of meshes in total.

## Parts

Parts are drawn by `order` (from -1000 to 1000, default 0; it can follow keys) and, if they tie, in
the order of the file. `clip_to` is another part over which this one is only seen (a stencil
clip), and `visible` hides it.

```json
"parts": { "chest": { "order": 1 }, "legs": { "order": 0 } }
```

`depth` makes the depth of the vertices (the z of the file, toward the viewer, in pixels) push them
with the perspective of `canvas.perspective` or of the 3D camera, even under `tilt`. `scale` goes
from 0 to 10 (default 1), `cull` hides triangles seen from behind and `sort` (default `true`)
draws the parts back to front every frame. With `depth`, the nearest vertices cover the far ones
whatever the order of their parts.
