# 3D models

The desktop and the phone draw 3D models (`type: model`). This page covers model files, 3D groups,
materials, the 3D camera, cost limits, HDR, animation, special materials and shadows.

## Model files

Models are binary glTF with:

- meshes and nodes;
- metallic-roughness materials (color, normals, metal and roughness, occlusion, own light,
  transparency modes and double face);
- PNG, JPEG or WebP textures of up to 4096 pixels (2048 on phones and modest computers).

## Loose models and 3D groups

A loose model is drawn in its own box.

A group with `space: "3d"` gathers its models in a pass with 32-bit reversed depth and is composed
like one more layer, with effects, masks and blending.

The flat layers of the group stay in front unless they ask for `depth_test`. Then they are placed
on the plane the camera faces and the nearest models cover them.

## Materials and light

Scene materials change by name:

- the shader (`pbr` or `unlit`);
- the tint;
- the own light;
- the double face;
- the numbers.

Light layers light the models (12 at most, 8 on phones), with ambient light and sky light. A
panoramic image (`env`) gives the surrounding light and the reflections, without reflections at the
lowest quality. See [Lights](lights.md).

## 3D camera

The 3D camera (`camera3d`: field of view, near and far planes, eye, center and up) serves models,
3D groups and tilted layers. Scripts change it for the next frame.

The 3D camera follows paths (`camera3d.paths`) by keys with a centripetal Catmull-Rom curve:

- in order or at random;
- with a fade to black or crossfade between them;
- with closed loops without jumps.

## Cost and triangle limits

- A still model is drawn once and stored.
- A 3D group is only redrawn when something of its own changes.

Each class of computer accepts 50,000, 150,000, 500,000 or 2,000,000 triangles (with the fur layers
of the lowest quality). If the models draw more:

- with automatic quality, that computer shows the poster of the scene;
- with a fixed level or a chosen minimum, it plays it anyway and warns of what it costs.

## HDR and tone mapping

With `canvas.hdr` the colors brighter than white are kept in 16-bit images, and `canvas.tonemap`
(`aces`, `agx` or `srgb_to_linear`) adjusts them to the screen in a last pass. Without `tonemap`, the scene looks as
always.

## Animation

Models are animated with the same system as meshes (see [Skeleton meshes](meshes.md)):

- clips of the animations of the file or with keys of their own (x, y and z rotations and 3D
  displacements per bone, blend shape weights);
- animation layers with weight, speed, additive mode, fades and bone mask;
- bone physics, which freezes at the lowest quality;
- root motion (`root_motion`), with which an advancing clip moves the layer instead of the skeleton.

The skeleton is computed on the CPU only for visible models, and the GPU deforms the vertices (4
bones per vertex) and blends the shapes of the file.

## Special materials

| Material | What it does |
| --- | --- |
| `fur` | Fur layers pushed along the normal, from 1 to 32 and fewer at low qualities, with length, density and combing direction. |
| `foliage` | Swaying with the canvas wind, measured by the red of the vertex colors. |
| `chroma` | A key color that becomes transparent. |
| Package material shader | `fn material(s: Surface) -> Surface`, validated and limited like any package effect. |

## Shadows

Point, spot and directional lights can cast shadows (`shadows`) with maps of 2048, 1024 and 6×512
pixels, filtered with four samples.

| Computer | Lights with shadows |
| --- | --- |
| Desktop | Two |
| T2 class | One |
| More modest classes | None |

A map is only redrawn when its light or what it casts changes.
