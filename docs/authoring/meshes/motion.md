# Clips, bones and expressions

On this page: clips, animation layers, reaching targets, physics, rules and blend shapes.

## Clips

A clip is a named stretch of animation. It can come from the file or carry keys of its own.

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

| Field | Prefill | What it does |
|-------|---------|--------------|
| `id` | required | The name of the clip. |
| `animation` | the first of the file | Which animation of the file it uses. Without it and without keys, it uses the first. |
| `from`, `to` | 0 and the end | The stretch, in seconds. |
| `loop` | `repeat` | `repeat`, `pingpong` or `once`. |
| `events` | | Named events `{ "t", "name" }`, in seconds from the start of the clip. They fire every time the clip passes them, and start whatever `event:<name>` triggers; they reach scripts as `timelineEvent` with the timeline `<layer>#<clip>`. |
| `bones` | | Own keys per bone. |
| `shapes` | | Own keys per blend shape. |
| `order` | | Own keys of the order of each part. |

The keys are lists of `[time, value]` with an optional easing, as in timelines (see
[scene.md](../scene.md)). A clip with its own keys and without `animation` lasts until its last key.

| Key | What it does |
|-----|--------------|
| `bones` | Per bone: `rotation` in degrees, clockwise; `position` `[x, y]` in pixels, with y downward, in the space of its parent; and `scale` `[x, y]`. They are added to its rest pose and replace the tracks of the file for that bone. In a 3D model also `angles` (degrees around x, y, z) and `shift` (model units). |
| `shapes` | Per blend shape: its weight, from -1 to 1. |
| `order` | Per part: the `order` it takes while the clip plays, rounded; `parts.<name>.order` wins if set. |

From the terminal: `reiwand-ctl scene act FOLDER clip`, `scene act FOLDER key`, `scene ask FOLDER keys`.

## Animation layers

`animations` blends clips, in order: first the layers that replace and then the additive ones.

| Field | Prefill | Range | What it does |
|-------|---------|-------|--------------|
| `clip` | required | | The id of a clip. |
| `weight` | 1 | 0 to 1 | How much it weighs. It can follow values. |
| `rate` | 1 | 0 to 16 | The speed. |
| `additive` | `false` | | Adds what the clip moves away from rest, by its weight. |
| `blend_in`, `blend_out` | 0 | 0 to 10 s | How long the weight takes to rise when starting or turning on, and to fall when turning off, stopping or (with `once`) ending. |
| `enabled` | `true` | | Turns the layer on or off. Two layers that swap `enabled` cross-fade. |
| `bones` | all | | Limits the layer to these bones. |

A layer that replaces takes each bone from where the previous ones left it toward its pose, by its
weight; rotations are blended the short way. Bones without a track stay in their rest pose. A
`once` clip ends and stays in its last pose, and notifies scripts. Blend shapes follow the same
blending. `reiwand-ctl scene act FOLDER animate` adds animation layers and lists the clips and the
bones of a mesh or model layer.

## Inverse kinematics

`ik` makes a chain of bones reach a target, like an arm that reaches a hand.

| Field | Prefill | What it does |
|-------|---------|--------------|
| `chain` | required | From 2 to 8 bones, each hanging from the previous one. |
| `target` | required | A point `[x, y]` in pixels of the box from its top-left corner (it can follow values), `"cursor"` (the pointer, which needs the `pointer` input), `"bone:<name>"` or `"layer:<id>"`. |
| `limits` | | Per bone `[min, max]` degrees relative to its rest. |
| `weight` | 1 | From 0 to 1: blends the rotation with the pose. |
| `bend` | `clockwise` | `clockwise` or `counterclockwise`: which way each joint bends. |

A chain of two bones is solved exactly; a longer one does up to 10 rounds. A target out of reach
stretches the chain toward it.

## Bone physics

`physics` makes a bone sway. Each body takes one step per drawn frame, and the movement is the
same at any rate.

| Field | Prefill | Range | What it does |
|-------|---------|-------|--------------|
| `bone` | required | | The bone. |
| `mode` | `spring` | `spring`, `rigid`, `rope` | `spring` pulls the tip of its bone toward where the pose puts it; `rigid` swings like a loose pendulum; `rope` joins the bone and its first children (up to 8 links) into a rope that hangs and sways. |
| `stiffness` | 0.5 | 0 to 1 | The stiffness of the spring. |
| `damping` | 0.5 | 0 to 1 | How fast it calms down. |
| `inertia` | 0.5 | 0 to 1 | How much it lags behind when its parent moves. |
| `wind` | 0 | 0 to 1 | How much `canvas.wind` pushes it. |
| `gravity` | 0 | -10 to 10 | Pulls it down (98 pixels per second squared for each unit). |
| `rotation` | | `[min, max]` degrees | How much each link can turn from its pose. |
| `translation` | | `[min, max]` px | How much the head of the bone can move. |

The wind of the scene (`canvas.wind`, with `canvas.gusts`) is set from the terminal with
`reiwand-ctl scene act FOLDER wind`. In 3D models, physics moves in the x and y plane of the model
and counts one model unit as 100 pixels for gravity, wind and translation. Wind and gravity act
along x, y and -y of the model, not of the world: a rotated model receives them rotated. A body
that has not moved in half a second, with the pose and the wind still, asks for no frames. At the
lowest level the bodies stay with the pose of the animation. `reiwand-ctl scene act FOLDER physics`
creates the bodies and draws a rope of new bones.

## Bone rules

`rules` changes the parent of a bone while `weight` goes from 0 to 1, keeping its pose, like a sword
that passes from a rock to a hand. Each rule is `{ "bone", "parent", "weight" }` and cannot close a
cycle of parents.

## Blend shapes and expressions

Blend shapes are deformations that come in the file (a smile). `shapes` gives them a weight from 0
to 1 by name:

(In the example, `"shapes": { "smile": 0.2 }`; instead of a number it can be a value that moves,
such as `{ "prop": "mood" }`.)

`expressions` defines up to 64 named sets of shape weights, `{ "<name>": { "<shape>": weight } }`
(weights from -1 to 1), whose name `shapes` can also set. At most 8 shapes act at once on each
part: the heaviest ones.
