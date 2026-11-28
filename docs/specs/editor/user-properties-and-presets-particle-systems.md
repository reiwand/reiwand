# Particle systems

The *Particle systems* button under the layers list selects `/scene/particles`, and the inspector
shows the systems panel: every system of the scene, with or without a layer, in a field that renames
it, with buttons to edit it, copy it (`create` with `from`), add a system that it starts (`create`
with `parent`), and remove it. A rename that leaves scripts naming the old name shows a warning with
each `file:line` and the button *Rewrite the exact names*, which runs `scripts`. The panel also shows
under a selected system. `scene act DIR particles` prints the same warning on stderr,
with the action to run: `scene act DIR particles '{"scripts":{"from":"OLD","to":"NEW"}}'`. The query
`{"script_uses": {"name": ID}}` of `rig` lists the `path` and `line` where a script names an id.
Each emitter section has *Most particles started in one step* (`pace` for that emitter) next to its
rate, bursts, timeline, pointer, sound gate and inherit controls.

Selecting a particle system, or a particles layer, draws its tools over the preview in pixels of
the canvas, through the transform of the layer: every emitter shape with handles (the point; the
corners of a box and the center and radius of a circle, around the control point they are anchored to;
a `sphere` as an ellipse with handles on its center and its two radii in the plane and a second, dashed ring for `inner`, plus a dotted ring at `push` from the middle with a handle on its top when the shape has one (the rings in the plane then sit `push` further out, where the births are) (the emitter panel adds a side view, x against depth, whose handles drag the center depth and the depth radius, a hollow-core slider, a push slider (pixels; 0 takes `push` out of the shape) and three buttons that cycle each direction through both sides, one side or the other; `handles` returns `center`, `east`, `south`, `inner_east`, `push_north`, `radius`, `depth`, `inner`, `push` (0 without it) and `sign`; the shape is written through the `particles` action `shape` with every field, and `push` when it is set);
the ends of a line; for `around`, the center, a handle for the radius and one for the end of the
arc, with a dot at each place a birth takes; for `between`, its two ends, a handle that bows the
line and one that slides the widest point of the taper, with dots sized as the taper sizes them;
for `layer`, the outline of the layer and the pixels above the threshold of an image, mesh, text
or group layer, or only its outline where none can be read (a model layer, or dynamic text such as
a clock), the control points `p0` to `p7` as markers to drag, and a ring
whose handle sets the focal length of the perspective. A turbulent start draws, from the middle of
the first emitter, a fan of the directions the noise may give, the exact directions of the first
particles born now with the seed the preview runs (`clock.seed`, or while it is `null` one seed the
editor picks for its session and gives the preview, shown by the controls and printed as `seed` by
`handles`; *Keep this seed* writes it to `clock.seed`, and while the start follows a sound band a
slider sets how loud the band is for the rays, which turn with it as the engine turns them) and an arrow with a handle that turns its direction and one on the edge of the fan that widens
or narrows it; a fractal offset draws arrows from a grid of birth places to where the noise moves
them, computed as the engine does, and a ring with a handle for the distance. An end of `around` or `between` that names a
control point moves that point. The inspector of the system lists its emitters with a picker of
the shape, of the control points or the pointer that `around` and `between` follow and of their
count; a picker of the layers it may emit from (never one that holds the system), with the
threshold, its color, its motion and when it is read again; the control points with a toggle to
follow the pointer and a dial for their angle; the remaps of the starting values, each with its
input and output, two-handle ranges for `in` and `out`, its clamps, operation and wave, and the
curve of the output over the input; a cutout with a two-handle range, its opacity and the alpha
ramp it gives; toggles and sliders for the direction, turn, noise speed, spread, phase and speed of
a turbulent start with a picker of the sound band that moves it, and for the distance, size,
detail, scatter, drift and sides of a fractal offset; the orientation with a dial for its angle; the perspective with its focal length
and two-handle ranges for the depth and its speed; what each child takes from its parent particle;
and the particle shaders of the package with a warning that the engine checks them and may leave
them out. The curve is a component of its own that plots any `{ points, x, y }` the editor computes,
so other tools can draw their curves with it. `reiwand-ctl scene act DIR particles` prints
the same data: the transform of the layer and its inverse, the points, each emitter with its
handles, the curves and the lists the pickers offer.

The preview also draws the colliders of the system, colored by their response (red where particles
die, green where they stick): the border of the canvas, a plane as a line with a handle on its
point and one that turns the side particles stay on, a circle with handles on its center and radius,
a box with handles on two corners and a segment with handles on its ends, dashed when they keep
particles inside, a layer as its outline with its field of distances over it (computed from the
picture of an image layer and from the pixels of a text, a mesh texture or the children of a group,
which a group shows approximately) and a mask as the canvas rectangle with the field of distances of its
picture stretched over it (a layer has a handle in the middle of its outline, `grip`, which retargets
the collider to the topmost layer under the point where it is dropped, and a mask has one on the
top edge, `grip`, which sets the opacity from which it is solid by its position along the edge; the
border of the canvas has no field to change and no handle) (`colliders` of the query: `field` is a list of `[x, y, distance]`, negative
inside, and `approximate` is true when nothing could be read). A force that keeps a distance
draws a ring of that distance around its target, with a handle on the target and one on the ring;
one that keeps particles between two points draws the line between them with a handle on each end;
an `attractor`, a `vortex`, an `orbit` and a `slow_near` (the `pulls` of the query: `index`, `kind`,
`field`, `ref`, `at`, `axis`, `angles` (tilt from z and turn, in degrees), `falloff` and `rings` of `{key, value, px, at}`) draw a dashed ring for each reach they
have (`radius`, `inner`, `outer` and the `ring` of a vortex) with a handle on its top, which sets
it, and a handle on their center or target; and an end or a target that names a control point
moves that point (the pointer has no handle). The steering section of the inspector adds any of
these forces and lists them with the point they follow, their strength (the speed a `slow_near`
keeps), a picker of the `falloff` and a slider for each reach. The inspector lists the
colliders with a picker of the response (bounce, slide, stop, stick or kill) and the bounce and
friction it uses, whether a circle or a box keeps particles inside, a picker of the layers it may
collide with and, for a picture mask, its picture and the opacity from which it is solid; the kept distances with pickers of their targets (a control point, the pointer or a fixed point whose X and Y are typed in pixels of the system, the same picker for the `from`, `to`, `target` and `center` of every force) and sliders for the distance and the strength; a toggle and sliders for the neighbour radius,
the separation, the alignment, the cohesion and the most speed of a flock; the remap operators with
the same editor as the remaps of the starting values and `age` as one more input; and the rope and
the trail of the render with their fields, live on the preview, and a picker of the texture among
the image assets. When a rope is on and an emitter may start more than one particle a step, a note
offers to set `max_per_step` to 1 on every emitter, which keeps the rope even; `reiwand-ctl scene
particles DIR SYSTEM rope` prints the same advice.

*Render settings for* picks whose render the sections below edit: the system, for every layer that
draws it, or one of those layers. With a layer picked, the rope, the trail, the texture, the
cutout, the orientation, the shader, the light, the refraction, the colours and sizes over life, the
frames, the drawing order and the other render fields are written to the render of that layer (the
first change starts it as a copy of the render of the system), the panel shows what that layer's
render has now and lists the values that differ from the system's, and *Use the render of the system
again* drops it (`particles` `scope`, reached with `reiwand-ctl scene act`). Without a layer picked the
panel shows the render of the system even when a layer draws it with its own.

The particle budget of the inspector reads the real numbers of the running preview when there is
one: `GetParticleStats` of the daemon for the output of the preview gives the particles alive, the
slots, the path that simulates them (graphics card or processor) and why, and the quality and card
class the preview draws at, refreshed every two seconds and after each edit. Only when no preview
runs does the panel show the count of its own simulation, labelled as an estimate; the box that
shows the particles over the preview always uses that simulation.

The particle budget of the inspector simulates the system on the processor up to the time of the
timeline (at most 10 seconds), as the CPU path of the engine does, and tells how many particles live
of its capacity, the slots it keeps on the GPU and on the processor for each device tier (phones and
older computers fall on the two lower ones) with the trail points, neighbours of a flock and rope
segments each keeps, and what each quality level keeps of births, neighbours, trail points and rope
points between particles. When asked, the simulated particles, the rope and a few trails are drawn
over the preview, and clicking a particle draws the neighbour radius of the flock around it and
marks its neighbours. A picker of the quality level, Q0 to Q3, caps the quality of the preview, and
*Restart systems* starts only the particle systems of the preview again, keeping the scene and the
rest of its state; `reiwand-ctl scene preview DIR --quality Q --restart` does the same, and `reiwand-ctl scene act DIR particles` prints the
budget as JSON.


The systems panel has a section with the settings of the system, all through `settings`: the most particles alive (`capacity`, up to 200 000), the `seed` typed, picked at random or taken from the preview, the seconds simulated before it shows (`prewarm`) and whether particles move with the layer or stay where they were born (`space`).

Each emitter of the inspector has its rate (a live slider through `settings` with the `emitter`; a
rate that follows keyframes or a link shows it and offers *Fixed rate*, which replaces it) and an
emission lane, from 0 to at least 10 s: the delay, the end of the `duration`, the on and off spans
of `periodic` (the shortest on and off spans in a top row and the longest in a bottom row) and the
the marks of every timed burst (a dot at its `at`, a square at each repetition), with handles that drag the delay, the duration, the length of the on
and off spans (keeping the width of their ranges), the `at` of each burst and the `every` of each repeating one, each drag one gesture,
the scale of the lane held while a handle is dragged; below it the list of bursts (add, remove, size, whether each fires on time, the beat or a
click, and its `at` and `every`), following the pointer, `audio_gate` and `inherit`, all through `emit`. *Drawing order and what layers may change* picks
the render `order` and `cycles` and ticks the values particle layers may change (the rest is `lock`).
A selected particles layer shows its `instance`: a slider for each number, a tint and a reset for
each to the value of the system, and *Draw everything as the system does* (`clear`); with a layer
selected, the control points of the canvas are those of the layer (`particles` with `layer`
answers them with `own`) and dragging one writes only that point in the `points` of its `instance`;
the other points keep following the system, its keyframes and its links. A layer's copy of a
`world` point is drawn in canvas pixels, where its particles feel it.
