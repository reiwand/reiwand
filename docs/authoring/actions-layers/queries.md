# Queries

### Before you start

Queries are made with `reiwand-ctl scene ask FOLDER NAME 'JSON'` and answer with JSON; they change nothing. The rules for ids, paths and coordinates are those of [the actions](before-you-start.md#before-you-start). Those that are about a layer also accept `--layer ID` when the arguments do not name it.

### Layers, assets and video

#### `copy`

Answers with the layers being copied and everything they need: the assets, particle systems and package effects they use. It is the text that the [`paste`](layers.md#paste) action pastes, in the same scene or another. In the app, **Copy** in the layers menu.

| Argument | Type | What it means |
|---|---|---|
| `ids` | list of texts | (required) Ids of the layers being copied. |

The answer has `from` (the folder of the scene it comes from), `layers`, `assets`, `particles` and `effects`. If an id does not exist, it brings `error`. From the command line there is also `reiwand-ctl scene ask FOLDER copy`.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"solid","id":"background"}'
reiwand-ctl scene ask my-scene copy '{"ids":["background"]}'
```

#### `unkept`

Says which layers would change place if a layer or a group were removed: those that have it as a parent and whose position cannot be kept without it. The app uses it to warn before removing.

| Argument | Type | What it means |
|---|---|---|
| `id` | text | Id of the layer or group. If missing, `--layer` is used. |

The answer is `{"layers": [...]}` with the ids of those layers. An empty list means that removing it moves nothing.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"solid","id":"background"}'
reiwand-ctl scene ask my-scene unkept '{"id":"background"}'
```

#### `asset_uses`

Says where the scene uses an asset: each document path that names it and the layer it belongs to. It is for knowing whether an asset can be removed without breaking anything.

| Argument | Type | What it means |
|---|---|---|
| `asset` | text | (required) Id of the asset. |

The answer has `asset` and `uses`, a list of `{pointer, layer}` where `layer` is `{id, type}` or `null` if the use is not from a layer (for example, one from a property).

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png","id":"sky"}'
reiwand-ctl scene ask my-scene asset_uses '{"asset":"picture"}'
```

#### `film`

Answers with the video of a video layer: its file, size, length, the trim, the speed and whether the speed has keys. It is for seeing which part of the video is shown. In the app, the layer's video trim.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text | (required) Id of the video layer. |
| `strip` | integer | How many thumbnails to ask for, spread over the length. Default 0, none. |

The answer has `layer`, `pointer`, `asset`, `file`, `size`, `length` (in seconds), `start`, `end`, `rate`, `loop` and `keyed`. With `strip`, it also brings `strip`, a list of `{at, file}` with the thumbnail of each instant. If the layer does not exist or has no video, the answer is `null`.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"video","file":"video.webm","id":"clip"}'
reiwand-ctl scene ask my-scene film '{"layer":"clip","strip":3}'
```

#### `sprites`

Answers with the frames an image is cut into: the rectangles, the atlas, the speed and the durations. They are written with the [`sprites`](assets.md#sprites) action. In the app, the frame-sheet cutter.

| Argument | Type | What it means |
|---|---|---|
| `asset` | text | (required) Id of the image. |

The answer has `asset`, `file`, `size`, `rects` (each rectangle `[x, y, width, height]` in pixels), `atlas`, `fps`, `frames`, `durations` and `most` (the maximum number of frames). If the asset is not an image, it is `null`.

```sh
reiwand-ctl scene act my-scene import '{"file":"sheet.png","id":"sheet"}'
reiwand-ctl scene act my-scene sprites '{"grid":{"asset":"sheet","columns":4,"rows":2}}'
reiwand-ctl scene ask my-scene sprites '{"asset":"sheet"}'
```

#### `touch`

Answers with the layers that receive the pointer and how they receive it: bones with a touch zone and layers with `touch` other than `off`. It is written with the `touch` variant of [`layer`](place-and-shape-a-layer.md#layer).

It takes no arguments: pass `{}`.

The answer is `{"layers": [...]}`, and each layer brings `id`, `pointer`, `takes` (`all` or `alpha`), `alpha` (the threshold) and `bones` (how many bone zones it has).

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png","id":"sky"}'
reiwand-ctl scene act my-scene layer '{"touch":{"layer":"sky","takes":"alpha","alpha":0.5}}'
reiwand-ctl scene ask my-scene touch '{}'
```

### Shapes, text and zones

#### `shape_handles`

Answers with the handles shown on the canvas to shape a shape layer, and with the ends of its gradient. They are moved by the `shape` variant of [`layer`](place-and-shape-a-layer.md#layer) (with `figure`). In the app, the shape's handles.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text | (required) Id of the shape layer. |
| `time` | number | Second at which the layer's position is measured. Default 0. |

The answer has `kind` (the kind of shape), `handles` (each handle with `name`, `kind`, `u`, its place inside the shape, and `at`, its place on the canvas) and `gradient` (`radial`, `stops`, `from` and `to`, or `null` with no gradient). If the layer is not a shape, it is `null`.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"shape","id":"shape"}'
reiwand-ctl scene ask my-scene shape_handles '{"layer":"shape","time":0}'
```

#### `text_box`

Answers with the box a text layer's text occupies: its size, how many lines it has, the width handle and the corners. In the app, the text's width handle.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text | (required) Id of the text layer. |
| `time` | number | Second at which it is measured. Default 0. |

The answer has `size`, `lines`, `widest` (the widest line), `wrapped` (whether the text is broken by a width), `width`, `handle` and `corners`. If the layer is not a text layer, it is `null`.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"text","id":"title"}'
reiwand-ctl scene ask my-scene text_box '{"layer":"title","time":0}'
```

#### `font_list`

Answers with the typefaces a text layer can use and with the chain it has already chosen. In the app, the typeface selector.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text | (required) Id of the text layer. |

The answer has `chain` (the chosen typefaces, in order), `package` (those the package brings), `system` (those of the system), `generic` (`sans-serif`, `serif` and `monospace`) and `most` (how many the chain can have).

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"text","id":"title"}'
reiwand-ctl scene ask my-scene font_list '{"layer":"title"}'
```

#### `capture_view`

Answers with the zone an adjustment layer captures, to draw its handles on the canvas. It goes with the [`rgb`](layers.md#rgb) action, which sends that zone to the RGB lighting.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text | (required) Id of the adjustment layer, which must have `size`. |
| `time` | number | Second at which it is measured. Default 0. |

The answer has `size`, `max` (the largest size that is accepted), `over` (whether the size exceeds that maximum), `redraw`, `zone` and `limit` (the four corners of the zone and of the limit), `drawn` and `handles` (each handle with its `corner` and its place `at`). If the layer is not an adjustment layer or has no `size`, it is `null`.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"adjustment","id":"adjust"}'
reiwand-ctl scene act my-scene layer '{"transform":{"layer":"adjust","size":[400,300]}}'
reiwand-ctl scene ask my-scene capture_view '{"layer":"adjust","time":0}'
```
