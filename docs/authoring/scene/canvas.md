# The canvas

The canvas is the reference size of your scene. Whatever the screen, you place everything as if the
canvas measured `size` pixels; the program fits it to each screen.

```json
  "canvas": {
    "size": [1920, 1080], "fit": "fill", "background": "#0b1020", "blend_space": "linear",
    "parallax": { "amount": 1, "delay": 0.15, "pointer": 1, "gyro": false, "offset": [0, 0] },
    "bloom": {
      "threshold": 0.8, "strength": 0.6, "radius": 24, "radius_unit": "canvas", "knee": 0.5,
      "saturation": 1.0, "limit": null, "tint": "#ffffff"
    },
    "camera": {
      "zoom": 1.05, "shake": { "enabled": false, "amplitude": 6, "roughness": 0.5, "speed": 1 },
      "center": "canvas", "rotation": 0.0
    },
    "clear": true, "hdr": false
  }
```

(From [`examples/canvas-and-clock`](../examples/canvas-and-clock/scene.json).)

> In these tables, *Prefill* is the value the editor and the templates write for you. The file still has to contain every field: the loader refuses a missing one and never fills it in (see `../specs/explicit-values.md`).

| Field | Type | Prefill | Range | What it does |
|-------|------|---------|-------|--------------|
| `size` | `[width, height]` | required | 1 to 16384 each | Reference size in pixels. |
| `fit` | text | `fill` | `fill`, `fit`, `stretch`, `center` | How the canvas is placed on each screen: `fill` covers it and crops what is left over, `fit` leaves it whole and the background fills the rest, `stretch` stretches it to the edges, `center` leaves it unscaled in the middle. |
| `background` | color | `#000000` | | Background color, also the one seen outside the canvas with `fit`. |
| `blend_space` | text | `linear` | `linear`, `srgb` | `linear` blends colors like light; `srgb` blends them already encoded, as most image editors do. |

Each layer also carries its own `blend_space`, required (the editor fills it with `canvas`): `canvas` follows the value of the canvas, `linear` or `srgb` set how that layer is blended with what is below it, with any blend mode and also in particles. It is useful, for example, so that a semi-transparent smoke looks as it does in an image editor in a linear scene. A layer with its own space different from the canvas is blended whole: a group is composed first and then blended in its space, and its children use their own value (`canvas` is the canvas, it is not inherited). An adjustment layer uses it only in the final fade when its opacity is below 1. Effects still read the canvas space. A layer without `blend_space` is rejected naming the field. In the terminal: `reiwand-ctl scene act FOLDER layer '{"blend_space":{"layer":"smoke","space":"srgb"}}'`.
| `parallax` | object | no parallax | see below | How much the layers with `parallax` follow the pointer. |
| `camera` | object | no camera | see below | Moves, zooms and rotates the whole scene. |
| `bloom` | object | no glow | see below | A glow of the bright parts over the whole scene. |
| `hdr` | yes or no | `false` | | Stores colors brighter than white. |
| `tonemap` | text | `none` | `none`, `aces`, `agx`, `srgb_to_linear` | How those colors are squeezed onto the screen, after the glow; only with `hdr`. `srgb_to_linear` reads the final colors once more through the sRGB curve. |
| `clear` | yes or no | `true` | | With `false` each frame is drawn on top of the previous one and leaves trails; it costs more. |
| `perspective` | number | undefined | 100 to 100000 px | Distance of the viewer for layers with `transform.tilt` and `transform.z`. |
| `ambient`, `sky` | color | black | | Surrounding light and light from above for the layers with light. See [3d.md](../3d.md). |
| `wind` | `[x, y]` | no wind | pixels per second | The wind that moves bones with physics and foliage. |
| `gusts` | number | 0.35 | 0 to 1 | How much of the wind comes in gusts. |

Limits of `hdr` and `tonemap`: output brighter than white was not checked on a real HDR screen, and a video inside a deep canvas is assumed to be in linear light.

### Parallax

`canvas.parallax` regulates the movement of the layers that have `parallax` (see
[layers.md](../layers.md)).

| Field | Prefill | Range | What it does |
|-------|---------|-------|--------------|
| `amount` | 1 | 0 to 4 | How much they move in total. |
| `delay` | 0.15 | 0 to 2 s | How smoothly the layers follow the pointer. |
| `pointer` | 1 | 0 to 1 | How much the pointer counts. With 0 no pointer is needed. |
| `offset` | `[0, 0]` | -1 to 1 each | A fixed offset, as if the pointer were at that place. |
| `gyro` | `false` | | On the phone, the tilt also moves them. |

`amount`, `pointer` and `offset` can follow a property, a timeline or the sound. If the system asks
for less motion, the layers stay still.

### Camera

`canvas.camera` moves the whole scene at once, without extra passes.

| Field | Prefill | Range | What it does |
|-------|---------|-------|--------------|
| `center` | the middle | pixels | The point of the canvas seen in the middle. |
| `zoom` | 1 | 0.1 to 10 | How many times larger it looks. |
| `rotation` | 0 | -360 to 360 | Rotation in degrees. |
| `shake` | no shake | | Soft shake, below. |

`shake`: `enabled` (`true`), `amplitude` (8 px, 0 to 1000), `roughness` (0.5, 0 to 1) and `speed`
(1 Hz, 0 to 60). While it shakes it asks for a frame every time. It turns off if the user asks for
less motion and at the lowest quality level. All the fields of the camera can follow a value.

### Glow

`canvas.bloom` makes the light parts of the whole scene glow, like an adjustment layer at the very
top with the `bloom` effect (see [effects.md](../effects.md)).

All the fields are always saved; the "Default" column is what the editor proposes.

| Field | Prefill | Range | What it does |
|-------|---------|-------|--------------|
| `threshold` | 0.8 | 0 to 4 | From what brightness it starts to glow. |
| `strength` | 1 | 0 to 8 | How much it glows. With 0 it costs nothing. |
| `radius` | 16 | 0 to 128 | How far it spreads, in the unit of `radius_unit`. |
| `radius_unit` | `canvas` | `canvas` or `screen` | With `screen`, `radius` is screen pixels: a canvas twice the screen keeps the same halo on it. |
| `knee` | 0.5 | 0 to 1 | The part of the threshold where the cutoff is softened; with 0 the cutoff is hard and what is under the threshold is not touched. |
| `saturation` | 1 | 0 to 4 | Mixes the bright part with its gray before the blur: 0 gives a gray glow, 4 a vivid one. |
| `limit` | `null` | 0 to 16, or `null` | Cap of each channel of the bright part times the strength, so that a very bright point does not cover a weaker one; `null` sets no cap. |
| `tint` | white | | Color of the glow. |
| `hdr` | | see below | Values that replace the previous ones at level `q3` and at `q2` on mains power. |

`hdr` is an object with `threshold` (0 to 16), `strength` (0 to 8), `knee` (0 to 1, default 0.5),
`levels` (1 to 8, default 1) and `scatter` (0 to 2, default 0.5). Each level adds a blur twice as
wide as the previous one, with the weight `scatter` raised to the level: with `scatter` 0 or with
a single level it is the simple glow. Quality limits the levels (1 at `q0`, 2 at `q1`, 4 at `q2`
and up to 8 at `q3`) and each level costs about a third of the previous one.

In the editor, the canvas panel has a control per field: sliders for the threshold, the strength,
the radius, the soft edge and the saturation; a switch to read the radius in screen pixels; a
switch to put a cap on the brightest part, with its slider from 0 to 16; and a switch for the
brightness above white, with sliders for its threshold, strength, soft edge, levels and scatter.
From the terminal: `reiwand-ctl scene act FOLDER canvas '{"bloom":{"saturation":1.6}}'`.
