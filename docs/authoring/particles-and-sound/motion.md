# How particles start and move

On this page: `init`, forces and collisions.

## How they come out (`init`)

Each value is a range `[min, max]` that is chosen at random for each particle. With a third number,
`[min, max, exponent]` (from 0.1 to 10), the randomness leans toward one end.

| Field | Prefill | What it does |
|-------|---------|--------------|
| `life` | `[1, 2]` | Seconds of life. |
| `speed` | `[10, 40]` | Pixels per second. |
| `angle` | `[0, 360]` | Direction in degrees, clockwise, 0 to the right (270 is up). |
| `size` | `[2, 6]` | Size in pixels. |
| `rotation`, `spin` | `[0, 0]` | Initial rotation, and rotation per second, in degrees. |
| `frame` | `[0, 0]` | First frame of the sprite sheet. |
| `color` | white | Two colors to choose between. |
| `alpha` | | A range of opacity. |
| `hsv` | | `{ "hue": [0, 360], "steps", "sat", "val" }`: color by hue, saturation and value. |
| `palette` | | `{ "colors": [...], "noise": [h, s, v] }`: one of up to 10 colors, with a little variation. |
| `gradient` | | Up to 8 `[position, color]`, read at a random point. |
| `velocity` | | `[[vx0, vy0], [vx1, vy1]]`, instead of `speed` and `angle`. |
| `radial` | | Velocity that moves them away from the center of the shape. |
| `inherit` | | `{ "from": "cursor" or a point, "speed": [min, max] }`: they inherit the velocity of the pointer or of a control point. |
| `turbulent` | | An initial velocity that turns with a soft noise. With `audio` (a band) also write `kind`, `unit` and, with `surge`, `window`, `reference` and `hold_silence`, as in a sound link. |
| `offset_noise` | | A fractal offset of the birth place. |
| `remap` | | Up to 8 rules that take an input (`life`, `speed`, `age`, `daytime`, `runtime`, `random`, `distance:pN`, `audio:<band>`, `cursor:x`, `cursor:y`) to an initial value (`output`: `size`, `alpha`, `color`, `speed`, `life`, `rotation`, `spin`, `frame`) with `in`, `out`, `op` (`set`, `mul`, `add`, `sub`) and `wave` (`none`, `sine`, `noise`). With an `audio:<band>` input also write `kind`, `unit` and, with `surge`, `window`, `reference` and `hold_silence`. |
| `depth`, `depth_speed` | | Initial depth and its change, in pixels; they need `perspective`. |
| `tilt` | | Rotation in depth: `{ "angle": [[xmin, ymin], [xmax, ymax]], "spin": [[xmin, ymin], [xmax, ymax]] }`, both required, in degrees and degrees per second; the editor always writes it and with zeros it does not turn. Each particle turns about the horizontal line (x) and the vertical line (y) of its own sprite and is drawn with the width times the cosine of y and the height times that of the x turn: a petal is seen edge-on and face-on as it falls. With `perspective` the near edge looks larger; with `lit` the light uses the turned normal. The back shows the same image. |

The ends of the numeric ranges (`life`, `speed`, `angle`, `size`...) can follow values, as in
`"size": [2, { "audio": "bass", "kind": "relative", "unit": "db", "scale": 8 }]`; they are read on
every frame. Colors and velocities are read on load.

## Forces

A list of up to 16, applied at each step:

| Force | What it does |
|-------|--------------|
| `{ "gravity": [x, y] }` | A constant acceleration, in pixels per second squared. |
| `{ "drag": k }` | Slows them down. |
| `{ "turbulence": { "strength", "scale" } }` | A noise that drags them. Also `speed`, `time_scale` and `mask`. |
| `{ "attractor": { "target", "strength" } }` | Attracts them (with negative strength, repels them). `target` is a point, `"cursor"` or a control point. With `radius` it does not act beyond it, and `falloff` is `constant`, `linear` or `inverse_square`. |
| `{ "vortex": { "axis", "center", "strength" } }` | Makes them spin around the line that passes through the center with the direction `axis` (required, length 1; `[0, 0, 1]` spins on the screen). With `canvas.perspective` it also pushes in depth; without it the z part is ignored. Also `inner`, `outer` and `ring`. |
| `{ "orbit": { "center", "strength" } }` | Makes them orbit. |
| `{ "noise": strength }` | A random jitter. |
| `{ "noise_field": { "strength", "phase", "scale", "drift", "mask" } }` | Pushes each particle along a smooth 3D noise at a strength of its own, so particles bunch up and spread out like embers or dust (`turbulence` swirls them but never gathers them). `strength` and `phase` are `{ "min", "max" }`; see below. Needs `particle_noise`. |
| `{ "oscillate": { "what", "from", "freq", "scale", "phase", "mask" } }` | Makes them oscillate in position, opacity or size. `from` is `top` (goes down from the full value) or `floor` (goes up from a random floor between the two scales; not valid for position). Add `"random": "shared"` to draw the frequency, scale and phase of each particle from one random number, so the fast ones also swing widest and latest (the editor's *One random number for speed, size and phase* in *Fades and pulses*); without it, or with `"separate"`, each comes from its own. Needs `oscillate_random`. |
| `{ "cap": speed }` | Limits the speed. |
| `{ "torque": k }`, `{ "spin_drag": k }` | Change and slow down the spin. |
| `{ "slow_near": { "target", "inner", "outer", "factor" } }` | Slow them near a point. |
| `{ "keep_distance": { "target", "distance", "strength" } }` | Keep them at a distance, like a spring. |
| `{ "keep_between": { "from", "to", "strength" } }` | Attract them to a segment. |
| `{ "boids": { "radius", "separation", "alignment", "cohesion", "max_speed" } }` | Flocks: they separate, align and gather with their neighbors. Up to 8192 particles. |
| `{ "remap": { ... } }` | A rule of `init.remap` applied at each step. |

`noise_field` picks one random number `u` from 0 to 1 per particle and uses it for both ranges: the
particle pushes with `strength.min + u·(strength.max − strength.min)` pixels per second squared
and reads the noise `phase.min + u·(phase.max − phase.min)` pixels ahead of where it is. The noise is
read at `(position + phase + t·drift)·scale`, the same phase and drift added to x, y and z, with `t`
the seconds of the system's own clock, so a `scale` of 0.01 makes swirls about 100 pixels across and
`drift` slides the whole field, in pixels per second. The push along x, y and depth is the noise at
that point read three ways, `[N(x, y, z), N(z, x, y), N(y, z, x)]`, times `mask` (each 0 to 1). A
system without `perspective` reads it at depth 0 and ignores the third part of `mask`; with
`perspective` the third part pushes the depth. Every field is written. The `embers` system of
[`examples/particles`](../examples/particles/scene.json) rises with this one, which only starts after
the first 15 % of each life: each ember picks its own strength and phase, so they bunch into
drifting wisps instead of moving as one.

```json
{
  "noise_field": {
    "strength": { "min": 40, "max": 160 }, "phase": { "min": 0, "max": 600 },
    "scale": 0.003, "drift": 20, "mask": [1, 0.6, 0]
  },
  "ramp": "linear", "window": [0, 0.15, 1, 1]
}
```

Any force accepts `window: [in_start, in_end, out_start, out_end]`, fractions of the life in which
it fades in and out. The fade eases in and out; next to the window, `"ramp": "linear"` makes it a
straight line instead (the editor's *Fade in and out in a straight line* under *When in the life
each force acts*; needs `window_ramp`), and `"smooth"` or no `ramp` keeps the easing. Gravity and the forces can follow values, as in
`{ "noise": { "audio": "bass", "kind": "relative", "unit": "db", "scale": 300 } }`.

## Collisions

Up to 8 colliders, each `{ "bounds", "bounce", "friction", "kill", "stick" }`. `bounds`:
`"canvas"`, `{ "plane": { "point", "normal" } }`, `{ "circle": { "center", "radius", "inside" } }`,
`{ "box": { "from", "to", "inside" } }`, `{ "segment": { "from", "to" } }`,
`{ "layer": { "id" } }` (the opaque part of a layer) or `{ "mask": { "texture", "threshold" } }`.
`bounce` and `friction` go from 0 to 1; `kill` deletes the particle on touching; `stick` leaves it
stuck.
