# Lights, shadows and the camera

On this page: lights among models, shadows and the 3D camera.

## Lights among models

Without `position3d`, each light is on the plane that passes through the point the camera looks
at, below the point of the box where it is drawn, brought closer to the viewer by its `z`. With
`position3d` the light is at that point among the models, whatever happens with the camera: it can
end up behind or beside a model. With `aim3d`, a spot or a directional light shines from there
toward that point.

```json
    {
      "blend_space": "canvas", "id": "sun", "type": "light", "light": "directional",
      "color": "#fff1d6", "intensity": 1.4, "shadows": true, "shadow_resolution": 1024,
      "shadow_bias": 0.2, "position3d": [4, 6, 3], "aim3d": [0, 0, 0], "blend": "normal",
      "cache": false, "cone": [30, 60], "depth_test": false, "effects": [], "length": 256,
      "opacity": 1.0, "parallax": [0, 0], "radius": 512.0, "solid": false,
      "transform": { "origin": "anchor", "rotation": 0.0, "scale": [1.0, 1.0], "skew": [0.0, 0.0] },
      "visible": true, "z": 100.0
    }
```

```json
    {
      "blend_space": "canvas", "id": "lamp", "type": "light", "light": "point",
      "color": "#8fb4ff", "intensity": 1.2, "radius": 600, "position3d": [-3, 2, 2],
      "blend": "normal", "cache": false, "cone": [30, 60], "depth_test": false, "effects": [],
      "length": 256, "opacity": 1.0, "parallax": [0, 0], "shadow_bias": 0.2,
      "shadow_resolution": null, "shadows": false, "solid": false,
      "transform": { "origin": "anchor", "rotation": 0.0, "scale": [1.0, 1.0], "skew": [0.0, 0.0] },
      "visible": true, "z": 100.0
    }
```

A scene with no lights, no surrounding light, no sky and no `env` lights its models with a soft
light from the upper left of the camera, so that a model never comes out black. The surrounding
light and the sky (`canvas.ambient`, `canvas.sky`) arrive from below and from above the camera. At
most 12 lights count (8 in `t0` and `t1`).

## Shadows

A point light, a spot or a directional light with `"shadows": true` casts shadows of the models
onto the models whose materials receive them. A scene can have up to 2 lights with shadows.

Shadow maps are 2048 pixels per side for a directional light, 1024 for a spot and a cube of six
512 faces for a point light, up to 4096 (1024 on phones, 2048 in `t2`) and half that at the lowest
level; `shadow_resolution` sets another size. `shadow_bias` avoids stripes (raise it) or shadows
detached from their object (lower it). Desktop equipment (`t3`) draws the shadows of the first two
lights that cast them, `t2` of the first one, and `t0` and `t1` of none.

A map is redrawn only when its light, an object, a pose or a swaying material changes. What is
drawn with `blend` or `add` and materials with `cast_shadows: false` do not cast; cut-out pieces
(`mask`) cast only where they are drawn. Fur casts the shadow of its length in `q2` and `q3`. It is
an approximation: it draws a single shell, at 0.6 of the length, cut to the density with the same
pattern, and not all the fur layers; in `q0` and `q1` only the skin casts.

From the terminal: `reiwand-ctl scene act DIR scene3d`.

## The 3D camera

`camera3d` at the root of the scene is the camera of the models, the 3D groups and the tilted
layers:

```json
  "camera3d": {
    "fov": 40, "eye": [5, 3.5, 9], "center": [0, 0.5, 0],
    "paths": [
      {
        "id": "lap", "loop": true, "ease": "in_out_sine",
        "keys": [
          { "t": 0, "eye": [5, 3.5, 9], "center": [0, 0.5, 0], "fov": null },
          { "t": 8, "eye": [-5, 3.5, 9], "center": [0, 0.5, 0], "fov": null }
        ]
      }
    ],
    "far": 1000.0, "near": 0.1, "order": "sequential", "up": [0.0, 1.0, 0.0]
  }
```

| Field | Prefill | Range | What it does |
|-------|---------|-------|--------------|
| `fov` | 60 | 1 to 179 degrees | The field of view. It can follow values. |
| `near`, `far` | 0.1 and 1000 | | The near and far planes. |
| `eye` | `[0, 0, 5]` | | Where the camera is. It can follow values. |
| `center` | `[0, 0, 0]` | | Where it looks. It can follow values. |
| `up` | `[0, 1, 0]` | | Which way is up. |
| `paths` | | up to 16 | Paths the camera follows. |
| `order` | `sequential` | `sequential`, `random` | How the paths are chained. |
| `fade` | | | How they are joined: `{ "to": "black" or "cross", "duration": 0 to 10 s }`. |

### Paths

Each path is `{ "id", "keys", "ease", "loop" }`, with 2 to 256 keys
`{ "t", "eye", "center", "up", "fov" }` in increasing time (each key must carry `eye` and
`center`; `up` and `fov` are optional and, without them, keep those of the camera). The camera
passes through each key along a smooth curve (Catmull-Rom). An open path holds its first key until
its first `t` and lasts until its last key; with `loop` it closes back to the first, without
jumps. `ease` shapes the time of a lap.

An open path plays once and stays at the last key, without asking for more frames; one with `loop`
goes around forever. Several paths go one after another (`sequential`) or in a new order each
round (`random`). `fade` joins them: `black` darkens the second half of the duration of one path
and brightens the first half of the next (the whole scene, flat layers included); `cross` draws
the end of one path over the start of the next, blending the models. Paths that move ask for
frames, up to `max_fps` and at most 30 per second at the lowest quality level.

Scripts read and change `scene.camera3d` (`fov`, `eye`, `center`, `up`); what they set wins over
the paths (see [scripts.md](../scripts.md)).

From the terminal: `reiwand-ctl scene act DIR scene3d` sets the camera and `reiwand-ctl scene act
DIR scene3d` adds, keys, orders and blends paths.
