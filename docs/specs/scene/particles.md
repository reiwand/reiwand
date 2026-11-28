# Particles

`particles` maps at most 8 ids to systems, which hold at most 200 000 particles together. A
`particles` layer draws one system; several layers may draw the same system, which then moves only
once per frame and shares its budget between them. Positions are canvas pixels, and the transform of the layer moves, turns and
scales the whole system around the canvas center.

```json
"particles": {
  "sparks": {
    "capacity": 4000, "seed": 7, "prewarm": 2.0,
    "emitters": [{ "shape": { "box": { "from": [0, 1080], "to": [1920, 1100] } }, "rate": { "prop": "density" },
                   "bursts": [{ "count": 40, "on": "beat" }] }],
    "init": { "life": [2, 5], "speed": [40, 120], "angle": [250, 290], "size": [4, 12], "color": ["#ffd27a", "#ff7a3d"] },
    "forces": [{ "gravity": [0, -20] }, { "turbulence": { "strength": 30, "scale": 0.004 } },
               { "attractor": { "target": "cursor", "strength": 5000 } }, { "drag": 0.4 }],
    "collide": [{ "bounds": "canvas", "bounce": 0.3 }],
    "render": { "texture": "spark", "frames": "life", "size_over_life": [[0, 0], [0.1, 1], [1, 0]],
                "trail": { "length": 8, "width": 0.5 } }
  }
}
```

| Field | Meaning |
|-------|---------|
| `capacity`, `seed`, `prewarm` | 1 to 200 000 particles; the seed of the system, mixed with `clock.seed`; seconds simulated when the system first shows (at most 10) |
| `emitters` | at most 4 of `{ "shape", "rate", "bursts", "follow", "inherit" }`. Shapes are `{ "point": [x, y] }`, `{ "box": { "from": [x0, y0], "to": [x1, y1], "inner", "at" } }`, `{ "circle": { "center", "radius", "inner" } }` (a ring when `inner` is above 0), `{ "line": [[x0, y0], [x1, y1]] }`, `"edge"` (the border of the canvas) and `{ "mask": { "texture", "threshold" } }` (where the alpha of an image stretched over the canvas is above `threshold`, prefilled 0.5). `rate` is particles per second and may be bound like any value. `bursts` is a list of at most 8 independent bursts, each emitting `count` particles: `on` each `beat` or `click`; otherwise once `at` seconds (prefilled 0) after the emitter starts and, with `every`, again every that many seconds. `follow: "cursor"` moves the shape with the pointer and `inherit` adds that fraction of the pointer velocity |
| `init` | `[min, max]` ranges picked for each particle: `life` (seconds), `speed` (pixels per second), `angle` (clockwise degrees, 0 to the right), `size` (pixels), `rotation`, `spin` (degrees per second), `frame` (first atlas frame) and `color` (two colors mixed at random) |
| `forces` | at most 8 of `{ "gravity": [x, y] }`, `{ "drag": k }`, `{ "turbulence": { "strength", "scale" } }` (curl noise), `{ "attractor": { "target", "strength" } }` (negative strengths repel), `{ "vortex": { "axis", "center", "strength" } }`, `{ "orbit": { "center", "strength" } }` and `{ "noise": strength }`. Gravity and strengths may be bound, as in `{ "noise": { "audio": "bass", "kind": "relative", "unit": "db", "scale": 300 } }`. Targets and centers are a point or `"cursor"`, which pulls nothing while the pointer is away |
| `collide` | at most 4 of `{ "bounds", "bounce", "friction", "kill" }` where `bounds` is `"canvas"`, `{ "plane": { "point", "normal" } }` (particles stay on the side the normal points to) or `{ "circle": { "center", "radius" } }` (particles stay outside). `bounce` and `friction` go from 0 to 1 and `kill` removes a particle when it touches |
| `render` | `texture` (an image asset, split into frames by its `atlas: [rows, columns]`), `aspect` (required, 0.05 to 20: width divided by height of the sprite, 1 for a square one; `size` is the longer side, so the quad is `size` by `size/aspect` for a tall sprite and `size·aspect` by `size` for a wide one, it turns with the particle, `stretch` lengthens its long side, and the lit and refract normals follow it; a tall sprite that is carried by its motion (`orient` velocity, a stretch or a turned sprite) turns so that the top of the texture leads; a non-1 value needs `particles2`; the budget counts `size²·min(aspect, 1/aspect)`), `frames` (`time` at `fps`, `life` or `random`), `mix` (blends one frame into the next), `size_over_life` and `color_over_life` (`[t, value]` keys from 0 to 1, at most 64, that multiply size and color), `stretch` (`{ "time", "min", "max" }`: seconds of motion a particle is stretched along, and the pixel limits) and `trail` (`length` from 1 to 16 steps, `width` from 0 to 1). Without a texture particles are soft discs. The layer's `blend` (`add` suits glowing particles), `opacity`, `mask` and effects apply |

The simulation advances in fixed steps of 1/60 s, at most 15 per frame; a longer frame takes 15
equal steps up to a second, so particles keep real time down to one frame per second and never
catch up more than a second, and the same seed and the same frames give the same particles. While
no particle lives and none is emitted, the system skips ahead to its next timed burst. A paused
scene frees its particles and starts them again, prewarmed, when it resumes. A still render, a poster or a seek to the second T (`--time`) starts each system at `prewarm` plus T seconds of
simulation on its own clock, T times the `time_scale` of the `instance` of its layer, as it would be after T seconds live, in the same steps of 1/15 s and under the same 30 s and shared-budget caps as `prewarm`, so beyond 30 s
the system is as it is at 30 s (`reiwand-ctl scene render --simulate` instead plays the scene from 0 to T, frame by frame, for checking longer cycles). Only an explicit seek does this; a system revealed while the scene runs starts from its `prewarm` alone. It runs in compute shaders
on the GPU, and on the CPU with software rendering or where compute shaders are missing. A render
(`reiwand-ctl scene render` and the renders of the editor) always steps it on the CPU: on the GPU
threads race for the free slots and for the order of equal depths, so translucent particles could
blend in another order from one run to the next, and a render of a fixed seed gives the same pixels
every time. Particles
only move while their layer is shown and the scene runs, and a scene keeps rendering only while
particles live, are emitted or wait for their next timed burst. A scene holds at most 2 000
particles with software rendering, 5 000 on other devices without compute shaders and 10 000,
50 000 or 200 000 on the GPU depending on the device; the budget is shared by capacity, particles
are drawn smaller where the particles that can live at once would cover the screen more than 1, 1.5, 3 or 6 times over (and the cost of the scene counts those same particles): that is the slots of the system, or fewer when its emitters cannot fill them, namely the largest `rate` times the longest `life`, plus what each burst can add in that time (once, every `every` seconds, or again at each window of `periodic`) and at most `max_per_step` per step; a burst on a cue, a script that drives particles, a `follow` child or a `life` bound without a `clamp` counts every slot, a `rate` bound without one counts as 100 000 per second, a `spawn` or `death` child counts what its host can give it times the chance that each host event makes it, and the instance `count` and `life` of the layer are counted at their peak; and trails
are limited to 0, 4, 8 or 16 steps. Lower quality levels let only 75 %, 50 % or 25 % of the
particles be born and halve or drop trails. `reiwand-ctl scene bench` reports where particles run,
their slots, how many are alive at the end, the steps per frame and the simulation time after the
warm-up frames. When they run on the processor, the state of the output says `"particles": "cpu"`
(`"gpu"` otherwise) and the app, the tray, `reiwand-ctl` after showing the scene and the settings of
the phone say so, because fewer of them are shown.
