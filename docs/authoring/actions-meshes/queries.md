# Queries

### Before you start with the mesh and animation queries

- Almost all these queries are about a mesh or model layer. On the command line the layer goes with the `--layer ID` option after the arguments. `motion` and `script_calls` also take it inside the JSON (`"layer"`). With no layer, or with one that is neither a mesh nor a model, the answer is `null`.
- The `[x, y]` places in the answers are fractions of the layer: `[0, 0]` is the top-left corner and `[1, 1]` the bottom-right one.
- The examples start from a mesh made from an image, with two bones and a clip:

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"arm","parent":"root","at":[0.5,0.5]}'
reiwand-ctl scene act my-scene clip '{"layer":"mesh","id":"greeting","add":true,"length":2}'
reiwand-ctl scene act my-scene key '{"layer":"mesh","clip":"greeting","time":1,"bone":"arm","rotation":40}'
```

### What a layer has

#### `rig`

Lists the bones, points, parts, blend shapes and clips of a mesh or model layer. It is the layer's usual query.

No arguments (`'{}'`). The answer has:

- `bones`: the bones, each with `name`, `parent` (or `null`) and, on a mesh, `at` (the place of its head).
- `points`: the other nodes in the file: the mesh's parts (`part: true`) and the attachment points (`part: false`), with `name`, `at` and `parent`.
- `parts`: names of the parts.
- `shapes` and `defaults`: blend shapes and their rest weight.
- `animations`: animations of the `.glb` file, with `name` and `length` in seconds.
- `clips`: ids of the layer's clips.
- `expressions`: the expressions with the weights of their shapes.
- `hit`: bones that receive the pointer.
- `depth`: depth setting, or `null`.
- `auto`: `true` if the mesh's weights follow the bones.
- `editable`: `true` if the bones can be edited. On a model it is `false`: its bones come from its file.
- `model`: `true` if the layer is a model.

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"arm","parent":"root","at":[0.5,0.5]}'
reiwand-ctl scene ask my-scene rig '{}' --layer mesh
```

With a model:

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"model","file":"model.glb","id":"doll"}'
reiwand-ctl scene ask my-scene rig '{}' --layer doll
```

#### `motion`

Describes the clips and all the motion of a mesh or model layer in a single answer: it is what the layer inspector shows.

Argument: `{"layer": "ID"}`, or `--layer`. The answer has:

- `layer`, `pointer` (where the layer is in the scene file, e.g. `/scene/layers/0`) and `model`.
- `clips`: each clip with `id`, `animation`, `from`, `to`, `length`, `loop`, `events`, `bones` and `shapes` (how many channels it has) and `path`, the root bone's route on a model.
- `files`: animations of the file with `name` and `length`.
- `layers`: the animation layers, with `index`, `clip`, `weight`, `rate`, `additive`, `blend_in`, `blend_out`, `enabled` and `bones`.
- `bones`, `shapes` and `defaults`, and `weights` (weights set with `animate morph`).
- `physics`, `colliders`, `ik` and `root_motion`: the same as what the actions of the same name write.
- `presets`: the modes of `animate` with the `preset` variant.

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene clip '{"layer":"mesh","id":"greeting","add":true,"length":2,"play":true}'
reiwand-ctl scene ask my-scene motion '{"layer":"mesh"}'
```

#### `outline`

Gives the edge of each part of a mesh, as the editor draws it.

No arguments (`'{}'`) and with `--layer`. The answer is an object with one field per part (e.g. `body`), and each is a list of segments `[[x1, y1], [x2, y2]]`: the sides of the triangles that no other triangle shares.

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh","cells":4}'
reiwand-ctl scene ask my-scene outline '{}' --layer mesh
```

### Keys and pose

#### `keys`

Gives a clip's keys, row by row, in seconds from the start of the clip.

Argument: the clip's id, as a JSON text (`'"greeting"'`), and `--layer`. The answer has:

- `clip`, `length` (seconds), `loop` and `model`.
- `rows`: one per channel with keys: `kind` (`bone`, `shape` or `order`), `name` (the bone, shape or part), `channel` (`rotation`, `position`, `scale`, `angles` or `shift` on bones; empty on shapes and orders) and `keys`, each with `t` (second), `value` and `ease` (the curve up to the next key, `"linear"` if there is no other).
- `events`: the clip's events.
- `shapes`, `defaults` and `parts`: what the layer offers to animate.

If the layer or the clip does not exist, the answer is `null`.

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"arm","parent":"root","at":[0.5,0.5]}'
reiwand-ctl scene act my-scene clip '{"layer":"mesh","id":"greeting","add":true,"length":2}'
reiwand-ctl scene act my-scene key '{"layer":"mesh","clip":"greeting","time":1,"bone":"arm","rotation":40}'
reiwand-ctl scene ask my-scene keys '"greeting"' --layer mesh
```

#### `pose`

Gives a mesh's pose at a second of a clip: where each bone ends up. You can try a key without saving it.

Argument: an object, and `--layer`.

| Field | Type | What it means |
|---|---|---|
| `clip` | text (required) | Id of the clip. |
| `time` | number | Second, from the start of the clip. Default 0. It is adjusted to the duration according to the clip's `loop`. |
| `bone` | text | A bone that gets a key at that second, only for this query. |
| `rotation` | number | Rotation of that key, in degrees. |
| `position` | `[x, y]` | Shift of that key, in layer pixels. |
| `scale` | `[x, y]` | Size factor of that key. |

The answer has `time` and `length` (seconds), `size` (the layer's size in pixels) and `bones`: each bone with `at` (its head), `angle` (where it points, in degrees), `rotation`, `position` and `scale` (its channels at that second) and `frame`, its parent's space (four numbers).

The query is for meshes; on a model it gives no usable pose.

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"arm","parent":"root","at":[0.5,0.5]}'
reiwand-ctl scene act my-scene clip '{"layer":"mesh","id":"greeting","add":true,"length":2}'
reiwand-ctl scene act my-scene key '{"layer":"mesh","clip":"greeting","time":1,"bone":"arm","rotation":40}'
reiwand-ctl scene ask my-scene pose '{"clip":"greeting","time":1}' --layer mesh
```

Try a key without saving it:

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"arm","parent":"root","at":[0.5,0.5]}'
reiwand-ctl scene act my-scene clip '{"layer":"mesh","id":"greeting","add":true,"length":2}'
reiwand-ctl scene ask my-scene pose '{"clip":"greeting","time":1,"bone":"arm","rotation":20,"position":[0,10]}' --layer mesh
```

#### `weights`

Gives a bone's weight map over the mesh, as two images. It is what the app paints in **Paint weights on the preview**.

Argument: the bone's name, as a JSON text (`'"arm"'`), and `--layer`. The answer has:

- `bone`: the requested bone.
- `heat`: a PNG image, as a `data:image/png;base64,…` address, with the bone's weight on each triangle: blue for no weight, red for all the weight.
- `owner`: another PNG image with the color of the bone that drives each triangle the most.
- `bones` and `colors`: the bones and the color of each one in `owner`.
- `size`: size of the images in pixels (up to 384 per side).
- `auto`: `true` if the weights are automatic.

An empty bone name gives a `heat` with no weight.

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"arm","parent":"root","at":[0.5,0.5]}'
reiwand-ctl scene ask my-scene weights '"arm"' --layer mesh
```

#### `handles`

Gives the places the app draws to edit inverse kinematics chains and bodies with physics: where their bones, their ends and their targets are.

No arguments (`'{}'`) and with `--layer`. The answer has `size` (the layer's size in pixels) and two lists:

- `ik`: one per chain, with `chain` (the bones), `at` (each one's head), `end` (the end of the last one), `target` (the target, if it is a point or a bone; `null` if it is the pointer, another layer or a binding), `follows` (`point`, `bone`, `layer`, `cursor` or `bound`), `bend` and `limits` (per bone: `bone`, `at`, `rest` the rest angle in degrees, `min` and `max`).
- `bodies`: one per bone with physics, with `bone`, `mode`, `gravity`, `at` (its head), `tip` (its tip), `rest` (the rest angle in degrees), `rotation` and `translation` (the bounds, or `null`).

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"arm","parent":"root","at":[0.5,0.5]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"hand","parent":"arm","at":[0.5,0.2]}'
reiwand-ctl scene act my-scene ik '{"layer":"mesh","root":"root","tip":"hand","target":[128,20]}'
reiwand-ctl scene act my-scene physics '{"layer":"mesh","bones":["hand"],"gravity":1}'
reiwand-ctl scene ask my-scene handles '{}' --layer mesh
```

### Scripts, wind and pointer

#### `script_calls`

Gives, for a mesh or model layer, the names a script can move and the line of code that does it.

Argument: `{"layer": "ID"}` and, to ask for a call's code, also `kind`, `name` and `what`. Without them, the answer lists:

- `bones`: bones and points of the layer.
- `shapes`: blend shapes and expressions.
- `bodies`: bones with physics.
- `calls`: the calls that exist by kind: `bone` (`turn`, `move`, `scale`, `read`, `reset`), `shape` (`set`, `read`) and `impulse` (`push`).
- `uses`: what the script must declare to use them (`animations`).

With `kind` (`bone`, `shape` or `impulse`), `name` (a name of that kind that the layer has; for `impulse`, a bone with physics) and `what` (one of the calls above), the answer brings `code`, a script line ready to paste. A name the layer does not have, or a call that does not exist, gives an error.

For what these calls are for in a script, see [scripts](../scripts.md).

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"hand","parent":"root","at":[0.5,0.2]}'
reiwand-ctl scene act my-scene physics '{"layer":"mesh","bones":["hand"]}'
reiwand-ctl scene ask my-scene script_calls '{"layer":"mesh"}'
```

Ask for the code to rotate a bone and to give a bone with physics a push:

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"hand","parent":"root","at":[0.5,0.2]}'
reiwand-ctl scene act my-scene physics '{"layer":"mesh","bones":["hand"]}'
reiwand-ctl scene ask my-scene script_calls '{"layer":"mesh","kind":"bone","name":"hand","what":"turn"}'
reiwand-ctl scene ask my-scene script_calls '{"layer":"mesh","kind":"impulse","name":"hand","what":"push"}'
```

#### `wind`

Gives the scene's wind as an arrow.

No arguments (`'{}'`). The answer has `angle` (degrees clockwise from the right), `strength` (pixels per second), `gusts` (from 0 to 1) and `fixed`: `true` if the wind is a fixed value and `false` if it follows keys or a binding.

```sh
reiwand-ctl scene act my-scene wind '{"angle":90,"strength":200,"gusts":0.2}'
reiwand-ctl scene ask my-scene wind '{}'
```

#### `wander`

Gives where, at a given second, the made-up pointer is that travels over the preview when you press **Try the pointer** on a pointer binding. It is for knowing, without a preview, where it will pass.

Argument: `{"time": SECONDS}` (default 0). The answer has `x` and `y`, the pointer's place in fractions of the canvas, and `buttons`, which is always 0.

```sh
reiwand-ctl scene ask my-scene wander '{"time":1.5}'
```
