# Putting effects on a layer

On this page: the effect list of a layer and its masks.

## Putting effects on a layer

Each layer has an `effects` list of up to 16 effects. They are applied in order: the first acts on
the layer as it is, the second on the result of the first, and so on.

```json
    {
      "blend_space": "canvas", "id": "background", "type": "image", "image": "sky",
      "size": [1920, 1080],
      "effects": [
        {
          "effect": "color_adjust", "name": "color",
          "enabled": { "prop": "color_vivid", "invert": false },
          "params": { "saturation": 1.8, "contrast": 1.15, "exposure": 0.0, "hue": 0.0 },
          "optional": false
        },
        {
          "effect": "waves",
          "params": {
            "amplitude": 10, "frequency": 5, "speed": 1.5, "angle2": 90.0, "count": 1.0,
            "direction": 0.0, "perspective": 0.0, "phase": ""
          },
          "mask": {
            "texture": "stain", "channel": "r", "feather": 0, "invert": false, "strength": 1
          },
          "enabled": true, "optional": false
        },
        {
          "effect": "blur", "params": { "radius": 6, "amount": 1.0, "blend": "normal" },
          "mask": {
            "shape": { "rect": { "size": [1920, 260], "radius": 0.0 } }, "invert": true,
            "strength": 0.8, "channel": "a", "feather": 0.0
          },
          "optional": true, "enabled": true
        }
      ],
      "blend": "normal", "cache": false, "color": "#ffffff", "depth_test": false, "opacity": 1.0,
      "parallax": [0, 0], "solid": false,
      "transform": { "origin": "anchor", "rotation": 0.0, "scale": [1.0, 1.0], "skew": [0.0, 0.0] },
      "visible": true
    }
```

Each element of the list is a use of the effect:

> In these tables, *Prefill* is the value the editor and the templates write for you. The file still has to contain every field: the loader refuses a missing one and never fills it in (see `../specs/explicit-values.md`).

| Field | Prefill | What it does |
|-------|---------|--------------|
| `effect` | required | The id of the effect, one of those below or one of the package. |
| `params` | those of each effect | The parameters you change. Each one can be a fixed value or follow a property, time, sound or the pointer. |
| `enabled` | `true` | `false` turns it off without removing it. It can follow a property. |
| `mask` | no mask | Limits the effect to a part of the layer. See "Masks". |
| `optional` | `false` | `true` leaves out the effect, and not the scene, where it cannot run. |
| `name` | | A name of your own for this use, which scripts and links call it by. |

Parameters you do not set are worth what their descriptor says. In parameters of image type
(`texture`) you write the id of an image asset, and in those of table type (`lut`) that of a `.cube`
asset. An image can also be `{ "layer": "id" }`, a layer drawn separately (see
[scene.md](../scene.md)).

Points such as `center` or `source` are fractions of the layer (of the canvas, in an adjustment
layer). Those that have a `follow` switch move to the pointer while it is on the screen and return
smoothly to their place in a quarter of a second when it leaves.

To set them from the terminal:

```
reiwand-ctl scene act MY-FOLDER effects '{"add":{"layer":"background","effect":"waves","params":{"amplitude":6,"frequency":3}}}'
reiwand-ctl scene act MY-FOLDER effects '{"set":{"layer":"background","index":0,"params":{"amplitude":10}}}'
reiwand-ctl scene act MY-FOLDER effects '{"move":{"layer":"background","index":0,"to":1}}'
reiwand-ctl scene act MY-FOLDER effects '{"remove":{"layer":"background","index":0}}'
```

(`set`, `move` and `remove` take the place of the effect in the list, starting from 0.)
In the editor this is done from the layer's effects list, with *Add an effect*.

### Effects on everything below

An adjustment layer (`adjustment`) applies its effects to everything below it. It is the way to
give a finish to the whole scene, like the glow and the grain of the example:

```json
    {
      "blend_space": "canvas", "id": "screen", "type": "adjustment",
      "effects": [
        {
          "effect": "bloom",
          "params": {
            "threshold": 0.7, "intensity": 0.8, "radius": 24, "capped": false, "knee": 0.5,
            "levels": 1, "limit": 1.0, "saturation": 1.0, "scatter": 0.5, "tint": "#ffffff",
            "unit": 0
          },
          "enabled": true, "optional": false
        },
        {
          "effect": "vignette",
          "params": {
            "intensity": 0.45, "color": "#000010", "center": [0.5, 0.5], "radius": 1.0,
            "roundness": 1.0, "softness": 0.65
          },
          "enabled": true, "optional": false
        },
        {
          "effect": "film_grain",
          "params": {
            "amount": 0.04, "fps": 12, "bias": 0.0, "blend": "overlay", "mono": false, "size": 1.5
          },
          "enabled": true, "optional": false
        }
      ],
      "blend": "normal", "cache": false, "depth_test": false, "opacity": 1.0, "parallax": [0, 0],
      "redraw": false, "rgb": false, "size": "canvas", "solid": false,
      "transform": { "origin": "anchor", "rotation": 0.0, "scale": [1.0, 1.0], "skew": [0.0, 0.0] },
      "visible": true
    }
```

The canvas also has a glow of its own, `canvas.bloom`, which does the same as a `bloom` at the very
top (see [scene.md](../scene.md)).

### Quality and cost

The quality level goes down with battery, heat or the power of the computer. Then effects change
like this:

- **Light** effects run at all levels.
- **Medium** ones are removed at `q0`, and at `q1` if you marked them `optional`.
- **Heavy** ones run their inner passes at a quarter size at `q1`.
- The pass that writes the result always goes at full size.

The catalog says which effects are light, medium or heavy. An effect that does not run at the
current level does not ask for its input, and neither does one whose `enabled` is turned off by a
property. What each one costs and what the whole scene costs is explained in [scene.md](../scene.md).

Blurs and distortions treat the edges of the screen as the edges of the image: a layer that goes
beyond the screen keeps the color of its outermost pixels.

## Masks

A mask limits an effect to a part of the layer, and it can be an image, another layer or a shape:

```json
    {
      "blend_space": "canvas", "id": "background", "type": "image", "image": "sky",
      "size": [1920, 1080],
      "effects": [
        {
          "effect": "color_adjust", "name": "color",
          "enabled": { "prop": "color_vivid", "invert": false },
          "params": { "saturation": 1.8, "contrast": 1.15, "exposure": 0.0, "hue": 0.0 },
          "optional": false
        },
        {
          "effect": "waves",
          "params": {
            "amplitude": 10, "frequency": 5, "speed": 1.5, "angle2": 90.0, "count": 1.0,
            "direction": 0.0, "perspective": 0.0, "phase": ""
          },
          "mask": {
            "texture": "stain", "channel": "r", "feather": 0, "invert": false, "strength": 1
          },
          "enabled": true, "optional": false
        },
        {
          "effect": "blur", "params": { "radius": 6, "amount": 1.0, "blend": "normal" },
          "mask": {
            "shape": { "rect": { "size": [1920, 260], "radius": 0.0 } }, "invert": true,
            "strength": 0.8, "channel": "a", "feather": 0.0
          },
          "optional": true, "enabled": true
        }
      ],
      "blend": "normal", "cache": false, "color": "#ffffff", "depth_test": false, "opacity": 1.0,
      "parallax": [0, 0], "solid": false,
      "transform": { "origin": "anchor", "rotation": 0.0, "scale": [1.0, 1.0], "skew": [0.0, 0.0] },
      "visible": true
    }
```

(In the first effect with a mask, `waves` only acts where the `stain` image is light; in the
second, `blur` only acts outside a rectangle, and with the strength at 80 %.)

| Field | Prefill | What it does |
|-------|---------|--------------|
| `texture` | | An image asset. It must be `linear` if it is a data mask. |
| `layer` | | The id of another layer. It is drawn separately and masks by where it falls on screen. |
| `shape` | | A shape (see [layers.md](../layers.md)), centered on the layer and measured in its pixels. |
| `channel` | `luminance` with an image, `a` with a layer or shape | `r`, `g`, `b`, `a` or `luminance`. |
| `invert` | `false` | Masks the outside. |
| `feather` | 0 | Softens the edge. |
| `strength` | 1 | From 0 to 1: `mix(1, mask, strength)`. |

Set exactly one of `texture`, `layer` and `shape`. From the terminal:

```
reiwand-ctl scene act MY-FOLDER effects '{"mask":{"layer":"background","index":1,"shape":{"rect":{"size":[1920,260]}},"invert":true}}'
```

With `scene act FOLDER paint_mask` an image mask is painted with strokes and, in the editor, with a
brush (see [editor.md](../editor.md)).
