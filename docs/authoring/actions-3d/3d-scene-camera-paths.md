# Camera paths and dragging handles

On this page: 3D scene: camera paths, 3D scene: dragging handles.

## 3D scene: camera paths

A path is a route the camera follows: a list of poses (called keys) with their time in seconds, and the camera moves smoothly from one to the next. A scene has at most 16 paths and each has up to 256 keys, and always at least 2. Ids are 1 to 32 lowercase letters, digits or `_`. In the app it is the **Camera paths** panel.

### Variant path: create, edit and order paths

Six forms, each inside `path`.

#### path → add

Creates a new path with two keys, at second 0 and at second 2, both where the authoring view is looking now. It is **Add from the view**.

| Argument | Type | What it means |
|---|---|---|
| `id` | text (required) | Id of the new path. |

```sh
reiwand-ctl scene act my-scene scene3d '{"path":{"add":{"id":"flight"}}}'
```

#### path → key

Creates, changes, moves or removes a key.

| Argument | Type | What it means |
|---|---|---|
| `path` | text (required in practice) | Id of the path. |
| `time` | number | Second of the key, from 0 to 3600. Default 0. |
| `eye` | list of 3 numbers | Where the camera is at that key. |
| `center` | list of 3 numbers | Point it looks at. |
| `up` | list of 3 numbers | "Up" direction. |
| `fov` | number | Field of view, from 1 to 179. |
| `from_view` | yes/no | `true` takes the pose of the authoring view (a new key always takes it if you give nothing). Default `false`. |
| `from` | number | Moves the key that is at this second to second `time`. |
| `remove` | yes/no | `true` removes the key that is at `time`. The path does not go below 2 keys. Default `false`. |

If there is already a key at `time`, only the fields you give are changed.

```sh
reiwand-ctl scene act my-scene scene3d '{"path":{"add":{"id":"flight"}}}'
reiwand-ctl scene act my-scene scene3d '{"path":{"key":{"path":"flight","time":1,"eye":[3,1,4],"center":[0,0,0],"fov":50}}}'
reiwand-ctl scene act my-scene scene3d '{"path":{"key":{"path":"flight","time":3,"from":1}}}'
reiwand-ctl scene act my-scene scene3d '{"path":{"key":{"path":"flight","time":3,"remove":true}}}'
```

#### path → set

Changes the loop, the easing or the name of a path.

| Argument | Type | What it means |
|---|---|---|
| `path` | text (required) | Id of the path. |
| `loop` | yes/no | `true` makes it go back to the start and repeat (**Loop**). |
| `ease` | text | Smoothness of the movement: `linear`, `step`, `hold`, or `in_`, `out_` or `in_out_` followed by `sine`, `quad`, `cubic`, `quart`, `quint`, `expo`, `circ`, `back`, `elastic` or `bounce` (for example `in_out_sine`). |
| `to` | text | New id, not already in use. |

```sh
reiwand-ctl scene act my-scene scene3d '{"path":{"add":{"id":"flight"}}}'
reiwand-ctl scene act my-scene scene3d '{"path":{"set":{"path":"flight","loop":true,"ease":"in_out_sine","to":"tour"}}}'
```

#### path → remove

Deletes a path. If it was the last one, the camera's list of paths disappears too.

| Argument | Type | What it means |
|---|---|---|
| `path` | text (required) | Id of the path. |

```sh
reiwand-ctl scene act my-scene scene3d '{"path":{"add":{"id":"flight"}}}'
reiwand-ctl scene act my-scene scene3d '{"path":{"remove":{"path":"flight"}}}'
```

#### path → order

Chooses the order in which several paths play: **In order** or **At random**.

| Argument | Type | What it means |
|---|---|---|
| `order` | text (required) | `sequential` (in order) or `random` (at random). |

```sh
reiwand-ctl scene act my-scene scene3d '{"path":{"order":{"order":"random"}}}'
```

#### path → fade

Chooses how to go from one path to the next: `black` (**Through black**) or `cross` (**Crossing**), with its duration.

| Argument | Type | What it means |
|---|---|---|
| `to` | text | `black` or `cross`. |
| `duration` | number | Seconds from 0 to 10. If you do not give it, the previous one stays (1 at the start). |
| `clear` | yes/no | `true` erases the fade (a hard cut). Default `false`. |

```sh
reiwand-ctl scene act my-scene scene3d '{"path":{"fade":{"to":"black","duration":2}}}'
reiwand-ctl scene act my-scene scene3d '{"path":{"fade":{"clear":true}}}'
```

## 3D scene: dragging handles

### Variant grab: drag a handle

Does the same as dragging a handle of the authoring view with the mouse: an arrow, a ring or a point of a model, of the camera, of a path key or of a light. You give the point where the drag started and the point where it ended (`from` and `to`, in canvas pixels); the program works out what changes. The handle positions come from the [`stage3d`](queries.md#stage3d) query. If the field being moved has keyframes, the drag puts a key at second `time`; if it is bound to a property, it refuses unless you set `replace: true`.

| Argument | Type | What it means |
|---|---|---|
| `what` | object or text (required) | What is dragged. See below. |
| `handle` | text (required) | The handle: `x`, `y`, `z`, `turn_x`, `turn_y`, `turn_z`, `plane`, `depth`, `orbit`, `dolly` or `roll`. |
| `from` | list of 2 numbers (required) | Where the drag started, in canvas pixels. |
| `to` | list of 2 numbers (required) | Where it ended. |
| `time` | number | Second of the scene at which it is done (for looking and for placing the key). Default 0. |
| `replace` | yes/no | `true` replaces a binding with the new value. Default `false`. |

The values of `what`:

| `what` | What it moves |
|---|---|
| `{"model":{"layer":"ID"}}` | The position or rotation of a model layer. |
| `"eye"` | The position of the scene's camera. |
| `"center"` | The point the scene's camera looks at. |
| `{"key":{"path":"ID","key":SECOND}}` | The camera of the path key that is at that second. With `"aim":true` it moves the point it looks at instead of the camera. |
| `{"light":{"layer":"ID"}}` | The 3D position of a light. With `"aim":true` it moves the point it aims at. The light has to be lighting a 3D view. |

The values of `handle`:

| `handle` | What it does |
|---|---|
| `x`, `y`, `z` | Moves the point along that direction (the handle's arrow). |
| `plane` | Slides it in the screen plane. |
| `depth` | Brings it closer to or farther from the viewer. |
| `turn_x`, `turn_y`, `turn_z` | Rotates the model around that direction (the rings). Only valid with `model`. |
| `orbit` | Rotates the camera around the other end (around `center` if you drag `eye`, and the reverse). Valid with `eye`, `center` and `key`. |
| `dolly` | Moves the camera closer or farther without changing where it looks. Same cases as `orbit`. |
| `roll` | Tilts the camera around its line of sight. Only valid with `eye` and `center`. |

A combination that makes no sense (for example `turn_x` with `eye`) returns an error.

```sh
reiwand-ctl scene act my-scene import '{"file":"figure.glb"}'
reiwand-ctl scene act my-scene scene3d '{"model":{"add":{"asset":"figure","id":"cube"}}}'
reiwand-ctl scene act my-scene scene3d '{"grab":{"what":{"model":{"layer":"cube"}},"handle":"x","from":[1030,540],"to":[1100,540]}}'
reiwand-ctl scene act my-scene scene3d '{"grab":{"what":{"model":{"layer":"cube"}},"handle":"turn_y","from":[1023,540],"to":[1050,560]}}'
reiwand-ctl scene act my-scene scene3d '{"grab":{"what":{"model":{"layer":"cube"}},"handle":"plane","from":[960,540],"to":[1000,500]}}'
reiwand-ctl scene act my-scene scene3d '{"grab":{"what":{"model":{"layer":"cube"}},"handle":"depth","from":[908,592],"to":[900,600]}}'
```

The camera is dragged with the free view on (if you look through the camera its handles are not shown):

```sh
reiwand-ctl scene act my-scene import '{"file":"figure.glb"}'
reiwand-ctl scene act my-scene scene3d '{"model":{"add":{"asset":"figure","id":"cube"}}}'
reiwand-ctl scene act my-scene scene3d '{"view":{"orbit":[30,10]}}'
reiwand-ctl scene act my-scene scene3d '{"grab":{"what":"eye","handle":"orbit","from":[960,540],"to":[1000,540]}}'
reiwand-ctl scene act my-scene scene3d '{"grab":{"what":"center","handle":"plane","from":[960,540],"to":[1000,540]}}'
reiwand-ctl scene act my-scene scene3d '{"grab":{"what":"eye","handle":"dolly","from":[960,540],"to":[960,500]}}'
reiwand-ctl scene act my-scene scene3d '{"grab":{"what":"eye","handle":"roll","from":[1000,540],"to":[1000,600]}}'
```

A path key and a light:

```sh
reiwand-ctl scene act my-scene import '{"file":"figure.glb"}'
reiwand-ctl scene act my-scene scene3d '{"model":{"add":{"asset":"figure","id":"cube"}}}'
reiwand-ctl scene act my-scene scene3d '{"path":{"add":{"id":"flight"}}}'
reiwand-ctl scene act my-scene scene3d '{"grab":{"what":{"key":{"path":"flight","key":2,"aim":true}},"handle":"plane","from":[960,540],"to":[1000,500]}}'
reiwand-ctl scene act my-scene add_layer '{"kind":"light","id":"lamp"}'
reiwand-ctl scene act my-scene scene3d '{"grab":{"what":{"light":{"layer":"lamp"}},"handle":"plane","from":[960,540],"to":[1000,500]}}'
reiwand-ctl scene act my-scene scene3d '{"grab":{"what":{"light":{"layer":"lamp","aim":true}},"handle":"x","from":[960,540],"to":[1000,540]}}'
```
