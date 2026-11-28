# Exports, events and inputs

On this page: the functions a module exports and the events it gets.

## The functions a module exports

A module exports the functions it wants. The scene calls each one within the time of a frame.

| Function | When it is called |
|----------|-------------------|
| `init(ctx)` | Once, on the first frame. `ctx.params` are the default values of `params` and `ctx.engine` is `engine`. |
| `update(frame)` | On every frame while the screen is in view. What it returns is the value of the `{ "script": "<its path>" }` sources of `scene.json`. |
| `destroy()` | When the scene stops. |
| `resize({ screen, canvas })` | When the size or the rotation of the screen changes. |
| `properties(changed)` | On the first frame, with all the properties the user chose; afterwards, only with those that change. |
| `settings({ language, fps, quality })` | On the first frame and when the frame rate or the quality change. |
| `timelineEvent(e)` | For each `{ timeline, name, t }` event of a timeline or a clip. |
| `mediaStatus({ enabled })` | On the first frame and when the user grants or withdraws `media` (modules that declare `media`). |
| `mediaPlayback({ state })` | When the player starts (`playing`), pauses (`paused`) or stops (`stopped`). |
| `mediaProperties({ title, artist, album, album_artist, subtitle, genres, kind })` | When another track arrives or its texts change; also with empty texts when it ends. |
| `mediaThumbnail({ has_cover, colors })` | When a cover arrives or goes away. `colors` brings `primary`, `secondary`, `tertiary`, `text` and `contrast` as `{ r, g, b, a }`, and never the image. |
| `mediaTimeline({ position, duration, rate })` | When a track starts, its duration or rate changes, or the player seeks. In seconds. |

`frame`, the argument of `update`, has `time` and `dt` (seconds), `props`, `audio`, `cursor`,
`actions`, `wall` (the wall clock time in seconds), `zone`, `timelines` and `events`. It is almost
always more convenient to use `engine`, `props`, `audio` and `input`, which give the same.

### One module, many values

Any animatable value of `scene.json` can be tied to a function of a module with
`{ "script", "fn", "params" }`. `fn` is an export that is not one of the above. This way one module
serves many values, each with its own parameters. There are two ways to export it:

- a function that is called on every frame as `fn(value, ctx)`;
- an object with `init(value, ctx)` (first frame) and `update(value, ctx)` (every frame).

`value` is what it returned last time (`null` the first time). `ctx` has `path` (the JSON Pointer
of the value in `scene.json`), `layer` (the id of the layer that has it), `key` and `params` (the
`params` of the module with those of the link on top). What it returns is added to `base` like any
other source. `params` without `fn` is not valid. Example 7 (below) uses it.

## Events and inputs

A script learns what happens in four ways.

**Time.** `update` runs on every frame and `engine.time` and `engine.dt` say how much has passed.
To do something later or every so often, use `setTimeout` and `setInterval`. With a scene without
`update`, a timer wakes the scene only when it is due.

**The pointer.** Read `input.cursor` in `update` to follow it, or handle `enter`, `leave`, `move`,
`down`, `up` and `click` of a layer with `layer.on(...)`. A layer only receives those events if it
is of type `solid` (or a `mesh` layer with `hit` bones). The pointer and clicks are inputs: the
manifest declares them in `inputs` and the user grants them per package. Clicks, in addition, the
user turns on for each screen. The keyboard never reaches a scene.

**Sound.** `audio.bass`, `mid`, `treble`, `level` and `beat` in `update`, or the bands with
`audio.bands()` and `audio.buffers()`. It is the `audio` input; without it (or without sound)
everything is 0. Those values are relative and in decibels. For another reading (absolute, or a
`surge` that jumps when the music comes back after a silence) declare up to 4 `listens` at the root
of the scene, each with `kind`, `unit` and, with `surge`, `window`, `reference` and `hold_silence`,
and read them with `audio.heard(n)` (64 bands) or `audio.heard(n, 16)`; an `n` you did not declare
throws `RangeError`.

**Properties.** `props.get(id)` at any time, or the `properties(changed)` function, which is only
called when something changed. `props.pressed(id)` reports a button.

Other sources: `timelineEvent` and `timeline(id).on(...)` for timelines, `settings` for the frame
rate and the quality, `resize` for the rotation of the screen and the `media...` functions for the
player.

A scene is only redrawn because of an input it uses.
