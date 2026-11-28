# Layer basics

On this page: placing, blending, masks, following and reacting.

## What every layer has

Each layer carries an `id`, which can have 1 to 32 lowercase letters, digits or `_`, and a `type`.
Ids are not repeated. In the editor, moreover, layers, assets and particle systems share names:
do not repeat an id among them even if the scene allows it.

A scene can have up to 256 layers, nested up to 8 levels.

> In these tables, *Prefill* is the value the editor and the templates write for you. The file still has to contain every field: the loader refuses a missing one and never fills it in (see `../specs/explicit-values.md`).

| Field | Type | Prefill | Range | What it does |
|-------|------|---------|-------|--------------|
| `id` | text | required | `[a-z0-9_]{1,32}` | The name of the layer. |
| `type` | text | required | see below | The type of layer. |
| `transform` | object | no changes | see "Placing a layer" | Position, rotation, scale, skew. |
| `anchor` | text | none | `top_left`, `top`, `top_right`, `left`, `center`, `right`, `bottom_left`, `bottom`, `bottom_right` | Hangs the layer from that point of the screen. |
| `opacity` | number | 1 | 0 to 1 | How visible it is. |
| `visible` | yes or no | `true` | | Whether it is drawn. |
| `blend` | text | `normal` | see "Blending" | How it blends with what is underneath. |
| `mask` | text or object | no mask | see "Masks" | Lets the layer be seen only in one part. |
| `effects` | list | empty | up to 16 | Effects of the layer. See [effects.md](../effects.md). |
| `parallax` | `[x, y]` | 0 | -10 to 10 each | How much the layer follows the pointer. |
| `brightness` | number | 1 | 0 to 16 | Multiplies the color. Above 1 it only shows with `canvas.hdr`. |
| `color` | color | white | | Multiplies the color of image, video, particle and group layers; in a group, all its layers together. |
| `parent` | id | none | | The layer follows another. See "Layers that follow others". |
| `inherit` | list | `transform`, `visible` | `transform`, `visible`, `opacity` | What it takes from the parent; only with `parent`. |
| `solid` | yes or no, or `{ "alpha": 0 to 1 }` | `false` | | The layer receives clicks and touches. See "Layers that react". |
| `cache` | yes or no | `false` | | Draws the layer separately, in its own image. It counts in the cost (see [scene.md](../scene.md)). |
| `lighting` | object | no light | | How it receives the light of `light` layers. See [3d.md](../3d.md). |
| `attach` | object | none | | Attaches the layer to a bone. See [meshes.md](../meshes.md). |
| `depth_test` | yes or no | `false` | | Inside a 3D group, lets models cover it. See [3d.md](../3d.md). |

Almost all those numbers, colors and switches can be a fixed value or follow a property, a
timeline, the sound or the pointer (see "Values that move" in [scene.md](../scene.md)).

The layer types are: `image`, `video`, `text`, `shape`, `solid`, `spectrum`, `particles`, `sound`,
`group`, `adjustment`, `light`, `mesh` and `model`.

## Placing a layer

```json
"transform": { "position": [1040, 540], "scale": [1.0, 1.0], "rotation": 15, "skew": [0.0, 0.0],
               "origin": "anchor" }
```

| Field | Prefill | Range | What it does |
|-------|---------|-------|--------------|
| `position` | the middle of the canvas | canvas pixels | Where the origin point of the layer ends up. |
| `scale` | `[1, 1]` | | Scale in x and y; a single number applies to both. |
| `rotation` | 0 | -360 to 360 | Rotation in degrees, clockwise. |
| `skew` | `[0, 0]` | degrees | Oblique distortion. |
| `origin` | `[0.5, 0.5]` | fraction of the layer | The point it rotates and scales around. With `anchor`, the anchor's own point. |
| `tilt` | `[0, 0]` | -89 to 89 degrees each | Tilt in depth. It needs `canvas.perspective` to be seen. See [3d.md](../3d.md). |
| `z` | 0 | -100000 to 100000 px | Brings closer to or moves away from the viewer. See [3d.md](../3d.md). |

Without `position`, a layer stays at the center of the canvas. With `position`, the origin of the
layer is put at that point of the canvas: `[0, 0]` is the top-left corner, `x` grows to the right
and `y` downward. `origin` is a fraction of the layer's own box; with `[0, 0]` the position is its
top-left corner.

With `anchor`, the position is measured from that point of the visible part of the screen, and the
origin of the layer is that same point of it: a layer with `"anchor": "bottom_right"` and
`"position": [-100, -100]` leaves its bottom-right corner 100 pixels from the bottom-right corner.
It is the way to put things "stuck to the edge" on any screen.

A group has no box of its own: its `position` shifts all the layers inside by that distance (and
its `rotation` and `scale` rotate and scale them together). Without `position` the group does not
move them.

`parallax` moves the layer with the pointer: the layer shifts by its `parallax` times the size of
the canvas times `canvas.parallax.amount` times how far the pointer is from the middle of the
screen. A layer with value 0.02 moves little, like something far away; 0.05 moves more, like
something close. It needs the `pointer` input (see [scene.md](../scene.md)).

## Blending

`blend` says how the layer is blended with what is underneath. The modes are:

- Basic: `normal`, `add`, `multiply`, `screen`.
- Brightness-based: `lighten`, `darken`, `overlay`, `soft_light`, `hard_light`, `difference`,
  `exclusion`, `color_dodge`, `color_burn`.
- Color-based: `hue`, `saturation`, `color`, `luminosity`.
- Others: `linear_burn`, `linear_dodge`, `darker_color`, `lighter_color`, `vivid_light`,
  `linear_light`, `pin_light`, `hard_mix`, `subtract`, `divide`, `reflect`, `glow`, `phoenix`,
  `average`, `negation`, `tint`, `diffuse_light`.

The first four are the cheapest (see [scene.md](../scene.md)): `add` and `screen` are usually good for
lights, sparks and glows. `add` and `diffuse_light` do not clip at 1 while blending, so they keep
the light above white for the following layers and for `canvas.hdr`. `darker_color` and
`lighter_color` ignore the `opacity` of the layer.

## Masks

With `mask`, the layer is only seen where the mask allows it. The mask is an object with exactly
one of these three things: `texture` (the id of an image asset, as in `"mask": { "texture":
"stain" }`), `layer` (the id of another layer) or `shape` (a shape, with the same fields as a
`shape` layer).

And, in addition:

| Field | Prefill | Range | What it does |
|-------|---------|-------|--------------|
| `channel` | `luminance` with an image, `a` with a layer or a shape | `r`, `g`, `b`, `a`, `luminance` | Which part of the image or layer rules. A shape rules by its silhouette. |
| `invert` | `false` | | Flips the mask. |
| `feather` | 0 | 0 to 10000 | Softens the edge. |
| `strength` | 1 | 0 to 1 | Weakens the mask: with 0 it does nothing. |

A layer used as a mask, or a group that contains it, can carry `"visible": false`: it is not seen,
but it still masks. Masks cannot refer to each other in a circle. A mask can also go on an effect
(see [effects.md](../effects.md)).

## Layers that follow others

`parent` makes a layer follow the movement of another drawn layer, without having to put it in a
group. The position of the layer is then measured from the position of the parent, in the parent's
rotated and scaled system. `inherit` says what it follows: `transform`, `visible` and `opacity`
(default, the first two). Chains can reach 8 parents and cannot loop back to themselves. The
drawing order does not change.

## Layers that react

With `"solid": true` the layer receives the pointer over its box; with `{ "alpha": 0.5 }` only
where it is at least that opaque. That enables timelines with `trigger` `click:<layer>`,
`enter:<layer>` and `leave:<layer>` and sounds with `play_on` (see [scene.md](../scene.md) and
[particles-and-sound.md](../particles-and-sound.md)). `click:` asks for the `clicks` input and
`enter:` and `leave:` ask for `pointer`. Layers are tested from top to bottom and hidden ones
never receive anything.

The example [`animation-and-interaction`](../examples/animation-and-interaction/scene.json) makes a
button grow on click and a hit sound plays:

```json
    {
      "blend_space": "canvas", "id": "button", "type": "shape",
      "shape": { "rect": { "size": [360, 140], "radius": 70 } }, "fill": "#3b82f6", "solid": true,
      "transform": {
        "position": [1250, 540],
        "scale": {
          "timeline": "press",
          "keys": [[0, [1, 1]], [0.15, [0.9, 0.9], "out_quad"], [0.4, [1, 1], "out_back"]],
          "invert": false
        },
        "origin": "anchor", "rotation": 0.0, "skew": [0.0, 0.0]
      },
      "blend": "normal", "cache": false, "depth_test": false, "effects": [], "opacity": 1.0,
      "parallax": [0, 0], "visible": true
    }
```

```json
  "timelines": [
    {
      "id": "press", "duration": 0.4, "loop": "once", "autoplay": false, "trigger": "click:button",
      "events": [{ "t": 0.15, "name": "hit" }], "delay": 0.0, "fps": null, "frames": null,
      "speed": 1.0, "wrap": false
    }
  ]
```
