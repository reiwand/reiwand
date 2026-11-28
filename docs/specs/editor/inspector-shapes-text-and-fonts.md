# Shapes, text and fonts, Hearing, shaping and trying a binding and Sound layers

On this page: shapes, text and fonts, hearing, shaping and trying a binding, sound layers.

## Shapes, text and fonts

A `shape` layer shows the handles of its shape on the canvas while it is selected, placed where the
renderer draws the layer (under the parents, groups, anchor, origin and parallax) and listed by the
query `shape_handles` (`layer`, `time`): each handle has a `name`, a `kind` (`edge`, `corner`,
`round`, `count`, `depth`, `end` or `point`), its position in fractions of the box of the layer
(`u`) and on the canvas (`at`), and `gradient` has the ends of the gradient on the canvas. A `rect`
has `width`, `height`, `size` (both, with `even` the proportions are kept) and `radius`; an `ellipse`
and a `ring` the same without `radius`, and the ring `band`, its thickness; a `polygon` has `size`
and `sides`, which is the second vertex: dragging it around the shape gives the number of sides; a
`star` has `size`, `points`, the second tip, and `inner`, the depth of the points; a `line` has
`start`, `end` and `width`, and moving an end leaves the other where it is by moving `position`;
`stroke` sets the width of the outline of any shape but a line, and `gradient_from` and
`gradient_to` the ends of the gradient, which are the center and the edge of a radial one. The
handles that resize sit a little inside the box so that they do not cover the grips of the layer.
`layer figure shape` (`layer`, `handle`, `to`, optional `even`, `at`, `replace`) writes what the
handle does when it is dragged to `to`, a point of the canvas in pixels; it keeps the limits of the
scene (3 to 64 sides, 2 to 64 points, the depth from 0.01 to 1, a radius up to half the smaller side).
`layer figure stop` (`layer`, `stop`) edits the stops of the gradient: `{ "make": { "radial" } }`
adds one of two stops, `clear` removes it, `{ "add": { "at", "color" } }` adds a stop (the color of
the one that follows when none is given), `{ "move": { "index", "at" } }`, `{ "color": { "index",
"color" } }` and `{ "remove": { "index" } }`; stops stay sorted, between 2 and 4. The inspector shows
a bar with the gradient: clicking adds a stop, dragging moves it and every stop has a color and a
button to remove it.

The box of a `text` layer is the size the renderer draws (`reiwand-type`, the typesetter both share):
the same fonts, wrapping, `line_height`, `letter_spacing`, `padding`, background padding and the
margin of the outline, shadow and blur, so that the handle box of the canvas matches the text. The
query `text_box` (`layer`, `time`) gives its `size`, the `lines` it takes, the `widest` line, the
`corners` and the `handle` at the right edge, and `layer figure text_width` (`layer`, `to`, `at`,
`replace`) sets `wrap.width` so that the right edge lands on `to` while the left edge stays (it
moves `position`); with `remove` it takes `wrap` away. A line break typed in the field of the text
(a multi-line box, Enter writes a new line) is part of the text and takes a line of the box.

`layer figure fonts` (`layer`, optional `font`, `fallback`) sets the font (an empty name takes it
away) and the list of fallback fonts of a text layer, at most 8; a letter the font lacks is drawn
with the first font of the list that has it. The query `font_list` (`layer`) gives the `system`
families installed, sorted, the generic ones, the fonts in the `package` (assets of font files), the
`most` fallback fonts and the `chain` of the layer, each name with its `source`: `package`, `system`,
`generic` or `missing`. The font picker lists them all, each written in its own font and with a
search field, and the list of fallback fonts is edited in place, reordered and warns about a font that
is neither in the package nor installed.

A value of a text that follows the clock or the date has a format. The binding menu of a text offers
the time and the date with ready formats and a sample of each (`HH:mm`, `hh:mm a`,
`EEEE d MMMM yyyy`…), and that of a number the time of the day, its hour, minute, second, weekday and
month. The field of the format shows what it reads now, from the same formatter the scene uses
(query `clock_sample`: `pattern`, `dates`, optional `wall`, the local time in seconds, and `lang`),
and a help with every piece and its sample (query `clock_help`: `tokens` and `numbers`) whose pieces
add themselves to the format when clicked.
The text field of a text layer (`SceneMultiline`) wraps at word boundaries; a run of 2 000
characters or more with no space or line break is wrapped anywhere instead, since wrapping it at
word boundaries takes Qt seconds and froze the window (half a minute for a pasted megabyte). Ctrl+V
(and the other keys of Paste) reads the clipboard before the text is laid out, so the wrap is chosen
first; a megabyte is pasted and applied within a couple of seconds. Check: `SceneTextPasteCheck`.

## Hearing, shaping and trying a binding

The menu of a bound field has *Try and shape the response*:

- *Watch the levels of the sound* (bindings that follow the sound) opens a meter: the bands of the
  sound the preview hears, as bars, drawn over a strip in which the bands the binding reads
  (`audio` as a band, a range or `bass`, `mid`, `treble`, with `of`, `reduce` and `channel`) are
  outlined, and a bar with the value the binding gives, before and after `bounds`, `exponent`,
  `scale`, `offset`, `clamp` and `invert` (not `smooth`). The meter is the component
  `SceneBandMeter.qml` (`page`, `source`, `of`), so the band picker places it over its own strip. It
  reads the levels with the `Bands` call of the daemon (`dbus.md`) about ten times a second, only
  while the window is open: closing it stops the calls and the daemon forgets the reading. The
  levels are the ones the scene itself would hear, so the scene must declare and hold the sound
  grant; without it the meter says so instead of drawing, and it says to play something when
  nothing arrives. Nothing is recorded, kept or sent. The query `meter` (`source`, `reading` as the
  daemon gives it, `of`: 16, 32 or 64) returns the folded `bands`, the `marked` range, the `value`,
  its `raw` value and the `levels` `bass`, `mid`, `treble`, `level` and `beat`; with `gain:
  "fixed"` in the scene it reads the levels measured from full scale.
- *Try the pointer* (bindings that follow the pointer) starts a made-up pointer that wanders over
  the preview, the same `SimulateTouch` as Test the pointer and clicks, until it is turned off,
  which reports the pointer left; the made-up path is the query `wander` (`time`).
- *Smooth the changes* gives the binding a `smooth` of 0.2 s and opens it. Every binding of a
  number, a pair of numbers or a color has the `smooth` field as a control of its own: one slider
  for the same time up and down, or two with *Different speed going up and down*, and a plot of the
  response of the smoothing to a step up and then down, with the seconds it takes to settle
  (three times the constant). The query `response` (`smooth`: a number or `{ "up", "down" }`, 0 to
  60 s) returns the points of the plot.

Next to the menus for scripts (*Follow a script* lists every script and function of the scene, with
its settings) and the media player (above), these complete the entries of the binding menu.

## Sound layers

When the canvas or a sound layer is selected, the inspector lists the sounds of the scene (query
`sounds`): each sound layer with its sounds, its trigger (`start` when it has none; the picker of
`play_on` offers the buttons and properties of the package, `event:`, `click:`, `enter:`, `leave:`
and `media:`, see above), whether it is muted, a *Listen* button and, below, the waveform of each
of its sounds. Sound assets that no layer plays have a waveform and a *Listen* button too. The
waveform is the query `wave` (`asset`, `buckets` up to 2048): the peaks of the decoded sound from 0
to 1 and its length, decoded once off the window thread and kept in the cache folder
(`reiwand/waves`, keyed by file, size, date and buckets, at most 256 files), so it is read from
disk the next time. The fades of the layer are drawn over it and their two grips set `fade_in`
and `fade_out`. *Listen* adds `listen` to the look of the preview, the layer id or `{ "asset" }`
for a sound without a layer: the preview restarts and plays, once, only that sound (the other sound
layers are muted, `play_on` ignored), through the daemon, so the volume and the mute of the
application, the setting that turns the sound of scenes off and the sound of other programs apply as
they do to a wallpaper. Listening again, or leaving the panel, goes back to the scene. The format
has no trim of start and end for a sound: the fades are what shapes the audible part.
