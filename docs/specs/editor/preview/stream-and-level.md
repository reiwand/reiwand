# The preview stream and its level

On this page: the stream, the preview level, testing the pointer, comparing and the glow.

## The preview stream

The app asks the daemon for a virtual output named `preview:<n>` with `RequestStream` and shows its
PipeWire stream on the canvas, as the Plasma wallpaper does. The stream has the pixels the canvas
takes on the screen (its size times the screen's pixel ratio, rounded up to 640-pixel steps so that
zooming or resizing the window asks for a new stream only now and then), never more than the scene's
own size; at 100 % zoom and zoomed in each screen pixel has a pixel of its own. The levels of
*Preview quality* that draw smaller say it in their row (Q0 half size, Q1 60 to 75 %);
Q2 and Q3 draw the full size of the stream. Every change sends the unsaved document
with `PreviewScene` (`dbus.md`), at most once per 50 ms, with `solo` applied; a paused or scrubbed
timeline holds the scene at that second, so a still preview renders only when something changes. A
new document replaces the preview without a crossfade, and a playing preview keeps its time; package
details that are still incomplete, such as a missing rating, do not stop it. *View as wallpaper* shows the
same preview on the desktop, on the chosen screen or on all of them. The app repeats the call every 30 seconds while a preview is
on and ends it when the editor closes; the daemon ends it 60 seconds after the last call. A
minimized window marks the preview output covered. `reiwand-ctl scene preview DIR [-o OUTPUT] [--time
T] [--quality Q] [--restart]` shows the saved folder for 60 seconds, at most at that quality level and
started again, and `--stop` ends it; `--media FILE` shows a made-up track from a JSON file of at
most 64 KiB (`title`, `artist`, `album`, `album_artist`, `subtitle`, `genres`, the last three
words or lists of words, `playing`, `position` and `duration` in seconds from 0 to 10 000 000 and
`rate` from −100 to 100, `cover`, a picture inside the folder; texts of at most 512 characters
without control characters) instead of a
media player. The document of a preview may carry `preview: { quality, restart, media, battery, calm }`, which the
daemon reads only for previews; `media` is refused, naming the field, when a value is out of
range, and never reads the media players or needs the `media` grant. `battery` and `calm` are
booleans: the preview then runs as the daemon runs a scene on battery (a lower level, which
takes Q3 to Q2 whatever the automatic choice would be, and at most 60 frames a second) or with
reduced motion (parallax and camera shake stand still, as the daemon does with the desktop
setting). `reiwand-ctl scene preview DIR [--battery] [--calm] [--without-bloom]` sets them (`--without-bloom` shows the preview without the glow of the canvas, to compare it with the glow).

A preview document that carries `scene.ext.editor.probe` (`{pointer: [x, y], tilt: [x, y]}`, the
pointer as fractions of the canvas from 0 to 1 and the tilt of a phone from −1 to 1, either
part optional) feeds the scene a pointer that rests there and that tilt, so layers with
parallax depth sit in the picture where the canvas outlines put them; out-of-range values are
clamped, and the daemon reads it only for previews. The scene runtime takes the tilt as a
device tilt, so `canvas.parallax.gyro` follows it.

## Preview level

*Preview level*, a button of the canvas toolbar, opens the panel that sets what the preview
shows. It has the same quality selector as the cost panels (Q0, Q1, Q2, Q3 and Restart), *On
battery* and *Reduced motion*, and every change sends the preview again. Under them the panel
answers the editor query `{"level": {"quality", "battery", "calm"}}` (`rigs::query`,
`levels::query`): `quality` is the level the daemon reaches (`asked` is the cap that was
chosen), `render_scale`, `density` and `particles` are what that level keeps of the resolution,
the crowd and the particles, `bloom` says whether the canvas has a glow (`on`), whether it has a
glare above white (`hdr`) and whether this level draws it (`glare`: Q3, or Q2 off battery),
`effects` has one row per effect use with its `layer`, `effect`, whether it is `kept`, its
`scale`, its `units` of work at this level (null when it is left out), the `full_units` it costs
at Q3, the `levels` of all four levels as the effect-cost panel shows them and `follows` when it
reacts to the pointer, `pointer.emitters` are the particle systems that follow the pointer,
`left_out` counts the effects left out and `units` adds up the work that stays. The panel
writes the same rows, in bold when an effect is left out, and names what follows the pointer so
that it can be tried with *Test the pointer and clicks*. The cost panel of an effect marks the
level the daemon would reach, not the one that was asked for.

## Testing the pointer, comparing and the glow

While *Test the pointer and clicks* is on, the preview shows a ring where the pointer is, red
while a button is held, and every press sends out a pulse from the point it fell on, so that the
author sees where a click reached the scene.

*Compare with* picks a second level and *Without the glow of the canvas* the same frame with the
bloom taken out (`look.bloom: false` leaves `canvas.bloom` out of the previewed document, never
out of the saved one). Holding *Hold to see the other side* shows that second look in the same
preview and letting go returns to the first, so two levels, or the glow on and off, can be
told apart on the same frame; *Split the picture with the other side* in the panel writes both
looks into one picture, the first on the left and the second on the right.

*Compare the glow*, a button of the canvas toolbar that appears while the canvas has `bloom`, splits
the canvas itself in two live views: the app asks the daemon for a second stream of the preview
(`RequestStream`, released when the button is turned off, the canvas changes size or the editor
closes) and sends it the same document without `canvas.bloom` (`SceneEditor.halve`, the property
`split` carries `{ on, node, serial }`). The left of the canvas shows the first stream, with the
glow, and the right the second, without it, cut by a line with a handle that is dragged to move
the split (the position is not saved). Both streams follow the same time and edits, the simulated
pointer and clicks reach both, and both are covered together. `reiwand-ctl scene preview DIR
--without-bloom` previews without the glow from the terminal.
