# Assets

Assets are the scene's files: images, videos, sounds, fonts, `.cube` color tables and `.glb` models. They live in the `assets/` folder and are declared in `/scene/assets`. The list is in the app's **Assets** panel. For effects and scripts that are imported as files, see [Shaders](../actions.md) and [Scripts and templates](../actions.md).

### variant `start`

Ready-made starting points for what is asked for most: a clock or a date, and a movement that never ends. Inside `start` you choose `clock` or `forever`.

#### `start` with `clock`

Adds a text layer that shows the machine's time or date and updates by itself. It is the same as writing `{ "clock": "HH:mm" }` or `{ "date": "EEEE d MMMM yyyy" }` in a layer's text; writing `{time}` does nothing, because the text you write comes out as is.

| Argument | Type | What it means |
|-----------|------|---------------|
| `format` | text | The time or date format, such as `HH:mm`, `hh:mm a` or `d MMMM yyyy` (the letters are in [scene](../scene/values.md#clock-and-dates), and the `clock_help` query lists them with an example). If missing, `HH:mm` for the time and `EEEE d MMMM yyyy` for the date. Up to 64 characters. |
| `date` | yes/no | `true` shows a date (`date`) instead of the time (`clock`). Default `false`. |
| `id` | text | Id of the layer. If missing, `clock` or `date` with a number if it already exists. |
| `into` | text | Id of a group to put the layer in. |
| `size` | number | Text size, from 1 to 4000 pixels. Default 160. |

```sh
reiwand-ctl scene act my-scene layer '{"start":{"clock":{"format":"HH:mm","id":"time"}}}'
reiwand-ctl scene act my-scene layer '{"start":{"clock":{"date":true,"id":"day","size":80}}}'
```

#### `start` with `forever`

Makes a layer spin or drift without stopping, without building keys by hand: it writes two linear keys with `loop` on its rotation or its position, which repeats or goes back and forth.

| Argument | Type | What it means |
|-----------|------|---------------|
| `layer` | text (required) | Id of the layer. |
| `motion` | text (required) | `spin` makes a full turn every `seconds` and starts another; `drift` moves it `by` pixels and brings it back. |
| `seconds` | number | How long a turn (`spin`) or a trip (`drift`) lasts, from 0.5 to 86400 seconds. Default 20. |
| `reverse` | yes/no | Turns the other way, or drifts to the other side. Default `false`. |
| `by` | list of 2 numbers | Only with `drift`: how far it moves, in canvas pixels. Default `[200, 0]`. |
| `back` | yes/no | Only with `drift`: `true` (default) goes back and forth smoothly; `false` jumps to the start and repeats the trip. |

It starts from the rotation or position the layer has now (with no position, the center of the canvas) and replaces any keys there were. If the rotation or the position follows another source (sound, a property…), the action rejects it: first set a value with `transform` and `replace`.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"shape","id":"wheel"}'
reiwand-ctl scene act my-scene layer '{"start":{"forever":{"layer":"wheel","motion":"spin","seconds":12}}}'
reiwand-ctl scene act my-scene add_layer '{"kind":"shape","id":"cloud"}'
reiwand-ctl scene act my-scene layer '{"start":{"forever":{"layer":"cloud","motion":"drift","by":[300,0],"seconds":30}}}'
```

## `import`

Copies a file to `assets/` and declares it as an asset. It prints the asset's id (by default, the file name without the extension). If an identical file is already in `assets/`, it reuses it. It accepts images, videos, sounds, fonts, `.cube` tables and `.glb`.

| Argument | Type | What it means |
|-----------|------|---------------|
| `file` | text (required) | File that is copied, with its path from where you run the command. |
| `id` | text | Id of the asset. If it already exists, the action fails. |
| `linear` | yes/no | `true` if the image stores data (a normal map, a mask) and not colors; that way its color is not corrected. Images only. Default `false`. |
| `atlas` | list of 2 integers | `[rows, columns]` of a regular sprite sheet, from 1 to 256 each. Images only. For irregular cuts use [`sprites`](#sprites). |

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene import '{"file":"picture.png","id":"map","linear":true}'
reiwand-ctl scene act my-scene import '{"file":"sheet.png","id":"sheet","atlas":[2,4]}'
reiwand-ctl scene act my-scene import '{"file":"sound.ogg"}'
reiwand-ctl scene act my-scene import '{"file":"model.glb","id":"model"}'
reiwand-ctl scene act my-scene import '{"file":"video.webm"}'
```

## `asset`

Changes how an image asset is stored. It does one of three things, depending on the arguments: convert it to a compressed texture (KTX2), make it the cover art of what the media player is playing, or show that cover art in a scene field. In the app, **Convert to KTX2…** is on each image in the **Assets** panel.

| Argument | Type | What it means |
|-----------|------|---------------|
| `asset` | text (required) | Id of the asset. |
| `ktx2` | yes/no | `true` converts the image to a KTX2 texture with its mipmaps, writes `assets/NAME.ktx2` and points the asset at that file. It does not work on an image that is already KTX2. Default `false`. |
| `codec` | text | With `ktx2`: `uastc` (sharper, the normal choice) or `etc1s` (smaller). |
| `quality` | integer | With `ktx2`: quality from 1 to 100. Default 80. |
| `cube` | yes/no | With `ktx2`: the image is a panorama and is converted into a six-face cube. |
| `faces` | list of 6 texts | With `ktx2`: ids of six square images of the same size (`+X`, `-X`, `+Y`, `-Y`, `+Z`, `-Z`, in that order) that form a cube. It is written over `asset`. It cannot be combined with `cube`. |
| `cover` | text | `current` makes the asset the cover art of what is playing now, `previous` that of what played before, and `none` turns it back into a normal image. |
| `placeholder` | text | With `cover`: the still image that is shown while there is no cover art (an asset id or a file in the folder). Required if the asset is new. |
| `field` | text | With `cover` (`current` or `previous`): path of a scene field, such as `/scene/layers/0/image`. It puts the cover art there, and if there is none it creates it with the name `asset`. |

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene asset '{"asset":"picture","ktx2":true,"codec":"etc1s","quality":50}'
reiwand-ctl scene act my-scene import '{"file":"picture.png","id":"panorama"}'
reiwand-ctl scene act my-scene asset '{"asset":"panorama","ktx2":true,"cube":true}'
reiwand-ctl scene act my-scene import '{"file":"picture.png","id":"cover"}'
reiwand-ctl scene act my-scene asset '{"asset":"cover","cover":"current","placeholder":"picture"}'
reiwand-ctl scene act my-scene add_layer '{"kind":"image","source":"picture","id":"sky"}'
reiwand-ctl scene act my-scene asset '{"asset":"previous","cover":"previous","placeholder":"picture","field":"/scene/layers/0/image"}'
```

## `sprites`

Cuts an image into frames for an animation: with a grid or with hand-drawn rectangles. An image layer that uses the asset plays the frames. One variant per gesture; all of them require `asset`, the id of an image. An asset has at most 256 frames. A rectangle is `[x, y, width, height]` in image pixels, and must fit inside it.

### variant `grid`

Cuts a regular grid. It replaces the previous frames.

| Argument | Type | What it means |
|-----------|------|---------------|
| `asset` | text (required) | Id of the image. |
| `columns` | integer (required) | Columns of the grid. |
| `rows` | integer (required) | Rows of the grid. |
| `count` | integer | How many frames to use; default all of them. |
| `margin` | integer | Unused border of the image, in pixels. Default 0. |
| `spacing` | integer | Space between frames, in pixels. Default 0. |
| `order` | text | Reading order: `rows` (by rows), `columns` (by columns), `rows_reversed`, `columns_reversed`. Default `rows`. |

```sh
reiwand-ctl scene act my-scene import '{"file":"sheet.png","id":"sheet"}'
reiwand-ctl scene act my-scene sprites '{"grid":{"asset":"sheet","columns":4,"rows":2}}'
reiwand-ctl scene act my-scene sprites '{"grid":{"asset":"sheet","columns":4,"rows":2,"count":6,"order":"columns"}}'
```

### variant `rects`

Defines all the frames at once with rectangles.

| Argument | Type | What it means |
|-----------|------|---------------|
| `asset` | text (required) | Id of the image. |
| `rects` | list of lists of 4 integers (required) | The frames, in order. |

```sh
reiwand-ctl scene act my-scene import '{"file":"sheet.png","id":"sheet"}'
reiwand-ctl scene act my-scene sprites '{"rects":{"asset":"sheet","rects":[[0,0,100,90],[100,0,100,90]]}}'
```

### variant `add`

Adds a frame at the end.

| Argument | Type | What it means |
|-----------|------|---------------|
| `asset` | text (required) | Id of the image. |
| `rect` | list of 4 integers (required) | The new frame. |

```sh
reiwand-ctl scene act my-scene import '{"file":"sheet.png","id":"sheet"}'
reiwand-ctl scene act my-scene sprites '{"add":{"asset":"sheet","rect":[200,0,100,90]}}'
```

### variant `change`

Changes a frame's rectangle.

| Argument | Type | What it means |
|-----------|------|---------------|
| `asset` | text (required) | Id of the image. |
| `index` | integer (required) | Number of the frame (0 is the first). |
| `rect` | list of 4 integers (required) | The new rectangle. |

```sh
reiwand-ctl scene act my-scene import '{"file":"sheet.png","id":"sheet"}'
reiwand-ctl scene act my-scene sprites '{"grid":{"asset":"sheet","columns":4,"rows":2}}'
reiwand-ctl scene act my-scene sprites '{"change":{"asset":"sheet","index":0,"rect":[0,0,50,50]}}'
```

### variant `remove`

Removes a frame.

| Argument | Type | What it means |
|-----------|------|---------------|
| `asset` | text (required) | Id of the image. |
| `index` | integer (required) | Number of the frame. |

```sh
reiwand-ctl scene act my-scene import '{"file":"sheet.png","id":"sheet"}'
reiwand-ctl scene act my-scene sprites '{"grid":{"asset":"sheet","columns":4,"rows":2}}'
reiwand-ctl scene act my-scene sprites '{"remove":{"asset":"sheet","index":1}}'
```

### variant `reorder`

Moves a frame to another position.

| Argument | Type | What it means |
|-----------|------|---------------|
| `asset` | text (required) | Id of the image. |
| `from` | integer (required) | Current position. |
| `to` | integer (required) | New position. |

```sh
reiwand-ctl scene act my-scene import '{"file":"sheet.png","id":"sheet"}'
reiwand-ctl scene act my-scene sprites '{"grid":{"asset":"sheet","columns":4,"rows":2}}'
reiwand-ctl scene act my-scene sprites '{"reorder":{"asset":"sheet","from":0,"to":2}}'
```

### variant `play`

Sets how the frames play.

| Argument | Type | What it means |
|-----------|------|---------------|
| `asset` | text (required) | Id of the image. |
| `fps` | number | Frames per second, from 0.1 to 120. |
| `frames` | integer | How many frames play, from 1 to the number of frames. |

```sh
reiwand-ctl scene act my-scene import '{"file":"sheet.png","id":"sheet"}'
reiwand-ctl scene act my-scene sprites '{"grid":{"asset":"sheet","columns":4,"rows":2}}'
reiwand-ctl scene act my-scene sprites '{"play":{"asset":"sheet","fps":12,"frames":6}}'
```

### variant `clear`

Removes the cutting: the image goes back to being a normal image.

| Argument | Type | What it means |
|-----------|------|---------------|
| `asset` | text (required) | Id of the image. |

```sh
reiwand-ctl scene act my-scene import '{"file":"sheet.png","id":"sheet"}'
reiwand-ctl scene act my-scene sprites '{"grid":{"asset":"sheet","columns":4,"rows":2}}'
reiwand-ctl scene act my-scene sprites '{"clear":{"asset":"sheet"}}'
```

## `film`

Adjusts a video layer (or an image layer that shows a video): trimming it, changing its speed or giving its box the video's real size. Three variants.

### variant `trim`

Plays only part of the video. The part must fall inside the video and last at least 0.05 s.

| Argument | Type | What it means |
|-----------|------|---------------|
| `layer` | text (required) | Id of the layer. |
| `start` | number (required) | Second where it starts. |
| `end` | number | Second where it ends. Without it, until the end. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"video","file":"video.webm","id":"clip"}'
reiwand-ctl scene act my-scene film '{"trim":{"layer":"clip","start":0.1,"end":0.5}}'
```

### variant `rate`

Changes the speed.

| Argument | Type | What it means |
|-----------|------|---------------|
| `layer` | text (required) | Id of the layer. |
| `rate` | number (required) | Speed from 0 to 4; 1 is normal. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"video","file":"video.webm","id":"clip"}'
reiwand-ctl scene act my-scene film '{"rate":{"layer":"clip","rate":0.5}}'
```

### variant `fit`

Sets `size` to the video's real size.

| Argument | Type | What it means |
|-----------|------|---------------|
| `layer` | text (required) | Id of the layer. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"video","file":"video.webm","id":"clip"}'
reiwand-ctl scene act my-scene film '{"fit":{"layer":"clip"}}'
```

## `import_assets`

Copies parts of another asset package (a `.wpak` of the assets type) into the scene. The parts go to `assets/ID-VERSION/`, are renamed if their ids already exist, are credited with their license and author, and the package is noted in the scene's origin as a derived work. It prints the ids it added.

| Argument | Type | What it means |
|-----------|------|---------------|
| `package` | text (required) | `.wpak` file of the asset package. |
| `parts` | list of texts | Which parts to bring: `layer:ID`, `particles:ID`, `script:PATH` or `effect:ID`. Empty (default), all of them. |

```sh
reiwand-ctl scene new origin
reiwand-ctl scene act origin add_layer '{"kind":"image","file":"picture.png","id":"sky"}'
reiwand-ctl scene act origin manifest '{"credit":{"paths":["assets/picture.png"],"license":"CC-BY-4.0","attribution":"Alex Morgan"}}'
reiwand-ctl pack new-assets origin parts --part layer:sky --id org.example.parts --title Parts --license CC-BY-4.0 --attribution "Alex Morgan"
reiwand-ctl pack build parts -o parts.wpak
reiwand-ctl scene new my-scene
reiwand-ctl scene act my-scene import_assets '{"package":"parts.wpak","parts":["layer:sky"]}'
```
