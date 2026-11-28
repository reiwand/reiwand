# Drawing less

On this page: tiled GPUs, one submission per frame, battery, redrawing only what changed and fewer flashes.

## Tiled GPUs and render passes

On GPUs that draw in tiles, as those of phones do, every render pass costs a fixed share of the
frame on its own, about a millisecond on a phone at 60 Hz, more than its pixels or the size of its
texels do, so the engine draws fewer of them there. The first effect of an image, video or mesh
layer reads the picture where it lies, through the mapping between its target and the layer, instead
of a copy of the layer drawn first into a target of its own; it does so when that effect is a single
pass that reads only the previous result (a fused chain of built-in effects, or an effect without a
state, render targets, a copy, a blend of its own or a mix by its mask), no effect of the layer
reads `input` or `source`, and the layer has no mask, light, tint or brightness of its own, is not
turned in 3D, shows its whole picture without tiling and blends in linear space. A hidden mesh layer
placed and sized like the layer whose effect it masks or feeds is read from its posed mesh instead
of being drawn into a target for that effect. The picture is then resampled once instead of twice,
which changes only the softness of its edges. Effects that fuse into one pass (`fs_chain`) stay
fused when the quality level leaves out an effect between them, on every GPU.

## One submission per frame

A frame reaches the GPU in one submission: posed meshes, molds, 3D models and the particle
simulation are recorded with the passes of the frame instead of being sent on their own, so the
frame time that the quality levels and `reiwand-ctl scene bench` read covers them too. Mesh layers
that share a rig, a pose, a size and the size of their target, without relief, clipped parts or a
video, are posed together in one pass with one target each, up to four. Layers that blend with
`add`, `multiply`, `screen` or `exclusion` use the blend unit of the GPU; every other mode
(`lighten`, `darken`, `overlay` and the rest) reads what lies under the layer from a copy, which
ends the pass of its target and starts another, about a millisecond on a phone for each such layer
at the top of the canvas. No fixed blend gives `lighten` or `darken` exactly for a partly
transparent layer, so an author counting passes on phones gets more from `add` or `screen` when the
look allows it.

## Battery

On battery the level drops one step, which lowers resolution and effects, not the rate: the rate
follows the display up to 60 frames per second unless the user chose a rate (`playback.max_fps`,
on the phone its rate on battery, 60 by default), which then holds as on mains power; power saving
(the `power-saver` profile on Linux, battery saver on Android) drops one step more and caps
`display` at 30. A GPU or CPU within 20 °C of its critical temperature (95 °C when its driver
reports none) drops one level and caps the rate at 30; within 10 °C, or once the system starts
throttling it, it falls to the lowest level and 15 frames per second. Other sensors, such as
drives, memory or network cards, do not count, and GPUs whose driver exposes no sensor are not read. Where the system asks for
reduced motion, the scene stays at `poster_time`: parallax, camera shake and timelines stop, and
the frame is drawn again only once a minute, so that clocks and dates stay right.

## Redrawing only what changed

Each frame redraws only what changed: the layers that moved, appeared, disappeared or changed,
both where they were and where they are, plus adjustment layers with effects that read around a
pixel when a change reaches them. When that covers less than half of the screen, the rest of the
previous frame is kept; when nothing changed, nothing is drawn, and on the desktop the frame is not
composed onto the screen or sent to a stream either, unless a stream would otherwise go without
frames. Layer masks that use other layers redraw everything. Particles count as changed only when
their simulation took a step, so a held scene or particles at rest leave the damage to what moves,
and a view whose memory was freed while the scene stood still stays clean until something changes,
when it redraws everything.

On the desktop the damage of the scene, grown by a pixel for filtering and scaled to the output,
also limits what is composed and sent. A PipeWire stream in shared memory composes and reads back
only that rectangle and copies into each buffer the rectangles of every frame it missed (a buffer
older than the last eight frames, or one that was never drawn, gets everything); a stream in GPU
memory draws into each buffer only what changed since that buffer was last drawn, keeping the rest
of its pixels between frames. Both kinds of buffer carry `SPA_META_VideoDamage` with the rectangle
that changed since the previous buffer sent (a resent frame carries none), for consumers that read
it. Transitions, parallax and framed scenes compose the whole frame. Screens drawn through the
graphics API (Wayland and X11 surfaces, the phone) still compose every frame whole, because the
API exposes neither the age of the buffer it hands out nor a partial present; the scene behind
them keeps redrawing only what changed.

## Fewer flashes

While the user leaves `playback.allow_flashing` off, the engine measures the brightness of each frame on
a 32 × 18 grid. A flash is a pair of opposite changes of at least 10 % of the full brightness, the
darker side below 80 %, in at least a thirty-sixth of the grid at once (about a quarter of the
central field of view); when a scene flashes more than three times a second, it plays slower until
it flashes at most three times, down to a quarter of its speed, and if it still flashes faster it
stops at its current frame. The guard measures every frame that changed: the grid is drawn by a
small pass recorded in the frame's own submission and read back from the GPU one frame later, so
drawing never waits for it and the guard adds no submission of its own. Since the setting is off by
default, the guard runs by default.
