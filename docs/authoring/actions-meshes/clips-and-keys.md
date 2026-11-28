# Clips and keys

## `clip`

Creates, changes or removes a clip of a mesh or model layer. In the app, **Edit clips on the timeline** (in the layer inspector) opens the clip bar: **Add a clip**, **Add a clip and play it in a new animation layer**, **Rename the clip**, **Remove the clip**, the **Length** and **Repeat**, **Back and forth** or **Once**.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text (required) | Mesh or model layer. |
| `id` | text (required) | Id of the clip: 1 to 32 lowercase letters, digits or `_`. |
| `add` | yes/no | `true` creates the clip. By default it lasts 2 seconds. On a mesh with no animations of its own, it starts with a key at 0 s with the rest pose of its first bone (or the order of its first part). A repeated id is rejected. Up to 64 clips per layer are accepted. |
| `remove` | yes/no | `true` removes the clip and the animation layers that play it. |
| `to` | text | New id. The animation layers that play it follow it. |
| `length` | number | Duration in seconds, greater than 0 and up to 3600. |
| `loop` | text | `once`, `repeat` (the default) or `pingpong` (back and forth). |
| `animation` | text | Name of an animation in the layer's `.glb` file. The clip plays it whole and takes its duration. |
| `from` | number | Second of the file's animation where the clip starts. It keeps the clip's duration inside the animation. |
| `play` | yes/no | Only together with `add`: also adds an animation layer that plays the new clip (up to 16 per layer). |

A clip with an animation from the file (`animation`) plays the file's motion; the keys you add with `key` are added on top.

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene clip '{"layer":"mesh","id":"greeting","add":true,"length":1.5,"loop":"pingpong","play":true}'
```

Change the duration, rename and remove:

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene clip '{"layer":"mesh","id":"greeting","add":true}'
reiwand-ctl scene act my-scene clip '{"layer":"mesh","id":"greeting","length":3,"loop":"once"}'
reiwand-ctl scene act my-scene clip '{"layer":"mesh","id":"greeting","to":"goodbye"}'
reiwand-ctl scene act my-scene clip '{"layer":"mesh","id":"goodbye","remove":true}'
```

A clip that plays an animation from a model's file:

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"model","file":"model.glb","id":"doll"}'
reiwand-ctl scene act my-scene clip '{"layer":"doll","id":"step","add":true,"animation":"walk"}'
```

A layer's clips and file animations are listed by the `motion` query.

## `key`

Sets, moves, copies or removes a key of a clip. A key fixes the value of a channel at a second, and between two keys the value moves gradually. In the app, the clip timeline: the **Key** button writes the channels that the chosen bone's handles changed, **Curve of the key** curves the stretch up to the next key and **Remove the key** deletes it.

Each key is about one thing only: a bone, a blend shape or a part's order.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text (required) | Mesh or model layer. |
| `clip` | text (required) | Id of the clip. |
| `time` | number (required) | Second of the key, counted from the start of the clip and rounded to the millisecond. From 0 to 3600. |
| `bone` | text | Bone of the key. |
| `rotation` | number | Mesh only: bone rotation in degrees. |
| `position` | `[x, y]` | Mesh only: how far the bone shifts, in layer pixels. `x` to the right, `y` downward. |
| `scale` | `[x, y]` | Mesh only: size factor of the bone. 1 is the rest size. |
| `angles` | `[x, y, z]` | Model only: bone rotation in degrees on each axis. |
| `shift` | `[x, y, z]` | Model only: bone shift in model units. |
| `shape` | text | Blend shape of the key. |
| `weight` | number | Weight of the shape, from -1 to 1. Without it, the value the clip gives at that second is used. |
| `order` | text | Part whose draw order is animated. |
| `value` | number | Order of the part (rounded to an integer). Without it, the value the clip gives at that second is used. |
| `ease` | text or list of 4 numbers | How the value moves up to the next key: `linear` (default), `step`, `hold`, or `in_`, `out_` or `in_out_` followed by `sine`, `quad`, `cubic`, `quart`, `quint`, `expo`, `circ`, `back`, `elastic` or `bounce` (for example `in_out_sine`). Also four numbers `[x1, y1, x2, y2]` of a cubic Bézier curve. A key with `ease` loses its handles. |
| `in_handle` | `[dt, dv]` | Bézier handle with which the curve arrives at the key: seconds back to the previous key and how much the value changes there. It curves a single channel. It cannot be combined with `ease`. |
| `out_handle` | `[dt, dv]` | The same for the curve that leaves toward the next key. |
| `remove` | yes/no | `true` removes the key at that second. |
| `from` | number | Second of an existing key: moves it to `time`, or copies it if you set `copy`. |
| `copy` | yes/no | With `from`: leaves the original key and puts a copy. |

With a mesh bone and no values, the key takes all three channels (`rotation`, `position`, `scale`) with the pose the clip gives at that second. With values, only the channels you name are touched. On a model, a bone with no values takes `angles` and `shift`.

The ranges are: `rotation` and `position` up to 100000 in absolute value, `scale` up to 1000. Handles extend from 0 s up to the neighboring key.

**Key of a mesh bone**

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"arm","parent":"root","at":[0.5,0.5]}'
reiwand-ctl scene act my-scene clip '{"layer":"mesh","id":"greeting","add":true,"length":2}'
reiwand-ctl scene act my-scene key '{"layer":"mesh","clip":"greeting","time":0,"bone":"arm","rotation":0}'
reiwand-ctl scene act my-scene key '{"layer":"mesh","clip":"greeting","time":1,"bone":"arm","rotation":40,"ease":"in_out_sine"}'
reiwand-ctl scene act my-scene key '{"layer":"mesh","clip":"greeting","time":2,"bone":"arm","rotation":0}'
reiwand-ctl scene act my-scene key '{"layer":"mesh","clip":"greeting","time":1,"bone":"arm","position":[10,5],"scale":[1.2,1.2]}'
```

Move a key, copy it and remove it:

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"arm","parent":"root","at":[0.5,0.5]}'
reiwand-ctl scene act my-scene clip '{"layer":"mesh","id":"greeting","add":true,"length":2}'
reiwand-ctl scene act my-scene key '{"layer":"mesh","clip":"greeting","time":1,"bone":"arm","rotation":40}'
reiwand-ctl scene act my-scene key '{"layer":"mesh","clip":"greeting","time":1.5,"bone":"arm","from":1}'
reiwand-ctl scene act my-scene key '{"layer":"mesh","clip":"greeting","time":0.5,"bone":"arm","from":1.5,"copy":true}'
reiwand-ctl scene act my-scene key '{"layer":"mesh","clip":"greeting","time":0.5,"bone":"arm","remove":true}'
```

With Bézier handles:

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"arm","parent":"root","at":[0.5,0.5]}'
reiwand-ctl scene act my-scene clip '{"layer":"mesh","id":"greeting","add":true,"length":2}'
reiwand-ctl scene act my-scene key '{"layer":"mesh","clip":"greeting","time":0,"bone":"arm","rotation":0}'
reiwand-ctl scene act my-scene key '{"layer":"mesh","clip":"greeting","time":1,"bone":"arm","rotation":40}'
reiwand-ctl scene act my-scene key '{"layer":"mesh","clip":"greeting","time":0,"bone":"arm","rotation":0,"out_handle":[0.4,25]}'
```

**Key of a model bone**

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"model","file":"model.glb","id":"doll"}'
reiwand-ctl scene act my-scene clip '{"layer":"doll","id":"nod","add":true,"length":1}'
reiwand-ctl scene act my-scene key '{"layer":"doll","clip":"nod","time":0.5,"bone":"tip","angles":[0,20,0],"shift":[0,0.1,0]}'
```

**Key of a blend shape**

The blend shape must exist: the file brings it or you create it with `shape_sculpt`.

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene shape_sculpt '{"layer":"mesh","shape":"smile","points":[[0.4,0.5],[0.6,0.5]],"mode":"push","by":20,"radius":0.2}'
reiwand-ctl scene act my-scene clip '{"layer":"mesh","id":"greeting","add":true,"length":2}'
reiwand-ctl scene act my-scene key '{"layer":"mesh","clip":"greeting","time":1,"shape":"smile","weight":1}'
```

**Key of a part's order**

Animates a part's draw order (see `parts`). It asks for a part that exists.

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene clip '{"layer":"mesh","id":"swap","add":true,"length":2}'
reiwand-ctl scene act my-scene key '{"layer":"mesh","clip":"swap","time":1,"order":"body","value":5}'
```

A clip's keys are shown by the `keys` query and the pose they give, by the `pose` query.

## `edit_keys`

Edits several keys at once, already picked: moves them in time, stretches the times, raises or sets their value, curves or straightens their handles, removes them or pastes copied keys. It is the action behind the app's **Curve editor** (the **Curve editor** button): **Stretch times ×**, **Scale values ×**, **Aligned handles** or **Broken handles**, **Linked parts** or **Separate parts**, **Straighten**, **Copy**, **Paste at the playhead** and **Remove the picked keys**. It works for any animated value in the scene, not only clips.

Each call does a single thing, and is a single undo step.

Each element of `picks` chooses keys:

| Field of `picks` | Type | What it means |
|---|---|---|
| `pointer` | text | Where the animated value is in the scene file. For a clip channel it is `/scene/layers/N/clips/M/bones/BONE/rotation` (also `position`, `scale`, `angles`, `shift`), `/scene/layers/N/clips/M/shapes/SHAPE` or `/scene/layers/N/clips/M/order/PART`. `N` is the layer's place in the scene and `M` the clip's place in the layer, both counted from 0. The `motion` query gives the layer's (`pointer`) and the order of its clips. |
| `times` | list of numbers | Seconds of the picked keys, as they are written in the file. If the clip starts past second 0 of its animation (`from` in `clip`), add that value to the times you see in the clip bar. |
| `keys` | list of keys | Only for pasting: copied keys, each `[time, value]` or `[time, value, curve]`. |

| Argument | Type | What it means |
|---|---|---|
| `picks` | list of objects (required) | The keys that are acted on. |
| `shift` | number | Seconds added to the times of the picked keys. Default 0. |
| `scale` | number | Stretches the times of the picked keys, and the length of their handles, around `pivot`. Greater than 0. |
| `pivot` | number | Second around which `scale` stretches. Default 0. |
| `part` | integer | Which part of a multi-part value `lift`, `value`, `value_scale` and the handles touch: 0 is the first (x or red), 1 the second (y or green), and so on. |
| `lift` | number | Added to the value of the picked keys. Without `part`, to all parts (on a color, except alpha). Default 0. |
| `value` | number | Sets the value of part `part`. On lists and colors `part` is required. |
| `value_scale` | number | Multiplies the value of the picked keys around `value_pivot`. It scales the handles too. |
| `value_pivot` | number | Value around which `value_scale` scales. Without it, the middle of each part's picked values. |
| `in_handle` | `[dt, dv]` | Sets a key's incoming handle. |
| `out_handle` | `[dt, dv]` | Sets a key's outgoing handle. |
| `aligned` | yes/no | With a single handle given: the other one is left aligned with the same slope (with its own length, or a third of the stretch if it had none). Default `false`. |
| `link` | yes/no | Joins a key's per-part handles into a single pair for all parts, with the shape of `part` (the first if missing). |
| `straighten` | yes/no | Straightens the curve between the picked key and the next one: removes the handles of that stretch. It goes alone. |
| `remove` | yes/no | Removes the picked keys. At least one key always remains in each value. |
| `at` | number | For pasting: second where the first of the pasted keys lands. Keys at those seconds are replaced. |

To curve handles (`in_handle`, `out_handle`, `link`, `straighten`) you pick a single key: one `pick` with a single time. Keys that would end up before 0 s or less than a millisecond from another, and handles that go past the neighboring key, are rejected.

First a clip with an animated rotation:

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"arm","parent":"root","at":[0.5,0.5]}'
reiwand-ctl scene act my-scene clip '{"layer":"mesh","id":"greeting","add":true,"length":2}'
reiwand-ctl scene act my-scene key '{"layer":"mesh","clip":"greeting","time":0,"bone":"arm","rotation":0}'
reiwand-ctl scene act my-scene key '{"layer":"mesh","clip":"greeting","time":1,"bone":"arm","rotation":40}'
reiwand-ctl scene act my-scene key '{"layer":"mesh","clip":"greeting","time":2,"bone":"arm","rotation":0}'
```

Move a key half a second and raise its value:

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"arm","parent":"root","at":[0.5,0.5]}'
reiwand-ctl scene act my-scene clip '{"layer":"mesh","id":"greeting","add":true,"length":2}'
reiwand-ctl scene act my-scene key '{"layer":"mesh","clip":"greeting","time":0,"bone":"arm","rotation":0}'
reiwand-ctl scene act my-scene key '{"layer":"mesh","clip":"greeting","time":1,"bone":"arm","rotation":40}'
reiwand-ctl scene act my-scene key '{"layer":"mesh","clip":"greeting","time":2,"bone":"arm","rotation":0}'
reiwand-ctl scene act my-scene edit_keys '{"picks":[{"pointer":"/scene/layers/0/clips/0/bones/arm/rotation","times":[1]}],"shift":0.5}'
reiwand-ctl scene act my-scene edit_keys '{"picks":[{"pointer":"/scene/layers/0/clips/0/bones/arm/rotation","times":[1.5]}],"lift":10}'
```

Stretch all the times to half:

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"arm","parent":"root","at":[0.5,0.5]}'
reiwand-ctl scene act my-scene clip '{"layer":"mesh","id":"greeting","add":true,"length":2}'
reiwand-ctl scene act my-scene key '{"layer":"mesh","clip":"greeting","time":0,"bone":"arm","rotation":0}'
reiwand-ctl scene act my-scene key '{"layer":"mesh","clip":"greeting","time":1,"bone":"arm","rotation":40}'
reiwand-ctl scene act my-scene key '{"layer":"mesh","clip":"greeting","time":2,"bone":"arm","rotation":0}'
reiwand-ctl scene act my-scene edit_keys '{"picks":[{"pointer":"/scene/layers/0/clips/0/bones/arm/rotation","times":[0,1,2]}],"scale":0.5}'
```

Multiply the values of two keys around 0 and remove one:

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"arm","parent":"root","at":[0.5,0.5]}'
reiwand-ctl scene act my-scene clip '{"layer":"mesh","id":"greeting","add":true,"length":2}'
reiwand-ctl scene act my-scene key '{"layer":"mesh","clip":"greeting","time":0,"bone":"arm","rotation":10}'
reiwand-ctl scene act my-scene key '{"layer":"mesh","clip":"greeting","time":1,"bone":"arm","rotation":40}'
reiwand-ctl scene act my-scene key '{"layer":"mesh","clip":"greeting","time":2,"bone":"arm","rotation":0}'
reiwand-ctl scene act my-scene edit_keys '{"picks":[{"pointer":"/scene/layers/0/clips/0/bones/arm/rotation","times":[0,1]}],"value_scale":0.5,"value_pivot":0}'
reiwand-ctl scene act my-scene edit_keys '{"picks":[{"pointer":"/scene/layers/0/clips/0/bones/arm/rotation","times":[2]}],"remove":true}'
```

Set a single part of a two-part value (the `y` of a position):

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"arm","parent":"root","at":[0.5,0.5]}'
reiwand-ctl scene act my-scene clip '{"layer":"mesh","id":"greeting","add":true,"length":2}'
reiwand-ctl scene act my-scene key '{"layer":"mesh","clip":"greeting","time":1,"bone":"arm","position":[10,5]}'
reiwand-ctl scene act my-scene edit_keys '{"picks":[{"pointer":"/scene/layers/0/clips/0/bones/arm/position","times":[1]}],"part":1,"value":30}'
```

Handles: set an outgoing one with the incoming one aligned, straighten the stretch and link the parts:

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"arm","parent":"root","at":[0.5,0.5]}'
reiwand-ctl scene act my-scene clip '{"layer":"mesh","id":"greeting","add":true,"length":2}'
reiwand-ctl scene act my-scene key '{"layer":"mesh","clip":"greeting","time":0,"bone":"arm","rotation":0}'
reiwand-ctl scene act my-scene key '{"layer":"mesh","clip":"greeting","time":1,"bone":"arm","rotation":40}'
reiwand-ctl scene act my-scene key '{"layer":"mesh","clip":"greeting","time":2,"bone":"arm","rotation":0}'
reiwand-ctl scene act my-scene key '{"layer":"mesh","clip":"greeting","time":1,"bone":"arm","position":[10,5]}'
reiwand-ctl scene act my-scene edit_keys '{"picks":[{"pointer":"/scene/layers/0/clips/0/bones/arm/rotation","times":[1]}],"out_handle":[0.3,10],"aligned":true}'
reiwand-ctl scene act my-scene edit_keys '{"picks":[{"pointer":"/scene/layers/0/clips/0/bones/arm/rotation","times":[1]}],"straighten":true}'
reiwand-ctl scene act my-scene edit_keys '{"picks":[{"pointer":"/scene/layers/0/clips/0/bones/arm/position","times":[1]}],"link":true}'
```

Paste copied keys:

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"arm","parent":"root","at":[0.5,0.5]}'
reiwand-ctl scene act my-scene clip '{"layer":"mesh","id":"greeting","add":true,"length":2}'
reiwand-ctl scene act my-scene key '{"layer":"mesh","clip":"greeting","time":0,"bone":"arm","rotation":0}'
reiwand-ctl scene act my-scene edit_keys '{"picks":[{"pointer":"/scene/layers/0/clips/0/bones/arm/rotation","keys":[[0,0],[0.4,25]]}],"at":1.2}'
```

## `clip_event`

Adds, moves, renames or removes an event of a clip. An event is a named mark at a second of the clip: it fires when the clip passes it, and triggers and scripts can start from its name. In the app, the **Events** strip above the clip's rows: a double click adds an event, dragging the mark moves it and a click opens **Name of the event**, **Rename** and **Remove**.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text (required) | Mesh or model layer. |
| `clip` | text (required) | Id of the clip. |
| `name` | text (required) | Name of the event, 1 to 64 characters without `/` or `#`. |
| `time` | number | Second of the event, from the start of the clip and within its duration. If an event with that name already exists, it moves it. Otherwise, it creates it. |
| `at` | number | Second the event is at today, to tell apart events with the same name. |
| `to` | text | New name. |
| `remove` | yes/no | `true` removes the event. |

A clip accepts up to 64 events and stores them sorted by time.

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene clip '{"layer":"mesh","id":"greeting","add":true,"length":2}'
reiwand-ctl scene act my-scene clip_event '{"layer":"mesh","clip":"greeting","name":"hello","time":0.5}'
```

Move, rename and remove:

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene clip '{"layer":"mesh","id":"greeting","add":true,"length":2}'
reiwand-ctl scene act my-scene clip_event '{"layer":"mesh","clip":"greeting","name":"hello","time":0.5}'
reiwand-ctl scene act my-scene clip_event '{"layer":"mesh","clip":"greeting","name":"hello","time":1.2}'
reiwand-ctl scene act my-scene clip_event '{"layer":"mesh","clip":"greeting","name":"hello","to":"goodbye"}'
reiwand-ctl scene act my-scene clip_event '{"layer":"mesh","clip":"greeting","name":"goodbye","remove":true}'
```
