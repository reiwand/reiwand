# Running package effects

On this page: frames on the GPU, strikes, backdrops, the full contract and importing in the editor.

## Frames on the GPU and strikes

Each view keeps at most two frames on the GPU: before it sends a frame it waits for the one two frames
back. A frame with package effects, or of a view with effects on trial, may take at most 10 seconds,
and so may every other wait for the GPU while such a frame was sent less than 20 seconds before and
has not been seen to finish, such as composing it into a stream; other frames and waits may take 10 seconds on a GPU and, with software
rendering, 10 seconds times 32 divided by the number of cores, between 10 and 60 seconds (20 with 16
cores, 40 with 8, 60 with 4 or fewer; `LP_NUM_THREADS` counts when set). A frame with package effects still running after its
limit leaves out the package effects of the
frames of that view still on the GPU and of its trials, with `it kept the graphics card busy for more
than 10 s`, counts a strike for each and fails with the GPU stopped. With software rendering a scene
draws at most a megapixel per core (an 8K screen with two cores draws a 1080p frame, scaled up), and
when a measured frame takes more than an eighth of its limit the view draws the next ones at 70 % of
the size, down to a quarter, growing back the same way once frames take less than a sixteenth of
that, so a legitimate heavy frame never reaches the limit. The engine also measures the GPU time of
every frame and of each effect: where the device can write timestamps it times the whole frame, the
particle simulation and sorting sent before it, the posed meshes, molds and the 3D stages (models,
shadow maps and the depth read by particles) sent with it, and each run of passes of one effect, built in or
packaged, on every frame (31 runs a frame; a frame
with more takes turns among them, and then shares the time of the frame not spent in timed built-in
effects among the timed package effects by their part of the timed time), and reads the times back
when the frame has finished, without waiting for it, in the order they were sent. The GPU time of a
frame is what it adds: the union of the spans of its particle submission and of the frame, cut at
the end of the frame before, so neither the wait behind a frame still on the GPU (with two or three
frames in flight the span of a frame holds the tail of the one before) nor passes that run side by
side on tile-based GPUs count twice. When the timed runs of a frame add up to more than that, as
they do where passes overlap, they are scaled down to it before any effect is judged, so the runs
only share the frame among the effects and never decide alone. `reiwand-ctl scene bench` shows it as
`gpu_ms`, the old span sum (first to last timestamp of the frame plus its particles) as `span_ms`,
and the particle part in its simulation line.
Elsewhere it waits for the GPU before and after every fifteenth frame of a scene that draws (and
around its particle simulation), every frame with package effects of a trial or while one of them is
suspected, and counts the whole frame. When the package effects of a frame take more than 400, 60, 40
or 30 ms on the four tiers on their own (a fifth more than a frame at 30 or 40 frames a second, so
effects right at the limit are not left out by noise), the package effects that take the most of it,
until the rest of their time fits, are left out with `it is
too slow for this graphics card: a frame took <total> ms on it, <n> ms of them in this effect, and it
allows <limit> ms` once that happened on three frames within 8 seconds, cheap frames in between or
not, or at once for an effect over four times the limit (and 100 ms), whatever the rest of the frame
costs. Built-in effects are timed but never left out, and what layers and built-in effects cost never
counts against a package effect: the quality level is what makes such a frame fit. Without timestamps only a frame over a second (and four
times the limit) leaves its package effects out. A slow effect is left out for as long as the program runs, also in its other views,
and counts no strike: it may run well at another size or on another day. Only a frame that keeps
the GPU busy for 10 s, a lost GPU or a crash counts strikes. The desktop reports it
in `GetState().outputs[x].left_out` while the scene keeps playing, so the app, the tray and
`reiwand-ctl state` show `the effect <id> was left out: <reason>` as a note, and the phone lists it
among its diagnostics.

Package effects that stop the graphics card or the program count strikes, by a hash of their
descriptor and shader, in `stopped-effects` in the cache folder of the desktop (the daemon, its
workers, the app and `reiwand-ctl` share it) or in the files of the phone. The newest 1024 effects
with one strike are kept, and the newest 4096 blocked ones; single strikes never push out a blocked
effect. Before a program checks a package shader for the first time (after the helper accepted it, so only its own check counts) it writes
`<hash> trying <process>` there and removes the line once the check ends; before it builds the effect for a scene
it writes the line again and removes it once the first three frames that follow have finished on the
GPU, once a second has passed since the first one finished with no frame on the GPU (a still scene
draws once), or once the scene stops drawing. Each process that writes such a line holds a lock on a file named
after it in `stopped-effects.trials` beside the book for as long as it runs, so processes in other
pid namespaces (separate sandboxes sharing the cache folder) see each other's trials alive. A line
whose process no longer holds its lock counts as a strike the next time any of them reads the file,
so a crash, an abort, an out-of-memory kill or a hang the user ends while an effect is checked,
built or first drawn counts like a lost GPU. A clean exit (the daemon stopped or its display server
gone, a worker, the app or `reiwand-ctl` ending) removes the lines of the process first. A running
process reads the file again whenever it changes, so a block set by another process holds at once.
When the GPU stops (the device is lost, or runs out of memory, or a resource it made turns out
invalid) while frames are in flight, the effects of the frames that had not finished count a
strike, not those of outputs whose frames had (each frame is followed until the GPU reports it
done); a shader still building after 10 seconds counts one too. An effect with
two strikes is no longer checked (`its shader is not checked again`, so the package is refused) or
built (`the graphics card or the program stopped twice while it built or drew this effect`); an
effect that reaches two strikes while it is drawn is left out with `the graphics card or the
program stopped while it drew this effect`. A single strike is forgotten after a day, since a
suspend or a driver reset also stops the GPU. `reiwand-ctl retry-effects`, `RetryEffects` on D-Bus, Try
again next to the note in the app or Try the left-out effects again among the diagnostics of the
phone forget them and draw the scenes again, building every effect that was left out (also those
whose shader took too long to build, the driver refused or was too slow, which are measured
again); library packages refused
because a shader was not checked again are read again (also each time the daemon starts), and a
refusal from the book is never kept in the cache of checked shaders. A new version of the effect has
another hash and runs again.

## Backdrops and the full contract

Layers with a `compose` effect or a pass that reads `backdrop`, built-in or not, and layers whose
effects take a layer as texture or mask are drawn again whenever anything in the scene changes, since
what lies below them or the layer they read is part of their picture. A layer that feeds others is
drawn once a frame however many effects read it, and a change to it, hidden or not, or to a layer it
reads in turn, redraws the layers that read it.

The contract is the prelude above plus `aux4` to `aux7` (eight auxiliary textures in all), `mask_rgb(i, uv)` (every
channel of `aux<i>` placed on the layer, for `mask` parameters, which the scene editor paints one channel at a time), `PI`,
`tex_size(i)` (size in pixels of `aux<i>`), `daytime()` (0 at midnight to 1, in steps of a
minute), `parallax()` (the current parallax shift, before each layer's own depth), `cursor_last()`
(the pointer one frame earlier, in fractions of the output), `grid_uv(i)` (the corner of the grid
that vertex `i` of a pass with `grid` draws, six vertices a cell), `grid_position(uv)` (that point
of the layer as a clip-space position), `layer_color()` (the tint of the layer times its
brightness, linear) and `layer_alpha()` (its opacity),
`band_left(i)` and `band_right(i)` (64 bands per channel), `band_of(i, n)`, `band_left_of(i, n)`
and `band_right_of(i, n)` (band `i` of 16, 32 or 64, mixed or per channel), `rotate2(v, degrees)`, `greyscale(rgb)`, `hsv(rgb)`, `from_hsv(hsv)` and `blend_mode(mode, b,
s, amount)`, which blends straight colors with any of the blend modes. A vertex entry point returns
`VertexOut(position, uv)`, usually `VertexOut(grid_position(bent), grid_uv(i))`. The six vertices
of a cell are its corners top-left, top-right, bottom-left, bottom-left, top-right, bottom-right:
two triangles split along the diagonal from top-right to bottom-left. The grid covers the whole
box the effect draws into (the layer grown by the `reach` of its effects and its `margin`), so a
shader that bends the layer itself goes through `from_layer`, as in
`grid_position(from_layer(bent))` with `bent` in layer fractions. Every triangle is drawn as given
unless it spans more than 6 / cells of clip space on an axis (three views wide for a grid of one
cell), which the engine shrinks towards its centre so a runaway corner cannot cover the screen.
`CONTRACT` in
`crates/reiwand-scene` holds it, and package shaders are checked against it whole. Package effects run
the same WGSL on every platform.

## Importing an effect in the editor

The scene editor imports an effect with its two files (`reiwand-ctl scene act <dir>
import_effect '{"descriptor":"descriptor.json","wgsl":"shader.wgsl"}'`, or Import an effect in the app), copies them to
`effects/<id>.json` and `effects/<id>.wgsl`, refuses them with the reason when they fail the checks
above, and then lists the effect among the ones a layer can add. Its parameters become controls in
the inspector with their localized `label`, sliders within `min` and `max`, choices for `enum` and
`blend`, and may be bound to properties, keyframes or scripts like any parameter.
