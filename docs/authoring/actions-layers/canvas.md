# Canvas

## `canvas`

Changes the canvas: its size, the clock, the glow, the parallax, the 2D camera and the preview aids. It is a single action with several variants. In the app, almost everything is in the canvas inspector and in the **Canvas and clock** menu.

### variant `size`

Sets the canvas's reference size.

| Argument | Type | What it means |
|-----------|------|---------------|
| `size` | list of 2 integers (required) | Width and height in pixels, from 1 to 16384 each. |

```sh
reiwand-ctl scene act my-scene canvas '{"size":{"size":[1280,720]}}'
```

### variant `clock`

Sets the frame that is shown where there is no motion and the seed for randomness.

| Argument | Type | What it means |
|-----------|------|---------------|
| `poster` | number | Second of the still frame (the scene's image without motion), from 0 to 86400. |
| `seed` | integer | Seed of the procedural effects: with the same one, it always comes out the same. |
| `fresh` | yes/no | `true` makes each start use a new seed. It cannot be combined with `seed`. Default `false`. |

```sh
reiwand-ctl scene act my-scene canvas '{"clock":{"poster":2.5,"seed":7}}'
reiwand-ctl scene act my-scene canvas '{"clock":{"fresh":true}}'
```

### variant `bloom`

Makes bright things "glow" (bloom). What is not named starts with the default values the first time; what is saved carries all the fields.

| Argument | Type | What it means |
|-----------|------|---------------|
| `threshold` | number | From what brightness the glow starts, from 0 to 4. Default 0.8. |
| `strength` | number | Strength, from 0 to 8. Default 1. |
| `radius` | number | Reach in pixels, from 0 to 128. Default 16. |
| `radius_unit` | text | `canvas` (canvas pixels) or `screen` (screen pixels: a canvas twice the size of the screen keeps the same halo). Default `canvas`. |
| `knee` | number | The part of the threshold over which the cutoff is softened, from 0 to 1; with 0 the cutoff is hard. Default 0.5. |
| `saturation` | number | From 0 to 4; 0 gives a gray glow, 1 keeps the colors and 4 makes them more vivid. Default 1. |
| `limit` | number or `"none"` | Cap on each channel of the bright parts, from 0 to 16, so that a very bright point does not cover a weaker one; `"none"` removes the cap. Default `"none"`. |
| `tint` | text | Color of the glow, such as `#ffddaa`. |
| `hdr` | object or `"none"` | The glow of colors brighter than white: `threshold` (from 0 to 16) and `strength` (from 0 to 8), and optionally `knee` (from 0 to 1, default 0.5), `levels` (from 1 to 8, default 1: how many blurs, each twice as wide as the last, are added together; quality limits them) and `scatter` (from 0 to 2, default 0.5: the weight of each wider level; with 0 or with a single level it is the simple glow). What is not named stays as it was; `"none"` removes it. |
| `off` | yes/no | `true` removes the glow. Default `false`. |

```sh
reiwand-ctl scene act my-scene canvas '{"bloom":{"threshold":0.8,"strength":0.6,"radius":8,"tint":"#ffddaa"}}'
reiwand-ctl scene act my-scene canvas '{"bloom":{"saturation":1.6,"limit":4,"radius_unit":"screen"}}'
reiwand-ctl scene act my-scene canvas '{"bloom":{"hdr":{"threshold":1.5,"strength":0.5,"levels":4,"scatter":0.6}}}'
reiwand-ctl scene act my-scene canvas '{"bloom":{"hdr":"none"}}'
reiwand-ctl scene act my-scene canvas '{"bloom":{"off":true}}'
```

### variant `parallax`

Makes the scene shift in depth according to the pointer or a phone's tilt. How much each layer moves is decided by its depth (see `depth` in [`layer`](place-and-shape-a-layer.md#layer)).

| Argument | Type | What it means |
|-----------|------|---------------|
| `amount` | number | Overall strength, from 0 to 4. |
| `delay` | number | Soft delay in seconds, from 0 to 2. |
| `pointer` | number | How much the pointer drives it, from 0 to 1. |
| `gyro` | yes/no | `true` also uses the phone's tilt. |
| `offset` | list of 2 numbers | Fixed offset, from -1 to 1 in each direction. |
| `off` | yes/no | `true` removes the parallax. Default `false`. |

```sh
reiwand-ctl scene act my-scene canvas '{"parallax":{"amount":1.5,"delay":0.2,"pointer":1,"gyro":true}}'
reiwand-ctl scene act my-scene canvas '{"parallax":{"off":true}}'
```

### variant `camera`

Sets the scene's 2D camera (it creates it if missing).

| Argument | Type | What it means |
|-----------|------|---------------|
| `center` | list of 2 numbers | Canvas point, in pixels, that ends up in the center of what is seen. |
| `zoom` | number | Zoom from 0.1 to 10. |
| `rotation` | number | Rotation in degrees, from -360 to 360. |
| `at` | number | Second at which it is written, if the field has keys. |
| `replace` | yes/no | As in `transform` of [`layer`](place-and-shape-a-layer.md#layer). |
| `off` | yes/no | `true` removes the camera. Default `false`. |

```sh
reiwand-ctl scene act my-scene canvas '{"camera":{"center":[960,540],"zoom":1.5,"rotation":5}}'
reiwand-ctl scene act my-scene canvas '{"camera":{"off":true}}'
```

### variant `camera_grip`

Does what dragging a camera handle on the canvas to the point `to` would do.

| Argument | Type | What it means |
|-----------|------|---------------|
| `hold` | text or object (required) | `"move"` puts the middle of what the camera sees at `to`; `{"zoom":{"corner":[1,1]}}` adjusts the zoom so that the screen corner `corner` (fractions) lies on the line from the center to `to`; `"turn"` rotates the camera so its top points to `to`. |
| `to` | list of 2 numbers (required) | Canvas point in pixels. |
| `at`, `replace` | | As in `camera`. |

```sh
reiwand-ctl scene act my-scene canvas '{"camera_grip":{"hold":"move","to":[800,500]}}'
reiwand-ctl scene act my-scene canvas '{"camera_grip":{"hold":{"zoom":{"corner":[1,1]}},"to":[1500,900]}}'
reiwand-ctl scene act my-scene canvas '{"camera_grip":{"hold":"turn","to":[1500,100]}}'
```

### variant `camera_view`

Makes the preview show the whole scene with the camera's frame. It only changes the editor's view.

| Argument | Type | What it means |
|-----------|------|---------------|
| `whole` | yes/no (required) | `true` shows the whole scene, `false` goes back to what the camera sees. |

```sh
reiwand-ctl scene act my-scene canvas '{"camera_view":{"whole":true}}'
```

### variant `probe`

Simulates a pointer and a phone tilt in the preview, to see how the parallax moves. It only changes the editor's view.

| Argument | Type | What it means |
|-----------|------|---------------|
| `pointer` | list of 2 numbers | Simulated pointer position as a fraction of the canvas, from 0 to 1 on each side. |
| `tilt` | list of 2 numbers | Simulated tilt, from -1 to 1 on each side. |
| `off` | yes/no | `true` removes the simulation. Default `false`. |

```sh
reiwand-ctl scene act my-scene canvas '{"probe":{"pointer":[0.7,0.3],"tilt":[0.2,0]}}'
reiwand-ctl scene act my-scene canvas '{"probe":{"off":true}}'
```
