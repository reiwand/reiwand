# Shaders, colliders and limits

On this page: particle shaders, keeping distance, flocks, colliders, ropes, long trails, limits and instances.

## Shaders, distance, flocks and colliders

- **Particle shaders.** `render.shader` names a package effect with `stage: "particle"`, checked and
  limited like every package shader. Its `fn particle(p: Sprite) -> Sprite` runs for each corner of
  each sprite with the center, velocity, corner, uv, color, size, rotation (degrees), age and life,
  and may change all of them; the engine then draws the corner. It may read `frame` and the noise
  and color helpers. A shader that cannot be built is left out like any other package effect and
  the particles are drawn without it. Its module and pipelines are built away from drawing within
  the build time of package shaders, its time on the GPU counts like the time of a package effect,
  and one that takes too long to build, stopped the graphics card or is too slow is left out for the
  run the same way. Ropes and trails are drawn as usual.
  The WGSL may also define `fn pixel(p: Pixel) -> vec4<f32>`, which runs for each pixel of a sprite
  before the alpha. `Pixel` is `sample: vec4<f32>` (the texture, or the soft disc, after `cutout`),
  `color: vec4<f32>` (the color and alpha of the particle after `color_over_life`, the layer tint and
  the opacity, without the brightness and not multiplied by its alpha), `brightness: f32` (the layer
  brightness times `render.brightness`), `alpha: f32` (`sample.a * color.a`), `uv: vec2<f32>`,
  `age: f32` and `life: f32`. It returns the straight color and alpha; the engine multiplies the color by
  that alpha and blends as always. Without `fn pixel` the result is `sample * color`, with the
  brightness already in the color, and every pixel is as before. With it, the brightness is not
  multiplied into the color of the sprite (a `fn particle` sees `color` without it), so the shader
  chooses where to apply it, and it clips before the alpha:
  `fn pixel(p: Pixel) -> vec4<f32> { let lit = min(p.sample.rgb * p.color.rgb * p.brightness, vec3<f32>(1.0)); return vec4<f32>(lit, p.sample.a * p.color.a); }`.
  A white sprite at alpha 0.5 and brightness 2.5 then stays at 127 over black, and an additive
  halo reaches the same level in all three channels. The function is checked and limited with the
  package shader (its signature, its work and its build time), may use the same helpers and `frame`, and
  reads no textures. If it does not compile the whole effect is left out. Every system that names the
  effect uses it; `lit` lights the result of `fn pixel`, `refract` only reads its alpha, and trails,
  long trails and ropes do not call it: they take the brightness as before, unclipped.
- **Keeping distance.** `keep_distance` pulls each particle toward its target or pushes it away, as
  a spring that rests `distance` pixels from it: the acceleration is `strength · (d − distance)` along
  the line to the target, in pixels a second squared for each pixel. `keep_between` pulls each particle
  toward the nearest point of the segment from `from` to `to` by `strength` times its distance from
  it. Neither damps the motion, so a `drag` settles particles on the ring or the line.
- **Remap operators.** A `remap` force applies the mapping of `init.remap` every 1/60 s step, with
  the `age` of the particle as one more input and `random` fixed for each particle and force. `set`
  replaces the value, `add` and `sub` add or subtract the mapped value each second and `mul`
  multiplies it by the mapped value each second (`v · y^dt`). The `window` of the force fades it in
  and out: `set` then moves the value only that fraction of the way.
- **Flocks.** `boids` steers each particle with its neighbors closer than `radius`: `separation`
  pushes it away from them (their mean direction away, fading to nothing at `radius`, times
  `radius`), `alignment` turns its velocity toward their mean velocity and `cohesion` pulls it toward
  their mean position, each in pixels a second squared; `max_speed` (0 for none) caps its speed.
  Neighbors are found in a hash grid of cells `radius` pixels wide rebuilt every step (in compute
  shaders on the GPU, on the CPU in the order of the particles; the GPU fills a cell in any order,
  so where a cell holds more neighbors than count the two paths may pick different ones), and at
  most 32 of them count at quality 3 and 2, 16 at quality 1 and 8 at quality 0, or 8 and 16 on the
  two lower device tiers; a particle looks at no more than four times that many. Neighbors at the
  same place count too and push each pair apart along a direction picked for the pair, so a flock
  born at one point spreads out. The first `boids`
  force of a system sets the flock, which holds at most 8 192 particles, and 2 000 on the CPU.
- **Colliders.** `box` keeps particles out of the box, or inside it with `inside`; it turns and
  scales with the layer. `segment` stops particles crossing the line from `from` to `to` from either
  side; particles that go around its ends pass. `circle` with `inside` keeps particles within the
  circle. `layer` keeps particles out of the opaque part (alpha above 0.5) of a layer that does not
  hold the system: the engine draws the layer as for emission, turns it into a field of at most
  128 × 128 distances, rebuilt only when the drawing of the layer changes (at most four times a
  second), and the field follows the position, turn and scale of the layer between readings; while it
  was never drawn nothing collides. `stick` stops a particle where it touches and keeps it there
  until it dies, without forces or other colliders. `kill` ends the particle where it touches, which
  starts the `death` children of the system like any other death.

## Ropes, long trails and limits

- **Ropes.** `render.rope` joins the living particles of a system's own emitters into one strip in
  the order they were born, from the newest, with `subdivide` (0 to 7) points between two particles
  on a Catmull-Rom curve (`smooth`, prefilled) or a straight line. It is `width` pixels wide, times
  `size_over_life` and the size an `oscillate` force gives, and takes the color of the particles
  with `color_over_life`. With a `texture`, the strip shows it along its length: `uv_scale` times per
  particle, moving `uv_scroll` times a second. The strip is built by a compute shader after the last
  step of a frame (and the same way on the CPU), with at most 4 096 segments (1 024 and 2 048 on the
  two lower device tiers) and fewer points between particles at quality 1 (half) and 0 (none). Ropes
  are not drawn for systems born from other particles. One particle emitted per step
  (`max_per_step: 1`) keeps the rope even.
- **Long trails.** A trail longer than 16 steps, or with `segments`, `subdivide`, `uv_scale`,
  `uv_scroll` or `textured`, keeps `segments + 1` positions (`length` without `segments`), one every
  `(length − 1) / segments` steps, so it covers `length` steps with fewer points, and draws a smooth
  ribbon through them with `subdivide` (0 to 7) points between two kept positions. It keeps at most
  64 positions, 32 or 16 on the middle device tiers and none on the lowest, spacing them further
  apart when it keeps fewer; quality 2, 1 and 0 draw half, a quarter or none of them, and quality 1
  and 0 fewer points between them. With `textured` and a texture, the ribbon shows it along its
  length, `uv_scale` times from head to tail, moving `uv_scroll` times a second.
- **Limits.** A scene simulates at most 64 systems, counting children and layers with particles of
  their own, and their capacities share the 200 000 particles. `prewarm` runs in steps of 1/15 s,
  at most 30 million particle steps for the whole scene, and stops once it takes more than 200 ms.
  On the processor, when a frame has more than one step to take for some systems (a seek, or a
  jump of the clock), those systems share the 200 ms evenly, in their order, each passing on what it
  leaves to the next, so a later system is not left without particles; a frame of one step per
  system keeps the whole 200 ms for each, as before.
  A child never starts more particles in a step than it has room for.

## Instances of a particles layer

A `particles` layer may also set `instance` (`alpha`, `size`, `count`, `speed`, `life`, `time_scale`,
`brightness`, `color` and `points`, which may be bound and multiply or move what the system does for
that layer only, unless the system locks them; `count`, 0 to 4, multiplies the births and the room of the system, and above 1 the layer reserves that many times its `capacity` when the scene loads: the literal value, the upper end of the `clamp` of a binding, or 4 for a binding without one, so a count bound to a property should clamp it) and a `render` of its own that replaces the one of
the system (`particles2`). Each end of the `init` ranges of numbers (`life`, `speed`, `angle`,
`size`, `rotation`, `spin`, `frame`, `alpha`, `radial`, `depth`, `depth_speed`) and the numbers of
emitter shapes (`radius` and `inner` of `circle`, `threshold` of `layer`, `arc`, `radius` and
`speed` of `around`, `span` of `between`) may be bound like any value, for instance
`"size": [2, { "audio": "bass", "kind": "relative", "unit": "db", "scale": 8 }]` (`particle_values`), and are read every frame and
kept within the limits of the field; colors, velocities and the `threshold` of `mask`, read when the
scene loads, stay literal. Births may also follow the sound or the pointer through the `rate` of the emitter, the
`instance` multipliers of the layer or control points.
