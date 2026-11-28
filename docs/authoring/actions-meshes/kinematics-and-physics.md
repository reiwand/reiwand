# Kinematics and physics

## `ik`

Adds, changes or removes an inverse kinematics chain: bones that stretch and bend by themselves so that the last one reaches a target. It is for making an arm reach a point, a bone or the pointer. In the app, **Chains that reach** and **Make a chain on the preview** (you click the bone where it starts and then the bone where it ends). Each chain has **Bend the other way**, **Weight**, **Limit** and **Free**; on the preview you drag the ring of a target that is a point and the points of the limits.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text (required) | Mesh or model layer. |
| `index` | integer | Place of the chain that is changed or removed (0 is the first). Without it, a new chain is added. |
| `root` | text | Bone where the chain starts. Goes together with `tip`. |
| `tip` | text | Bone where it ends: it must hang below `root`. The chain has 2 to 8 bones. |
| `target` | text or `[x, y]` | Target: `"cursor"` (the pointer), `"bone:NAME"`, `"layer:ID"` or a point `[x, y]` in layer pixels, from its top-left corner. A new chain without `target` aims at the place where its end is at rest, and does not move. |
| `limits` | object | Rotation limits per bone, `{"arm": [-90, 90]}`: minimum and maximum in degrees from rest, from -180 to 180. The bone must be in the chain. |
| `unlimit` | list of texts | Bones whose limit is removed. |
| `weight` | number | How much the chain drives, from 0 to 1. Default 1. |
| `bend` | text | `clockwise` (default) or `counterclockwise`: which way it bends. |
| `remove` | yes/no | With `index`: removes the chain. |

A layer accepts up to 16 chains. A chain that follows the pointer needs the pointer permission, and the scene warns the user that it follows it.

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"arm","parent":"root","at":[0.5,0.5]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"hand","parent":"arm","at":[0.5,0.2]}'
reiwand-ctl scene act my-scene ik '{"layer":"mesh","root":"root","tip":"hand","target":[128,20]}'
```

Change the chain: limits, which way it bends and weight; then make it follow the pointer, then a bone, and remove it:

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"arm","parent":"root","at":[0.5,0.5]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"hand","parent":"arm","at":[0.5,0.2]}'
reiwand-ctl scene act my-scene ik '{"layer":"mesh","root":"root","tip":"hand"}'
reiwand-ctl scene act my-scene ik '{"layer":"mesh","index":0,"limits":{"arm":[-90,90]},"bend":"counterclockwise","weight":0.8}'
reiwand-ctl scene act my-scene ik '{"layer":"mesh","index":0,"target":"cursor"}'
reiwand-ctl scene act my-scene ik '{"layer":"mesh","index":0,"target":"bone:root","unlimit":["arm"]}'
reiwand-ctl scene act my-scene ik '{"layer":"mesh","index":0,"remove":true}'
```

The chains and their targets are listed by the `handles` query.

## `physics`

Makes bones swing with physics, or changes or removes that physics. It is for hair, tails, cloth or antennas. In the app, on a mesh **Bones that swing**: **Pick bones that swing on the preview**, **Draw a rope on the preview**, **Simulate** and **Reset**; on a model, **Swinging bones** with ready-made modes (**Hair**, **Tail**, **Cloth**, **Antenna**, **Chain**, **Jelly**, which are the `preset` variant of `animate`).

| Argument | Type | What it means |
|---|---|---|
| `layer` | text (required) | Mesh or model layer. |
| `bones` | list of texts (required) | Bones that swing. Each one that has no physics gets it, and the others get the values you give changed. Up to 64 per layer. |
| `mode` | text | `spring` (the value of a new bone), `rigid` or `rope`. |
| `stiffness` | number | How much the bone returns to its pose, from 0 to 1. Default 0.5. |
| `damping` | number | How much the swing is slowed, from 0 to 1. Default 0.5. |
| `inertia` | number | How much it lags behind when its parent moves, from 0 to 1. Default 0.5. |
| `gravity` | number | Vertical pull, from -10 to 10. A positive value pulls it down. Default 0. |
| `wind` | number | How much the wind affects it (see `wind`), from 0 to 1. Default 0. |
| `rotation` | `[min, max]` | Bounds the rotation, in degrees within ±180. |
| `translation` | `[min, max]` | Bounds how far its head slides, in pixels. |
| `free` | list of texts | Bounds that are removed: `"rotation"` or `"translation"`. |
| `remove` | yes/no | `true` removes the physics from those bones. |
| `rope` | list of `[x, y]` | From 2 to 9 points, in fractions of the layer, that draw a rope. It asks for a single name in `bones`: the bones `NAME_1`, `NAME_2`… are created along the points, each hanging from the previous one, and the first swings like a rope. Meshes only. |
| `parent` | text | With `rope`: bone the first one of the rope hangs from. |

A rope is removed by naming its first bone (`NAME_1`).

Models with colliders keep the swinging bones out of them (see `collider`). The swing stops at the lowest quality level, where the bones keep their animation.

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"hand","parent":"root","at":[0.5,0.2]}'
reiwand-ctl scene act my-scene physics '{"layer":"mesh","bones":["hand"],"mode":"spring","stiffness":0.4,"gravity":2}'
```

Bound the rotation, release the bound and remove the physics:

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"hand","parent":"root","at":[0.5,0.2]}'
reiwand-ctl scene act my-scene physics '{"layer":"mesh","bones":["hand"],"mode":"rigid","rotation":[-30,30],"translation":[-5,5],"inertia":0.5,"damping":0.2,"wind":0.3}'
reiwand-ctl scene act my-scene physics '{"layer":"mesh","bones":["hand"],"free":["rotation"]}'
reiwand-ctl scene act my-scene physics '{"layer":"mesh","bones":["hand"],"remove":true}'
```

A rope hanging from the root:

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene physics '{"layer":"mesh","bones":["tail"],"rope":[[0.5,0.9],[0.4,0.95],[0.3,1.0]],"parent":"root"}'
```

## `collider`

Adds, changes or removes a capsule or a sphere that keeps a model's or mesh's swinging bones out. It is for keeping hair from going through the head. In the app, **Colliders**: **Pick a bone to add a collider**, **Capsule along the bone** or **Sphere at its head**, and **Remove**. On the preview you drag the width.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text (required) | Model or mesh layer. |
| `index` | integer | Place of the collider that is changed or removed (0 is the first). Without it, a new one is added. |
| `bone` | text | Bone it is bound to. Required when adding. |
| `shape` | text | `capsule` (along the bone) or `sphere` (at its head). |
| `radius` | number | Radius, from 0 to 100: in model units, or in pixels on a mesh. A new collider starts at 0.1 on a model and at 8 on a mesh. |
| `offset` | `[x, y, z]` | Offset in the bone's space. Three zeros leave it with no offset. |
| `remove` | yes/no | With `index`: removes the collider. |

A model accepts up to 8 colliders. The swinging bones are kept out of the colliders: of all of them at the two highest quality levels, of the first two at the second-lowest and of none at the lowest, where the bones keep their animation.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"model","file":"model.glb","id":"doll"}'
reiwand-ctl scene act my-scene collider '{"layer":"doll","bone":"tip","shape":"capsule","radius":0.2}'
```

Change and remove:

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"model","file":"model.glb","id":"doll"}'
reiwand-ctl scene act my-scene collider '{"layer":"doll","bone":"tip","radius":0.2}'
reiwand-ctl scene act my-scene collider '{"layer":"doll","index":0,"shape":"sphere","offset":[0,0.1,0]}'
reiwand-ctl scene act my-scene collider '{"layer":"doll","index":0,"remove":true}'
```

## `bone_rule`

Adds, changes or removes a rule that hands a bone to another parent as its weight rises from 0 to 1, keeping its pose. It is useful, for example, for a hand to let go of the body and follow an object. In the app, **Rules that hand a bone to another parent**: **Bone**, **New parent**, **Weight**, **Add a rule** and **Remove**.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text (required) | Mesh layer. |
| `index` | integer | Place of the rule that is changed or removed (0 is the first). Without it, a new rule is added. |
| `bone` | text | Bone that changes parent. Required when adding. |
| `parent` | text | Parent it moves to. It must be another bone and not close a loop. Required when adding. |
| `weight` | number | From 0 (hangs from its usual parent) to 1 (hangs from the new one). Default 1. |
| `remove` | yes/no | With `index`: removes the rule. |

Up to 64 rules per layer. The action writes a fixed weight; in the scene that weight can be animated like other values (see [properties](../properties.md)).

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"arm","parent":"root","at":[0.5,0.5]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"hand","parent":"arm","at":[0.5,0.2]}'
reiwand-ctl scene act my-scene bone_rule '{"layer":"mesh","bone":"hand","parent":"root","weight":0.5}'
```

Change the weight and remove:

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"arm","parent":"root","at":[0.5,0.5]}'
reiwand-ctl scene act my-scene bone_rule '{"layer":"mesh","bone":"arm","parent":"root"}'
reiwand-ctl scene act my-scene bone_rule '{"layer":"mesh","index":0,"weight":0.2}'
reiwand-ctl scene act my-scene bone_rule '{"layer":"mesh","index":0,"remove":true}'
```

## `wind`

Sets the scene's wind, which moves bones with physics and particles. There is only one for the whole scene. In the app, **Wind**: **Angle**, **Strength**, **Gusts**, **Show on the preview** (an arrow from the center of the canvas whose tip you drag) and **No wind**.

| Argument | Type | What it means |
|---|---|---|
| `angle` | number | Which way it blows, in degrees clockwise from the right: 0 to the right, 90 downward. |
| `strength` | number | Strength in pixels per second, from 0 to 10000. |
| `gusts` | number | How much it varies with gusts, from 0 to 1. |
| `off` | yes/no | `true` removes the wind and the gusts. Default `false`. |

You have to give at least `angle`, `strength` or `gusts` (or `off`). What you do not give stays as it was. A wind that followed keys or a binding becomes fixed.

```sh
reiwand-ctl scene act my-scene wind '{"angle":0,"strength":300,"gusts":0.5}'
```

Change only the direction and remove the wind:

```sh
reiwand-ctl scene act my-scene wind '{"angle":90}'
reiwand-ctl scene act my-scene wind '{"off":true}'
```

The current wind is given by the `wind` query.
