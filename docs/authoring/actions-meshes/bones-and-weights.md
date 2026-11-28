# Bones and weights

**Weights** say how much each bone drives each vertex. Each vertex is shared among up to 4 bones and its weights add up to 1.

While weights are automatic, they follow the bones: when you add, move or remove a bone, the mesh is bound again by itself. When you paint weights with `weights`, they become painted and are kept even if the bones change. `auto_weights` turns them back to automatic. A mesh freshly cut from an image starts with automatic weights.

## `bone`

Adds, moves, reparents, renames or removes a bone of a mesh. In the app, the **Bones** tree in the layer inspector and **Place bones on the preview**, which drags each bone's head.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text (required) | Mesh layer. |
| `name` | text (required) | Name of the bone, 1 to 64 characters without `/` or `#`. If it does not exist, it is created. It cannot repeat the name of another bone, point or part of the mesh. |
| `parent` | text | Bone it hangs from. Empty text leaves it without a parent. If you omit it on an existing bone, it does not change. |
| `at` | `[x, y]` | Where its head goes, in fractions of the layer. A new bone without `at` goes to the center. The children of the moved bone keep their place. |
| `to` | text | New name. Clips, animation layers, chains, bodies with physics, rules, bones that receive the pointer and layers attached to that bone follow it. |
| `remove` | yes/no | `true` removes the bone: its children hang from its parent and the parent receives its weights. With it, the keys, chains, bodies and rules that named it are removed too. A bone that carries a part of the mesh cannot be removed. |

A mesh accepts up to 128 bones. A bone cannot hang from itself or from its children.

Create three chained bones:

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"arm","parent":"root","at":[0.5,0.5]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"hand","parent":"arm","at":[0.5,0.2]}'
```

Move, rename, reparent and remove:

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"arm","parent":"root","at":[0.5,0.5]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"arm","at":[0.6,0.5]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"arm","to":"forearm"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"forearm","parent":""}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"forearm","remove":true}'
```

To see a layer's bones, use the `rig` query.

## `auto_weights`

Binds each vertex again to its nearest bones and makes the weights follow the bones until you paint them again. It discards painted weights. In the app, **Auto weights**.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text (required) | Mesh layer. It needs at least one bone. |

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"arm","parent":"root","at":[0.5,0.5]}'
reiwand-ctl scene act my-scene auto_weights '{"layer":"mesh"}'
```

## `weights`

Paints a bone's weights with a brush stroke. In the app, **Paint weights on the preview**: you choose the **Bone**, the **Brush** (**Add**, **Subtract**, **Smooth**, **Normalise**), the **Strength**, the **Brush size**, the **Soft edge** and the **Locked bones**. **Show** switches between the **Weight of the bone** (blue for no weight, red for all the weight) and the **Bone that leads each part**.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text (required) | Mesh layer. |
| `bone` | text (required) | Bone that is painted. |
| `points` | list of `[x, y]` (required) | The stroke, from 1 to 1024 points, in fractions of the layer. |
| `mode` | text (required) | `add` adds weight to the bone. `subtract` takes it away. `smooth` brings each vertex's weight closer to its neighbors'. `normalise` removes the weights smaller than `by`. |
| `by` | number (required) | Strength at the center of the brush, from 0 to 1. |
| `radius` | number | Brush size as a fraction of the longer side, from 0 (exclusive) to 1. Default 0.1. |
| `falloff` | number | How much of the brush is soft edge, from 0 (hard) to 1 (all soft). Default 0.5. |
| `lock` | list of texts | Bones whose weight the stroke does not touch. None by default. The bone you are painting cannot be among them. |

If a vertex already has 4 bones, the stroke makes room by taking weight from the others. The weight the painted bone gives up goes to its parent or to another bone that holds the vertex.

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"arm","parent":"root","at":[0.5,0.5]}'
reiwand-ctl scene act my-scene weights '{"layer":"mesh","bone":"arm","points":[[0.5,0.4],[0.5,0.6]],"mode":"add","by":0.8}'
```

Smooth, protecting the root, with a larger brush:

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"arm","parent":"root","at":[0.5,0.5]}'
reiwand-ctl scene act my-scene weights '{"layer":"mesh","bone":"arm","points":[[0.5,0.5]],"mode":"smooth","by":0.5,"radius":0.3,"falloff":0.8,"lock":["root"]}'
```

The heat map of the result is given by the `weights` query (the query, not this action).
