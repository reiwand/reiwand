# Shape, solid and spectrum layers

On this page: `shape`, `solid` and `spectrum`.

## Shape layer (`shape`)

```json
    {
      "blend_space": "canvas", "id": "panel", "type": "shape",
      "shape": { "rect": { "size": [1200, 500], "radius": 40 } },
      "gradient": {
        "from": [0, 0], "to": [0, 1], "stops": [[0, "#2b3a67"], [1, "#101828"]], "radial": false
      },
      "stroke": { "color": "#8aa4ff", "width": 4 }, "blend": "normal", "cache": false,
      "depth_test": false, "effects": [], "opacity": 1.0, "parallax": [0, 0], "solid": false,
      "transform": { "origin": "anchor", "rotation": 0.0, "scale": [1.0, 1.0], "skew": [0.0, 0.0] },
      "visible": true
    }
```

| Field | Prefill | What it does |
|-------|---------|--------------|
| `shape` | required | One of the six shapes below. |
| `fill` | none | The fill color. |
| `gradient` | none | A gradient that replaces the fill. |
| `stroke` | none | A border. |

The shapes:

| Shape | Fields | Range |
|-------|--------|-------|
| `rect` | `size` `[width, height]`, `radius` (0 default) | `radius` up to half the shorter side |
| `ellipse` | `size` `[width, height]` | |
| `ring` | `size` `[width, height]`, `width` | `width` from 0.001 up to half the shorter side |
| `polygon` | `sides`, `size` | `sides` from 3 to 64 |
| `star` | `points`, `size`, `inner` (0.5 default) | `points` from 2 to 64, `inner` from 0.01 to 1 |
| `line` | `to` `[x, y]`, `width` | `to` from -100000 to 100000 |

A `line` measures what `to` says and is centered on the position of the layer.

`gradient`: `from` (`[0, 0]`) and `to` (`[0, 1]`) are fractions of the shape's box; `radial`
(`false`) makes it circular from `from`, with `to` marking the edge; `stops` is a list of 2 to 4
`[position, color]` with the positions from 0 to 1 in order. `stroke`: `color` (white) and `width`
(0 to 10000 px).

```json
    {
      "blend_space": "canvas", "id": "star", "type": "shape",
      "shape": { "star": { "points": 5, "size": 120, "inner": 0.45 } }, "fill": "#f472b6",
      "transform": {
        "position": [1280, 540], "origin": "anchor", "rotation": 0.0, "scale": [1.0, 1.0],
        "skew": [0.0, 0.0]
      },
      "blend": "normal", "cache": false, "depth_test": false, "effects": [], "opacity": 1.0,
      "parallax": [0, 0], "solid": false, "visible": true
    }
```

```json
    {
      "blend_space": "canvas", "id": "shine", "type": "shape",
      "shape": { "ellipse": { "size": [700, 700] } }, "blend": "add",
      "gradient": {
        "radial": true, "from": [0.5, 0.5], "to": [1, 0.5],
        "stops": [[0, "#ffffff60"], [0.5, "#5b7cff30"], [1, "#5b7cff00"]]
      },
      "cache": false, "depth_test": false, "effects": [], "opacity": 1.0, "parallax": [0, 0],
      "solid": false,
      "transform": { "origin": "anchor", "rotation": 0.0, "scale": [1.0, 1.0], "skew": [0.0, 0.0] },
      "visible": true
    }
```

## Solid layer (`solid`)

Fills a rectangle with a color. Without `size` it covers the whole canvas.

```json
    {
      "blend_space": "canvas", "id": "background", "type": "solid", "color": "#101828", "blend": "normal",
      "cache": false, "depth_test": false, "effects": [], "opacity": 1.0, "parallax": [0, 0],
      "size": "canvas", "solid": false,
      "transform": { "origin": "anchor", "rotation": 0.0, "scale": [1.0, 1.0], "skew": [0.0, 0.0] },
      "visible": true
    }
```

| Field | Prefill | What it does |
|-------|---------|--------------|
| `color` | required | The color (can be a value that moves). |
| `size` | the canvas | `[width, height]` in pixels. |

With a full-screen solid layer, an `add` `blend` and a mask from another layer, glows are made (see
[`groups-and-masks`](../examples/groups-and-masks/scene.json)).

## Spectrum layer (`spectrum`)

Draws the sound that plays on the computer. It needs the `audio` input.

```json
    {
      "blend_space": "canvas", "id": "spectrum", "type": "spectrum", "size": [1600, 360],
      "style": "bars", "bands": 32, "gap": 0.3, "color": "#7dd3fc", "anchor": "bottom",
      "transform": {
        "position": [0, -80], "origin": "anchor", "rotation": 0.0, "scale": [1.0, 1.0],
        "skew": [0.0, 0.0]
      },
      "blend": "normal", "cache": false, "channel": "average", "depth_test": false, "effects": [],
      "opacity": 1.0, "parallax": [0, 0], "solid": false, "visible": true
    }
```

| Field | Prefill | Range | What it does |
|-------|---------|-------|--------------|
| `size` | required | | `[width, height]` of the layer. |
| `style` | `bars` | `bars`, `circle`, `wave` | Bars, circle or wave. |
| `bands` | 32 | 16, 32 or 64 | How many bars. |
| `channel` | `average` | `average`, `left`, `right` | Which channel. |
| `color` | white | | The color. |
| `gap` | 0.2 | 0 to 0.95 | The gap between bars. |
