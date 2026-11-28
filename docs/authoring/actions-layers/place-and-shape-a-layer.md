# Place and shape a layer

## `layer`

A single action with several variants: you pass an object with the variant's name (`transform`, `grip`, `fit`, `parent`, `blend`, `depth`, `instance`, `touch`, `figure` or `start`) and inside it its arguments. All of them require `layer`, the layer's id, except `start` with `clock`, which creates a new layer. Whatever is not named stays as it was. If a field follows an animation or another source, see "Animated values" above.

### variant `transform`

Writes a layer's position, rotation, scale, skew and size all at once. In the app these are the canvas handles and the layer's transform fields.

| Argument | Type | What it means |
|-----------|------|---------------|
| `layer` | text (required) | Id of the layer. |
| `position` | list of 2 numbers | Position of the layer's anchor point on the canvas, in pixels. |
| `rotation` | number | Rotation in degrees, from -360 to 360. |
| `scale` | list of 2 numbers | Scale in `x` and in `y`; 1 is the original size. |
| `skew` | list of 2 numbers | Skew in degrees. |
| `origin` | list of 2 numbers | Anchor point as a fraction of the layer's box: `[0.5, 0.5]` is the center, `[0, 0]` the top-left corner. |
| `tilt` | list of 2 numbers | 3D tilt in degrees, from -89 to 89, around the layer's width and height. |
| `z` | number | Height toward the viewer, from -100000 to 100000. |
| `size` | list of 2 numbers | Width and height of the box in pixels (more than 0). Only image, video, solid, adjustment and spectrum layers. |
| `at` | number | Second at which it is written, if the field has keys. |
| `replace` | yes/no | `true` writes a fixed value even if the field follows keys or another source. Default `false`. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png","id":"sky"}'
reiwand-ctl scene act my-scene layer '{"transform":{"layer":"sky","position":[400,300],"rotation":10,"scale":[1.5,1.5],"size":[800,600]}}'
```

### variant `grip`

Does what dragging a canvas handle to the point `to` would do. It is the way to repeat an app gesture from the command line: the action computes the position, rotation, scale or skew numbers that leave the handle at `to`, taking parents, groups and the camera into account.

| Argument | Type | What it means |
|-----------|------|---------------|
| `layer` | text (required) | Id of the layer. |
| `grip` | text or object (required) | Which handle is dragged. See the table below. |
| `to` | list of 2 numbers (required) | Canvas point in pixels the handle goes to. With `nudge` it is the offset. |
| `snap` | number | Distance in pixels. With `move`, if an edge or the center of the layer ends up closer than this to the edges or center of the canvas or of other layers, it snaps. |
| `at` | number | Second at which it is written, if the field has keys. |
| `replace` | yes/no | As in `transform`. |

| `grip` | What it does |
|--------|----------|
| `"move"` | Puts the layer's anchor point at `to`. |
| `"nudge"` | Moves the layer by `to` (for example `[10, 0]`, ten pixels to the right). |
| `"turn"` | Rotates the layer so its top points to `to`. |
| `"origin"` | Moves the anchor point to `to` without the layer moving. |
| `{"scale":{"corner":[1,1],"even":true}}` | Scales so the corner `corner` (fractions of the box: `[1,1]` bottom right, `[1,0.5]` the middle of the right edge) lands on `to`. With `even`, it keeps the proportion. |
| `{"skew":{"edge":[0.5,0]}}` | Skews along the edge whose middle is `edge`, so that it passes through `to`. |
| `{"size":{"corner":[1,1]}}` | For an adjustment layer that captures a region: changes `size` so the corner lands on `to`. |
| `{"tilt":{"axis":0}}` | Tilts the layer around its width (`axis` 0) or its height (`axis` 1) until its edge gets close to `to`. |
| `"lift"` | Changes `z` to bring the layer closer to `to`. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png","id":"sky"}'
reiwand-ctl scene act my-scene layer '{"grip":{"layer":"sky","grip":"move","to":[500,500],"snap":10}}'
reiwand-ctl scene act my-scene layer '{"grip":{"layer":"sky","grip":"nudge","to":[10,0]}}'
reiwand-ctl scene act my-scene layer '{"grip":{"layer":"sky","grip":"turn","to":[900,100]}}'
reiwand-ctl scene act my-scene layer '{"grip":{"layer":"sky","grip":{"scale":{"corner":[1,1],"even":true}},"to":[900,700]}}'
reiwand-ctl scene act my-scene layer '{"grip":{"layer":"sky","grip":{"skew":{"edge":[0.5,0]}},"to":[600,100]}}'
reiwand-ctl scene act my-scene layer '{"grip":{"layer":"sky","grip":{"tilt":{"axis":0}},"to":[600,100]}}'
reiwand-ctl scene act my-scene layer '{"grip":{"layer":"sky","grip":"lift","to":[600,100]}}'
reiwand-ctl scene act my-scene add_layer '{"kind":"adjustment","id":"adjust"}'
reiwand-ctl scene act my-scene layer '{"grip":{"layer":"adjust","grip":{"size":{"corner":[1,1]}},"to":[900,700]}}'
```

### variant `fit`

Scales and places the layer so that it covers the canvas, fits inside it or fills it by stretching. It keeps the rotation and the skew.

| Argument | Type | What it means |
|-----------|------|---------------|
| `layer` | text (required) | Id of the layer. |
| `fit` | text (required) | `cover` (covers the whole canvas, may be cropped), `contain` (fits entirely) or `stretch` (fills the canvas by stretching). |
| `at`, `replace` | | As in `transform`. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png","id":"sky"}'
reiwand-ctl scene act my-scene layer '{"fit":{"layer":"sky","fit":"cover"}}'
```

### variant `parent`

Makes a layer follow another (its parent): it moves, rotates and scales with it. It is rejected for the same layer, for sounds and lights, if the parent already follows it or is inside it, and if the chain exceeds 8 layers.

| Argument | Type | What it means |
|-----------|------|---------------|
| `layer` | text (required) | Id of the layer that follows. |
| `parent` | text | Id of the parent. Without it, the layer stops following its parent. |
| `keep` | yes/no | `true` rewrites position, rotation, skew and scale so the layer stays where it was. Default `false`. |
| `at`, `replace` | | As in `transform`. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png","id":"sky"}'
reiwand-ctl scene act my-scene add_layer '{"kind":"text","id":"title"}'
reiwand-ctl scene act my-scene layer '{"parent":{"layer":"title","parent":"sky","keep":true}}'
reiwand-ctl scene act my-scene layer '{"parent":{"layer":"title"}}'
```

### variant `blend`

Changes the layer's blend mode with what is underneath.

| Argument | Type | What it means |
|-----------|------|---------------|
| `layer` | text (required) | Id of the layer. |
| `blend` | text (required) | One of: `normal`, `add`, `multiply`, `screen`, `lighten`, `darken`, `overlay`, `soft_light`, `hard_light`, `difference`, `exclusion`, `color_dodge`, `color_burn`, `hue`, `saturation`, `color`, `luminosity`, `linear_burn`, `linear_dodge`, `darker_color`, `lighter_color`, `vivid_light`, `linear_light`, `pin_light`, `hard_mix`, `subtract`, `divide`, `reflect`, `glow`, `phoenix`, `average`, `negation`, `tint`, `diffuse_light`. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png","id":"sky"}'
reiwand-ctl scene act my-scene layer '{"blend":{"layer":"sky","blend":"multiply"}}'
```

### variant `depth`

Sets the layer's parallax depth: how much it moves with the pointer or the phone's tilt. If the package does not declare the `pointer` input, it adds it.

| Argument | Type | What it means |
|-----------|------|---------------|
| `layer` | text (required) | Id of the layer. |
| `depth` | list of 2 numbers (required) | Horizontal and vertical depth, each from -10 to 10. If both are equal a single number is saved; with `[0, 0]` it is removed. |
| `replace` | yes/no | `true` writes a fixed value even if the parallax follows another source. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png","id":"sky"}'
reiwand-ctl scene act my-scene layer '{"depth":{"layer":"sky","depth":[2,1]}}'
```

### variant `touch`

Chooses whether a layer receives the pointer (for clicks, and so a script or a trigger knows the pointer is passing over it).

| Argument | Type | What it means |
|-----------|------|---------------|
| `layer` | text (required) | Id of the layer. |
| `takes` | text (required) | `off` lets the pointer through, `all` takes it over the whole box, `alpha` only where the layer is opaque. |
| `alpha` | number | With `alpha`, how opaque a point must be to take the pointer, from 0 to 1. If missing, 0.5 or the value it had. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png","id":"sky"}'
reiwand-ctl scene act my-scene layer '{"touch":{"layer":"sky","takes":"alpha","alpha":0.3}}'
```

### variant `instance`

Changes, for one particle layer only, what the system it draws does (unless the system locks those settings).

| Argument | Type | What it means |
|-----------|------|---------------|
| `layer` | text (required) | Id of a particle layer. |
| `values` | object | What to change. Fields: `alpha`, `size`, `speed`, `life`, `time_scale` and `brightness` (multipliers), `count` (multiplies the births, from 0 to 4), `color` (a color) and `points` (control points `p0` to `p7`: a list of positions, or an object with the numbers 0 to 7 that changes only those). A field set to `null` goes back to what the system says. |
| `clear` | yes/no | `true` removes all the layer's settings. Default `false`. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"dust"}'
reiwand-ctl scene act my-scene layer '{"instance":{"layer":"dust","values":{"alpha":0.5,"size":2,"count":2,"speed":1.5,"brightness":1.2,"color":"#ffcc00"}}}'
reiwand-ctl scene act my-scene layer '{"instance":{"layer":"dust","values":{"points":{"1":[100,100]}}}}'
reiwand-ctl scene act my-scene layer '{"instance":{"layer":"dust","values":{"alpha":null}}}'
reiwand-ctl scene act my-scene layer '{"instance":{"layer":"dust","clear":true}}'
```

### variant `figure`

Edits the figure of a shape or text layer. Inside `figure` you choose one of four things: `shape`, `stop`, `text_width` or `fonts`.

#### `figure` with `shape`

Does what dragging a handle of the shape to the point `to` would do.

| Argument | Type | What it means |
|-----------|------|---------------|
| `layer` | text (required) | Id of a shape layer. |
| `handle` | text (required) | Name of the handle. Depending on the shape: rectangle `width`, `height`, `size`, `radius`; ellipse `width`, `height`, `size`; ring the same and `band` (the thickness); polygon `size`, `sides` (the number of sides); star `size`, `points` (the points), `inner` (the depth); line `start`, `end`, `width`. Any shape that is not a line and has an outline: `stroke`. If it has a gradient: `gradient_from`, `gradient_to`. |
| `to` | list of 2 numbers (required) | Canvas point, in pixels, the handle goes to. |
| `even` | yes/no | Keeps the proportion. Default `false`. |
| `at`, `replace` | | As in `transform`. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"shape","id":"shape"}'
reiwand-ctl scene act my-scene layer '{"figure":{"shape":{"layer":"shape","handle":"radius","to":[1000,400]}}}'
reiwand-ctl scene act my-scene layer '{"figure":{"shape":{"layer":"shape","handle":"width","to":[1200,540],"even":true}}}'
```

#### `figure` with `stop`

Edits the colors of a shape's gradient (2 to 4 stops, always in order).

| Argument | Type | What it means |
|-----------|------|---------------|
| `layer` | text (required) | Id of a shape layer. |
| `stop` | text or object (required) | What to do. See the table below. |

| `stop` | What it does |
|--------|----------|
| `{"make":{"radial":false}}` | Creates a two-stop gradient; with `"radial": true` it is circular. |
| `"clear"` | Removes the gradient. |
| `{"add":{"at":0.5,"color":"#ff0000"}}` | Adds a stop at `at` (0 to 1) with a color; without `color` it takes the next one's. |
| `{"move":{"index":1,"at":0.3}}` | Moves stop number `index` (0 is the first) to `at`. |
| `{"color":{"index":0,"color":"#00ff00"}}` | Changes the color of a stop. |
| `{"remove":{"index":1}}` | Removes a stop. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"shape","id":"shape"}'
reiwand-ctl scene act my-scene layer '{"figure":{"stop":{"layer":"shape","stop":{"make":{"radial":false}}}}}'
reiwand-ctl scene act my-scene layer '{"figure":{"stop":{"layer":"shape","stop":{"add":{"at":0.5,"color":"#ff0000"}}}}}'
reiwand-ctl scene act my-scene layer '{"figure":{"stop":{"layer":"shape","stop":{"move":{"index":1,"at":0.3}}}}}'
reiwand-ctl scene act my-scene layer '{"figure":{"stop":{"layer":"shape","stop":{"color":{"index":0,"color":"#00ff00"}}}}}'
reiwand-ctl scene act my-scene layer '{"figure":{"stop":{"layer":"shape","stop":{"remove":{"index":1}}}}}'
reiwand-ctl scene act my-scene layer '{"figure":{"stop":{"layer":"shape","stop":"clear"}}}'
```

#### `figure` with `text_width`

Sets the width of a multi-line text by moving its right edge to `to` without moving the left one.

| Argument | Type | What it means |
|-----------|------|---------------|
| `layer` | text (required) | Id of a text layer. |
| `to` | list of 2 numbers | Canvas point the right edge goes to. |
| `remove` | yes/no | `true` removes the fixed width. Default `false`. |
| `at`, `replace` | | As in `transform`. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"text","id":"text"}'
reiwand-ctl scene act my-scene layer '{"figure":{"text_width":{"layer":"text","to":[1400,540]}}}'
reiwand-ctl scene act my-scene layer '{"figure":{"text_width":{"layer":"text","remove":true}}}'
```

#### `figure` with `fonts`

Chooses a text's typeface and the fallback ones, which are used for the letters the main one does not have.

| Argument | Type | What it means |
|-----------|------|---------------|
| `layer` | text (required) | Id of a text layer. |
| `font` | text | Name of the font; an empty string removes it. |
| `fallback` | list of texts | Fallback fonts, in order, up to 8. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"text","id":"text"}'
reiwand-ctl scene act my-scene layer '{"figure":{"fonts":{"layer":"text","font":"serif","fallback":["sans-serif"]}}}'
```
