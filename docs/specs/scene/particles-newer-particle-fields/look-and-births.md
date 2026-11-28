# Looks and births

On this page: how particles look and where they are born.

## How particles look

- **Instances.** `alpha`, `size`, `brightness` and `color` multiply the look of the layer; `count` is
  the fraction of births and slots it keeps, `speed` and `life` multiply the starting speed and life
  and `time_scale` the speed of its time. A layer whose `instance` sets `count`, `speed`, `life`,
  `time_scale` or `points` moves particles of its own; the others share them. A system may `lock`
  what layers and scripts may not change.
- **Looks.** `render.brightness` multiplies the color with the brightness of the layer, `cycles`
  repeats the frames over the life and the `stretch` range limits how long a particle is drawn along
  its motion, in pixels.
- **Frames.** A texture whose asset has `rects` draws each frame from its own rectangle, so frames
  may differ in size; an animated picture is split into its own frames the same way.
- **Gradients.** `init.gradient` picks a color between the two stops around a random place and the
  first or last color outside them.
- **Trails.** `render.trail` keeps the last `length` positions of each particle, one per 1/60 s
  step, and draws a ribbon through them that is `width` times as wide as the particle was there and
  fades to nothing at its end, with the color and size the particle had there. It holds 16 bytes
  per particle and kept position, at most 16 steps for trails that are not long (4, 8 or 16 by device class, none with software rendering
  or on the CPU of the weakest devices, half of them or none at lower quality levels) and at most
  1 048 576 points per system. A layer with a `render` of its own draws as many of the kept
positions as its own `trail` asks for, or none.
- **Order.** `render.order` draws the particles from the bottom up in 256 steps of age, size or
  height, which matters with the `normal` blend; it costs a pass over the particles after each
  step that moved them.
- **Masks.** A `mask` collider turns the image into a field of distances, at most 16 384 cells over
  the canvas (the image itself when it is smaller), and pushes particles out of the opaque part
  along it, at the place where they went in. It works on world and local systems alike, in pixels
  of the canvas, and reads the image once when the scene loads.
- **Perspective.** With `perspective`, each particle keeps a depth `z` picked from `init.depth`
  that changes by `init.depth_speed` pixels a second, while forces stay flat. A particle is drawn at
  `c + (p - c) · f / (f + z)` and `f / (f + z)` times its size, where `c` is the center of the canvas
  and `f` the `focal` length, so positive depths go away and negative ones come closer; a particle
  whose distance from the eye `f + z` is not above `near` (`z <= near - f`) is not drawn, nor is a child born where it is drawn, so a larger `near` clears what flies past the eye. Trails follow the drawn positions, and `render.stretch` with `depth: motion` follows the on-screen velocity including the depth speed. Under the
  `normal` blend and without `render.order`, particles are drawn from the farthest to the nearest
  (in 256 steps of depth, on the GPU and on the CPU); `add` needs no order.
- **Cutouts.** `render.cutout: [start, end, opacity]` turns the alpha `a` of the texture (or of the
  soft disc) into `clamp((a - start) / (end - start), 0, 1) · opacity` before the alpha of the
  particle multiplies it, so a soft texture becomes a hard edge at `start`..`end`. A sprite whose drawn width or length (stroke and tilt included) is under one pixel of the picture it is drawn into, at every quality level, is drawn one pixel wide on that side with its alpha times the fraction of the pixel it covers, and samples the middle of its texture there, so a far or tiny particle fades by its area instead of lighting a whole pixel or flickering between pixels as it moves; one under a thousandth of a pixel is not drawn.
- **Orientation.** `render.orient` draws a sprite turned by its `rotation` (`screen`, prefilled),
  along its motion (`velocity`) or at `angle` degrees on the screen whatever the rotation of the
  particle and of the layer (`fixed`). World systems never turn with their layer.

## Where particles are born

- **Emission from a layer.** The `layer` shape draws the layer `id` (an image, a text, a mesh, a
  group, with its effects) once, at most 256 pixels on its longer side (512 when it covers more than
  1 024 pixels), keeps the opaque pixels whose alpha is above `threshold` (at most 4 096 of them,
  evenly picked) and births particles there with its color when `color` is set. The pixels are
  read again only when the drawing of the layer changes and `refresh` is `on_change`, at most four
  times a second; between readings they follow the position, turn and scale of the layer, and
  `inherit` adds the speed of the layer where each particle is born. `jitter` spreads births over
  that many pixels. The layer cannot hold the system it feeds, and while it was never drawn no
  particle is born.
- **Turbulent start.** `init.turbulent` replaces `speed` and `angle`: each particle starts at a speed
  picked from `speed`, along `forward` degrees turned by up to `scale` half turns by a smooth noise
  read at `phase + noise_speed · t + offset · u + s` (`t` the time of the system, `u` a number picked per
  particle, so `offset` spreads the directions of the particles born together, and `s` a whole number
  below 1024 taken from the `seed` of the system mixed with `clock.seed`, so each seed reads another
  stretch of the noise); with `audio` the
  level of that band multiplies `phase`. The noise is read again from its start every 4 096 units,
  so a long run never loses precision.
- **Fractal offset.** `init.offset_noise` moves each birth by up to `distance` pixels along the
  `fbm` of the shader helpers with `octaves` octaves, read at the place of birth times `scale` per
  100 pixels, drifting `noise_speed` units a second, and moved for each particle by `noise_scale`
  (0 keeps neighbors together, larger values scatter them); `sign` keeps the offset on one side as
  for circles. The drift also starts again every 4 096 units.
- **Around a point.** `around` births on a circle of `radius` around `at` (prefilled `p0`), one
  place after another in birth order: `count` places over `arc` degrees (the last one meets the
  first on a full turn), turning `speed` degrees a second with the angle of the control point.
  `mirror` goes back and forth instead of starting again, and `restart` starts the sequence again
  whenever the emitter starts.
- **Between two points.** `between` births along the line from `from` to `to` in birth order, at
  `count` places over the first `span` of it, back and forth with `mirror`. `arc.amount` bows the
  line by that fraction of its length to the left (negative to the right), `arc.side` scales the
  bow or, at 0, picks a side for each round of `count` births. `taper` shrinks births toward the
  ends: full size at `position`, moving `speed` lengths a second, and `size` times as large at the
  ends.
- **Remaps.** `init.remap` maps an input onto a starting value when a particle is born, in order:
  `x` from `in` becomes `t = (x - in[0]) / (in[1] - in[0])` (clamped to 0..1 with `clamp_in`),
  shaped by `sine` (`0.5 + 0.5 sin(2π t)`) or `noise`, mapped onto `out` (clamped with `clamp_out`)
  and `set`, multiplied, added or subtracted to `size`, `alpha`, `color` (its red, green and blue),
  `speed` (the length of the velocity), `life`, `rotation`, `spin` or `frame`. `daytime` goes from 0
  at local midnight to 1, `runtime` counts the seconds of the system (with a `noise` wave it starts again every 4 096 periods), `life` and `speed` are the
  picked ones, `random` is new for each particle and each remap, `distance:pN` is the distance in
  pixels to a control point, `audio:<band>` a band or its name and `cursor:x|y` the pointer across the
  screen from 0 to 1 (0.5 while it is away), which needs the pointer as any pointer input does.
