# Particle tools

On this page: particle tools on the preview, the system inspector, colliders and forces.

## Particle tools on the preview

When you choose a particle system or a particle layer, the preview draws its tools on top: each
emitter shape with handles (the point, the corners of a box, the center and radius of a circle, also
around the control point they are tied to, the ends of a line; for `around`, the center, the radius
and the end of the arc, with a point at each birth place; for `between`, its ends, a handle that
curves the line and another that slides the widest point of the taper; for `layer`, the layer's
outline and the pixels that pass the threshold), the control points `p0` to `p7` as markers that are
dragged and a ring with a handle for the perspective's focal length. The turbulent velocity draws
the fan of directions the noise can give, the exact ones of the first particles that are born now
with the seed the preview runs with (`clock.seed` or, if it is `null`, one the editor chooses for
the session and passes to the preview; *Keep this seed* writes it into `clock.seed`, and if the
start follows a sound band a control sets how much sound there is for the rays, which turn with it
as in the engine; `reiwand-ctl scene act DIR particles` draws the same) and an arrow with a handle
that rotates it and another that opens or closes the fan; the fractal offset draws arrows from a
grid of birth places to where the noise takes them, computed as in the engine, and a ring with a
handle for the distance. Attractors, vortices, orbits and braking near a point draw a dashed ring
for each reach (radius, inner, outer and the vortex ring) with a handle that changes it, and a
handle at its center; the distances and flocks section adds them and shows the point they follow,
their strength and a slider per reach. The system's inspector offers the type of each emitter, which
control point or the pointer `around` and `between` follow, a list of the layers it can emit from
(never one that contains the system) with the threshold, its color, its motion and when it is read
again, the control points with their angle dial, the remaps of initial values with their inputs and
outputs, two-handle ranges, limits, operation, wave and the curve of the output against the input (a
curves component that other tools can also use), the alpha cutout with its range, its opacity and the
ramp it gives, the fields of the turbulent velocity (direction, turn, noise speed, spread, phase,
speed and sound band) and of the fractal offset (distance, size, detail, spread, drift and sides),
the orientation with a dial, the perspective with its focal length and the depth ranges, what each
child takes from its parent particle and the package's particle shaders with the notice that the
engine checks them and may leave them out. From the terminal `reiwand-ctl scene act DIR particles`
does the same with `shape`, `control-point`, `remap add|set|remove`, `cutout`, `orient`,
`perspective`, `emit-from-layer`, `inherit` and `shader`, and `handles` prints what the preview
draws.

## The system inspector

A system's inspector starts with the list of the scene's systems, where each one is renamed (the
layers and systems that use it follow it; scripts that name it in quotes are not rewritten and the
app and `reiwand-ctl` warn about each one), opened or removed, and one is added without a layer.
The initial ranges (life, speed, direction, size, spin, rotation and alpha) have a two-ended slider,
a curve that distributes the values toward one end with its drawing and a selector per end that
links it to a package setting. The initial colors come from two colors, from hues with their
saturation and brightness (with steps), from a palette with its noise or from a gradient bar where a
click adds a stop and dragging it moves it. Each force can act only on part of the particle's life,
with a four-handle shape that says where it appears, acts and fades. Each emitter has its rate (a
live slider, with *Fixed rate* if it follows keys or a link) and a time strip that shows when it
emits: the delay, the end of the duration, the emission and pause spans (the shortest in a row above
and the longest in another below) and the bursts, with handles that drag the delay, the duration, the
length of the spans and the separation between bursts (the strip's scale stays still while dragging),
and below it the size of the burst and whether it comes out with time, the rate or a click, follow
the pointer, emit only with sound and the velocity it takes from the emitter. The *Drawing order and
what layers may change* section chooses which particles stay on top and how many times the frames
repeat in a life, and which values the particle layers may change (the rest stays locked). When you
choose a particle layer, the inspector offers its own values over those of the system (opacity,
size, part of the particles, speed, life, time pace, brightness and tint, each with its return to the
system's) and the control points that are dragged on the canvas become that layer's alone. From the
terminal, `reiwand-ctl scene act DIR particles` adds `emitter N` with `--burst`, `--every`, `--on`,
`--periodic`, `--duration`, `--delay`, `--follow`, `--audio-gate` and `--inherit`,
`window N A,B,C,D|none`, `rename NEW`, `start --hsv|--palette|--gradient`, `render --cycles --order`,
`settings --lock`, `force turbulence --time-scale --mask` and the forces `noise`, `cap`, `torque`,
`spin-drag`, `vortex`, `orbit` and `slow-near`. The `start` ranges accept `MIN,MAX,CURVE`, with
`prop:ID` at one end to link it to a setting and `_` to leave it as it is; without a curve the
existing one is kept and `1` removes it (for example `start --life 1,prop:life,2`).

## Colliders and forces on the preview

The preview also draws the system's colliders with the color of their response (red where the
particles die, green where they stick) and handles for each type: the canvas edge, the plane with
its point and the side they stay on, the circle with its center and radius, the box with two corners,
the segment with its ends and the layer with its outline and its distance field; a ring with the
distance that is kept around its target, or the line between two points, with handles that move the
target, the ends and the distance. The inspector offers each collider's response (bounce, slide,
stop, stick or remove) with the bounce and the friction, whether a circle or a box holds the
particles inside, and the layer they collide with; the targets and sliders of the distances; the
flocks with the neighbor radius, the separation, the alignment, the cohesion and the maximum speed;
the remap operators with the same curves editor and age as input; the rope and the trail with their
fields and a texture selector, and a notice that leaves one particle per step in the emitters that
would release several, so that the rope comes out even. The particle budget simulates the system on
the processor up to the timeline's instant and says how many are alive out of its capacity, the
slots on the GPU and on the processor of each equipment class (phones and old computers fall into
the two lowest) and what each quality level keeps of births, neighbors, trail and rope points; it can
draw on top the simulated particles, the rope and some trails, and when you click a particle it marks
the radius of its neighbors. A quality selector from Q0 to Q3 limits the preview's and *Restart
systems* starts only their particles over, without rebuilding the scene. In the terminal,
`reiwand-ctl scene act DIR particles` adds `collider add|set|remove|response`,
`keep distance|between|remove`, `boids`, `operator remap add|set|remove`, `rope`, `trail`,
`texture`, `turbulent`, `offset`, `pace` and `budget` (`rope` also warns about the emitters that pile
up the rope), and `reiwand-ctl scene preview --quality Q --restart` does the same with the preview.
The rest of the system also has subcommands: `create` (a system without a layer, a copy of another
with `--from` or a child of another with `--parent` and `--on`, like *New child system* of the
children panel), `delete`, `settings` (capacity, seed, prewarm, space and emitter pace), `start`
(initial ranges of life, speed, angle, size, spin, color, alpha, speed in X and Y and radial speed),
`force gravity|drag|turbulence|oscillate|attractor|remove`, `child add|set|remove`, `size-over-life`,
`color-over-life` and `render` (stretch, atlas frames at random, by time or by life, atlas pace and
brightness above white).
