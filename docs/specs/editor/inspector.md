# Inspector

The inspector is generated from the JSON Schema of scenes (`reiwand-ctl scene schema`) and of user
properties (`--property`), produced by `schemars` from the types of `reiwand-scene`, so a field added
to the format appears without changes to the editor. `reiwand-ctl scene fields DIR POINTER` prints
the fields it shows for a pointer, as JSON. Annotations in the schema guide the widgets:

| Annotation | Meaning |
|------------|---------|
| `x-unit` | unit shown after the label (`px`, `°`, `s`) |
| `x-range` | `[min, max]` of a slider |
| `x-ref` | the value names an `image`, `video`, `sound`, `font` or `lut` asset, a `layer` (for the `layer` shape of a particle emitter only the layers that may feed its system), a particle `system`, a `control` point of the system (`p0` and the `points` it has), a `timeline`, a user `property` that holds a value or, for the `trigger` of a timeline, the triggers listed for `trigger` above, or a built-in `effect`, offered in a list; properties show their labels and effects their translated names |
| `x-key` | a keyframe `[time, value, ease]` |
| `x-curve` | a list of `[t, value]` points edited on a plot: a click adds a point, a drag moves it (one gesture) and a double click removes it; `t` goes from `0` to `1` across and the value up to its `x-range`, or a quarter above the highest point |

Numbers become sliders when they have a range, colors a color picker, enumerations and tagged
variants a list with translated names, pairs two fields, `x-curve` lists a curve, color stops
`[t, color]` rows of a time and a color picker, fixed pairs of points (a box, a line) two rows of
coordinates, objects a group of fields and other lists a count with *Add* and *Edit…*. A number
that may also be a name, such as `clock.max_fps` (`display`), has the widget of the number and the
names in `options`, shown as check boxes. The app names every field, value, effect, easing, layer
kind, asset kind, language and computed feature in the language of the desktop; a test fails when
the schema gains one without a name, and another walks the inspector of every starting point with
every layer kind, variant and list item and fails when an option or a shown value has no Spanish
translation or a raw JSON field holds a plain text. Effect parameters come from the descriptor of
the effect, with its `label` as `title` and its `help` as `help` (a tooltip) in the language of the
desktop; a `texture` or `mask` parameter offers the image assets and every other layer, written as
`{ "layer": "<id>" }`, and every entry of `options` is a field of its own, with its values as
choices and bindable to a property. A parameter that has a handle on the canvas draws it
while the effect use or that field is selected, placed where the renderer puts it under the parents,
groups, anchor, origin and parallax of the layer, and the query `effect_handles` (`layer`, `index`,
`time`) lists them with their canvas position, what the descriptor declares being what decides: a
`position` parameter, a `vec2` named `center`, `source` or `pivot`, or the four corners of a homography
effect such as `perspective` (linked as a quad) is a point; a `vec2` from −1 to 1, such as a direction,
is an arrow from the first point of the effect (the middle of the layer without one) with a handle at
its tip; a `float` with `unit: degrees`, such as the `angle` of `spin` or `waves`, is a dial: a ring
around that same point with a handle that turns it, and the field gets a dial next to its slider, as
particle emitter angles already have; two points whose names pair up (`from` and `to`, `start` and
`end`, `a` and `b`, `first` and `second`) are joined by a line that has a handle in the middle to move
both, and Shift while dragging an end moves both; every drag is one gesture of `effects grip`. A
package effect gets all of this from its own descriptor. An adjustment layer with `size` shows what it captures: the rectangle (in the coordinates of its parents, around its position as `origin` places it, without its own turn, skew or scale, as the renderer captures it), a dashed frame with the most it may capture (twice the canvas on each side, in red when `size` goes past it), with `redraw` the outline where the capture is drawn again, a caption with the size and the limit, and eight handles on the corners and the middles of the sides that edit `size`, not the scale (the generic scale handles are not drawn for such a layer). The query `capture_view` (`layer`, `time`) lists them with their canvas positions and is `null` for any other layer. Above the
fields of an effect use, the inspector offers its
mask: none, a picture, another layer or a shape (rect, ellipse, ring, polygon, star or line, each
started at half the size of the layer), the channel, invert, a soft edge (a slider up to 200 px and
a field up to 10000) and the strength, all through `effects mask`; the same panel offers the mask of a layer itself (`layer.mask`, which takes the same fields; `effects mask` without `index`), with a thumbnail of what the mask looks like (the channel of the picture, the outline of the shape, the name of the masking layer). The canvas draws the mask while its use, the layer or the mask is selected, placed where the renderer draws the layer under its parents, groups, origin and parallax: the box of the layer in dashes; for a picture, the channel it reads (brightness, red, green, blue or alpha), inverted and weakened as the mask is, as a tinted picture over the layer, decoded to at most 512 pixels and kept as a PNG in the cache of the app (the 32 most recent kept); the masking layer framed; and for a shape, its outline filled where the effect applies (everything but the shape when `invert` is on), its feather range as two dashed outlines, one grown and one shrunk by `feather`, and handles: the corner that sizes it (the end of a line, which runs from −`to`/2 to `to`/2 around the middle), a dot past its edge that sets the feather, and one more for the width of a ring, the corner radius of a rect and the depth of the points of a star. The query `mask_view` (`layer`, optional `index`, `time`) lists all of it in canvas pixels, `mask_picture` (`asset`, `channel`, `invert`, `strength`, `tint`) returns the file of the tinted channel, and the action `effects mask_grip` (`layer`, optional `index`, `hold`: `size`, `feather`, `width`, `radius` or `inner`, `to`: a point of the canvas, optional `at`) writes what a handle does when it is dragged to `to`, in the pixels of the layer around its middle: `size` doubles the distance (each side of a rect, ellipse or ring, the larger of the two for a polygon or a star, `to` of a line), `feather` is the distance past the edge (across the line for a line), `width` and `radius` are measured inwards from the edge, and `inner` is the distance of the inner points over the outer radius; it is refused for a handle the shape does not have. Each drag is one gesture.
The outline adds `effect_names`, the `name` of each effect of
the package in the language of the desktop, and `effect_catalog`, the `id`, `category`, `description`
and cost of every built-in effect and effect of the package, the description in the language of the
desktop. *Add an effect* opens a dialog of that list grouped by category, searchable, each effect
with its cost next to its description; `reiwand-ctl scene catalog DIR [--category CAT] [--json]` prints
the same list on a terminal, as a table or one JSON object per effect. Problems and refused actions show the message of the validator and of the editor, the
same English text that `reiwand-ctl pack check` and the commands print, under a translated
heading; each problem leads to its field, whose name is translated. A field that may follow a
source (keyframes, a user property, the sound analysis, the pointer or the media player) has
a menu to add or remove a keyframe at the current time, bind it, make it fixed again or edit the
binding itself (timeline, loop, scale, offset, clamp, smoothing). Follow the media player lists
only the `media` values that fit the field (the texts of the track for a text, the numbers and
`playing` for a number, `playing` for a switch, the colors of the cover for a color), each with
what it would show now with the simulated player of the Player tab; on a text that follows the
track, Text while nothing plays opens its `placeholder` with a row per language. An optional field that is set
can go back to its default.

The `texture` parameters that are gradients (`gradient`) show a gradient bar under the picker:
click it to add a stop, drag a stop to move it, click its swatch to recolor it, remove it with its button; the
bar draws the ramp live, and when the editing pauses or ends it runs `ramp` once (one undo step), making the
picture again when the parameter already names a ramp made here. A `lut` parameter shows the same bar and
makes a color table. The bar is the component `SceneStops.qml`, shared by every gradient of the editor: its
`stops` property takes a list of `[position, "#rrggbb"]` (positions 0 to 1, at most 8) and its `committed(stops)`
signal gives the new list once per finished edit, never while a stop is being dragged; the owner decides what
to write (the ramp action here, the `gradient` start of the particles, the stops of a shape gradient). The initial
colors of the particles (pair, hues, palette or gradient) have their pickers in `SceneSwarmTint.qml`, with the hue
strip, the palette swatches with their noise sliders and `SceneGradientBar.qml`, through `particles ... start`.
A package effect declares what it needs only through its parameters: a `position` for a point on the canvas (the
canvas handles are driven by the descriptor), and `texture` parameters named `gradient`, `map`, `phase`, `walls`,
`reflection` or `depth` to get the tools above; the picker of every `texture` parameter offers the assets, `white`,
`black`, `noise`, `normal_flat` and the other layers.
Above the

The Player tab is a simulated media player for the preview, so that a scene can be made without
music playing: a switch turns it on, and it has the title, artist and album of a made-up track,
whether it plays, its position and length (sliders up to four hours, with a field in seconds for longer tracks, and times written as `m:ss` or, from one hour, `h:mm:ss` as the scene writes them) and its cover, picked among the pictures of the scene, with the five colors that the scene would
get from it (or from the placeholder) as swatches. Next track gives the made-up track another title
and the next picture as its cover, so that what waits for `media:track` and `media:cover` starts in
the preview, and the playing switch starts what waits for `media:play` or `media:pause`; the track
and its cover stay the same while only the position, length or playing change.
Scripts of the preview get the made-up track too, with their media events, and `reiwand-ctl media
simulate OUTPUT FILE.json` shows one on any screen (`dbus.md`, `SimulateMedia`).
The preview then shows that track instead of what a media player plays, without asking for the
`media` grant and without reading the players; it reaches only the preview.

A web package has the same simulated player in its Player tab (switch, title, artist, album, a
cover picked among the pictures of the folder, next track, playing, position and length), which
feeds `window.reiwand.onMedia` of the page in its Preview tab: the track goes in `preview.media` of
the `PreviewScene` document (`dbus.md`), so the page gets it, with the cover as a blob and the colors
of the cover, without the `media` grant and also when the page loads after the track started; it
reaches only the preview. Under it, *On a screen* still shows a made-up track from a JSON file
(Simulate from a file…) on the screen the package was last built and applied to (Stop ends it), the
same `SimulateMedia` call `reiwand-ctl media simulate` makes; until the package is applied once, the
tab says so instead. `reiwand-ctl scene preview DIR --media FILE.json` shows the track in the preview
of a page from a terminal.

The *Try* tab of the scene editor (and of a page package) shows the user properties of the manifest
with the same widget as the settings of a shown package, followed by the presets of the package. Choosing
a preset or changing a value sets `values` and `preset` in the look of the editor, and the preview
document carries the merged map in `preview.values` (`PreviewScene` in `dbus.md`): the daemon shows the
package with those values, over the defaults of its properties, instead of the values saved for the
package, and never writes them anywhere. Secret texts and buttons are left out. *Back to the defaults*
ends the trial. `reiwand-ctl scene preview DIR --values FILE.json [--preset ID]` does the same from a
terminal; the file is a JSON object of property ids and values and `--preset` names a preset of the
manifest whose values the file then overrides. `rig` with `{"tried": {...}}` returns the form
(`fields`, `presets`, `values`) that the tab draws.

Test the pointer and clicks, a toggle of the canvas toolbar, turns the mouse over the preview into
a made-up pointer on it instead of an editing tool: while it is on, moving or pressing over the
preview calls `SimulateTouch` (`dbus.md`) with the position as fractions of the output and the
held buttons as a bitmask (1 left, 2 right, 4 middle), so `input.cursor`, `click:`, `enter:` and
`leave:` can be tried without a real pointer there; turning it off, or the pointer leaving the
preview, reports it left. `reiwand-ctl input touch OUTPUT X,Y [--buttons N]` does the same on any
screen, real or `preview:`, and `--stop` ends it.
