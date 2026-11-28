# Parts and attachments

## `attach`

Attaches a layer to a bone or to a point of a mesh or model layer, so that it follows it. It is for putting a hat or an object in a hand. In the app, **Attach layers on the preview**: you drag a layer onto a bone, or onto the mesh for a new point there; in **Layers that follow this mesh** you can release them, and each one's ring is dragged to move it.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text (required) | The layer that follows. It can be of any kind. |
| `to` | text | Mesh or model layer that is followed. If missing, the one it already follows. A layer cannot follow itself, nor through others. |
| `bone` | text | Bone it follows. It cannot be combined with `point`. |
| `point` | text or `[x, y]` | A point of the followed layer, by name, or an `[x, y]` in fractions of the layer to create a new point there (meshes only). The new point is called `ID_point` and hangs from the bone that drives the nearest vertex. |
| `offset` | `[x, y]` | Offset in pixels, in the bone's or point's space. |
| `nudge` | `[x, y]` | Moves the layer those pixels of the mesh layer, on screen; the action turns it into `offset`. |
| `inherit` | list of texts | What it inherits from the bone: `position`, `rotation`, `scale`. If missing, it inherits all three. |
| `detach` | yes/no | `true` releases the layer. |

If you follow the same layer and do not give `bone`, `point`, `offset` or `inherit`, the ones it had are kept. A point marked as an attachment that nobody else uses is deleted when the layer is released or follows something else.

First a mesh with a bone and an image layer:

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"hand","parent":"root","at":[0.5,0.2]}'
reiwand-ctl scene act my-scene add_layer '{"kind":"image","source":"picture","id":"cap"}'
reiwand-ctl scene act my-scene attach '{"layer":"cap","to":"mesh","bone":"hand","offset":[5,5],"inherit":["position","rotation"]}'
```

Attach to a new point, nudge it and release it:

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene add_layer '{"kind":"image","source":"picture","id":"cap"}'
reiwand-ctl scene act my-scene attach '{"layer":"cap","to":"mesh","point":[0.5,0.5]}'
reiwand-ctl scene act my-scene attach '{"layer":"cap","nudge":[10,0]}'
reiwand-ctl scene act my-scene attach '{"layer":"cap","detach":true}'
```

Attach to a named point (see `point`):

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene point '{"layer":"mesh","name":"ring","at":[0.5,0.2],"parent":"root"}'
reiwand-ctl scene act my-scene add_layer '{"kind":"image","source":"picture","id":"cap"}'
reiwand-ctl scene act my-scene attach '{"layer":"cap","to":"mesh","point":"ring"}'
```

## `point`

Adds, moves, changes the bone of, renames or removes a named point of a mesh, a place to attach layers that is not a bone. In the app, **Attachment points**: **Add a point**, the name is edited in the list, the menu of the bone it hangs from (or **No bone**) and **Remove the point**; on the preview each point is a square that you drag.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text (required) | Mesh layer. |
| `name` | text (required) | Name of the point, like a bone's. If it does not exist, it is created. |
| `at` | `[x, y]` | Place in fractions of the layer. Required on a new point. |
| `parent` | text | Bone it hangs from. Empty text leaves it without a parent. A new point without `parent` hangs from the bone that drives that area of the mesh. |
| `to` | text | New name. The layers attached to it and the chains that look for it follow it. |
| `remove` | yes/no | `true` removes the point and the attachments and chains that named it. |

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene point '{"layer":"mesh","name":"ring","at":[0.5,0.2],"parent":"root"}'
```

Move, rename and remove:

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene bone '{"layer":"mesh","name":"root","at":[0.5,0.9]}'
reiwand-ctl scene act my-scene point '{"layer":"mesh","name":"ring","at":[0.5,0.2]}'
reiwand-ctl scene act my-scene point '{"layer":"mesh","name":"ring","at":[0.4,0.3],"parent":"root"}'
reiwand-ctl scene act my-scene point '{"layer":"mesh","name":"ring","to":"finger"}'
reiwand-ctl scene act my-scene point '{"layer":"mesh","name":"finger","remove":true}'
```

## `parts`

Orders, hides or shows a mesh's parts and turns on or adjusts its depth. A mesh from a file can have several parts; one that comes from an image has one, `body`. In the app, **Parts, from back to front**: you drag a part's handle to change its place, the **Shown** checkbox hides or shows it, and the **Over everything** menu chooses over which part it is the only one seen. The depth is in the same inspector: **Depth: nearer vertices cover farther ones**, **Perspective of the depth** and **Hide faces seen from behind**.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text (required) | Mesh or model layer. |
| `order` | object | Draw order per part, `{"body": 3}`: from -1000 to 1000, rounded. A higher value is drawn in front. |
| `hide` | list of texts | Parts that are hidden. |
| `show` | list of texts | Parts that are shown again. A part cannot be in `hide` and in `show`. |
| `clip` | object | `{"part": "other"}`: the part is only seen over `other`. Never over itself, nor through others. |
| `unclip` | list of texts | Parts that have that restriction removed. |
| `depth` | object | Depth of the mesh. With `scale` (from 0 to 10), `cull` (yes/no: hides faces seen from behind) and `sort` (yes/no: draws the parts from back to front; it is the normal behavior, so `true` deletes the field), it turns the depth on and changes those fields. `{"off": true}` turns it off and cannot be combined with the others. |

You have to give at least something to change. The parts must exist (`rig` lists them).

`clip` and `unclip` ask for a mesh with more than one part, like one from a `.glb` file with several; a mesh made from an image only has `body`. The sample file `pieces.glb` has two, `chest` and `legs`.

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene parts '{"layer":"mesh","order":{"body":3}}'
```

Make a part only visible over another, and remove that restriction:

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"mesh","file":"pieces.glb","id":"puppet"}'
reiwand-ctl scene act my-scene parts '{"layer":"puppet","clip":{"chest":"legs"}}'
reiwand-ctl scene act my-scene parts '{"layer":"puppet","unclip":["chest"]}'
```

Hide and show:

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene parts '{"layer":"mesh","hide":["body"]}'
reiwand-ctl scene act my-scene parts '{"layer":"mesh","show":["body"]}'
```

Turn depth on, adjust it and turn it off:

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","id":"mesh"}'
reiwand-ctl scene act my-scene parts '{"layer":"mesh","depth":{"scale":2,"cull":true}}'
reiwand-ctl scene act my-scene parts '{"layer":"mesh","depth":{"sort":false}}'
reiwand-ctl scene act my-scene parts '{"layer":"mesh","depth":{"off":true}}'
```
