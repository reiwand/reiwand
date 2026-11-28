# Depth maps and depth layers

On this page: the depth map of a picture and the layers made from it.

## Depth maps

The depth map of an image is a grey PNG, white near. It can sit next to the image
(`hills.jpg.depth.png`, where `reiwand-ctl edit depth import` and `export` put it), in
`~/.local/share/reiwand/depth/<SHA-256 of the image>.png` (imports whose folder cannot be
written) or in `~/.cache/reiwand/depth/<SHA-256>-<model>-<input>.png`, where
`reiwand-ctl edit depth generate` and the app write maps estimated in their own process; the newest
of these files wins, and on equal times an import comes first. An import whose width to height
ratio differs from the image's (after its EXIF orientation) by more than 2 % is refused. The cache
keeps at most 256 MB of estimated maps and deletes the oldest first; temporary files left by an
interrupted write or install are deleted by the next one in that folder once they are an hour
old. The daemon never runs the depth model: it reads the map at
1024 pixels at most, applies `depth` and reloads it when any of these files changes. A parallax
image is rendered only while its smoothed offset moves by a quarter of a pixel or more, at most
at the frame rate of the output, under the same pause rules as videos (`pause.when_covered`,
`pause.on_battery` (any of its values), `pause.when_locked` and `SetPaused`), and stays still while the desktop asks
for reduced motion (`reduced-motion` of the Settings portal). With software rendering (a GPU
emulated on the CPU) the image stays still unless `playback.software_parallax` is set, and then
moves at 15 frames per second at most. On `span` outputs every
screen moves by the same offset; `tile` images stay still. The model and ONNX Runtime are only
downloaded by `reiwand-ctl models download` or the app after showing their size, license and every
host they may come from, from their pinned address (the manifest may list mirrors, which must
serve the same bytes), with a generic User-Agent, no Referer, no redirect from HTTPS to HTTP and
their pinned SHA-256; `NOTICE.txt` and the full license text (`LICENSE.txt`, shipped with
Reiwand) are stored next to each file, and ONNX Runtime keeps the `THIRD-PARTY-NOTICES.txt`
of its archive. ONNX Runtime runs with its telemetry off (`ORT_DISABLE_TELEMETRY=1` and its
environment's switch), so it neither stores a device id nor contacts anyone.
`models.allow_download = false` turns downloads off on this device only: it is not synced and a
configuration that cannot be read counts as `false`, also when a setting change saves it again.
`reiwand-ctl models install` takes files fetched by other means.

## Depth layers

**Depth layers.** A picture whose `parallax` is on, with a depth map, and an edit
without enabled `ops`, moves as a stack of layers computed once, never per frame; nothing else moves
a picture. The daemon starts a `layers` job (target: the picture path) when it shows such a picture
and finds no set for its settings; until it ends, when it fails, when `ops`
are enabled (colour edits are drawn by the GPU on the picture, not on the layers) and when the set
gives no movement of at least one pixel, the picture stays still; without a depth map it stays still
too, and when the depth model is not installed the app (the parallax tool, as soon as it is turned on),
`reiwand-ctl edit parallax` and the phone say so with the download size and the action that downloads
it, which only runs when the person asks. A failed build leaves
`<SHA-256>-<key>.layers.failed` with the error beside the set, and the daemon does not try that
set again until the cache or the edit changes or `RetryLayers` is called; a set that cannot be read
back (a torn or damaged file) is deleted and marked failed the same way, so it is never rebuilt in a
loop. The job reports its progress from 0 to 100: 2 once the map is read, 10 once it is refined,
then each layer in equal shares, advancing after every LaMa crop within a layer. The job runs on its own thread at the lowest
priority, with at most four threads for LaMa, and reloads the screens when it ends. It works at the
size the output needs, rounded up to 1024, 1920, 2560 or 3840 pixels on the long side (never above
the picture; `reiwand-ctl` and the app build at 1920, the phone at 1920 at most), and computes:

1. the depth map at the size of the picture, refined with a guided filter that takes the picture
   as its guide (colour guide, radius 8 pixels, epsilon 0.002), then the `near`, `far` and `gamma`
   of `depth` and its `strokes` (`dilate` and `smooth` only serve the depth masks);
2. the regions: pixels whose depth spans more than 0.08 within 3 pixels (scaled with the size, 1920
   pixels being 1) are a jump, the rest is flood-filled into connected regions, the `splits` of
   `depth` are added to the jumps, the jump pixels then join the neighbouring region they reach
   through the smallest steps of depth (so a thin strand stays with the object it grows from),
   regions under 0.1 % of the picture join the neighbour they share the longest border with, and
   the `joins` of `depth` merge the regions under their two points, and the `moves` of `depth` then
   add their `by` to the depth of the region under their point (the regions keep their labels, the
   means and the depth fields of the layers follow the moved depth);
3. `parallax.layers` (of the edit of the picture) layers at most, grouping whole regions by their mean depth (a one dimensional
   k-means on those means, or the `cuts` of `depth`), so a region is never torn between two layers;
4. for every layer, its depth field: the depth of its own pixels only, smoothed, at a quarter of the
   size, with its slope capped by the mean of the upper and lower 8-neighbour envelopes of slope
   `0.04 / (shift × width)` and blurred twice with a box of 6 field pixels, so that at the shift the
   set is built for the local scale of a layer stays between 0.96 and 1.04 and a steep part flattens
   towards a rigid card; the field reaches past the layer with the depth of its edge;
5. for every layer, the band that moving can uncover: the pixels under the layers in front of it
   (and a matting band of 4 pixels around them, scaled with the size) within
   `ceil(shift × width × (nearest field − own field)) + 2 × (2 + matting band)` pixels of what the
   layer knows, where both fields are taken at that pixel; protected marks leave pixels out of it;
6. the edge of every layer but the farthest against the layers behind it: within the matting band,
   the transparency is the projection of the colour of the picture on the line between the local
   colour of the object and the local colour behind it (when they differ by 24 levels or more), and
   the colour is the object's alone, so no fringe of the background moves with the object;
7. the fill of those bands from what the layer knows (never the matting band), with `lama` (crops of
   512 pixels at the size of the picture, as many as the band needs, three quarters apart, blended
   back with a feathered edge over a push-pull base) or `simple` (push-pull pyramid from the known
   pixels followed by a blur of radius 2 over the filled pixels only). The farthest layer is opaque
   everywhere.

The set is one PNG with the layers stacked from the farthest, `<SHA-256>-<key>.layers.png`, one
16-bit grey PNG with their depth fields stacked the same way, `<SHA-256>-<key>.fields.png`, and its
`<SHA-256>-<key>.layers.json` (`format` 2, `size`, `fill`, `cuts`, `depths`, `bands` with the layer,
the reach in pixels and the pixels filled, `field` with the size of the fields, `regions`, `built`
with the shift it was built for and `reach` with the largest safe shift and the lowest and highest
field), in the cache of depth maps (written first the fields, then the JSON and then the PNG, which
the daemon watches). The key hashes the format, the picture, the model, the input size, the `near`,
`far` and `gamma` of `depth` and its `cuts`, `strokes`, `fills`, `joins`, `splits` and `moves`, the fill
actually used, the number of layers, the shift (rounded to a thousandth), the long side, the size
and time of the depth map file and the LaMa file, so sets of an older format are built again; the
256 MB limit of the cache covers them and trimming a PNG deletes its JSON, fields and failure mark.
`lama` becomes `simple` while LaMa or ONNX Runtime is not installed (`reiwand-ctl models download
lama-big-fp32`, which, like the other models, only downloads after showing its size, license and
server). At rest the screen shows the original picture; while it moves, the GPU draws the layers
from the farthest to the nearest in one pass, nearest occluding, each moved by its own field
(`uv − shift × (field − focus)`, with the field read where the pixel lands after one step), with
the same source and smoothing as before.

The movement of each picture is computed in one place for the desktop and the phone
(`reiwand_core::travel`): the shift asked for (`strength`, three times the visible share of the
picture on the phone) is cut to the safe shift of the set, so nothing uncovered passes the band, and
to what the zoom can hide with a margin of at most 20 % per side; the zoom then follows from the
shift, the `focus`, the farthest the fields lie from it, the width to height ratio of the picture
and the window shown, about the centre of the picture as the GPU zooms, and is never below
`overscan`. The phone builds its set for the shift it applies.

A `geometry` (crop, rotation, straightening or flip) does not make another set: the daemon and the
phone turn the layers and fields of the set of the whole picture, once and in memory, with the
matrix the GPU uses for the picture (bilinear, on premultiplied colours, at the size of the layers
cropped to the frame), so that they line up with the edited picture; strokes, cuts, marks, joins, splits and
moves stay in the coordinates of the original.

`reiwand-ctl edit depth layers FILE [--fill lama|simple|off] [--layers N] [--force] [--list] [--clear]`
builds the set in its own process (the same code, with the settings of the configuration
file as defaults), lists the sets of the picture or deletes them, and
`reiwand-ctl edit depth show FILE --layer N --out PNG [--band PNG] [--fill lama|simple]` saves one layer
with its transparency and, with `--band`, the pixels of it that moving can uncover, and
`reiwand-ctl edit depth compare FILE --out PNG [--layer N] [--at X]` saves one layer filled by LaMa on
one side and by the simple fill on the other (the commands of the depth tools are in `editor.md`,
"Picture depth tools").
