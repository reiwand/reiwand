# Depth and bright colors

On this page: depth for flat layers, HDR and tone mapping.

## Giving depth to flat layers

There are four ways to give depth to a scene of flat images.

### Tilt and move layers away (`tilt`, `z`, `perspective`)

`transform.tilt` tilts a layer in depth around its position (from -89 to 89 degrees each: a
positive `x` brings the bottom edge toward the viewer, a positive `y` the left edge), and
`transform.z` brings it closer (positive) or moves it away (negative) from the viewer. They are
seen with `canvas.perspective`, the viewer's distance in pixels (from 100 to 100000); without it,
the distance at which the canvas height fills the `fov` of `camera3d` (or 45 degrees) is used. A
layer at the viewer's height or behind them is not drawn.

Such a layer is drawn as a projected quadrilateral: no extra cost. Its effects run in its own
plane before being projected and its mask tilts with it. A tilted group is drawn separately once
more and projected whole, and its layers tilt with it. The pointer finds the tilted layer where it
is seen.

```json
    {
      "blend_space": "canvas", "id": "sign", "type": "shape",
      "shape": { "rect": { "size": [640, 360], "radius": 30 } }, "fill": "#ffffffcc",
      "transform": {
        "position": [960, 540], "tilt": [0, 25], "z": 120, "origin": "anchor", "rotation": 0.0,
        "scale": [1.0, 1.0], "skew": [0.0, 0.0]
      },
      "parallax": [0.03, 0.03], "blend": "normal", "cache": false, "depth_test": false,
      "effects": [], "opacity": 1.0, "solid": false, "visible": true
    }
```

### Parallax with the pointer (`parallax`)

With `canvas.parallax` and the `parallax` field of the layers (see [scene.md](../scene.md) and
[layers.md](../layers.md)), each layer moves a little differently when the pointer moves, or when the
phone is tilted (`gyro`): distant ones (0.01) less than near ones (0.05).

### A depth map (`depth_parallax`)

The `depth_parallax` effect moves each point of an image according to a depth map (a `linear`
asset: light is near): when the pointer moves (`follow`) or in a slow orbit (`orbit`), what is
near slides over what is far without breaking. With `smooth` it follows the pointer the way
`canvas.parallax` does, so that the depth and the parallax of the layers move together. `steps`
(from 8 to 24, 16 by default) are the steps of the search: 24 costs half as much again in `q3` and
is limited to 12 in `q2` and 8 in `q1`. `overscan` enlarges the layer to hide the edges. While it
moves nothing, it costs no pass; on the phone, whose pointer only follows touch, it almost always
looks like that.

```json
    {
      "blend_space": "canvas", "id": "background", "type": "image", "image": "sky",
      "size": [1920, 1080],
      "effects": [
        {
          "effect": "depth_parallax",
          "params": {
            "depth": "depth", "reach": 0, "strength": 40, "focus": 0.5, "steps": 16,
            "follow": true, "orbit": true, "overscan": 1.0, "scale": [1.0, 1.0], "smooth": false
          },
          "enabled": true, "optional": false
        }
      ],
      "blend": "normal", "cache": false, "color": "#ffffff", "depth_test": false, "opacity": 1.0,
      "parallax": [0, 0], "solid": false,
      "transform": { "origin": "anchor", "rotation": 0.0, "scale": [1.0, 1.0], "skew": [0.0, 0.0] },
      "visible": true
    }
```

A depth map is made with a depth tool or by hand (in the example, a gradient of grays). In the
editor there is a depth brush, and the program can estimate it from an image with a depth model
that is downloaded on request (`reiwand-ctl models download depth-anything-v2-small --yes`; see
[cli.md](../cli.md)).

### Mesh layers with depth

A `mesh` layer can use the depth of its vertices with the perspective of `canvas.perspective` (see
[meshes.md](../meshes.md)).

## HDR and tone mapping

With `canvas.hdr: true` colors brighter than white are stored in 16-bit images, and models are
drawn in 16 bits. `canvas.tonemap` (`none`, `aces` or `agx`, only with `hdr`) squeezes them down to
the screen in a last pass, after the glow. `srgb_to_linear` instead reads the final colors, glow
included, once more through the sRGB curve (0.5 of scene plus 0.5 of glow shows 1.0), for scenes
made for that last step. On a screen with HDR output the last step keeps the 16 bits and
smoothly bends what goes past 0.8 of white toward that screen's maximum, keeping the hue and the
midtones. `canvas.bloom.hdr` sets the glow threshold and strength in `q3`.

From the terminal: `reiwand-ctl scene act DIR scene3d`.
