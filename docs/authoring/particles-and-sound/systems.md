# Particle systems and emitters

On this page: starting points, a system and its emitters.

## Start with a starting point

A new particle layer already brings a system that looks good: snow falling from top to bottom over
the whole canvas, with soft turbulence and `prewarm`, so that it is full from the first frame.

```
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","name":"Snow"}'
```

It prints the id of the layer and of the system (`snow`). For another start, the `particles` action
with `kit` fills in at once the emitters, the life, the forces and the colors of the system
(children are kept) with `snow`, `sparks` (sparks that rise), `rain` or `dust` (floating motes),
adjusted to the size of the canvas:

```
reiwand-ctl scene act my-scene particles '{"kit":{"system":"snow","kit":"sparks"}}'
```

In the editor it is the *Start with…* dropdown of the system settings
([editor-particles-and-sound.md](../editor-particles-and-sound.md)). Then adjust whatever you like.

**If you write it by hand, set `prewarm`.** A system without `prewarm` starts empty: particles are
born little by little, and a still image of the first instant (`scene render`, a thumbnail) comes
out with none. Set in `prewarm` the seconds it takes to fill up (up to 30), on the order of the
life of the particles (`init.life`): the snow starting point, with lives of 12 to 18 s, uses 14.

## The system

| Field | Prefill | Range | What it does |
|-------|---------|-------|--------------|
| `capacity` | | 1 to 200000 | How many particles fit alive at once. |
| `seed` | 0 | | The seed of the system, which is mixed with that of `clock.seed`. |
| `prewarm` | 0 | 0 to 30 s | Seconds simulated before the first time it is seen. |
| `space` | `local` | `local`, `world` | In `world` the `transform` of the layer only places the particles when they are born; afterwards it no longer moves them. |
| `emitters` | | up to 8 | Where and how many are born. |
| `init` | | | How they come out. |
| `forces` | | up to 16 | What moves them. |
| `collide` | | up to 8 | What they collide with. |
| `render` | soft discs | | How they are drawn. |
| `children` | | up to 4 | Systems that are born from these particles. |
| `points` | | up to 8 | Control points, `p0` to `p7`. |
| `perspective` | no depth | `focal` 1 to 100000 px | Gives depth to the particles. |
| `lock` | | | Settings that layers and scripts cannot change with `instance`. |

The simulation advances in steps of 1/60 s, up to 15 per frame. With the same `seed` and the same
frames, the same particles come out. While none lives or is born, the system jumps to its next
burst and the scene can sleep. A paused scene frees its particles and starts them again,
prewarmed, when it continues.

## Emitters

Each emitter has a `shape`, a rate and, if you want, bursts. In the particles panel they are added
with "Add an emitter" and removed with the button of each emitter (a system keeps at least one).

| Field | What it does |
|-------|--------------|
| `shape` | Where it is born: see below. |
| `rate` | Particles per second; it can follow a value (a property, the sound...). |
| `bursts` | Up to 8 bursts, each of `count` particles: `on` each music beat (`beat`) or click (`click`); or once at `at` seconds after the emitter starts (0 by default) and, with `every`, again every so many seconds. |
| `follow` | `cursor`: the shape follows the pointer. |
| `inherit` | Fraction of the pointer's velocity they inherit. |
| `duration` | Seconds after which the emitter turns off (0, never). |
| `delay` | Seconds before starting (0 to 86400). |
| `periodic` | `{ "on": [min, max], "off": [min, max] }`: seconds of emission and of pause, chosen with the seed. |
| `max_per_step` | At most this many births per step of 1/60 s (1 to 1000). |
| `audio_gate` | `true`: nothing is born while the sound is silent. |

The shapes are:

| Shape | Example | Where they are born |
|-------|---------|---------------------|
| `point` | `{ "point": [x, y] }` | At a point. |
| `box` | `{ "box": { "from": [x0, y0], "to": [x1, y1] } }` | In a rectangle. With `inner: [width, height]` and `at` it is a hollow rectangle around a control point. |
| `circle` | `{ "circle": { "center", "radius", "inner" } }` | In a disc, or in a ring if `inner` is greater than 0. Also `scale: [x, y]` (0.01 to 100), `sign: [x, y]` (-1, 0 or 1: half circles or quarters) and `at`. |
| `sphere` | `{ "sphere": { "center": [x, y], "depth": z, "radius": [rx, ry, rz], "inner": 0, "sign": [0, 0, 0] } }` | Inside a sphere or ellipsoid, spread evenly through its volume (not on the surface). `inner` (0 to 1, fraction of the radii) leaves the core empty, and `sign` (-1, 0 or 1 for each of x, y, z) leaves them on one side only. `push` (pixels, optional) moves every birth that far outward from the middle along its own direction, which leaves a round hole that stays even when the ball is flat (`rz` 0): `{ "radius": [200, 200, 0], "push": 200, ... }` gives a ring from 200 to 400 px, denser in its middle than at its borders. All the other fields are required; `at` is optional. x and y give the position and z the depth: with `perspective` it is added to `init.depth` and to `depth` of the shape (those born near the eye look large and stay near the center of the image); without `perspective` z is discarded and what remains is the spread of a ball seen from the front, denser toward the center than a disc. |
| `line` | `{ "line": [[x0, y0], [x1, y1]] }` | On a line. |
| `edge` | `"edge"` | On the edge of the canvas. |
| `mask` | `{ "mask": { "texture", "threshold" } }` | Where the alpha of an image stretched over the canvas exceeds `threshold` (0.5 by default). |
| `layer` | `{ "layer": { "id", "threshold", "color", "inherit", "jitter", "refresh" } }` | Where a layer is opaque (up to 4096 points), with the color and the movement of the layer. `refresh` is `static` or `on_change`. |
| `around` | `{ "around": { "axis", "at", "count", "arc", "radius", "speed", "mode", "restart" } }` | One after another around a point (on the circle perpendicular to `axis`, which is required: three numbers of length 1, `[0, 0, 1]` is the flat spin on screen): `count` places (1 to 4096) over `arc` degrees, turning `speed` degrees per second. `mode`: `repeat` or `mirror`. |
| `between` | `{ "between": { "from", "to", "count", "span", "mode", "arc", "taper" } }` | Along a line or arc between two points. |
