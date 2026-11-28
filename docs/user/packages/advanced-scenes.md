# Advanced scenes

The format also describes the most advanced scenes: skeleton meshes, 3D models, rich particle
systems and more. This page gives an overview of what the format describes and how inputs are
declared. Later pages explain what the desktop and the phone already draw.

## What the format describes

| Area | What it includes |
| --- | --- |
| `mesh` layers | Deform an image with a skeleton (binary glTF with bones and weights), with clips and animation layers, inverse kinematics toward a point, the cursor, a bone or another layer, spring and rope physics with wind, rules that change the parent of a bone, blend shapes and expressions, parts with order and clipping, and bones that receive the pointer. |
| `model` layers | glTF models, metallic-roughness materials (also fur, foliage, chroma or package material shaders), 3D groups with shared depth, a 3D camera with paths, and light shadows. |
| Attachments | Layers attached to bones or points of a model. |
| Particle systems | World space with control points, per-layer settings, child systems, perspective, and emission from a layer, around or between points. |
| Particle initializers | With exponent, HSV, palette, vector velocity, turbulence, fractal noise and remapping. |
| Particle operators | Oscillate, limit, brake, keep distances or form flocks, with life windows; collisions with boxes, segments and layers. |
| Particle renderers | Rope, long trails, cutout, orientation, light, refraction and shaders of their own. |
| Script declarations | What each script uses: events, interface parts, parameters, layer templates and geometry reserves. |
| Media player | Values, covers and triggers. |
| Screens | Scenes spanning all screens. |
| RGB lighting | Output to RGB lighting with its permission (see ["RGB lighting"](../rgb-lighting.md)). |

Models are read when the package is opened, and each bone, animation, part or material that the
scene names is checked against them.

See [Skeleton meshes](meshes.md), [Particles](particles.md), [Lights](lights.md) and
[3D models](models.md).

## Declared inputs

The inputs of a version 2 package come only from what it declares. This includes those of its
scripts, which are no longer guessed from their text.

A version 1 script that declares nothing uses sound or the pointer only if it imports `audio` or
`input` from `reiwand`, never because those words appear in a comment.

## Feature names and rejection

Everything has its name in `requires.features`. While the engine does not draw it, the desktop and
the phone reject the scene with the newer-version notice.

> **Note:** The phone does not play scene sounds or have RGB lighting or several screens, so it
> only omits those parts.

## Pointer on solid layers

`solid` layers already receive the pointer:

- `click:`, `enter:` and `leave:` start timelines and sounds without scripts;
- scripts receive the events of each layer.

Scripts also play, pause and stop sound layers.
