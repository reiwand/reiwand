# The `reiwand` module

| Part | What it gives |
|------|---------------|
| `engine` | `time`, `dt`, `daytime` (0 at local midnight to almost 1), `fps`, `quality` (0 to 3), `locale` and `language`, which always hold the same value (the language picked among the package's languages, as for its texts, in lowercase, or `""`), `canvas` and `screen` (`[w, h]`, without names of outputs or monitors), `device` (`desktop` or `mobile`), `mode` (`wallpaper`, `lockscreen`, `screensaver`, `editor` or `preview`) and `orientation` (`landscape` or `portrait`) |
| `time` | `format(pattern, date)`: the date (`date` or now) with the tokens of `date` values in the language of `engine.locale`, such as `time.format("EEEE d 'de' MMMM")`; `Date` uses the local time zone of the computer, also after it changes, without reading any file |
| `props` | `get(id)` (a chosen file or folder reads `"wpak-user://<id>"`, never its path, and `null` while none is chosen), `all()`, `label(id)` (the label of the chosen option of a combo in that language), `color(id)` (`{ r, g, b, a }` from 0 to 1) and `pressed(id)` (a button was pressed since the last frame); scripts never write settings |
| `audio` | `bass`, `mid`, `treble`, `level`, `beat`, `bands(16 \| 32 \| 64, "left" \| "right" \| "average")` (a new array) and `buffers(16 \| 32 \| 64)`: `{ left, right, average }` as `Float32Array`s that the engine fills in place before each frame |
| `input` | `cursor`: `x`, `y` (fractions of the screen), `inside`, `buttons`, `left`, `scene` (canvas pixels) and `screen` (screen pixels); `buttons` and `left` stay 0 without the `clicks` input |
| `sound(id)` | a sound layer of the scene (`uses: ["sounds"]`, `script_sounds`): `play()` (from where `pause()` left it, or from the start), `pause()`, `stop()` (back to the start), `playing` and `volume` (0 to 1, multiplied by the volume of the layer and the user's). The first call takes the sound over from its `play_on`; the layer's own `play` and the user's mute still silence it. The phone obeys them like the desktop |
| `scene` | `layer(id)`, `layers()` (the ids of the top-level layers in drawing order), `spawn(template, options)`, `destroy(id)` and `order(id, index)`, and `mesh(pool)`, `camera`, `camera3d`, `parallax`, `bloom` and `background` (see below) |
| `storage` | `get`, `set`, `delete`, `clear` and `keys` of what the scripts keep (see below) |
| `folder(prop)` | the pictures of a folder the user chose (see below) |
| `media` | what the media player plays (`uses: ["media"]` or a media event, input `media`): `enabled`, `state` (`playing`, `paused` or `stopped`), `playing`, the texts `title`, `artist`, `album`, `album_artist`, `subtitle`, `genres` and `kind`, `has_cover`, `colors` (as in `mediaThumbnail`), `position` (running on from the last reading at its `rate`, without new frames), `duration`, `rate`, `progress` (0 to 1) and `snapshot()`, a frozen copy of all of them. Without the grant it reads `enabled: false`, `stopped` and empty texts; it never carries the cover or the name of the player, and the engine never logs it. Reading it from a module that does not declare it throws |
| `timeline(id)` | `play()`, `pause()`, `reverse()`, `stop()`, `seek(t)`, `speed` (−100 to 100; below zero the timeline runs backwards like after `reverse()`, and one that plays `once` ends at its start), `time`, `duration`, `playing` and `on(name, handler)` for its events and `ended` |
| `particles(layer)` | `play`, `pause` (no births, particles keep moving), `stop` (removes them until `play`), `playing`, `emit(n)` (1 to 1000), `point(n, [x, y])`, `from(n, id)` and `wall(n, id)` (emitter or collider `n`, 0 to 7, of kind `layer` takes the picture of layer `id`, any layer a frame draws, a copy of a template included, instead of the one its system names, until `null` gives it back) and the `instance` fields `alpha`, `size`, `count`, `speed`, `life`, `time_scale`, `brightness` and `color`, which last until changed |
| `shared` | an object shared by all the modules of the scene: at most 256 values, each a frozen copy of what was set (numbers, strings, booleans, arrays, typed arrays and objects of them, at most 65536 items and 32 levels in all), so a value changes by setting it again; getters, functions and other prototypes are refused; modules may also `import` each other with relative paths (`./lib/bands.js`), among the files of `scripts` |
| `setTimeout(fn, ms)`, `setInterval(fn, ms)` | timers on the scene act DIR canvas`clearTimeout` and `clearInterval` call it), run at the start of a frame, at most once a frame each, without making up for missed periods and never under 16 ms; at most 64 wait at once, they wait while the scene is paused, and the scene sleeps until the next one |
| `console` | `log`, `info`, `warn`, `error` and `debug`: at most 20 lines a second of at most 512 characters, to the log of the engine at debug level and to the editor |

`scene.layer(id)` returns the same handle every time; the first 64 layers a script reads, or asks
for while it loads or in `init`, report their evaluated values every frame (a layer read for the
first time later reads what the script wrote until the next frame). Reading `position`, `scale`, `rotation`, `skew`, `origin`,
`opacity`, `visible`, `color`, `parallax`, `size` (the box of pictures, videos, solids and spectra,
the points of a text), `text`, `font` or `align` gives what the script wrote, or else the value the
scene evaluated at the last frame; `id`, `name`, `type` and `initial` (the layer as `scene.json` writes
it, its children by id) describe it. Writing those fields lasts until the script writes them again or
calls `reset(field)` (`reset()` clears them all); vectors take arrays, `{ x, y }` or `reiwand/math`
vectors, colors also `{ r, g, b, a }` or `"#rrggbb"`, and `scale` and `parallax` a single number. The
engine bounds what it receives: positions and sizes to 100000 pixels, scales to 1000, `skew` to
±89°, `origin` and `parallax` to ±10, `opacity` and colors to 0..1, texts to 4096 characters and their
points to 1..2048, and fonts to the fonts the scene names or `sans-serif`, `serif` and `monospace`.
`layer.on(event, handler)` handles the pointer events of the layer (see the declarations): each
handler gets `{ type, layer, local, scene, bone }`, `local` in pixels of the layer from its top left corner
(of the group for a solid group), `scene` in canvas pixels and `bone` the touched bone of a `mesh`
layer that lists `hit` bones (else `null`). `enter`, `leave` and `move` follow
the topmost solid layer under the pointer; `down` goes to that layer, which then takes `move` and
`up` until every button is released, also off it, and `click` follows an `up` on the layer that took
the `down`. They reach the scripts in the next frame, at most 64 a frame.

`layer.video` controls the video the layer shows (`uses: ["animations"]`): `play()` (from `start`
once it ended), `pause()`, `stop()` (back to `start`, paused), `seek(seconds)` (at most one jump a
frame: the last `seek` of a frame counts), `time`, `duration` (`undefined` while it is not known),
`playing`, `rate` (0 to 16, read and written), `volume` (0 to 1, read and written: it
scales the sound of the video under the volume of the videos of the package and the general volume;
layers that share a clock share it; the videos inside a scene play without sound on the phone, so there it
changes nothing) and `on("ended", handler)`, called with `{ layer,
name }` in the frame after a video without `loop` reaches its end. Calls take the video over from the
`playback` of the layer until the scene starts again.

`layer.anim(clip)` controls the animation layers of a `mesh` layer that play the clip with that id
(`uses: ["animations"]`): `play()` (from the start once a `once` clip ended, fading in over its
`blend_in`), `pause()`, `stop()` (back to the start, fading out over its `blend_out`), `seek(seconds)`
(within the clip), `rate` (0 to 16), `weight` (0 to 1, the last one the script set, 1 before), `time`,
`duration`, `playing` and `on("ended", handler)`, called with `{ layer, clip, name }` in the frame
after a `once` clip ends. A clip that no animation layer plays gets a replacing one, at weight 1,
the first time a script calls it. Calls take the layer over from its `rate`, `weight` and
`enabled` until the scene starts again; animations run before the scripts of the next frame.

Bones, blend shapes and attachments of a `mesh` or a `model` layer take `uses: ["animations"]` too. `layer.bones`
lists its bones (every node of its file) and `layer.bone(name)` returns a handle with `name`,
`parent`, `rotation` (degrees clockwise), `position` (`[x, y]` pixels, y down, in the space of its
parent) and `scale`, measured from the rest pose like the keys of clips, and `worldPosition` (pixels
of the box of the layer from its top left) and `worldRotation`; reading gives what the script set
or the pose of the last frame. Writing sets the bone for the next frame after the animations, rules
and chains and before the bodies of `physics`, so a bone with a body springs back once let go;
writing `worldPosition` or `worldRotation` replaces the local `position` and `rotation` and the other
way round, and `reset()` lets the bone go. `layer.shape(name, weight)` sets a blend shape or
expression (−1 to 1) instead of what `shapes` gives it and `layer.shape(name)` reads what the script
set; `layer.impulse(bone, [vx, vy])` pushes the end of a bone that has a body by that speed in pixels
a second, once. `layer.hit(x, y)` names the bone nearest to that point of the box (among `hit` when
it lists any) and `layer.attachment(name)` gives `{ matrix, origin, angle, uv }` of a bone or node:
its frame in canvas pixels (`[a, b, c, d, e, f]`), where it is, its angle in degrees clockwise and
its place as fractions of the box. A layer that reads bones reports their pose every frame, at
most 256 of them; the engine bounds positions to 100000 pixels, scales to 1000 and impulses to 16 a
frame.
On a `model` layer the same calls work on the bones of its skeleton, in pixels of its box with the layer
centered on it (a pixel is 0.01 units of the model, the same as `impulse`), and the model keeps its depth:
a bone moved with `position` or `worldPosition` slides in the plane of x and y and keeps its z. A model
needs `clips`, `physics`, `ik` or `root_motion` to have a skeleton a script can move; a still model has none.

`layer.effect(index or name)` controls one effect of the layer: `enabled`, `set(parameter, value)`
(numbers, vectors, colors, switches, blend modes and emitter lists, checked against the effect's
description and clamped to its range, where an emitter list takes at most 64 `{ from, to, angle,
strength }` and clamps each field to its range; pictures cannot be changed), `get(parameter)`, `reset()` (clears what an effect
with `state` keeps) and `index`, `name` and `effect`. What a script sets lasts until changed, and the
inputs of the scene count every effect of a scene whose scripts use `effects` as shown.
