# Picture depth tools

The *Depth* page of the picture editor in the app and `reiwand-ctl edit depth` shape the depth layers
that parallax moves (`dbus.md`, "Depth layers") through the `depth` section of the edit of a
picture. They are actions of the edit, the same ones the app runs with `editor.run` and
`reiwand-ctl` with a command, so every step has the undo history of the picture: each stroke, each
change of the cuts and each mark is one step. All coordinates are fractions of the original
picture, before the crop and the rotation, and a radius is a fraction of its longer side.

| Action | Fields | `reiwand-ctl edit depth …` |
|--------|--------|-------------------------|
| `depth_brush` | `points` (a stroke of 1 to 1 024 points, `X,Y;X,Y;...`), `by` (0 to 1, 0.1 by default: the depth added at the middle of the brush, fading to nothing at its edge), `raise` or `lower` (`raise` when neither), `radius` (above 0 and up to 1, 0.05 by default), `falloff` (0 hard to 1 soft, 0.5 by default: how much of the radius fades) and `clear` (removes the earlier strokes first, or alone removes them); adds the stroke to `depth.strokes`, which are applied in order to the adjusted depth map (after `near`, `far`, `gamma`, `dilate` and `smooth`), so every use of the map sees them: the depth masks, the regions and layers and the overlay. At most 256 strokes and 3 000 points in all | `brush FILE X,Y;X,Y;... [--raise\|--lower] [--by V] [--radius R] [--falloff F] [--clear]` |
| `depth_cuts` | `cuts` (1 to 5 growing levels between 0.005 and 0.995, at least 0.005 apart, `L,L,...`) or `auto`; the regions of the picture (the connected parts between depth jumps, `dbus.md`, "Depth layers") are grouped into layers by their mean depth at those levels instead of at the ones the clustering finds, so a region is never torn (a layer left without pixels is dropped, and then the cut that bounded it), so the number of layers follows the cuts and not `parallax.layers`; `auto` goes back to the clustering | `cuts FILE L,L,... \| --auto` |
| `fill_mask` | `points`, `radius` and `falloff` as the brush, `refill` or `protect` (one of them with `points`) and `clear`; marks, in `depth.fills`, where the band of a layer, the pixels behind nearer layers that moving can uncover, ignores its reach and is always filled (`refill`), or where it is never filled (`protect`, the pixels stay transparent in the layers behind); a later mark paints over an earlier one where they meet | `fill-mask FILE X,Y;X,Y;... (--refill\|--protect) [--radius R] [--falloff F] [--clear]` |
| `depth_join` | `points` (2 to 8 points, `X,Y;X,Y;...`, one inside each region to make one) or `clear`; adds to `depth.joins` one join of the first point with each of the others, so the regions under them become one region, share a layer and move together; `clear` removes the earlier joins first, or alone removes them. At most 64 joins | `join FILE X,Y;X,Y;... [--clear]` |
| `depth_split` | `points` (a stroke of 1 to 1 024 points), `radius` (half the width of the cut as a fraction of the longer side, 0.004 by default) and `clear`; adds a hard stroke to `depth.splits`: the pixels it covers are a jump, so a region it crosses falls in two regions that can take different layers. At most 64 splits | `split FILE X,Y;X,Y;... [--radius R] [--clear]` |
| `depth_move` | `at` (`X,Y`, a point inside the region), `by` (`-1` to `1`: above 0 nearer, below 0 farther) and `clear`; adds to `depth.moves` a move of the whole region under `at`, after the regions are found: its depth, its layer and its movement follow; `clear` puts every moved region back first, or alone puts them back. At most 64 moves | `move FILE X,Y --by V [--clear]` |

The `joins`, `splits` and `moves` of `depth` (`dbus.md`, "Edits and spanning") merge two regions, cut one along a
stroke or move one as a whole; `edit depth regions FILE [--png FILE] [--json]` lists the regions from the nearest
with a point inside each (the ones to give to `join` and `move`) and saves them painted, blue far to red near.
The layers (`reiwand-ctl edit depth layers`) and the key of their cache follow the
strokes, cuts, marks, joins, splits and moves, so a change builds a new set and the old one is trimmed with the cache. The map of the app
and `edit depth export --adjusted` include the strokes. `edit depth show FILE --layer N --out PNG`
saves one layer with its transparency, `--band PNG` also saves the band of that layer, white on
black, computed again with the cuts, strokes and marks of the set, and `--fill lama|simple` picks
the set made with that fill instead of the newest. `edit depth compare FILE --out PNG [--layer N]
[--at X]` builds, when they are missing, the set filled by LaMa and the one filled by the simple
fill, with progress on the standard error, and saves one layer with LaMa on the left of `X` (0.5
by default) and the simple fill on the right; it refuses to run while LaMa is not installed.

In the app the *Depth* tool of the picture editor, once the picture has a depth map, shows the whole
picture, unturned and uncropped, so that what is painted lines up with the original (the crop, the
rotation and the straightening come back with the other tools), and offers:

- *Show the depth map over the picture*: the adjusted map, strokes included, in grey or in false
  colour, over the picture with its own opacity (`Relief.look`, drawn once per change of the edit and
  never per frame).
- *Paint depth*: the brush for `depth_brush`, with raise or lower, size, strength and soft edge. The
  canvas draws the circle of the brush and the stroke while the button is down, thinned to a point
  every quarter of the radius, and sends the stroke on release as one `depth_brush`; undo removes it
  alone. *Remove the strokes* is `depth_brush` with `clear`.
- *Layer cuts*: a strip with the histogram of the depth, far on the left, coloured by layer, and a
  handle for each cut that groups the regions into layers (the manual ones of `depth.cuts`, else the ones of the newest
  set or the ones the clustering would find). Dragging a handle keeps it at least 0.01 from its
  neighbours and draws, with `Relief.trace`, the outline of every layer tinted on the picture, at 512
  pixels and without building anything; releasing it sends one `depth_cuts`, and the layers are built
  again with those cuts half a second after the last change, a newer change cancelling the build in
  progress. A double click on a handle removes it, a click on the strip or *Add a cut* adds one in the
  widest gap (at most 5), and *Back to automatic* is `depth_cuts` with `auto`. The line under the
  strip counts the layers and the regions of the set, and when the picture stays still it says why
  in one line (no depth map, fill off, set being built, failed build, or depth
  that gives no movement without stretching); `reiwand-ctl edit depth layers` prints the same line on the standard error
  after building and `--list` shows the regions, the size, the safe shift and whether each set moves.
- *Fill brush*: the same brush for `fill_mask`, with *Refill* and *Protect*, size and soft edge (no
  strength: a mark is all or nothing where it covers at least half of the brush). The marks show
  in the layer views as the band they widen or empty, the layers are built again after the last
  stroke, and *Remove the marks* is `fill_mask` with `clear`.
- *Layers and fill*: a list of the layers, from the farthest, that shows one alone over the picture
  (`Relief.view`), with *Highlight the band that moving can uncover* tinting the pixels the set
  filled (the `--band` of `edit depth show`), and *Compare LaMa with the simple fill*, which shows the
  LaMa layer on the left of a divider that is dragged and the simple one on the right
  (`edit depth compare`). A set that is missing is built then, with its progress and a *Cancel*
  button, and the comparison is offered only while LaMa and ONNX Runtime are installed.

The *Parallax* tool of the picture editor, once the picture has a depth map, shows the whole picture
as the *Depth* tool does and adds the *Regions* group, the author tools for the regions of the picture:

- *Show the regions on the picture*: every region painted over the picture, blue the farthest to red
  the nearest, with a number at a point inside it that is its place in the order of depth (1 the
  nearest). The chart (`Relief.regionsOf`, `reiwand_depth::chart`, at 512 pixels, drawn once per change of
  the edit) also gives each region its depth, its share of the picture and a coarse grid of the order
  of each cell, which is what a click on the picture reads.
- Picking: a click on a region of the picture, or a tick in the list under the switch (each row
  reachable with Tab), picks it; a second click drops it. Picked regions are tinted white.
- *Join*: with two or more regions picked, sends one `depth_join` with a point of each; *Nearer* and
  *Farther* send one `depth_move` per picked region, `by` the amount of *Move by*; *Put the regions
  back* is the three `clear` actions (`depth_join`, `depth_split` and `depth_move`).
- *Split along a stroke*: while on, a drag on the picture draws a stroke (thinned to a point every 0.3
  % of the picture, 1 000 points at most) and sends it on release as one `depth_split`.
- *Straight split of the picked region*: with one region picked, a dashed line along the longer side of
  its bounds, moved across with *Where to cut* (a slider, so the arrow keys move it), and *Cut along the
  line* sends it as one `depth_split`; it is the keyboard way to split.
- Marks that no longer apply: `reiwand_depth::stale` checks every saved mark against the regions of the
  current depth map: a join whose points are already in one region, a split whose stroke crosses no
  region (taking it out leaves the same number of regions) and a move whose point is in a region that an
  earlier move or join already took. Such a move is not applied (the first one stays) and none of them
  fails the build; the tool lists them by number and `edit depth regions` prints them on the standard
  error, so they can be cleared or redone.
- *Undo* and *Redo* are the undo and redo of the picture, so each join, split and move is one step.
- *Preview the movement while you edit*: once the layers of the current edit are built, a CPU
  rendering of the layers at 480 pixels (`reiwand_depth::Preview`, the same travel limit, field lookups and
  zoom as the desktop, `Relief.peek`) replaces the painted regions on the canvas. The pointer is the
  mouse over the picture, the dot of a pad (arrow keys move it when it has focus) or an orbit; each new
  position replaces the one waiting, never queues, and nothing runs while the switch is off.
- The movement limit: the line *Movement limit* gives the safe shift of the layers of the current edit
  as a share of the width. When a change lowers it by 3 % or more, a warning says which change did it
  and the limit before and after; it goes when the limit comes back (undo) or on *Dismiss*.
- Progress of the layers: a bar at the top of the tool follows the build the app starts after each
  change and the `layers` job of the daemon, with *Cancel* for the app's own build.

The settings page of the app has, next to the LaMa download, *Check LaMa* (`reiwand-ctl models verify
lama-big-fp32`), *Remove LaMa* (`models remove lama-big-fp32`) and *Rebuild the layers*, which
deletes every layer set of the cache (the same as `edit depth layers --clear` for each picture) and
reloads the daemon so that it builds those of the pictures it shows; the button shows how many sets
are kept, or the progress of the `layers` job of the daemon while one runs.
