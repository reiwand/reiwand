# Attached layers and models

On this page: layers on a bone, touching a bone and animated models.

## Attaching layers to a bone

`attach` makes a layer follow a bone or a point of a mesh or a model, instead of a `parent`:

| Field | What it does |
|-------|--------------|
| `layer` | The mesh or model layer it attaches to. |
| `bone` or `point` | The bone or the attachment point. |
| `offset` | `[x, y]` in pixels along the bone. |
| `inherit` | What it takes: `position`, `rotation` and `scale` (all by default). Without `position`, the layer stays at the origin of the mesh. |

The layer hides with its mesh, and its own `transform` is applied on top. A layer cannot follow
itself, or do so through others. With `reiwand-ctl scene act FOLDER attach` and `scene act FOLDER
point`, attachments are made and attachment points created.

## Touching a bone

`hit` lists the bones whose zone receives the pointer for scripts: the layer receives the pointer
like a `solid` layer, and each event carries the listed bone that bends the triangle drawn under
the pointer, or the listed one it hangs from. The `click:`, `enter:` and `leave:` triggers of the
layer also work.

## Animated models

A 3D model with any of `clips`, `animations`, `ik`, `physics`, `rules`, `shapes`, `expressions` or
`root_motion` is read with its skeleton: the joints and the nodes with meshes become bones.
Everything works as on meshes, blending the animations of the file. The `ik` targets of a model
are `bone:`, a point `[x, y]` in pixels of the box, `layer:` and `cursor`.

With `root_motion: true`, the root bone keeps the pose it has when the clips start, and what the
clips do to it moves the layer: in x, y and z, turning where the clip turns, and accumulating each
loop of a repeating clip, so that a step walks and a step that turns walks in a circle. It needs
`clips`.

```json
      {
        "blend_space": "canvas", "id": "person", "type": "model", "model": "figure",
        "position": [1.8, -0.2, 0], "size": [800, 800],
        "clips": [
          {
            "id": "walk", "animation": "walk", "loop": "repeat", "bones": {}, "events": [],
            "from": 0.0, "order": {}, "shapes": {}, "to": null
          }
        ],
        "animations": [
          {
            "clip": "walk", "additive": false, "blend_in": 0.0, "blend_out": 0.0, "bones": [],
            "enabled": true, "rate": 1.0, "weight": 1.0
          }
        ],
        "materials": {
          "body": {
            "color": "#9ad1ff", "blend": null, "cast_shadows": null, "cull": null,
            "depth_test": null, "depth_write": null, "double_sided": null, "lit": null, "maps": {},
            "params": {}, "receive_shadows": null, "shader": "pbr"
          }
        },
        "blend": "normal", "cache": false, "colliders": [], "depth_test": false, "effects": [],
        "expressions": {}, "hit": [], "ik": [], "opacity": 1.0, "parallax": [0, 0], "physics": [],
        "root_motion": false, "rules": [], "shapes": {}, "solid": false,
        "transform": {
          "origin": "anchor", "rotation": 0.0, "scale": [1.0, 1.0], "skew": [0.0, 0.0]
        },
        "visible": true
      }
```

An animated model costs the same at all quality levels and in all categories: a few samples per
root bone per frame. A model with none of those fields applies the weights of its file just once
and costs nothing more while it plays.
