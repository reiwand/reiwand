# Space, forces and children

On this page: world space, control points, shapes, starting values, forces, emitters and children.

## Space, points, shapes and starting values

The newer fields behave as follows.

- **World space.** A `world` system moves in canvas pixels: the transform of its layer places,
  turns and scales particles, their speed and their shapes when they are born, and they are drawn
  without it, so moving the layer afterwards leaves them where they are. Gravity and the other
  forces act along the directions of the canvas, and targets, colliders and control points of the
  layer move with it. Each layer that draws a `world` system moves particles of its own.
- **Control points.** `at` is in pixels of the layer, or of the canvas with `space: world`; a point
  that follows the cursor goes back to the center of the canvas when the pointer leaves. A shape
  with `at` is drawn around that point, turned by its `angle`. The `points` of a layer's `instance`
  and the `point(n, [x, y])` of scripts move points `p0` to `p7` for that layer; a `null` entry
  of `points` leaves that point to the system.
  Without `points`, `p0` is the pixel (0, 0) of the layer, the top-left corner of its frame and not
  the middle of the canvas that the layer turns around; it moves, turns and scales with the layer in
  both spaces, exactly like a target or center given as `[0, 0]`. To aim at the middle of the system,
  give `p0` an `at` of half the canvas.
- **Shapes.** `inner` cuts a box of that size out of the middle of a `box`, which births fill evenly.
  `scale` stretches a `circle` into an ellipse and `sign` keeps its positive (1) or negative (−1)
  half along x and along y. A `sphere` draws the three coordinates of a point of the ball and uses
  its z as depth only with `perspective`: the depth of a birth is `init.depth` plus `depth` of the
  shape plus the z of the point (the same rule as `around`, so a system that wants only the ball
  has no `init.depth`, or one of `[0, 0]`), and a system without `perspective` drops z, which leaves
  births spread as the picture of a ball seen from the front, thicker towards the middle than a
  disc (the share of births inside 0.6 of the radius is 0.49, against 0.36 for a disc). The
  x and y of the point go through `space` and the control point `at` like those of a `circle`,
  so they are turned and scaled with the placement; z is not. `push` moves each point `v` of the
  ball (after `inner` and `sign`) to `v + push·v/|v|`, the length of `v` taken over x, y and z
  together, so a point keeps its direction and side and the middle gets an empty hole; a point at
  the very middle (`v = 0`) stays there. With `rz` 0 the result is a disc with a round hole of
  radius `push` whose births keep the spread of a ball seen from the
  front: the share at a normalised distance ρ from the hole (0 at the hole, 1 at the edge) has the
  density 3ρ√(1 − ρ²). Unlike `inner` of a `circle` (even by area, so loaded towards the edge) or
  of a `sphere` (a shell that leaves nothing when the ball is flat), it keeps the hole with any
  depth radius. The radius for the speed given by `radial` is measured from the
  middle in the plane. `tilt` of `init` and the turn of `around` do not depend on it. Both paths
  draw the same point from the same random numbers.
- **Starting values.** A range `[min, max, exponent]` picks `min + (max − min) · u^exponent`.
  `alpha` replaces the alpha of the color. `hsv` picks the hue in degrees (with `steps`, one of that
  many hues from one end of the range to the other), the saturation and the value from 0 to 1
  (prefilled `[1, 1]`);
  `palette` picks one of its colors and moves its hue by up to `noise[0]` half turns and its
  saturation and value by up to `noise[1]` and `noise[2]`. `radial` adds a speed away from the center
  of the shape (the center of the canvas for `edge` and `mask`, the direction of `angle` for a
  `point`), and `inherit` adds the velocity of the pointer or of a control point, times a number
  picked from `speed`.

## Forces, emitters and children

- **Forces.** `window` fades a force in between its first two fractions of the life and out between
  the last two. `attractor`, `vortex` and `orbit` weaken with distance as the inverse square, the
  inverse and not at all by default; with a `radius` they do nothing beyond it and weaken as
  `1 − d / radius` (`linear`) or its square (`inverse_square`). A `vortex` with `inner` and `outer`
  turns fully inside `inner` and fades out at `outer`, and its `ring` pulls particles toward a circle
  of that `radius` with `pull`, fully at `width` pixels away. `axis` is required in every `vortex` and
  every `around` shape: three numbers, a direction of length 1 (between 0.999 and 1.001; the editor
  pre-fills `[0, 0, 1]`, which is the plain turn on the screen). A `vortex` accelerates particles
  tangentially around the line through `center` along `axis`, and `inner`, `outer`, `falloff` and `ring`
  measure the distance to that line; an `around` shape places births on the circle of its `radius` that
  lies in the plane perpendicular to `axis`. Depth (z, towards the viewer) only moves with
  `canvas.perspective`: without it the z part of the axis is ignored and the particles see the axis
  projected on the screen. `turbulence` scrolls its noise `speed`
  pixels per second to the right, changes it `time_scale` times as fast and scales its x and y by
  `mask`. The `turbulence` force uses a periodic noise of 256 cells and the host wraps its phase,
  so it does not jump with the clock left on for days; with `smooth` and `gyro`,
  `depth_parallax` also follows the phone's tilt. `noise_field` picks one number `u` from 0 to 1
  for each particle (the random number every particle draws when it is born) and gives it the
  strength `s = strength.min + u·(strength.max − strength.min)` (pixels per second squared, negative
  pushes the other way) and the phase `φ = phase.min + u·(phase.max − phase.min)` (pixels). It reads
  the noise at `p = (x + φ + t·drift, y + φ + t·drift, z + φ + t·drift)·scale`, `t` being the
  seconds of the clock of the system and `drift` in pixels per second, and accelerates the particle by
  `s·mask·(N(p.x, p.y, p.z), N(p.z, p.x, p.y), N(p.y, p.z, p.x))`, times its `window`, read once per
  simulation step at the position and clock of the start of the step and integrated as a plain
  acceleration: the step adds acceleration·step to the velocity (1/60 s live), with no damping or
  scaling of its own (the `drag` force and `cap` of the system act on the sum of the forces as
  always), and the position then moves by velocity·step; the third part goes into the speed in depth
  the same way. `N` is a
  3D simplex noise of one octave over the standard table of 256 permutations `perm`: the point is
  skewed by 1/3 into the cells `i`, `j` and `k` of x, y and z and unskewed by 1/6, each of the four
  corners of its simplex adds `(0.6 − d²)⁴` times the dot product of its offset `d` with one of the
  12 edge gradients (nothing where `d²` is above 0.6), and the sum is multiplied by 32, so it stays
  within −1 and 1. The corner at cell offset `(a, b, c)` takes the gradient
  `perm[i + a + perm[j + b + perm[k + c]]] % 12`, all indices taken modulo 256, except the last
  corner `(1, 1, 1)`, which takes it from `perm[i + 1 + perm[j + 1 + perm[k]]] % 12`. Unlike the curl of `turbulence`
  it has divergence, so particles gather where it converges. `z` is the depth with `perspective`,
  and the third part moves the depth; a system without `perspective` reads the noise at `z = 0` and
  ignores the third part of `mask`. The noise repeats every 128 units along the diagonal, so the
  host wraps `t·drift·scale` by 128 and it does not jump with the clock left on for days. The GPU
  and the processor compute the same noise. `oscillate` picks a frequency, a scale and a phase for each particle (each from its own random number, or with `random: "shared"` all three from one) and moves it around
  where it would be without drifting (x and y a quarter of a turn apart, scaled by `mask`), fades its
  alpha by up to the scale or grows and shrinks its size by that fraction. `cap` limits the speed,
  `torque` adds degrees per second to the spin each second, `spin_drag` slows the spin and
  `slow_near` multiplies the speed every 1/60 s by `factor` near its target, fading to no effect
  between `inner` and `outer`.
- **Emitters.** `delay` and `duration` count from the start of the system or from the last
  `play()`; a finished emitter, with no particle alive, lets the scene sleep. `periodic` alternates
  emission and pause for times picked from the seed. One-time bursts fire whenever the emitter
  starts: at first, after its `delay`, at each `periodic` window and after `play()`. `max_per_step`
  keeps the rest of the births for the next steps.
- **Children.** `static` children emit on their own around the origin of their parent. `spawn` and
  `death` children start the one-time bursts of their emitters (one particle when they have none, at
  most 1000) where each particle is born or dies, in the same step, and `follow` children are born
  `rate` times a second at each living particle, ride along with it unless their system is `world`,
  and die with it. Shapes of children are placed around the particle, moved by `offset` and turned
  by `angle`; `scale` also scales their sizes. `probability` is drawn once per particle and `max`
  limits the children born in a step; at most 1024 births and 1024 deaths of a system start children
  in a step. `points_from` takes the control points of the parent. A child that is born from its
  parent (`spawn`, `death` or `follow`) cannot start particles from a `layer`.
- **Children in depth.** A child of a system with `perspective` that has `perspective` of its own is
  born at the depth of its parent plus its own `init.depth`, and a `follow` child moves in depth
  with its parent; with `velocity` in `inherit` it also adds the speed in depth of its parent to its
  own `init.depth_speed` (and takes it every step with `follow_inherit`); a child without `perspective` is born where its parent is drawn, at
  `f / (f + z)` times its size and inherited speed, and a `follow` child moves as far as its parent
  is drawn to move.
