# Group and adjustment layers

On this page: `group` and `adjustment`.

## Group layer (`group`)

Gathers layers. It is for moving, rotating, blending or applying effects to all of them at once.

```json
    {
      "blend_space": "canvas", "id": "window", "type": "group", "clip": true,
      "layers": [
        {
          "blend_space": "canvas", "id": "frame", "type": "shape",
          "shape": { "rect": { "size": [700, 420], "radius": 60 } }, "fill": "#ffffff",
          "blend": "normal", "cache": false, "depth_test": false, "effects": [], "opacity": 1.0,
          "parallax": [0, 0], "solid": false,
          "transform": {
            "origin": "anchor", "rotation": 0.0, "scale": [1.0, 1.0], "skew": [0.0, 0.0]
          },
          "visible": true
        },
        {
          "blend_space": "canvas", "id": "interior", "type": "image", "image": "sky",
          "size": [1400, 790],
          "transform": {
            "position": {
              "timeline": "stroll", "keys": [[0, [760, 540]], [8, [1160, 540]]], "invert": false
            },
            "origin": "anchor", "rotation": 0.0, "scale": [1.0, 1.0], "skew": [0.0, 0.0]
          },
          "effects": [
            {
              "effect": "color_adjust",
              "params": { "saturation": 1.6, "contrast": 1.2, "exposure": 0.0, "hue": 0.0 },
              "enabled": true, "optional": false
            }
          ],
          "blend": "normal", "cache": false, "color": "#ffffff", "depth_test": false, "opacity": 1.0,
          "parallax": [0, 0], "solid": false, "visible": true
        }
      ],
      "blend": "normal", "cache": false, "color": "#ffffff", "depth_test": false, "effects": [],
      "opacity": 1.0, "parallax": [0, 0], "solid": false, "space": "2d",
      "transform": { "origin": "anchor", "rotation": 0.0, "scale": [1.0, 1.0], "skew": [0.0, 0.0] },
      "visible": true
    }
```

| Field | Prefill | What it does |
|-------|---------|--------------|
| `layers` | required | The layers inside. |
| `clip` | `false` | The layers after the first one are only seen where the first one is seen. |
| `color` | white | Color that multiplies all of them together. |
| `space` | `2d` | With `3d`, the group draws its models together with a shared depth. See [3d.md](../3d.md). |

A group with effects, a mask, opacity below 1 or `cache` is first drawn separately and composed
whole (one more pass). The `transform` of a group moves all its layers (see "Placing a layer").

## Adjustment layer (`adjustment`)

It draws nothing of its own: it applies its `effects` to everything underneath it (the canvas, or
its group).

```json
    {
      "blend_space": "canvas", "id": "retouch", "type": "adjustment", "size": [900, 500],
      "transform": {
        "position": [960, 540], "origin": "anchor", "rotation": 0.0, "scale": [1.0, 1.0],
        "skew": [0.0, 0.0]
      },
      "effects": [
        {
          "effect": "vignette",
          "params": {
            "intensity": 0.5, "center": [0.5, 0.5], "color": "#000000", "radius": 1.0,
            "roundness": 1.0, "softness": 0.65
          },
          "enabled": true, "optional": false
        }
      ],
      "blend": "normal", "cache": false, "depth_test": false, "opacity": 1.0, "parallax": [0, 0],
      "redraw": false, "rgb": false, "solid": false, "visible": true
    }
```

| Field | Prefill | What it does |
|-------|---------|--------------|
| `effects` | | The effects that are applied. |
| `size` | the whole canvas | `[width, height]`, up to twice the canvas: only that rectangle around the position is captured and changed. |
| `redraw` | `false` | Draws the captured content again with the layer's `transform`, opacity, blend and mask, like a magnifier or a mirror. |
| `rgb` | `false` | Sends what it captures to the computer's RGB lighting. It needs `permissions.rgb` in the manifest and the user allowing it. |

With `size` the cost goes down because only those pixels are processed, but the capture is a copy
and one more pass.
