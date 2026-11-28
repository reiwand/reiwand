# Play and blend animations

## `animate`

Edits how a mesh or model layer is animated. It takes a single variant.

### variant `layer`

Adds, changes, reorders or removes an animation layer: each one plays a clip and they are blended in order. In the app, **Animation layers**: **Add a layer playing…**, and per layer **On**, **Adds**, **Weight**, **Speed**, **Fade in**, **Fade out** and **Only these bones**. You must create a clip first.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text (required) | Mesh or model layer. |
| `add` | text | Id of a clip: adds a new animation layer that plays it. The other fields in the same call (`weight`, `rate`…) apply to that new layer. |
| `index` | integer | Without `add`: place of the animation layer that is changed (0 is the first). |
| `clip` | text | Changes the clip it plays. |
| `remove` | yes/no | `true` removes the animation layer. |
| `weight` | number | How much it drives, from 0 to 1. Default 1. |
| `rate` | number | Playback speed, from 0 to 16. Default 1. |
| `additive` | yes/no | `true` adds the clip to what is underneath instead of replacing it. Default `false`. |
| `blend_in` | number | Seconds of fade-in, from 0 to 10. |
| `blend_out` | number | Seconds of fade-out, from 0 to 10. |
| `enabled` | yes/no | `false` turns the layer off without removing it. Default `true`. |
| `bones` | list of texts | Only these bones receive the clip. An empty list includes them all again. |
| `to` | integer | Moves the layer to that place in the list. |

A layer accepts up to 16 animation layers.

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene clip '{"layer":"mesh","id":"greeting","add":true}'
reiwand-ctl scene act my-scene animate '{"layer":{"layer":"mesh","add":"greeting","weight":0.8,"rate":1.5}}'
```

Change the layer that already exists, move and remove:

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"arm","parent":"root","at":[0.5,0.5]}'
reiwand-ctl scene act my-scene clip '{"layer":"mesh","id":"greeting","add":true}'
reiwand-ctl scene act my-scene clip '{"layer":"mesh","id":"other","add":true}'
reiwand-ctl scene act my-scene animate '{"layer":{"layer":"mesh","add":"greeting"}}'
reiwand-ctl scene act my-scene animate '{"layer":{"layer":"mesh","add":"other","additive":true}}'
reiwand-ctl scene act my-scene animate '{"layer":{"layer":"mesh","index":1,"bones":["arm"],"blend_in":0.3,"blend_out":0.3,"enabled":true}}'
reiwand-ctl scene act my-scene animate '{"layer":{"layer":"mesh","index":1,"clip":"greeting","to":0}}'
reiwand-ctl scene act my-scene animate '{"layer":{"layer":"mesh","index":0,"remove":true}}'
```

### variant `root`

Turns a model's root motion on or off: the displacement of its root bone moves the whole layer. Only for model layers, and to turn it on the layer needs a clip that moves the root.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text (required) | Model layer. |
| `on` | yes/no (required) | `true` turns it on, `false` turns it off. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"model","file":"model.glb","id":"doll"}'
reiwand-ctl scene act my-scene clip '{"layer":"doll","id":"step","add":true,"animation":"walk"}'
reiwand-ctl scene act my-scene animate '{"root":{"layer":"doll","on":true}}'
```

### variant `preset`

Gives a bone the physics of a ready-made mode. It is what the **Hair**, **Tail**, **Cloth**, **Antenna**, **Chain** and **Jelly** buttons of **Swinging bones** do. If the bone already had physics, it replaces it.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text (required) | Mesh or model layer. |
| `bone` | text (required) | Bone that swings. |
| `preset` | text (required) | `hair`, `tail`, `cloth`, `antenna`, `chain` or `jelly`. |

| Mode | Type | Stiffness | Damping | Inertia | Gravity | Wind |
|---|---|---|---|---|---|---|
| `hair` | `rope` | 0.2 | 0.3 | 0.7 | 1.0 | 0.6 |
| `tail` | `rope` | 0.4 | 0.4 | 0.5 | 0.5 | 0.3 |
| `cloth` | `spring` | 0.3 | 0.5 | 0.6 | 2.0 | 0.8 |
| `antenna` | `spring` | 0.7 | 0.2 | 0.4 | 0.0 | 0.2 |
| `chain` | `rigid` | 0.1 | 0.2 | 0.8 | 3.0 | 0.1 |
| `jelly` | `spring` | 0.5 | 0.1 | 0.9 | 0.0 | 0.0 |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"model","file":"model.glb","id":"doll"}'
reiwand-ctl scene act my-scene animate '{"preset":{"layer":"doll","bone":"tip","preset":"hair"}}'
```

### variant `morph`

Sets the weight of a blend shape or expression of the layer, without a clip. In the app, the sliders of **Blend shapes in the preview** and **Expressions in the preview** only change the preview; this action is the one that changes the scene.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text (required) | Mesh or model layer. |
| `shape` | text (required) | Blend shape or expression. |
| `weight` | number | Weight, from -1 to 1. If missing, the fixed weight is removed. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"model","file":"model.glb","id":"doll"}'
reiwand-ctl scene act my-scene animate '{"morph":{"layer":"doll","shape":"bulge","weight":0.6}}'
reiwand-ctl scene act my-scene animate '{"morph":{"layer":"doll","shape":"bulge"}}'
```
