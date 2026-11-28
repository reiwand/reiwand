# Mesh fields, clips and animation layers

On this page: the fields of a mesh layer, its clips and animation layers.

## Mesh fields and clips

| Field | Meaning |
|-------|---------|
| `mesh` | the `.glb` asset |
| `texture` | a picture or video asset (or bound value) that replaces the textures of the file; without it, each part shows the picture of its own material (up to 64 pictures a file; parts without one show the first), or white without any: several pictures are packed side by side into one atlas when the scene loads, shrunk together when they would not fit the largest texture of the GPU, and the texture coordinates of each part are moved into its picture; a picture that repeats over its part (coordinates outside 0 to 1, up to 4 repeats each way) is tiled in the atlas following the wrap mode of the sampler of its texture in the file, per direction (repeat, mirrored repeat, or clamp to the edge, which tiles nothing; repeat when the file names no sampler); beyond 4 repeats each way the last tile's edge is shown; with `texture`, the `wrap` of that asset decides it instead (`clamp` stretches the edge, `repeat` and `mirror` have no limit of repeats); the base color of each material multiplies its parts |
| `size`, `color` | the box in pixels the mesh fills (prefilled the canvas) and a color that multiplies it |
| `parts` | per part (mesh name of the file): `order` (−1000 to 1000, may follow keyframes with `step`), `clip_to` (another part it only shows over) and `visible`; parts are drawn by `order` (prefilled 0), in the order of the file when it is the same |
| `depth` | `{ "scale" (0 to 10, prefilled 1), "cull", "sort" }`: how far the depth of the vertices (z of the file, towards the viewer, in pixels) pushes them with the perspective of `canvas.perspective` or of the 3D camera, also under `transform.tilt` and under the tilts of the groups and parents around the layer, always along the face of the layer as all of them turn it, and whether triangles seen from behind (turning clockwise on screen) are hidden; with `depth` nearer vertices also cover farther ones whatever the order of their parts, and parts at the same depth keep their order, a later part covering the soft pixels of an earlier one as it does without `depth`; the opaque pixels of every part are drawn first, writing depth, and the soft ones (alpha below 1) after them, tested against that depth without writing it, so a soft edge never hides what lies behind it as a halo whatever the order of the parts, even where parts span each other's depth or cut through each other; `sort` (prefilled on, `false` keeps the order of the file and of `parts`) draws the parts from back to front each frame by the average depth of their vertices as posed, blend shapes included (a part cut to another one goes with it), so soft pixels over soft pixels blend in order, at no cost while depth is off; without `depth` the depth of vertices is ignored |
| `clips` | at most 64 `{ "id", "animation", "from", "to", "loop", "events", "bones", "shapes", "order" }`: a stretch of an animation of the file (the first one without `animation`, unless the clip has keys of its own) in seconds within its length (a clip with keys of its own and no `animation` lasts until its last key, or until a later `to`, holding its last keys), played `repeat` (prefilled), `pingpong` or `once`, with named `events` (seconds from the start of the clip) that fire like those of timelines each time a playing clip passes them, both ways for `pingpong`, start what `event:<name>` triggers and reach scripts as `timelineEvent` with the timeline `<layer>#<clip>`, and keys of its own on the same seconds (below) |
| `animations` | at most 16 animation layers `{ "clip", "weight" (0 to 1), "rate" (0 to 16), "additive", "blend_in", "blend_out" (0 to 10 s), "enabled", "bones" }`, mixed in order: replacing layers first, then additive ones; `bones` limits a layer to those bones; bones without a track keep their rest pose |
| `ik` | at most 16 chains `{ "chain" (2 to 8 bones, each hanging from the one before), "target", "limits": { "<bone>": [min, max] }, "weight", "bend": "clockwise" \| "counterclockwise" }` (the way every joint of the chain bends); `target` is a point in pixels of the box from its top left (which may be bound), `"cursor"`, `"bone:<name>"` (any node of the file) or `"layer:<id>"`; see "Reaching and swinging" (`ik`) |
| `physics` | at most 64 `{ "bone", "mode": "spring" \| "rigid" \| "rope", "stiffness", "damping", "inertia" (0 to 1, prefilled 0.5), "wind" (0 to 1), "gravity" (−10 to 10), "rotation": [min, max] degrees, "translation": [min, max] px }`; see "Reaching and swinging" (`bone_physics`) |
| `colliders` | at most 8 `{ "bone", "shape": "capsule" \| "sphere", "radius" (0 to 100), "offset": [x, y, z] }`: a capsule along a bone or a sphere at its head that keeps the bones of the bodies of `physics` (those of other bones) out, in units of the model on a `model` and in pixels on a `mesh` (where only x and y count); the first two count at the second lowest quality and none at the lowest (`bone_physics`) |
| `rules` | at most 64 `{ "bone", "parent", "weight" }`: moves a bone over to another parent as `weight` goes from 0 to 1, keeping its pose, as a sword passing from a rock to a hand (`bone_rules`) |
| `shapes`, `expressions` | weights from 0 to 1 of the blend shapes of the file, and at most 64 expressions `{ "<name>": { "<shape>": weight (−1 to 1) } }` whose names `shapes` may also set; at most 8 shapes act at once on each part, the heaviest (`blend_shapes`) |
| `hit` | bones whose area takes the pointer for scripts, which then learn the bone that was touched (`layer_events`): the layer takes the pointer like a `solid` one, and each event carries the listed bone that bends the drawn triangle under the pointer, or the listed bone it hangs from, else `null` |

The keys of a clip are lists of `[time, value]` keyframes with an optional easing, as in timelines:

| Key | Meaning |
|-----|---------|
| `bones` | per bone (a node of the file): `rotation` in degrees clockwise, `position` `[x, y]` in pixels with y down in the space of its parent, and `scale` `[x, y]`, each added to its rest pose; they replace the tracks of the file for that bone |
| `shapes` | per blend shape: its weight (−1 to 1) |
| `order` | per part: the `order` it takes while the clip plays, rounded to whole numbers; `parts.<name>.order` still wins when set |

## Animation layers

A replacing layer moves each bone from what the layers before it left towards its own pose by its
weight; rotations are blended along the shorter way and normalized. An additive layer adds what its
clip moves away from the rest pose, times its weight. The weight of a layer also follows its fade:
it grows over `blend_in` when the layer starts or is enabled and falls over `blend_out` when it is
disabled, stopped or, for `once`, when it ends, so two layers that swap `enabled` cross-fade. A layer
plays while it is shown and weighs more than nothing; a `once` clip then holds its last pose and
tells scripts that it ended. Blend shapes follow the same mix (the tracks of the file named `weights`
and the `shapes` of clips), and `shapes` and the expressions they name add to it. A `rules` entry
blends the place the bone would have under its own parent with the place it would have under the new
one, which keeps the place it had when the rule began to weigh (`bone_rules`); a rule may not close a
loop of parents. Cutouts use a stencil: a part with `clip_to` only shows where the part it follows
was drawn, as last drawn. The pose is worked out at most once per frame, only for shown layers, with
no allocation per frame; at the lowest quality moving skeletons ask for at most 30 frames a second.

```json
{ "blend_space": "canvas", "id": "fox", "type": "mesh", "mesh": "fox", "size": [600, 900],
  "clips": [{ "id": "idle", "animation": "idle", "events": [{ "t": 1.2, "name": "blink" }] },
            { "id": "wave", "loop": "pingpong", "bones": { "arm": { "rotation": [[0, 0], [0.8, 35, "in_out_sine"]] } },
              "order": { "tail": [[0, 1]] } }],
  "animations": [{ "clip": "idle" }, { "clip": "wave", "bones": ["arm", "hand"], "blend_in": 0.3 }],
  "ik": [{ "chain": ["neck", "head"], "target": "cursor", "limits": { "head": [-30, 30] } }],
  "physics": [{ "bone": "tail", "mode": "rope", "wind": 0.6 }],
  "shapes": { "smile": { "prop": "mood" } } }
```
