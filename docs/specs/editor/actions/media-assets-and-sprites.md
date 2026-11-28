# Media, assets, ids, keys and timelines

On this page: media, assets and sprites, new ids, keys and timelines.

## Media, assets and sprites

`film` and `sprites` have no command of their own: `scene act` runs them. The queries `film`
(`{"film": {"layer", "strip"}}`: real size, length, trim and speed of the video of a layer),
`sprites` (`{"sprites": {"asset"}}`: size of the picture, frames, `fps` and `frames`) and `uses`
(`{"asset_uses": {"asset"}}`: every pointer that names the asset, with the layer that holds it) go through
`rig`, like the other queries of the canvas.

A video layer has on the canvas the box of the real size of its video (that of `size` when the
layer sets it), not one of 640 × 360. In the inspector of a video layer, or of an image layer that
shows a video, a panel draws a strip of frames along the real length of the file, with two grips
that trim `start` and `end` to the hundredth of a second, a ruler in seconds, the speed (0 to 4
times, which is what the format takes; the range of `playback.rate` in the inspector ends at 4 for
these layers too), the loop, *Use the size of the video* and *Play the whole video*. Wherever a
texture is picked (the `texture` of a mask, an effect, a mesh or a spot light) the list also holds
the video assets; the lists of normal, environment and particle-pool maps still ask for pictures.

The list of assets shows a thumbnail of each (the picture, the first frame of a video, an "Aa"
sample of a font, an icon for sounds, color tables and models), in rows of 30 px with the name, the
kind and the size. Above it, filter words ("All" and each kind the scene has: Images, Videos,
Sounds, Meshes and models, Fonts, Color tables) and "Search for an asset" narrow the list to a kind
and to the assets whose name or file holds the text. It takes files dropped on it (it imports them)
and, before an asset in use is removed, lists every place that names it and asks to confirm. Files
dropped on the canvas import the pictures, videos and sounds among them and add one layer for each.
The import dialog also offers `.mp3`, `.ttc`, `.otc` and `.woff2` files.

The sprite cutter (the *Cut sprites* button of each picture) shows the picture with its numbered
frames: a grid is cut by rows, columns, margin, spacing, number of frames and order; dragging over
the picture draws a frame, dragging a frame moves it and dragging its corner resizes it; a strip of
the resulting frames plays them at the chosen frames a second, and lets them be reordered, removed
and limited to the first ones (`frames`).

## New ids, keys and timelines

A new layer gets an id made from its `name`, or else from its kind (`text`, `text_2`…; accents
are dropped, other characters become `_`, and a name with no Latin letter or digit falls back to
the kind), and sensible defaults; a text layer starts with its name as its text, and a particles
layer without a `source` also gets a new system. The app sends the name of the kind in the
language of the desktop (`Bild` becomes `bild`), and offers an image, video or sound layer only
once there is an asset of that kind. `keyframe` turns a plain value into
`{"keys": [[time, value]]}`, adds or replaces the key at that time (to the millisecond), keeps the
keys sorted and turns the value back into a plain one when its last key goes; it refuses a value
that follows another source. Keyframe times are seconds on the named timeline, or on the scene
clock without one. In the timeline of the app a keyframe is dragged along its track, to the
hundredth of a second, and dropped with `keyframe` and `from`.

The editor samples keys with the same code as the engine (`reiwand_scene::sample_json_as`): it reads
the list as the scene reads it (numbers, pairs, triples, colors, switches, texts and emitters), calls
the same sampling with its smoothing and its Bézier handles and writes the result as JSON, with
colors in hexadecimal and mixed in OKLab as on the screen. That way the handles, previews, lights,
clocks and estimates that the editor computes at a given second match what the engine draws; only a
list that the scene cannot read is mixed separately, part by part. The type is decided by the schema
of the animated field (`reiwand_scene::sample_json_as` with the name of the type of its keys): in a
color field, or in an effect parameter, a list of three or four numbers is mixed as a color in OKLab,
the same as in hexadecimal, and the sampled value (a new key, what the inspector shows, the curve of
the curve editor) keeps the list form; in a three-number field, such as a 3D angle, it is
interpolated part by part.

Key edits (`keyframe`, `key` and `edit_keys`) are checked against the scene before being applied, also
in `actions::plan`, and when something cannot be read the message names the field and the form it
needs instead of the interpreter's text: `ease` lists the format's smoothings and the cubic Bézier
of four numbers, a value gives the form of the keys it already has (a number, a color, a list of N
numbers), the base, the handles and the timeline state their own limit, and a clip key says the
channel and its range.

The panel of a timeline (menu *Timelines*, *Edit the timeline and its events…*) edits its length in
seconds or in frames and fps, the loop, the closing of the end with the start, the speed, the delay,
the self-start and the trigger, chosen among `start`, `beat`, the buttons and settings, the `event:`
of the scene's events, the `click:`, `enter:` and `leave:` of the layers that are touched and the
`media:`; its events are marks on a strip that are added with a double click, dragged, renamed and
removed with the `timeline` action. *Fire it in the preview* fires its trigger in the preview
(`SimulateTrigger`, `dbus.md`; `reiwand-ctl input trigger OUTPUT TRIGGER`), which starts that
timeline and all the timelines and sounds with the same trigger. Every trigger field of the
inspector, such as the `play_on` of a sound layer, has next to the picker the same fire-in-the-preview
button; its help says which timelines and which sound layers it starts, from the `fires` query
(`reiwand-ctl scene ask DIR fires '"event:boom"'`, which gives `timelines` and `sounds`).
