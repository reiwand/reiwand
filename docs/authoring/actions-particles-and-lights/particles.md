# Particles

On this page: Particles.

## Actions

## Particles

**Before you start.** A particle system has an id (for example `sparks`) and lives in the scene; a particle layer draws it. When you add a particle layer, `add_layer` with `"kind":"particles"` creates the layer and a system with the same id. All the variants of `particles` take `system`, the system's id, except `scripts`. Emitters, forces, colliders and children are named by their position in their list, from 0. Control points are called `p0` to `p7`. The variants that change the drawing (`cutout`, `orient`, `shader`, `strand`, `texture`, `lit`, `refract`, `over`, `render`) accept `layer`: with a particle layer, the change applies only to that layer, which from then on draws with its own copy of the system's drawing; without `layer`, it applies to the system and to all the layers that draw it. Each change is rejected, without touching anything, if it leaves the scene with a problem it did not have.

### `particles`

Edits a particle system: how it emits, its control points, its forces, its colliders, its children, how they are drawn and how light affects them. Each call uses a single variant. In the app these are the particle system panels.

#### variant `create`

Creates a new system, empty or a copy of another. It creates no layer; to see it, add a particle layer that names it in `source`. With `parent`, it becomes a child of another system.

| Argument | Type | What it means |
|---|---|---|
| `system` | text | (required) Id of the new system; it cannot be in use. |
| `from` | text | System that is copied. With no value, it starts with a basic system. |
| `parent` | text | System it will be a child of. |
| `on` | text | With `parent`, when the child is born: `static` (default, on its own), `spawn` (with each particle that is born), `death` (when it dies) or `follow` (following it). |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene act my-scene particles '{"create":{"system":"smoke","from":"sparks"}}'
reiwand-ctl scene act my-scene particles '{"create":{"system":"trail","parent":"sparks","on":"death"}}'
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"smoke-layer","source":"smoke"}'
```

#### variant `delete`

Deletes a system. It is rejected while a layer or another system names it.

| Argument | Type | What it means |
|---|---|---|
| `system` | text | (required) Id of the system. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene act my-scene particles '{"create":{"system":"smoke"}}'
reiwand-ctl scene act my-scene particles '{"delete":{"system":"smoke"}}'
```

#### variant `rename`

Gives a system another id. The layers and systems that name it are updated; scripts that name it in quotes are not (use `scripts`).

| Argument | Type | What it means |
|---|---|---|
| `system` | text | (required) Current id. |
| `to` | text | (required) New id; it cannot be in use. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene act my-scene particles '{"rename":{"system":"sparks","to":"embers"}}'
```

#### variant `scripts`

After renaming a system, rewrites in the scene's scripts every text that is exactly the old id (in double, single or backtick quotes) to the new one. Text that only contains the name inside a longer text is not touched. The files change at once and Undo does not bring them back. It is rejected while a system with the old id exists.

| Argument | Type | What it means |
|---|---|---|
| `from` | text | (required) Old id. |
| `to` | text | (required) Id of a system that exists. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene act my-scene particles '{"rename":{"system":"sparks","to":"embers"}}'
reiwand-ctl scene act my-scene particles '{"scripts":{"from":"sparks","to":"embers"}}'
```

#### variant `kit`

Replaces the system with a starting point that already looks good: it fills in the emitters, the life, the forces and the colors, and leaves the system with prewarm so it looks full from the first frame. The system's children are kept. The starting points adapt to the canvas size.

| Argument | Type | What it means |
|---|---|---|
| `system` | text | (required) Id of the system. |
| `kit` | text | (required) `snow` (snow falling from top to bottom), `sparks` (sparks rising from the bottom), `rain` (fast, slanted rain) or `dust` (motes floating all over the canvas). |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene act my-scene particles '{"kit":{"system":"sparks","kit":"sparks"}}'
```

#### variant `settings`

General settings of the system. Only what you name changes.

| Argument | Type | What it means |
|---|---|---|
| `system` | text | (required) Id of the system. |
| `capacity` | integer | Maximum number of live particles at once, from 1 to 200000. |
| `seed` | integer | Random seed: the same seed gives the same particles. |
| `prewarm` | number | Seconds simulated before the system is shown, so it does not start empty (from 0 to 30). |
| `space` | text | `local` (the particles follow the layer) or `world` (the layer only places them when they are born). |
| `rate` | number | Particles per second of the emitters. |
| `emitter` | integer | With `rate`, a single emitter; with no value, all of them. |
| `replace` | yes/no | With `rate`, overwrites a rate that followed keys or another source. Default `false`. |
| `lock` | list of texts | Values that a layer cannot change on its own: `alpha`, `size`, `count`, `speed`, `life`, `time_scale`, `brightness`, `color`, `points`. With `["none"]` all of them are removed. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene act my-scene particles '{"settings":{"system":"sparks","capacity":3000,"seed":7,"prewarm":2,"space":"world","rate":80,"lock":["alpha","size"]}}'
reiwand-ctl scene act my-scene particles '{"settings":{"system":"sparks","lock":["none"]}}'
```

#### variant `shape`

Changes the shape of an emitter, that is, where the particles are born. With the number of the emitter after the last one, it adds a new emitter with 10 particles per second.

| Argument | Type | What it means |
|---|---|---|
| `system` | text | (required) Id of the system. |
| `emitter` | integer | (required) Position of the emitter. |
| `shape` | any JSON value | (required) The shape, in canvas pixels: `"edge"` (the border), `{"point":[x,y]}`, `{"box":{"from":[x0,y0],"to":[x1,y1]}}`, `{"line":[[x0,y0],[x1,y1]]}`, `{"circle":{"center":[x,y],"radius":r,"inner":r2}}` (a ring if `inner` is greater than 0), `{"sphere":{"center":[x,y],"depth":z,"radius":[rx,ry,rz],"inner":0,"sign":[0,0,0]}}` (inside an ellipsoid; all fields required but `push`, pixels that move every birth outward from the middle), `{"mask":{"texture":"id","threshold":0.5}}` (where the image stretched over the canvas is opaque), `{"around":{"axis":[0,0,1],"at":"p0","count":8,"radius":100}}` (in turns around a point) or `{"between":{"from":"p0","to":"p1","count":10}}` (along a line between two points). |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene act my-scene particles '{"shape":{"system":"sparks","emitter":0,"shape":{"circle":{"center":[960,540],"radius":300,"inner":100}}}}'
reiwand-ctl scene act my-scene particles '{"shape":{"system":"sparks","emitter":1,"shape":{"box": { "from": [0, 1000], "to": [1920, 1080] }}}}'
```

#### variant `layer`

Makes an emitter spawn from where another layer is opaque, with its color and its motion. It is a `layer` shape of the emitter, with its own settings. The layer cannot be the one that draws the system itself.

| Argument | Type | What it means |
|---|---|---|
| `system` | text | (required) Id of the system. |
| `emitter` | integer | (required) Position of the emitter. |
| `id` | text | (required) Id of the layer they are born from. |
| `threshold` | number | Minimum opacity that counts as solid, from 0 to 1. Default 0.5. |
| `color` | yes/no | If `true`, the particles are born with the layer's color. Default `false`. |
| `inherit` | yes/no | If `true`, they inherit its motion. Default `false`. |
| `jitter` | number | Spread of the birth point. |
| `refresh` | text | `static` (reads the layer once) or `on_change` (again each time it changes). |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png","id":"frame"}'
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene act my-scene particles '{"layer":{"system":"sparks","emitter":0,"id":"frame","threshold":0.4,"color":true,"inherit":true,"jitter":2,"refresh":"on_change"}}'
```

#### variant `emit`

Changes values of an emitter that are neither its shape nor its rate: bursts, following the pointer, duration, delay, periodic pauses and silence. A `null` or `false` value removes that value.

| Argument | Type | What it means |
|---|---|---|
| `system` | text | (required) Id of the system. |
| `emitter` | integer | (required) Position of the emitter. |
| `values` | object | Values that change: `bursts` (up to 8 bursts `{"count": n, "every": seconds, "on": "beat" or "click", "at": seconds}`), `follow` (`"cursor"`: the shape follows the pointer), `inherit` (fraction of the pointer's speed they inherit), `duration` (seconds after which it stops emitting), `delay` (seconds of waiting before starting), `periodic` (`{"on":[min,max],"off":[min,max]}`, seconds of emission and of pause) and `audio_gate` (`true`: nothing is born while the sound is silent). |
| `clear` | list of texts | Names of values that are removed. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene act my-scene particles '{"emit":{"system":"sparks","emitter":0,"values":{"bursts":[{"count":30,"every":2}],"delay":1,"duration":10}}}'
reiwand-ctl scene act my-scene particles '{"emit":{"system":"sparks","emitter":0,"values":{"periodic":{"on":[1,2],"off":[1,3]},"audio_gate":true},"clear":["duration"]}}'
```

#### variant `pace`

Limits how many particles an emitter can spawn in each 1/60-second step; the extra ones wait for the next step.

| Argument | Type | What it means |
|---|---|---|
| `system` | text | (required) Id of the system. |
| `emitter` | integer | Emitter it applies to. With no value, all of them. |
| `max_per_step` | integer | Maximum births per step (at least 1). With no value, removes the limit. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene act my-scene particles '{"pace":{"system":"sparks","max_per_step":4}}'
```

#### variant `point`

Sets, moves or removes a control point (`p0` to `p7`), a place that shapes, forces and children can name. The points missing before the one you set start at `[0, 0]`.

| Argument | Type | What it means |
|---|---|---|
| `system` | text | (required) Id of the system. |
| `point` | integer | (required) Number of the point, from 0 to 7. |
| `at` | list of numbers | Place `[x, y]` in layer pixels. |
| `follow` | yes/no | `true` makes it follow the pointer; `false` releases it. |
| `angle` | number | Rotation of the point in degrees, from −360 to 360. |
| `remove` | yes/no | Removes the point; only the last one can be removed. Default `false`. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene act my-scene particles '{"point":{"system":"sparks","point":0,"at":[100,50],"angle":30}}'
reiwand-ctl scene act my-scene particles '{"point":{"system":"sparks","point":1,"follow":true}}'
reiwand-ctl scene act my-scene particles '{"point":{"system":"sparks","point":1,"remove":true}}'
```

#### variant `start`

Changes the values each particle is born with. Each range is `[minimum, maximum]` and the particle takes a random value between the two; with a third number (an exponent from 0.1 to 10) the randomness leans toward one end. An end set to `null` keeps the current one.

| Argument | Type | What it means |
|---|---|---|
| `system` | text | (required) Id of the system. |
| `ranges` | object | Ranges that change: `life` (seconds of life), `speed` (pixels per second), `angle` (degrees, 0 toward the right), `size` (pixels), `rotation`, `spin` (degrees per second), `frame` (first frame of a sprite sheet), `color` (two colors that are mixed at random), `alpha` (opacity), `velocity` (`[[vx0,vy0],[vx1,vy1]]`, in place of `speed` and `angle`), `radial` (speed moving away from the shape's center), `depth` and `depth_speed` (they need perspective), `tilt` (`{"angle":[[xmin,ymin],[xmax,ymax]],"spin":[[xmin,ymin],[xmax,ymax]]}`, rotation in depth; with zeros it does not rotate), and only one of `hsv` (`{"hue":[0,360],"steps":6,"sat":[a,b],"val":[a,b]}`), `palette` (`{"colors":[...],"noise":[h,s,v]}`) or `gradient` (stops `[t, color]`), which replace `color`. |
| `clear` | list of texts | Ranges that are removed: `alpha`, `velocity`, `radial`, `hsv`, `palette` or `gradient`. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene act my-scene particles '{"start":{"system":"sparks","ranges":{"life":[1,3],"size":[4,10,2],"color":["#ffd27a","#ff7a3d"]}}}'
reiwand-ctl scene act my-scene particles '{"start":{"system":"sparks","ranges":{"gradient":[[0,"#ff0000"],[1,"#0000ff"]],"alpha":[0.5,1]},"clear":["velocity"]}}'
```

#### variant `turbulent`

Gives each particle an initial velocity that follows a moving noise, like air. Only the fields you name change.

| Argument | Type | What it means |
|---|---|---|
| `system` | text | (required) Id of the system. |
| `turbulent` | object | Fields: `forward` (forward push), `scale` (size of the swirl), `noise_speed` (speed of the noise), `offset`, `speed` (`[min, max]`), `phase` and `audio` (sound band that moves the phase: `bass`, `mid`, `treble` or `level`); with `audio` also come `kind` (`relative`, `absolute` or `surge`), `unit` (`db` or `amplitude`) and, only with `surge`, `window`, `reference` and `hold_silence`. |
| `quiet` | yes/no | If `true`, removes the sound band. Default `false`. |
| `off` | yes/no | If `true`, removes the whole turbulence. Default `false`. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene act my-scene particles '{"turbulent":{"system":"sparks","turbulent":{"forward":20,"scale":0.01,"audio":"bass","kind":"relative","unit":"db"}}}'
reiwand-ctl scene act my-scene particles '{"turbulent":{"system":"sparks","quiet":true}}'
```

#### variant `offset`

Shifts each particle's birth point with a noise, so they are not born in a line. Only the fields you name change.

| Argument | Type | What it means |
|---|---|---|
| `system` | text | (required) Id of the system. |
| `offset` | object | Fields: `distance` (pixels), `scale`, `sign`, `octaves` (from 1 to 6), `noise_scale`, `noise_speed`. |
| `off` | yes/no | If `true`, removes the offset. Default `false`. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene act my-scene particles '{"offset":{"system":"sparks","offset":{"distance":30,"scale":0.01,"octaves":3}}}'
```

#### variant `remap`

Adds, changes or removes a rule that turns a piece of data (life, distance to a point, sound...) into a value of the particle. Without `index` it adds at the end.

| Argument | Type | What it means |
|---|---|---|
| `system` | text | (required) Id of the system. |
| `remap` | object | The rule: `input` (`daytime`, `runtime`, `life`, `age`, `speed`, `random`, `distance:p0`..`distance:p7`, `audio:band` or `cursor:x` / `cursor:y`), `in` (`[from, to]` of the data), `output` (`size`, `alpha`, `color`, `speed`, `life`, `rotation`, `spin` or `frame`), `out` (`[from, to]` of the result), and optional `clamp_in` and `clamp_out` (`true` to not overshoot), `op` (`set`, `mul`, `add` or `sub`) and `wave` (`sine` or `noise`). |
| `index` | integer | Rule that is replaced or removed. |
| `remove` | yes/no | With `index`, removes it. Default `false`. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene act my-scene particles '{"remap":{"system":"sparks","remap":{"input":"life","in":[0,1],"output":"alpha","out":[1,0]}}}'
reiwand-ctl scene act my-scene particles '{"remap":{"system":"sparks","index":0,"remove":true}}'
```

#### variant `force`

Adds, replaces or removes a force that acts on the particles. Without `index` it adds at the end; with `index` and `force`, it replaces that force and keeps its life window; with `index` and `remove`, it removes it.

| Argument | Type | What it means |
|---|---|---|
| `system` | text | (required) Id of the system. |
| `force` | any JSON value | The force: `{"gravity":[x,y]}`, `{"wind":[x,y]}`, `{"drag":k}`, `{"turbulence":{"strength":s,"scale":c}}`, `{"attractor":{"target":"cursor","strength":s}}` (negative, it repels), `{"vortex":{"axis":[0,0,1],"center":[x,y],"strength":s}}`, `{"orbit":{"center":[x,y],"strength":s}}`, `{"noise":s}`, `{"cap":speed}`, `{"torque":k}`, `{"spin_drag":k}`, `{"oscillate":{"what":"position","from":"top","freq":[a,b],"scale":[a,b]}}`, `{"slow_near":{"target":[x,y],"inner":r1,"outer":r2,"factor":f}}`, `{"keep_distance":{"target":"p0","distance":d,"strength":s}}` or `{"keep_between":{"from":"p0","to":"p1","strength":s}}`. A target is a point, `"cursor"` or a control point. |
| `index` | integer | Force that is replaced or removed. |
| `remove` | yes/no | With `index`, removes it. Default `false`. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene act my-scene particles '{"force":{"system":"sparks","force":{"gravity":[0,-30]}}}'
reiwand-ctl scene act my-scene particles '{"force":{"system":"sparks","force":{"attractor":{"target":"cursor","strength":5000}}}}'
reiwand-ctl scene act my-scene particles '{"force":{"system":"sparks","index":0,"force":{"gravity":[0,-60]}}}'
reiwand-ctl scene act my-scene particles '{"force":{"system":"sparks","index":1,"remove":true}}'
```

#### variant `window`

Sets or removes a force's life window: in what part of the particle's life it appears, is fully on and disappears.

| Argument | Type | What it means |
|---|---|---|
| `system` | text | (required) Id of the system. |
| `index` | integer | (required) Position of the force. |
| `window` | list of numbers | Four fractions of the life, from 0 to 1 and in order: `[starts coming in, fully in, starts going out, gone]`. With no value, removes the window. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene act my-scene particles '{"force":{"system":"sparks","force":{"gravity":[0,-30]}}}'
reiwand-ctl scene act my-scene particles '{"window":{"system":"sparks","index":0,"window":[0,0.1,0.8,1]}}'
```

#### variant `ramp`

Sets how a force's life window fades in and out: in a straight line or easing (the default).

| Argument | Type | What it means |
|---|---|---|
| `system` | text | (required) Id of the system. |
| `index` | integer | (required) Position of the force. |
| `ramp` | text | `linear` (a straight line) or `smooth` (eases in and out). With no value, removes the field, which fades smoothly. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene act my-scene particles '{"force":{"system":"sparks","force":{"gravity":[0,-30]}}}'
reiwand-ctl scene act my-scene particles '{"ramp":{"system":"sparks","index":0,"ramp":"linear"}}'
```

#### variant `boids`

Makes the particles move as a flock (they separate, align and gather). Only the fields you name change; if there was no flock, it starts with radius 40. It works for systems of up to 8192 particles.

| Argument | Type | What it means |
|---|---|---|
| `system` | text | (required) Id of the system. |
| `boids` | object | Fields: `radius` (distance at which they see their neighbors, from 1 to 10000), `separation`, `alignment`, `cohesion` (from 0 to 10) and `max_speed`. |
| `off` | yes/no | If `true`, removes the flock. Default `false`. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene act my-scene particles '{"boids":{"system":"sparks","boids":{"radius":50,"cohesion":0.8}}}'
reiwand-ctl scene act my-scene particles '{"boids":{"system":"sparks","off":true}}'
```

#### variant `collider`

Adds, replaces or removes a collider: a wall that particles bounce off, slide along, stop at, stick to or die on. Without `index` it adds at the end.

| Argument | Type | What it means |
|---|---|---|
| `system` | text | (required) Id of the system. |
| `collider` | any JSON value | The collider: `{"bounds": wall, "bounce": 0 to 1, "friction": 0 to 1, "kill": true, "stick": true}`. The wall can be `"canvas"` (the canvas edges), `{"plane":{"point":[x,y],"normal":[x,y]}}` (they stay on the side the normal points to), `{"circle":{"center":[x,y],"radius":r}}` (they stay outside), `{"box":{"from":[x,y],"to":[x,y],"inside":true}}`, `{"segment":{"from":[x,y],"to":[x,y]}}`, `{"mask":{"texture":"id","threshold":0.5}}` (the opaque part of an image) or `{"layer":{"id":"layer_id"}}` (the opaque shape of a layer). |
| `index` | integer | Collider that is replaced or removed. |
| `remove` | yes/no | With `index`, removes it. Default `false`. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene act my-scene particles '{"collider":{"system":"sparks","collider":{"bounds":"canvas","bounce":0.4}}}'
reiwand-ctl scene act my-scene particles '{"collider":{"system":"sparks","collider":{"bounds":{"segment":{"from":[0,500],"to":[1000,600]}},"bounce":0.5}}}'
reiwand-ctl scene act my-scene particles '{"collider":{"system":"sparks","index":1,"remove":true}}'
```

#### variant `response`

Changes what an already placed collider does when a particle touches it.

| Argument | Type | What it means |
|---|---|---|
| `system` | text | (required) Id of the system. |
| `index` | integer | (required) Position of the collider. |
| `response` | text | (required) `bounce`, `slide`, `stop`, `stick` or `kill` (dies). |
| `bounce` | number | With `bounce`, how much it bounces, from 0 to 1. By default it keeps the current one (0.5 if it had none). |
| `friction` | number | Friction, from 0 to 1. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene act my-scene particles '{"collider":{"system":"sparks","collider":{"bounds":"canvas","bounce":0.4}}}'
reiwand-ctl scene act my-scene particles '{"response":{"system":"sparks","index":0,"response":"slide","friction":0.2}}'
```

#### variant `child`

Adds, replaces or removes a child: another system that is born with each particle, when it dies or following it. Without `index` it adds at the end. Each system can have up to 4 children and there can be at most two levels.

| Argument | Type | What it means |
|---|---|---|
| `system` | text | (required) Id of the parent system. |
| `child` | any JSON value | The child: `{"system": "id", "on": "static" or "spawn" or "death" or "follow", "probability": 0 to 1, "max": n, "offset": [x,y], "angle": degrees, "scale": 0 to 100, "points_from": true}`. Only `system` and `on` are required. |
| `index` | integer | Child that is replaced or removed. |
| `remove` | yes/no | With `index`, removes it. Default `false`. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene act my-scene particles '{"create":{"system":"smoke"}}'
reiwand-ctl scene act my-scene particles '{"child":{"system":"sparks","child":{"system":"smoke","on":"spawn","probability":0.5}}}'
reiwand-ctl scene act my-scene particles '{"child":{"system":"sparks","index":0,"remove":true}}'
```

#### variant `inherit`

Chooses what a child takes from the particle that made it be born.

| Argument | Type | What it means |
|---|---|---|
| `system` | text | (required) Id of the parent system. |
| `child` | integer | (required) Position of the child. |
| `inherit` | list of texts | What it inherits: `color`, `size`, `alpha` and `velocity`. An empty list removes the inheritance. |
| `follow` | yes/no | If `true`, the child takes it again at each step while it follows. It only makes sense on a child that follows (`follow`). |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene act my-scene particles '{"create":{"system":"trail","parent":"sparks","on":"follow"}}'
reiwand-ctl scene act my-scene particles '{"inherit":{"system":"sparks","child":0,"inherit":["color","size"],"follow":true}}'
```

#### variant `strand`

Joins the particles into a rope (in the order they are born) or leaves a trail behind each one. Only the fields you name change.

| Argument | Type | What it means |
|---|---|---|
| `system` | text | (required) Id of the system. |
| `strand` | text | (required) `rope` or `trail`. |
| `layer` | text | Particle layer it applies to. |
| `settings` | object | Fields for the rope: `subdivide` (from 0 to 7), `width`, `uv_scale`, `uv_scroll`, `smooth`; for the trail: `length` (steps, 32 at the start), `width` (from 0 to 1, 0.5 at the start), `segments`, `subdivide`, `uv_scale`, `uv_scroll`, `textured`. |
| `off` | yes/no | If `true`, removes it. Default `false`. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene act my-scene particles '{"strand":{"system":"sparks","strand":"trail","settings":{"length":8}}}'
reiwand-ctl scene act my-scene particles '{"strand":{"system":"sparks","strand":"rope"}}'
reiwand-ctl scene act my-scene particles '{"strand":{"system":"sparks","strand":"trail","off":true}}'
```

#### variant `texture`

Chooses the image each particle is drawn with. Without an image, they are soft discs.

| Argument | Type | What it means |
|---|---|---|
| `system` | text | (required) Id of the system. |
| `layer` | text | Particle layer it applies to. |
| `texture` | text | Id of a package image. With no value, removes the image. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png"}'
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene act my-scene particles '{"texture":{"system":"sparks","texture":"picture"}}'
```

#### variant `render`

Adjusts how the particles are drawn. Only what you name changes.

| Argument | Type | What it means |
|---|---|---|
| `system` | text | (required) Id of the system. |
| `layer` | text | Particle layer it applies to. |
| `stretch` | any JSON value | How much each particle is stretched in the direction it moves: `{"mode": "add", "depth": "flat", "time": s, "min": px, "max": px}`: seconds of motion and the minimum and maximum; with `add` pixels are added to the side, with `relative` the length is the side times `clamp(time·speed, min, max)` (minimum and maximum in multiples of the side, can be smaller than the side). `depth` is `flat` (trail by speed without perspective), `projected` (the trail follows the on-screen movement) or `motion` (the length and direction come from the on-screen speed, counting the speed in depth; without `perspective` it is equivalent to `flat`). All five fields are required; `"none"` removes it. |
| `frames` | text | How the frames of a sprite sheet advance: `time` (with time), `life` (with life) or `random`; `none` removes it. |
| `fps` | number | Frames per second with `frames` set to `time`. |
| `mix` | yes/no | If `true`, blends each frame with the next. |
| `brightness` | number | Brightness, from 0 to 16. |
| `cycles` | integer | How many times the frames repeat in one life (from 1 to 64); only valid with `frames` set to `life`. `0` removes it. |
| `order` | text | Which particles are drawn on top: `newest`, `oldest`, `largest`, `smallest` or `lowest`; `none` removes it. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene act my-scene particles '{"render":{"system":"sparks","stretch":{"mode":"add","depth":"flat","time":0.05,"min":0,"max":60},"frames":"life","fps":12,"mix":true,"brightness":2,"cycles":2,"order":"newest"}}'
reiwand-ctl scene act my-scene particles '{"render":{"system":"sparks","stretch":"none","frames":"none","cycles":0,"order":"none"}}'
```

#### variant `over`

Makes each particle's size and color change over its life. These curves are multiplied by the size and color it is born with.

| Argument | Type | What it means |
|---|---|---|
| `system` | text | (required) Id of the system. |
| `layer` | text | Particle layer it applies to. |
| `size` | list of lists of numbers | Keys `[time, scale]`, with the time from 0 to 1 of the life. An empty list removes the curve. |
| `color` | list of lists | Stops `[time, "#color"]`. An empty list removes the curve. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene act my-scene particles '{"over":{"system":"sparks","size":[[0,0],[0.1,1],[1,0]],"color":[[0,"#ffffff"],[1,"#ff000000"]]}}'
reiwand-ctl scene act my-scene particles '{"over":{"system":"sparks","size":[]}}'
```

#### variant `scope`

Gives a particle layer its own drawing, a copy of the system's, or takes it away so it follows the system again. It is rejected for a layer that does not draw that system.

| Argument | Type | What it means |
|---|---|---|
| `system` | text | (required) Id of the system. |
| `layer` | text | (required) Id of the particle layer. |
| `own` | yes/no | (required) `true` gives it its own drawing; `false` releases it. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene act my-scene particles '{"scope":{"system":"sparks","layer":"sparks","own":true}}'
reiwand-ctl scene act my-scene particles '{"scope":{"system":"sparks","layer":"sparks","own":false}}'
```

#### variant `aspect`

Changes the sprite's proportion: width divided by height, from 0.05 to 20 (1 is square). `size` is the longer side. With `from_texture: true` it takes the texture's proportion. It accepts `layer`. A value other than 1 needs `particles2`. The budget counts `size²·min(aspect, 1/aspect)`.

```
reiwand-ctl scene act my-scene particles '{"aspect":{"system":"sparks","aspect":0.25}}'
```

#### variant `cutout`

Rewrites the opacity of the particles' image: below `start` it becomes 0, above `end` it becomes `opacity`, and between the two it ramps up.

| Argument | Type | What it means |
|---|---|---|
| `system` | text | (required) Id of the system. |
| `layer` | text | Particle layer it applies to. |
| `range` | list of numbers | `[start, end, opacity]`, with values from 0 to 1. With no value, removes the cutout. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene act my-scene particles '{"cutout":{"system":"sparks","range":[0.2,0.8,1]}}'
```

#### variant `orient`

Chooses which way the particles face.

| Argument | Type | What it means |
|---|---|---|
| `system` | text | (required) Id of the system. |
| `layer` | text | Particle layer it applies to. |
| `orient` | text | `screen` (upright), `velocity` (the way they are going) or `fixed` (a fixed angle). With no value, removes the orientation. |
| `angle` | number | Angle in degrees, only with `fixed`. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene act my-scene particles '{"orient":{"system":"sparks","orient":"velocity"}}'
reiwand-ctl scene act my-scene particles '{"orient":{"system":"sparks","orient":"fixed","angle":45}}'
```

#### variant `perspective`

Gives the system depth, with a focal length and a near plane in pixels, to use `depth` and `depth_speed` in `start`. What is not given stays as it was, or takes the suggested value if the system had no perspective.

| Argument | Type | What it means |
|---|---|---|
| `system` | text | (required) Id of the system. |
| `focal` | number | Focal length in pixels, from 1 to 100000. With no value, it uses 800 if there was none. |
| `near` | number | Near plane: distance to the eye in pixels, from 1 to 100000, below which no particle is drawn. With no value, it uses 1 if there was none. |
| `off` | yes/no | If `true`, removes the perspective and the `depth` and `depth_speed` ranges. Default `false`. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene act my-scene particles '{"perspective":{"system":"sparks","focal":600}}'
reiwand-ctl scene act my-scene particles '{"perspective":{"system":"sparks","near":50}}'
reiwand-ctl scene act my-scene particles '{"perspective":{"system":"sparks","off":true}}'
```

#### variant `shader`

Draws the particles with a package effect made for particles.

| Argument | Type | What it means |
|---|---|---|
| `system` | text | (required) Id of the system. |
| `layer` | text | Particle layer it applies to. |
| `shader` | text | Name of the package's particle effect. With no value, removes it. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene act my-scene particles '{"shader":{"system":"sparks"}}'
```

#### variant `lit`

Makes the scene's lights light the particles (up to 8 lights). With `normal`, a normal map gives them relief.

| Argument | Type | What it means |
|---|---|---|
| `system` | text | (required) Id of the system. |
| `layer` | text | Particle layer it applies to. |
| `lit` | yes/no | `true` lights them; `false` stops lighting them. |
| `normal` | text | Id of a package image that serves as the normal map. Empty text removes it. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png"}'
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene act my-scene particles '{"lit":{"system":"sparks","lit":true,"normal":"picture"}}'
```

#### variant `refract`

Makes the particles bend what is behind them, like water drops, according to a normal map; with `color` they can also tint or light with their color what they bend.

| Argument | Type | What it means |
|---|---|---|
| `system` | text | (required) Id of the system. |
| `layer` | text | Particle layer it applies to. |
| `normal` | text | Id of a package image that serves as the normal map. Required while the system has none. |
| `units` | text | `size` (the shift is `amount` times half the sprite's size) or `pixels` (it is `shift` canvas pixels, the same for any sprite size and along the whole trail). By default, the ones it already has; if it has none, `size`. When the units change, the current value is converted so the bend does not change. |
| `amount` | number | With `size`: how much they bend, from 0 to 1. Default 0.3. |
| `shift` | number | With `pixels`: maximum shift in canvas pixels, from 0 to 4096. Default 8. A thin 7 px drop with `shift` 40 bends the background up to 40 px by its opacity along its whole length. |
| `color` | text | `none` (only the bend), `multiply` (the bent background is multiplied by the particle's color, its texture and its opacity: a white one changes nothing, a red one tints it red) or `add` (adds the particle's color by its opacity: a black one adds nothing). By default, the one it already has; if it has none, `none`. |
| `off` | yes/no | If `true`, removes the refraction. Default `false`. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png"}'
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene act my-scene particles '{"refract":{"system":"sparks","normal":"picture","units":"size","amount":0.4}}'
reiwand-ctl scene act my-scene particles '{"refract":{"system":"sparks","units":"pixels","shift":40}}'
reiwand-ctl scene act my-scene particles '{"refract":{"system":"sparks","color":"multiply"}}'
reiwand-ctl scene act my-scene particles '{"refract":{"system":"sparks","off":true}}'
```
