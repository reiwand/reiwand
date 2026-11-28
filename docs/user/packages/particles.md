# Particles

The desktop and the phone already draw the new particle features, the same on the GPU as on the
CPU. This page covers emission, initializers, operators, child systems, trails, collisions,
perspective, flocks, ropes and script control. The rest of the list in
[Advanced scenes](advanced-scenes.md) still asks for a newer version.

## Emission

- World space: particles are born with the position, rotation and scale of their layer and then do
  not follow it.
- Up to eight control points, moved by values, the cursor, the layer or scripts.
- Hollow boxes and ellipses, or half circles with an anchor point.
- The duration, the delay, emission by random windows, the cap of one particle per step and waiting
  for something to sound.
- Emission from the visible pixels of another layer with its color and its movement (`layer` shape,
  read at 256 or 512 pixels and again only when the layer changes).
- Birth in turns around a point or between two, with arc and tuned ends.

## Initializers

- With exponent, own alpha, HSV and palette.
- Vector velocity, outward velocity, and velocity inherited from the cursor or a point.
- Turbulent initial velocity, and the fractal offset of birth.
- The initial color as a gradient (`init.gradient`, up to 8 stops).
- The ends of the initial ranges and the radius of circles bound to values, which are read on each
  frame.
- The remapping of values at birth (`init.remap`, up to eight), with the time, the time, the life,
  the speed, chance, the distance to a point, sound or the pointer.

## Operators

Operators have life windows. They include:

- oscillating position, alpha or size;
- limiting velocity;
- rotation torque and brake;
- braking near a point;
- reach and falloff of attractors;
- vortices with a ring;
- turbulence with offset and mask;
- keeping a distance from a point, or approaching the segment between two (`keep_distance`,
  `keep_between`);
- remapping applied on each step (`remap` force, also with age).

## Flocks

Flocks (`boids`) have separation, alignment, cohesion and maximum speed, with a hash grid rebuilt on
each step.

| Limit | Value |
| --- | --- |
| Particles on the GPU | Up to 8,192 |
| Particles on the CPU | 2,000 |
| Neighbors | 32, 16 or 8, depending on the quality and the computer |

Particles born at the same point are separated.

## Collisions

Particles collide against:

- boxes, from inside or outside;
- segments;
- the inside of a circle;
- the opaque part of another layer (a 128 × 128 distance field rebuilt only when the layer
  changes);
- the opaque part of an image (`collide` with `mask`, converted on load into a distance field of up
  to 16,384 cells).

A response can leave the particle stuck (`stick`) or kill it, which counts as a death for child
systems.

## Child systems and settings

Child systems can be static, at birth, at death or following each particle (two generations at
most). Children inherit the color, size, alpha and velocity of the parent particle.

- The children of a system with perspective are born at its depth or where it is seen drawn.
- When inheriting the velocity they also take the one it carries in depth.

Per-layer settings cover alpha, size, amount, speed, life, rate, brightness, color and points, with
lock in the system.

## Drawing

- Brightness, cycles and the bounded stretch of the drawing.
- The draw order (`render.order`): the newest, the oldest, the biggest, the smallest or the lowest
  on top, in 256 steps.
- Sprite sheets with rectangles of different size.
- The perspective of particles in 2D scenes (`perspective`, with the depth and its speed at birth,
  from far to near with normal blending).
- The alpha cutout of the texture (`render.cutout`).
- Orientation toward the screen, the movement or a fixed angle (`render.orient`).
- The particle shaders of a package effect (`render.shader`), with the same checks, the same build
  time and the same GPU time limit as the other package effects. They are left out, without
  stopping the drawing of the particles, if they cannot be built or are too slow.

See [Lights](lights.md) for lit and refracting particles.

## Trails

Trails with history: each particle keeps its last positions, up to 16 steps of 1/60 s, and leaves a
ribbon that narrows and fades with the color and size it had at each point.

| Computer | Steps |
| --- | --- |
| Depending on the class of computer | 4, 8 or 16 |
| Low quality levels | Fewer or none |
| Software rendering | None |

Long trails keep up to 64 positions spread over `length` steps (`segments`), with subdivision,
width, texture and its scale and offset. They keep 16 or 32 positions on intermediate computers and
none on the most modest.

## Ropes

Ropes (`render.rope`) join the particles in the order they were born with a Catmull-Rom strip built
in a compute shader. A rope has up to 4,096 segments, 1,024 and 2,048 on the more modest computers,
and fewer subdivisions at low quality.

## Sound links

Sound links take:

- band ranges in 16, 32 or 64 (`audio: [0, 3]`, `of`, `reduce: mean | max`) of one channel;
- bounds (`bounds`) and exponent;
- a different smoothing when rising and falling (`smooth: { up, down }`).

These also work for the cursor, properties and scripts. A scene can set with `audio` its attack,
its decay and whether it follows the system volume (`gain: fixed`) or adjusts itself (`auto`).

## Scripts

Scripts control particles with `particles(layer)`: `play`, `pause`, `stop`, `playing`, `emit(n)`,
`point(n, [x, y])`, `instance`, and `from(n, layer)` and `wall(n, layer)`.

`from(n, layer)` and `wall(n, layer)` make the emitter or collider `n` of layer type take another
layer, also a copy of a template (`null` goes back to that of the system).

## Limits

- A scene simulates at most 64 systems, counting children and layers with their own particles,
  within the 200,000 particles.
- Prewarming reaches 30 s in steps of 1/15 s.

> **Note:** When the particles move to the processor, the state of the screen says so
> (`"particles": "cpu"`), and the app, the tray, `reiwand-ctl` when showing the scene and the
> phone's settings warn that fewer are shown.
