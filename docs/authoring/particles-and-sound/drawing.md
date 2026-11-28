# How particles are drawn

On this page: `render`, instances and how many fit.

## How they are drawn (`render`)

| Field | What it does |
|-------|--------------|
| `texture` | An image asset, which is split into frames with the `atlas` or the `rects` of the asset. Without it, soft discs. |
| `frames` | `time` (at `fps`, 12 by default), `life` or `random`. |
| `mix` | Blends a frame with the next. |
| `cycles` | How many times the frames repeat over the life (1 to 64, with `frames: life`). |
| `size_over_life` | List of `[t, value]` (up to 64): multiplies the size. | In the particles panel, "Along the life of a particle" gives a curve to drag the points and, for color, a gradient bar.
| `color_over_life` | The same, with colors. |
| `brightness` | 0 to 16, can follow a value. |
| `stretch` | `{ "mode", "depth", "time", "min", "max" }`, all five required (`depth`: `flat` computes the streak with the velocity without perspective; `projected` multiplies it by the nearness of the particle, so near drops carry long streaks and far ones short, like their movement on screen; with `relative` nothing changes because it already follows the projected side; `motion` computes the length and direction with the velocity of the place drawn on screen, including the velocity in depth `init.depth_speed`: a star field coming toward the camera is drawn as strokes that point away from the center and are longer the farther from the center; with `add` the length is the side plus `clamp(time·v, min, max)` pixels and with `relative` the side times `clamp(time·v, min, max)`, without scaling `min` and `max` by nearness; without `perspective` it is equivalent to `flat`): `add` adds pixels to the side (minimum and maximum in pixels); `relative` multiplies the side (minimum and maximum in times the side, so it can end up shorter than the side). Add `"anchor": "centre"` to put the middle of the stroke on the particle, half ahead and half behind (the editor's *Centre the stroke on the particle*; needs `stretch_anchor`); without it, or with `"tip"`, the front stays on the particle and the trail runs back. `segment` strokes are always centred. |
| `trail` | A trail: `length` (1 to 16 steps, or up to 64 with `segments`), `width` (0 to 1), `segments`, `subdivide` (0 to 7), `uv_scale`, `uv_scroll`, `textured`. |
| `rope` | Joins the particles in a strip, in birth order: `width`, `subdivide`, `smooth`, `uv_scale`, `uv_scroll`. |
| `order` | Which one is drawn on top: `newest`, `oldest`, `largest`, `smallest` or `lowest`. |
| `cutout` | `[start, end, opacity]`: turns the alpha of a soft texture into a hard edge. |
| `orient` | `screen`, `velocity` or `fixed` (with `angle`). |
| `lit`, `normal` | `lit`: lights them with the lights of the scene; `normal` is a `linear` normal map. |
| `refract` | `{ "normal", "units": "size", "amount", "color" }` (0 to 1, proportional to half the size) or `{ "normal", "units": "pixels", "shift", "color" }` (0 to 4096 canvas pixels, the same for any size and along the whole trail): they refract what is behind. `color` is required: `none` draws only the shift, `multiply` tints the shifted background with the color of each particle (with its texture, its `color_over_life` and its opacity; a white particle changes nothing) and `add` adds its color on top (a black one adds nothing). Trails and ropes paint their color the same. At Q2 the shift goes at half resolution and the color at full resolution; at Q1 and Q0 particles that refract are not drawn. |
| `shader` | An own effect with `stage: "particle"` (see [effects.md](../effects.md)). |

Quality levels `q2`, `q1` and `q0` let only 75 %, 50 % and 25 % of the particles be born, and
shorten or remove trails.

## Instances

`instance` on a particle layer changes the system only for that layer: `alpha`, `size`,
`brightness` and `color` multiply the look; `count` (0 to 4) the fraction of births; `speed` and
`life` multiply the initial velocity and life; `time_scale` the speed of time; `points` moves the
control points. All can follow values. A layer with `count`, `speed`, `life`, `time_scale` or
`points` moves its own particles; the others share those of the system. With a `count` value above
1, the layer reserves that many times `capacity`: if you link it to a property, give it a `clamp`.

## How many fit

The budget depends on the computer: 2,000 particles with software drawing, 5,000 on computers
without compute shaders and 10,000, 50,000 or 200,000 on the graphics card depending on its class.
Trails are limited to 0, 4, 8 or 16 steps. At the lowest quality level fewer fit. `reiwand-ctl
scene bench` says where they run, how many are alive and how long the simulation takes; if they run
on the processor (a card without compute shaders), the status says so and fewer are seen.

The [`rockets` layer of the example](../examples/advanced-particles/scene.json) makes a rocket with a
trail fly and burst into stars, and there are fireflies in the air.

# Sound

There are two ways to use sound: put it (a `sound` layer) and hear it (values that follow the sound
that plays on the computer).
