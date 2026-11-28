# More of the script interface

On this page: sounds, timelines, particles, storage, folders, media, shared data, timers and math.

## `sound(id)`: a sound of the scene

Declare `sounds`. `sound(id)` has `play()` (continues where `pause()` left it, or starts),
`pause()`, `stop()` (goes back to the start), `playing` and `volume` (0 to 1, multiplied by that of
the layer and by the one the user sets). The first call takes control away from `play_on`. The
mute the user sets still applies.

## `timeline(id)`: timelines

`play()`, `pause()`, `reverse()`, `stop()`, `seek(t)`, `speed` (from −100 to 100; below zero it
runs backwards), `time`, `duration`, `playing` and `on(name, function)` for its events and for
`ended`.

## `particles(layer)`: particles

`play()`, `pause()` (no more are born, the existing ones go on), `stop()` (removes them until the
next `play`), `playing`, `emit(n)` (1 to 1000), `point(n, [x, y])`, `from(n, id)` and `wall(n, id)`
(emitter or collision `n`, from 0 to 7, takes the image of layer `id`; `null` gives it back) and
the fields `instance.alpha`, `size`, `count`, `speed`, `life`, `time_scale`, `brightness` and
`color`, which last until you change them.

## `storage`: what the scene remembers

Declare `storage`. `storage.get(key, scope)`, `set(key, value, scope)`, `delete`, `clear` and `keys`
store JSON values. The scope is `"package"` (the default, shared by the screens) or `"output"` (of
each screen). There are 64 KiB per scope and `set` throws an error if you go over or if the value
is not JSON. It is stored only on the computer (never synced or exported), in a private file that
is written at most every 5 s and when the scene stops, and is deleted with the package. The editor
previews read what is stored but do not write.

## `folder(property, options)`: a chosen folder

Declare `folders`. It lists a `folder` property with `mode: all`. `files` are opaque identifiers of
its images (also of its videos with `options.videos: true` if the property accepts `video`).
`name(id)` gives the file name, `kind(id)` gives `"image"` or `"video"`, and
`on("added" | "removed" | "changed", function)` receives `{ prop, files }` when the folder changes.
`show(layer, id)` makes an image layer show that image, or a video layer play it (`null` gives it
back to its own); up to eight files at a time. You never see paths.

## `media`: what plays in the player

Declare `media` and ask for the `media` input. `media` has `enabled`, `state` (`playing`,
`paused` or `stopped`), `playing`, the texts `title`, `artist`, `album`, `album_artist`,
`subtitle`, `genres` and `kind`, `has_cover`, `colors` (as in `mediaThumbnail`), `position` (keeps
running between readings), `duration`, `rate`, `progress` (0 to 1) and `snapshot()` (a frozen copy
of everything). Without the permission it says `enabled: false`, `stopped` and empty texts. It
never brings the cover art or the name of the player.

## `shared`: data between modules

An object that all the modules of the scene share. Each value is stored as a frozen copy (numbers,
texts, booleans, lists, typed arrays and objects of them), so to change it you assign again:
`shared.count = shared.count + 1`. It does not accept functions, `Map`, `Set`, `Date` or getters.

## Timers and console

| Function | What it does |
|----------|--------------|
| `setTimeout(f, ms)`, `setInterval(f, ms)` | Timers on the scene clock (declare `timers`). They return a function that cancels them; `clearTimeout` and `clearInterval` call it. They run at the start of a frame, at most once per frame each, without recovering the periods that were lost, and they wait while the scene is paused. |
| `console.log`, `info`, `warn`, `error`, `debug` | Write to the script console (below). |
| `openShortcut(property)` | Opens what the user chose in a `shortcut` property. Returns `true` only inside a `click` handler and up to four properties per frame (declare `shortcuts`). |

## `reiwand/math`

`Vec2`, `Vec3` and `Vec4` (`add`, `sub`, `mul`, `div`, `min`, `max`, `clamp`, `scale`, `negate`,
`abs`, `floor`, `ceil`, `round`, `dot`, `length`, `lengthSquared`, `normalize`, `distance`, `mix`,
`lerp`, `reflect`, `project`, `equals`, `isZero`, `toArray` and, depending on the size, `cross`,
`angle`, `rotate` and `perpendicular`), `Mat3` (2D: `translation`, `rotation`, `scaling`,
`transformPoint`, `transformVector`, `decompose`) and `Mat4` (`translation`, `scaling`,
`rotationX`, `rotationY`, `rotationZ`, `fromEuler`, `perspective`, `lookAt`, `transformPoint`,
`transformVector`, `decompose`), both with `multiply`, `transpose`, `determinant`, `invert`,
`equals` and `get(row, column)`. Also `clamp`, `mix`, `smoothstep`, `step`, `fract`, `remap`,
`degrees`, `radians`, `angleDelta`, `mixAngle`, `angleVector`, `vectorAngle`, `lookAt`, `rgbToHsv`
and `hsvToRgb` (hue goes from 0 to 1). Vectors and colors go in and out as these classes or as
lists.
