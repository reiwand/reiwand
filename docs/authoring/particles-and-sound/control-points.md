# Control points, children and depth

`points` defines up to 8 points (`p0` to `p7`) with `at` (which can follow a value), `follow:
"cursor"`, `space` and `angle`. Points can be used in shapes, forces and `init.inherit`. `p0` is
the origin of the system unless `points` changes it. A point that follows the pointer returns to
the center when the pointer leaves.

`children` makes other systems be born from these particles: each child is `{ "system", "on",
"probability", "max", "offset", "angle", "scale", "points_from", "inherit", "follow_inherit" }`.
`on` is `static` (born around the origin of the parent), `spawn` or `death` (a burst where each
particle of the parent is born or dies) or `follow` (born at their own rate on each live particle
and go with it). `inherit` copies `color`, `size`, `alpha` or `velocity`. They can nest two levels
and never loop back to themselves.

With `perspective`, each particle keeps a depth `z` and is drawn smaller the farther away it is.
`init.depth` and `init.depth_speed` set it.

```json
    {
      "blend_space": "canvas", "id": "stars", "type": "particles", "system": "stars",
      "blend": "add", "cache": false, "color": "#ffffff", "depth_test": false, "effects": [],
      "opacity": 1.0, "parallax": [0, 0], "solid": false,
      "transform": { "origin": "anchor", "rotation": 0.0, "scale": [1.0, 1.0], "skew": [0.0, 0.0] },
      "visible": true
    }
```

```json
  "particles": {
    "rockets": {
      "capacity": 200,
      "emitters": [
        {
          "shape": { "point": [960, 1040] }, "rate": 0,
          "bursts": [{ "count": 1, "every": 1.6, "at": 0 }], "audio_gate": false, "delay": 0,
          "duration": null, "inherit": 0
        }
      ],
      "init": {
        "life": [1.1, 1.5, 1], "speed": [700, 850, 1], "angle": [255, 285, 1], "size": [10, 12, 1],
        "color": ["#ffe9a8", "#ffb347", 1], "frame": [0.0, 0.0, 1], "gradient": [], "remap": [],
        "rotation": [0.0, 0.0, 1], "spin": [0.0, 0.0, 1],
        "tilt": { "angle": [[0, 0], [0, 0]], "spin": [[0, 0], [0, 0]] }
      },
      "forces": [{ "gravity": [0, 500], "window": [0, 0, 1, 1] }],
      "render": {
        "trail": {
          "length": 12, "width": 0.6, "segments": null, "subdivide": 0, "textured": false,
          "uv_scale": 1, "uv_scroll": 0
        },
        "angle": null, "aspect": 1, "color_over_life": [], "cycles": null, "fps": 12.0, "lit": false,
        "mix": false, "size_over_life": []
      },
      "children": [
        {
          "system": "stars", "on": "death", "inherit": ["color"], "angle": 0,
          "follow_inherit": false, "offset": [0, 0], "points_from": false, "probability": 1.0,
          "scale": 1.0
        }
      ],
      "collide": [], "lock": [], "points": [], "prewarm": 0.0, "seed": 0, "space": "local"
    },
    "fireflies": {
      "capacity": 600, "space": "world", "prewarm": 3,
      "points": [
        { "at": [960, 540], "angle": 0, "space": "local" },
        { "follow": "cursor", "angle": 0, "space": "local" }
      ],
      "emitters": [
        {
          "shape": {
            "around": {
              "at": "p0", "count": 24, "radius": 420, "speed": 20, "arc": 360.0, "axis": [0, 0, 1],
              "mode": "repeat", "restart": false
            }
          },
          "rate": 40, "periodic": { "on": [4, 6], "off": [1, 2] }, "max_per_step": 4,
          "audio_gate": false, "bursts": [], "delay": 0, "duration": null, "inherit": 0
        }
      ],
      "init": {
        "life": [3, 6, 1], "speed": [10, 30, 2], "angle": [0, 360, 1], "size": [6, 14, 1],
        "hsv": { "hue": [50, 120], "sat": [0.5, 0.9], "val": [0.9, 1], "steps": 0 },
        "alpha": [0.5, 1, 1],
        "remap": [
          {
            "input": "life", "in": [3, 6], "output": "size", "out": [1, 0.3], "op": "mul",
            "clamp_in": false, "clamp_out": false, "wave": "none"
          }
        ],
        "color": ["#ffffff", "#ffffff", 1], "frame": [0.0, 0.0, 1], "gradient": [],
        "rotation": [0.0, 0.0, 1], "spin": [0.0, 0.0, 1],
        "tilt": { "angle": [[0, 0], [0, 0]], "spin": [[0, 0], [0, 0]] }
      },
      "forces": [
        {
          "turbulence": {
            "strength": 40, "scale": 0.003, "mask": [1, 1], "speed": 0, "time_scale": 1
          },
          "window": [0, 0, 1, 1]
        },
        {
          "attractor": { "target": "p1", "strength": 4000, "radius": 500, "falloff": "linear" },
          "window": [0, 0, 1, 1]
        },
        {
          "oscillate": {
            "what": "alpha", "from": "top", "freq": [0.5, 1.5], "scale": [0.2, 0.5], "mask": [1, 1],
            "phase": [0.0, 0.0], "random": "shared"
          },
          "window": [0, 0, 1, 1]
        },
        { "drag": 0.3, "window": [0, 0.2, 0.8, 1] }
      ],
      "render": {
        "size_over_life": [[0, 0], [0.2, 1], [1, 0]], "cutout": [0.1, 0.9, 1], "order": "newest",
        "angle": null, "aspect": 1, "color_over_life": [], "cycles": null, "fps": 12.0, "lit": false,
        "mix": false
      },
      "children": [], "collide": [], "lock": [], "seed": 0
    },
    "stars": {
      "capacity": 3000,
      "emitters": [
        {
          "shape": { "point": [0, 0] }, "rate": 0,
          "bursts": [{ "count": 60, "at": 0, "every": null }], "audio_gate": false, "delay": 0,
          "duration": null, "inherit": 0
        }
      ],
      "init": {
        "life": [0.9, 1.6, 1], "speed": [80, 320, 1], "angle": [0, 360, 1], "size": [5, 9, 1],
        "color": ["#ffffff", "#9ad1ff", 1], "frame": [0.0, 0.0, 1], "gradient": [], "remap": [],
        "rotation": [0.0, 0.0, 1], "spin": [0.0, 0.0, 1],
        "tilt": { "angle": [[0, 0], [0, 0]], "spin": [[0, 0], [0, 0]] }
      },
      "forces": [
        { "gravity": [0, 160], "window": [0, 0, 1, 1] }, { "drag": 0.8, "window": [0, 0, 1, 1] }
      ],
      "render": {
        "size_over_life": [[0, 1], [1, 0]], "color_over_life": [[0, "#ffffff"], [1, "#ffffff00"]],
        "angle": null, "aspect": 1, "cycles": null, "fps": 12.0, "lit": false, "mix": false
      },
      "children": [], "collide": [], "lock": [], "points": [], "prewarm": 0.0, "seed": 0,
      "space": "local"
    }
  }
```
