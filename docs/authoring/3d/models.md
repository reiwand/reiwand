# 3D models

A `model` layer draws a binary glTF model (`.glb`) with metal and roughness materials. On its own,
it is drawn in its own box (`size`, the canvas by default) with depth and then composited like any
layer, with its effects, masks, blending and `transform`. There are up to 16 `mesh` and `model`
layers in a scene.

`.glb` files are made with common 3D tools (FBX and OBJ are converted to `.glb`). The program
reads the headers and the JSON part once and rejects models that load outside files or that ask
for extensions other than `KHR_materials_emissive_strength`, `KHR_materials_unlit`,
`KHR_mesh_quantization` and `KHR_texture_transform`. They can have up to 2,000,000 triangles and
256 bones per skeleton. The names of nodes (attachment points), joints (bones), meshes (parts),
materials, blend shapes and animations are read from the file, and every name you put in the scene
is checked against them. To see what a model contains:

```
reiwand-ctl scene import MY-DIR model.glb --id model
reiwand-ctl scene ask MY-DIR model_info '{"asset":"model"}'
```

It reports the triangles, the materials, the textures, the bones, the clips and which equipment
categories can handle it.

| Field | Prefill | What it does |
|-------|---------|--------------|
| `model` | required | The `.glb` asset. |
| `position`, `rotation`, `scale` | | Three numbers each (or a single one for all three), in model units and degrees (it is rotated around x, then y, then z). They can follow values. |
| `size` | the canvas | The box in pixels. |
| `env` | | A panoramic (equirectangular) image that the materials reflect: blurry for the surrounding light and sharper for the shiny parts. |
| `materials` | | By material name from the file, up to 64: see below. |
| `clips`, `animations`, `ik`, `physics`, `rules`, `shapes`, `expressions`, `hit`, `root_motion` | | Animation of the model. See [meshes.md](../meshes.md). |

```json
      {
        "blend_space": "canvas", "id": "cube", "type": "model", "model": "box",
        "position": [-1.8, 0, 0],
        "rotation": {
          "timeline": "spin", "keys": [[0, [0, 0, 0]], [10, [0, 360, 0]]], "invert": false
        },
        "materials": {
          "paint": {
            "color": "#ff8a3d", "params": { "metallic": 0.3, "roughness": 0.35 }, "blend": null,
            "cast_shadows": null, "cull": null, "depth_test": null, "depth_write": null,
            "double_sided": null, "lit": null, "maps": {}, "receive_shadows": null, "shader": "pbr"
          }
        },
        "animations": [], "blend": "normal", "cache": false, "clips": [], "colliders": [],
        "depth_test": false, "effects": [], "expressions": {}, "hit": [], "ik": [], "opacity": 1.0,
        "parallax": [0, 0], "physics": [], "root_motion": false, "rules": [], "shapes": {},
        "size": "canvas", "solid": false,
        "transform": {
          "origin": "anchor", "rotation": 0.0, "scale": [1.0, 1.0], "skew": [0.0, 0.0]
        },
        "visible": true
      }
```

### Materials

The materials in the file give the color, the metal, the roughness, the normal, the occlusion and
the glow with their factors. A scene material multiplies its color by `color`, replaces the glow
with `emissive`, chooses `shader` and changes the rest:

| Field | What it does |
|-------|--------------|
| `shader` | `pbr` (default), `unlit`, `fur`, `foliage`, `chroma`, or the id of an own effect with `stage: "material"`. |
| `color`, `emissive` | Multiplies the color; replaces the glow. |
| `lit` | `false` draws it without light. |
| `double_sided`, `cull` | Both faces, and which face is hidden: `back`, `front` or `none`. |
| `blend` | `opaque`, `mask`, `blend` or `add`. `blend` or an opacity below 1 draws it after the opaque things, from far to near. |
| `depth_test`, `depth_write` | How it treats depth. |
| `cast_shadows`, `receive_shadows` | Casts and receives shadows (both `true` by default). |
| `maps` | Replaces maps from the file with images from the scene: `base`, `orm`, `normal`, `occlusion`, `emissive`. Those of `orm`, `normal` and `occlusion` must be `linear`. |
| `params` | Shader parameters. |

Shader parameters:

- `pbr`: `metallic`, `roughness`, `occlusion` (0 to 1) and `normal_scale` (0 to 10).
- `fur`: `layers` (1 to 32, 12 by default: layers of fur, which drop at low quality levels),
  `length` (0 to 100, 10 by default, in hundredths of the model's radius), `density` (0 to 1, 0.5
  by default), and `comb_x`, `comb_y`, `comb_z` (-1 to 1) to comb it; `maps.comb` is a `linear`
  image that combs each spot on its own.
- `foliage` (foliage that sways with `canvas.wind`): `strength` (0 to 10, 1) and `speed` (0 to 10,
  1). The red of the file's vertex colors says how much each vertex moves. Without wind it stays
  still and does not ask for frames.
- `chroma`: makes colors close to `key` transparent (green by default), with `threshold` (0.4) and
  `softness` (0.1), from 0 to 1.
- An own effect with `stage: "material"` (see [effects.md](../effects.md)) defines
  `fn material(s: Surface) -> Surface`, which runs for every pixel before the light. It is
  compiled separately from the drawing, with the same 10-second limit as the other effects: if it
  cannot be compiled or takes longer, the model is shown with `pbr` and the editor reports it
  among what was left out, without the wallpaper stalling in the meantime.

A material is changed by name from the terminal with `reiwand-ctl scene act DIR scene3d`.

### 3D groups

A group with `space: "3d"` draws all its models at once with a shared depth, in a box the size of
the canvas, and is composited as a single layer in the group's place. A 3D group inside another
shares its depth if it is "simple" (no effects, mask, blending, opacity, `color`, `brightness`,
`clip`, `tilt`, `z` or `depth_test`).

Flat layers inside a 3D group are drawn in front of the models, in their order, unless they set
`depth_test`: then they are on a plane `transform.z` pixels from where the camera looks and the
models in front of them cover them. Up to 4 distinct planes; more layers share the nearest one.
Particle layers count as flat; with `perspective` each particle hides pixel by pixel behind the
models, also if they are lit, refract, have their own shader, trails or ropes (each point at the
depth of its particle), and child and grandchild systems likewise. Those that refract hide their
distortion behind the nearest models, but next to the edge of one they may drag some of its
pixels. At the lowest quality the whole layer goes on its plane.

A model inside a 3D group with `opacity` below 1 is drawn transparent, blended in linear light
after the opaque ones, from far to near. That is why it does not fade the same way as a standalone
model, and it casts no shadow.

```json
    {
      "blend_space": "canvas", "id": "stage", "type": "group", "space": "3d",
      "layers": [
        {
          "blend_space": "canvas", "id": "floor", "type": "model", "model": "box",
          "position": [0, -1.2, 0], "scale": [6, 0.2, 6],
          "materials": {
            "paint": {
              "color": "#46506b", "params": { "roughness": 0.8 }, "blend": null,
              "cast_shadows": null, "cull": null, "depth_test": null, "depth_write": null,
              "double_sided": null, "lit": null, "maps": {}, "receive_shadows": null,
              "shader": "pbr"
            }
          },
          "animations": [], "blend": "normal", "cache": false, "clips": [], "colliders": [],
          "depth_test": false, "effects": [], "expressions": {}, "hit": [], "ik": [], "opacity": 1.0,
          "parallax": [0, 0], "physics": [], "root_motion": false, "rules": [], "shapes": {},
          "size": "canvas", "solid": false,
          "transform": {
            "origin": "anchor", "rotation": 0.0, "scale": [1.0, 1.0], "skew": [0.0, 0.0]
          },
          "visible": true
        },
        {
          "blend_space": "canvas", "id": "cube", "type": "model", "model": "box",
          "position": [-1.8, 0, 0],
          "rotation": {
            "timeline": "spin", "keys": [[0, [0, 0, 0]], [10, [0, 360, 0]]], "invert": false
          },
          "materials": {
            "paint": {
              "color": "#ff8a3d", "params": { "metallic": 0.3, "roughness": 0.35 }, "blend": null,
              "cast_shadows": null, "cull": null, "depth_test": null, "depth_write": null,
              "double_sided": null, "lit": null, "maps": {}, "receive_shadows": null,
              "shader": "pbr"
            }
          },
          "animations": [], "blend": "normal", "cache": false, "clips": [], "colliders": [],
          "depth_test": false, "effects": [], "expressions": {}, "hit": [], "ik": [], "opacity": 1.0,
          "parallax": [0, 0], "physics": [], "root_motion": false, "rules": [], "shapes": {},
          "size": "canvas", "solid": false,
          "transform": {
            "origin": "anchor", "rotation": 0.0, "scale": [1.0, 1.0], "skew": [0.0, 0.0]
          },
          "visible": true
        },
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
      ],
      "blend": "normal", "cache": false, "clip": false, "color": "#ffffff", "depth_test": false,
      "effects": [], "opacity": 1.0, "parallax": [0, 0], "solid": false,
      "transform": { "origin": "anchor", "rotation": 0.0, "scale": [1.0, 1.0], "skew": [0.0, 0.0] },
      "visible": true
    }
```

### What it costs

Each model adds to the scene's cost one unit per 200,000 triangles on a dedicated computer GPU
(measured on such a card) and per 60,000 on phones and the more modest computers, which have not
been measured yet; the editor and the check warn with the cautious count. The same is counted
again for each shadow map it is drawn into (the first two lights with shadows; a point light draws
six). That count does not drop with the quality level (at `t2` only one light is drawn), and each
model also pays a fixed cost per stage, so a scene with many small models is counted as more
expensive than it really is. `reiwand-ctl scene bench --passes` shows how long each pass of a 3D
group takes: shadow maps, depth for particles, models and second camera. The equipment categories
admit up to 50,000 (`t0`), 150,000 (`t1`), 500,000 (`t2`) and 2,000,000 (`t3`) triangles,
skeletons of up to 256 bones and shadows from 0, 0, 1 and 2 lights. A category shows the poster of
a scene whose models draw more triangles than it admits at the lowest level; other heavy scenes
run there at lower quality. A model's textures can be PNG, JPEG or WebP of up to 4096 pixels (2048
on phones and in categories `t0` and `t1`).

```
reiwand-ctl scene ask MY-DIR cost3d '{"quality":"q2"}'
reiwand-ctl scene bench my.wpak --tier t1
```

A model whose values do not move is drawn once and stored; a 3D group is only redrawn when a
model, the camera, a light, the surrounding light or the size changes, so a still scene does not
ask for frames.
