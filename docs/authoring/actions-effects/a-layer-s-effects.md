# A layer's effects

On this page: A layer's effects.

## Actions

## A layer's effects

**Before you start.** These actions share three ideas. A canvas point is `[x, y]` in canvas pixels, with the origin at the top left and y growing downward (the canvas is 1920 by 1080 if you do not change its size). A color is written `"#rrggbb"` or `"#rrggbbaa"`. Brush strokes use fractions of the layer: `[0, 0]` is its top-left corner and `[1, 1]` the bottom-right one. The built-in effects are listed with `reiwand-ctl scene catalog`, and each one's parameters with the `rules` query.

### `effects`

Adds, changes, removes, reorders and masks a layer's effects, and moves their handles and their mask on the canvas. In the app it is the layer's **Effects** list, with **Add an effect** and the **Mask** box. A sound or light layer does not accept effects. An effect is named by its position in the layer's list, which starts at 0.

This action takes one variant at a time.

#### variant `add`

Adds an effect at the end of the layer's list, or at position `at`.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text | (required) Layer id. |
| `effect` | text | (required) Effect name, for example `blur` or `waves`; it can be a built-in one or one from the package. |
| `name` | text | A name of your own to recognize it in the list. With no value, it has no name. |
| `at` | integer | Position in the list (0 is the first). With no value, it goes at the end. |
| `params` | object | Initial parameter values, `{"radius": 6}`. A parameter the effect does not have, or one out of its range, is rejected. |
| `optional` | yes/no | If `true`, the effect is among the first to be left out when the machine is short on power. Default `false`. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png"}'
reiwand-ctl scene act my-scene effects '{"add":{"layer":"image","effect":"blur","params":{"radius":6},"name":"soft","optional":true}}'
```

#### variant `set`

Changes an effect that is already on the layer: its parameters, whether it is on, its name, or whether it is optional. Whatever you do not name stays as it was.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text | (required) Layer id. |
| `index` | integer | (required) Position of the effect in the list. |
| `params` | object | Parameters that change. A `null` value removes the parameter and leaves the effect's default value. |
| `enabled` | yes/no | `false` turns the effect off without removing it. |
| `name` | text | New name; empty text removes it. |
| `optional` | yes/no | Marks or unmarks the effect as optional. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png"}'
reiwand-ctl scene act my-scene effects '{"add":{"layer":"image","effect":"waves"}}'
reiwand-ctl scene act my-scene effects '{"set":{"layer":"image","index":0,"params":{"amplitude":9},"enabled":true,"name":"ripples"}}'
```

#### variant `remove`

Removes an effect from the list.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text | (required) Layer id. |
| `index` | integer | (required) Position of the effect. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png"}'
reiwand-ctl scene act my-scene effects '{"add":{"layer":"image","effect":"blur"}}'
reiwand-ctl scene act my-scene effects '{"remove":{"layer":"image","index":0}}'
```

#### variant `move`

Changes the order of an effect. Effects are applied from the top of the list down.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text | (required) Layer id. |
| `index` | integer | (required) Current position. |
| `to` | integer | (required) New position; it cannot go past the last one. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png"}'
reiwand-ctl scene act my-scene effects '{"add":{"layer":"image","effect":"blur"}}'
reiwand-ctl scene act my-scene effects '{"add":{"layer":"image","effect":"vignette"}}'
reiwand-ctl scene act my-scene effects '{"move":{"layer":"image","index":1,"to":0}}'
```

#### variant `grip`

Drags the handle of an effect parameter to a point on the canvas: a center, a corner, an arrow or a dial. It is what the app does when you drag the handle over the preview. It fails if the parameter has no handle on the canvas or if it already moves with keyframes, a sound or a property.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text | (required) Layer id. |
| `index` | integer | (required) Position of the effect. |
| `param` | text | (required) Parameter whose handle is dragged, for example `center`. |
| `to` | list of numbers | (required) Canvas point `[x, y]` the handle goes to. The stored value is the position inside the layer, from 0 to 1. |
| `both` | yes/no | If `true`, it also moves the other handle of the pair (for example, the end of a line whose start is dragged) by the same distance. Default `false`. |
| `at` | number | Second at which the layer's position is measured, in case it moves. Default 0. |
| `replace` | yes/no | If `true`, it overwrites a parameter that followed another source. Default `false`. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png"}'
reiwand-ctl scene act my-scene effects '{"add":{"layer":"image","effect":"blur_radial"}}'
reiwand-ctl scene act my-scene effects '{"grip":{"layer":"image","index":0,"param":"center","to":[960,300]}}'
```

#### variant `mask_grip`

Drags a handle of the shape mask to a point on the canvas: its size, the soft edge, the thickness of a ring, the rounded corner of a rectangle or the hollow of a star. The mask must be a shape (not an image or a layer). The shape is measured from the center of the layer, so the resulting size is twice the distance from the point to the center.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text | (required) Layer id. |
| `index` | integer | Position of the effect whose mask is dragged. With no value, it is the layer's own mask. |
| `hold` | text | (required) Handle that is dragged: `size`, `feather` (soft edge), `width` (thickness, rings only), `radius` (corner, rectangles only) or `inner` (hollow, stars only). A handle the shape does not have is rejected. |
| `to` | list of numbers | (required) Canvas point `[x, y]` the handle goes to. |
| `at` | number | Second at which the layer's position is measured. Default 0. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png"}'
reiwand-ctl scene act my-scene effects '{"add":{"layer":"image","effect":"blur"}}'
reiwand-ctl scene act my-scene effects '{"mask":{"layer":"image","index":0,"shape":{"ellipse":{"size":[600,400]}},"feather":40}}'
reiwand-ctl scene act my-scene effects '{"mask_grip":{"layer":"image","index":0,"hold":"size","to":[1500,700]}}'
```

#### variant `mask`

Sets, changes or removes the mask of an effect or of the layer itself. With the mask, the effect (or the layer) shows only where the mask allows it. A mask has a single source: an image, a layer or a shape. Changes you do not name are kept.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text | (required) Layer id. |
| `index` | integer | Position of the effect. With no value, the mask is the one for the whole layer. |
| `texture` | text | Id of a package image as the source. |
| `from` | text | Id of another layer as the source. |
| `shape` | object | A shape as the source, centered on the layer and measured in its pixels: `{"rect":{"size":[w,h],"radius":r}}`, `{"ellipse":{"size":[w,h]}}`, `{"ring":{"size":[w,h],"width":g}}`, `{"polygon":{"sides":n,"size":d}}`, `{"star":{"points":n,"size":d,"inner":0.5}}` or `{"line":{"to":[x,y],"width":g}}`. |
| `channel` | text | Channel of the image or layer that drives the mask: `r`, `g`, `b`, `a` or `luminance` (brightness). |
| `invert` | yes/no | If `true`, the effect acts where the mask is empty. |
| `feather` | number | Soft edge, in pixels. |
| `strength` | number | Mask strength, from 0 to 1. |
| `off` | yes/no | If `true`, removes the mask. Default `false`. |

A new mask needs a source.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png"}'
reiwand-ctl scene act my-scene effects '{"add":{"layer":"image","effect":"blur"}}'
reiwand-ctl scene act my-scene effects '{"mask":{"layer":"image","index":0,"shape":{"star":{"points":5,"size":300,"inner":0.4}},"feather":20}}'
reiwand-ctl scene act my-scene effects '{"mask":{"layer":"image","index":0,"invert":true}}'
reiwand-ctl scene act my-scene effects '{"mask":{"layer":"image","texture":"picture","channel":"luminance","strength":0.8}}'
reiwand-ctl scene act my-scene effects '{"mask":{"layer":"image","index":0,"off":true}}'
```

### `effect_rule`

Changes what a parameter of a package effect asks for in order to animate: an input (time, the pointer, sound or clicks), another parameter that must be on, or another parameter that takes over from it. The scene only spends frames on what someone asks for. It only applies to the package's own effects; the built-in ones keep their rules. In the app, the effect's **Asks for** section. Whatever you do not name stays; empty text removes that rule.

| Argument | Type | What it means |
|---|---|---|
| `effect` | text | (required) Name of a package effect. |
| `param` | text | (required) Parameter of the effect. |
| `uses` | text | Input it asks for: `time`, `cursor` (pointer), `clicks` or `audio` (sound). Empty removes it. |
| `waits_for` | text | Another parameter of the same effect that must be something other than zero for this one to ask for its input. Empty removes it. |
| `yields_to` | text | Another parameter of the same effect that, while it is not zero, takes over from this one: this one stops asking for its input. Empty removes it. |

```sh
reiwand-ctl scene new my-scene
reiwand-ctl scene act my-scene shader '{"new":{"name":"glow","template":"color"}}'
reiwand-ctl scene act my-scene effect_rule '{"effect":"glow","param":"amount","uses":"audio"}'
reiwand-ctl scene act my-scene effect_rule '{"effect":"glow","param":"amount","waits_for":"color"}'
reiwand-ctl scene act my-scene effect_rule '{"effect":"glow","param":"amount","uses":"","waits_for":""}'
```
