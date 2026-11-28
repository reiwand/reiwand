# The scene from a script

On this page: `engine`, `time`, `props`, `audio`, `input` and `scene`.

## The `reiwand` interface

Everything comes from `import { ... } from "reiwand"`.

## `engine`: the state of the scene

| Field | What it gives |
|-------|---------------|
| `time`, `dt` | The scene time and what has passed since the previous frame, in seconds. |
| `daytime` | The local time of day, from 0 (midnight) to almost 1. |
| `fps`, `quality` | The frame rate and the quality level (0 to 3). |
| `locale`, `language` | The language chosen among those of the package (including those of its text layers), as a valid language tag (`de`, `en-US`); `en-US` if the package has none. |
| `canvas`, `screen` | `[width, height]` of the canvas and of the screen, without output names. |
| `device` | `desktop` or `mobile`. |
| `mode` | `wallpaper`, `lockscreen`, `screensaver`, `editor` or `preview`. |
| `orientation` | `landscape` or `portrait`. |

## `time.format(pattern, date)`

Gives the date (that of `date`, or now) written with the letters of the `date` values of
`scene.json`, in the language of `engine.locale`: `time.format("HH:mm")`,
`time.format("EEEE d MMMM")`.

## `props`: the properties the user chooses

| Function | What it gives |
|----------|---------------|
| `props.get(id)` | The current value. A chosen file or folder gives `"wpak-user://<id>"`, never the path, and `null` while there is none. |
| `props.all()` | All the values, in one object. |
| `props.label(id)` | The text of the chosen option of a combo, in the language of whoever uses the wallpaper. |
| `props.color(id)` | A color as `{ r, g, b, a }` from 0 to 1. |
| `props.pressed(id)` | `true` if that button was pressed since the previous frame. |

A script never writes properties. If it wants something to change with a property, it reads it in
`update` or in `properties` and changes the layers.

## `audio`: the sound

| Part | What it gives |
|------|---------------|
| `bass`, `mid`, `treble`, `level` | Levels from 0 to 1. |
| `beat` | `true` on the frame of a beat. |
| `bands(n, channel)` | A new list of `n` bands (16, 32 or 64) of the channel `"left"`, `"right"` or `"average"` (the default). |
| `buffers(n)` | `{ left, right, average }` as `Float32Array`s of `n` bands that the scene fills by itself before each frame. Ask for them once, outside `update`. |

Without sound, or without the user allowing it, everything is 0.

## `input`: the pointer

`input.cursor` has `x` and `y` (fractions of the screen, from 0 to 1), `inside` (the pointer is on
the screen), `buttons` and `left` (the left button is pressed), `scene` (`[x, y]` in canvas pixels)
and `screen` (`[x, y]` in screen pixels). `buttons` and `left` are 0 without the `clicks` input.
When the pointer leaves, it goes smoothly back to the center in about 0.15 s.

## `scene`: the layers and the stage

| Function | What it does |
|----------|--------------|
| `scene.layer(id)` | The handle of a layer. It is always the same object for the same id. |
| `scene.layers()` | The ids of the top-level layers, in drawing order. |
| `scene.spawn(template, options)` | Adds a copy of a template from `templates` (declare `layers`). |
| `scene.destroy(id)` | Removes a copy that a script made. |
| `scene.order(id, index)` | Moves a top-level layer; 0 is the background. |

`scene.spawn` puts the copy on top of everything, or at `options.at`. Its id is `options.id`
(letters, digits, `_` and `-`, up to 64) or `<template>_<n>`. The other `options` are fields of
the layer (`position`, `opacity`...). The children of a copied group are called `<id>.<child>`.
Going over 256 copies, or over 512 layers in total, throws an error.

### The fields of a layer

They are read and written by name: `layer.opacity = 0.5`.

| Field | Value |
|-------|-------|
| `position`, `origin`, `skew` | `[x, y]`; `skew` in degrees, up to ±89 |
| `scale`, `parallax` | `[x, y]` or a single number |
| `rotation` | degrees, clockwise |
| `opacity` | 0 to 1 |
| `visible` | `true` or `false` |
| `color` | `[r, g, b, a]` from 0 to 1, `{ r, g, b, a }` or `"#rrggbb"` |
| `size` | `[width, height]` of images, videos, solids and spectra; the size in points of a text |
| `text`, `font`, `align` | the text (up to 4096 characters), the font (one of the scene, or `sans-serif`, `serif`, `monospace`) and `left`, `center` or `right`; the text a script sets applies in all languages, and when reading it from a layer with per-language text you receive that of the language being shown |

Vectors accept lists, `{ x, y }` or those of `reiwand/math`. Reading a field gives what the script
wrote or, if it wrote nothing, what the scene computed on the previous frame. What you write lasts
until you write it again or call `layer.reset("field")` (`layer.reset()` releases them all). The
scene clamps what it receives: positions and sizes to 100,000 pixels, scales to 1000, `origin` and
`parallax` to ±10, opacity and colors to 0..1.

Other members of a layer:

| Member | What it is |
|--------|------------|
| `id`, `name`, `type`, `initial` | Its identifier, its type and the layer as `scene.json` writes it. |
| `on(event, function)` | Handles `click`, `down`, `up`, `enter`, `leave` and `move` on `solid` layers or `hit` bones (below). |
| `effect(index or name)` | An effect of the layer (declare `effects`). |
| `video`, `anim(clip)` | Control of videos and clips (declare `animations`). |
| `parent`, `children`, `world`, `setParent(id, keep)` | The hierarchy (declare `hierarchy`). |
| `bones`, `bone(name)`, `shape(name, weight)`, `impulse(bone, [vx, vy])`, `hit(x, y)`, `attachment(name)` | Bones, blend shapes and anchors of a `mesh` or `model` layer (declare `animations`). In a model, positions are in pixels of its box and the bone keeps its depth. |
| `destroy()` | Removes the layer if it is a copy that a script made. |

**Effects.** `layer.effect(0)` or `layer.effect("glow")` (the name is put in `uses`,
`[a-z0-9_]{1,32}`) gives `enabled`, `set(parameter, value)`, `get(parameter)`, `reset()`, `index`,
`name` and `effect`. `set` accepts numbers, vectors, colors, switches, blend modes and lists of
emitters (up to 64 `{ from, to, angle, strength }`), checks them against the description of the
effect and clamps them to its range. The images of an effect cannot be changed. What you set lasts
until you change it.

**Pointer events.** `layer.on("click", (e) => ...)` receives `{ type, layer, local, scene, bone }`:
`local` is pixels of the layer from its top-left corner, `scene` canvas pixels and `bone` the bone
touched (`null` if none). `enter`, `leave` and `move` follow the topmost `solid` layer under the
pointer. `down` goes to that layer, which also receives `move` and `up` until all buttons are
released; `click` follows an `up` over the layer that received the `down`. They arrive on the next
frame. `enter`, `leave` and `move` ask for the `pointer` input; `click`, `down` and `up`, `clicks`.
Clicks are off on each screen until the user turns them on, and they come only from the compositor
or the desktop, never from another program.

**Video.** `layer.video` (declare `animations`) has `play()`, `pause()`, `stop()` (goes back to
`start`), `seek(seconds)`, `time`, `duration`, `playing`, `rate` (0 to 16), `volume` (0 to 1) and
`on("ended", function)`.

**Clips.** `layer.anim(clip)` controls the animation layers of a `mesh` layer that touch that
clip: `play()`, `pause()`, `stop()`, `seek(seconds)`, `rate` (0 to 16), `weight` (0 to 1), `time`,
`duration`, `playing` and `on("ended", function)`.

**Hierarchy.** With `hierarchy`, `layer.parent` is the id of the layer it follows (or `null`),
`layer.children` the ids of those that follow it and `layer.world` a `Mat3` of `reiwand/math` from
the pixels of the layer to those of the canvas. `layer.setParent(id, keep)` makes it follow another
(`null` for none); with `keep` the layer stays where it is. A layer cannot follow itself or follow
one that follows it, or have more than 8 chained parents.

### The stage

With `camera` (and `bloom` for the glow): `scene.camera` (`center`, `zoom`, `rotation`,
`shake: { enabled, amplitude, roughness, speed }`), `scene.parallax` (`amount`, `delay`, `pointer`,
`offset`), `scene.bloom` (`threshold`, `strength`, `radius`, `tint`), `scene.camera3d` (`fov`,
`eye`, `center`, `up`) and the colors `scene.background`, `scene.ambient` and `scene.sky`. They are
read as the scene left them in `canvas` plus what the script wrote. Writing an object changes only
the fields it names, within the ranges of `canvas`; writing `null` gives the field back to the
scene. The camera shake stands still with the reduce-motion option and at level Q0.

### Custom shapes

With `geometry` and a `pools` in `scene.json`: `scene.mesh(pool)` gives a set of shapes with
`create({ vertices, indices, format, visible })`, `shapes` and `clear()`; each shape has `update`,
`destroy`, `id` and `alive`. `vertices` is a `Float32Array` or a list of numbers in pixels of the
set, with `format` `"xy"`, `"xyuv"` (the default), `"xyrgba"` or `"xyuvrgba"`.
