# Particles and sound

These pages are about two things a scene can have besides images: particle systems (sparks, snow,
fireflies) and sound (one that plays and one you see move).

The examples come from [`examples/particles`](examples/particles/scene.json),
[`examples/advanced-particles`](examples/advanced-particles/scene.json) and
[`examples/sound`](examples/sound/scene.json), and they pass `reiwand-ctl pack check`.

# Particles

A particle system is a recipe: where they are born, how they come out, what forces move them and
how they are drawn. The recipe goes in `particles` of `scene.json`, with an id per system, and a
`particles` layer draws it.

```json
    {
      "blend_space": "canvas", "id": "sparks", "type": "particles", "system": "sparks",
      "blend": "add", "instance": { "alpha": 0.9, "count": 1, "points": [] }, "cache": false,
      "color": "#ffffff", "depth_test": false, "effects": [], "opacity": 1.0, "parallax": [0, 0],
      "solid": false,
      "transform": { "origin": "anchor", "rotation": 0.0, "scale": [1.0, 1.0], "skew": [0.0, 0.0] },
      "visible": true
    }
```

```json
  "particles": {
    "sparks": {
      "capacity": 4000, "seed": 7, "prewarm": 2,
      "emitters": [
        {
          "shape": { "box": { "from": [0, 1080], "to": [1920, 1100], "inner": [0, 0] } },
          "rate": { "prop": "density", "invert": false }, "audio_gate": false, "bursts": [],
          "delay": 0, "duration": null, "inherit": 0
        }
      ],
      "init": {
        "life": [2, 5, 1], "speed": [200, 500, 1], "angle": [250, 290, 1], "size": [4, 12, 1],
        "color": ["#ffd27a", "#ff7a3d", 1], "frame": [0.0, 0.0, 1], "gradient": [], "remap": [],
        "rotation": [0.0, 0.0, 1], "spin": [0.0, 0.0, 1],
        "tilt": { "angle": [[0, 0], [0, 0]], "spin": [[0, 0], [0, 0]] }
      },
      "forces": [
        { "gravity": [0, -20], "window": [0, 0, 1, 1] },
        {
          "turbulence": {
            "strength": 30, "scale": 0.004, "mask": [1, 1], "speed": 0, "time_scale": 1
          },
          "window": [0, 0, 1, 1]
        },
        { "drag": 0.4, "window": [0, 0, 1, 1] }
      ],
      "collide": [
        { "bounds": "canvas", "bounce": 0.3, "friction": 0, "kill": false, "stick": false }
      ],
      "render": {
        "size_over_life": [[0, 0], [0.1, 1], [1, 0]],
        "color_over_life": [[0, "#ffffff"], [1, "#ffffff00"]], "angle": null, "aspect": 1,
        "cycles": null, "fps": 12.0, "lit": false, "mix": false
      },
      "children": [], "lock": [], "points": [], "space": "local"
    },
    "snow": {
      "capacity": 3000,
      "emitters": [
        {
          "shape": { "line": [[0, -20], [1920, -20]] }, "rate": 120, "audio_gate": false,
          "bursts": [], "delay": 0, "duration": null, "inherit": 0
        }
      ],
      "init": {
        "life": [8, 12, 1], "speed": [40, 90, 1], "angle": [80, 100, 1], "size": [3, 8, 1],
        "color": ["#ffffff", "#cfe6ff", 1], "frame": [0.0, 0.0, 1], "gradient": [], "remap": [],
        "rotation": [0.0, 0.0, 1], "spin": [0.0, 0.0, 1],
        "tilt": { "angle": [[0, 0], [0, 0]], "spin": [[0, 0], [0, 0]] }
      },
      "forces": [
        { "gravity": [12, 0], "window": [0, 0, 1, 1] }, { "noise": 6, "window": [0, 0, 1, 1] }
      ],
      "render": {
        "size_over_life": [[0, 1], [1, 1]], "angle": null, "aspect": 1, "color_over_life": [],
        "cycles": null, "fps": 12.0, "lit": false, "mix": false
      },
      "children": [], "collide": [], "lock": [], "points": [], "prewarm": 0.0, "seed": 0,
      "space": "local"
    },
    "embers": {
      "capacity": 1200, "seed": 11, "prewarm": 8,
      "emitters": [
        {
          "shape": { "box": { "from": [160, 980], "to": [1760, 1080], "inner": [0, 0] } },
          "rate": 90, "audio_gate": false, "bursts": [], "delay": 0, "duration": null, "inherit": 0
        }
      ],
      "init": {
        "life": [7, 12, 1], "speed": [60, 140, 1], "angle": [255, 285, 1], "size": [8, 22, 2],
        "color": ["#ffc070", "#ff6a2a", 1], "frame": [0.0, 0.0, 1], "gradient": [], "remap": [],
        "rotation": [0.0, 0.0, 1], "spin": [0.0, 0.0, 1],
        "tilt": { "angle": [[0, 0], [0, 0]], "spin": [[0, 0], [0, 0]] }
      },
      "forces": [
        { "gravity": [0, -30], "window": [0, 0, 1, 1] },
        {
          "noise_field": {
            "strength": { "min": 40, "max": 160 }, "phase": { "min": 0, "max": 600 },
            "scale": 0.003, "drift": 20, "mask": [1, 0.6, 0]
          },
          "ramp": "linear", "window": [0, 0.15, 1, 1]
        },
        { "drag": 0.7, "window": [0, 0, 1, 1] }
      ],
      "render": {
        "size_over_life": [[0, 0], [0.06, 1], [0.7, 0.8], [1, 0.2]],
        "color_over_life": [[0, "#ffffff"], [0.7, "#ffd0a0"], [1, "#ff402000"]], "angle": null,
        "aspect": 1, "brightness": 1.6, "cycles": null, "fps": 12.0, "lit": false, "mix": false
      },
      "children": [], "collide": [], "lock": [], "points": [], "space": "local"
    }
  }
```

A scene can have up to 32 systems and up to 200,000 particles among all of them. Several layers
can draw the same system: the system moves only once per frame and shares its budget among them.
Positions are canvas pixels; the `transform` of the layer moves, rotates and scales the whole
system around the center of the canvas.

The `particles` layer:

> In these tables, *Prefill* is the value the editor and the templates write for you. The file still has to contain every field: the loader refuses a missing one and never fills it in (see `../specs/explicit-values.md`).

| Field | Prefill | What it does |
|-------|---------|--------------|
| `system` | required | The id of the system. |
| `color` | white | Color that multiplies the particles. |
| `instance` | | What changes only for this layer. See "Instances". |
| `render` | that of the system | A look of its own for this layer, which replaces that of the system. |

The fields of every layer apply: `blend` (`add` works well with glowing particles), `opacity`,
`mask` and `effects`.

Particles and sound, by topic:

- [Particle systems and emitters](particles-and-sound/systems.md): starting points, a system and its emitters.
- [How particles start and move](particles-and-sound/motion.md): `init`, forces and collisions.
- [How particles are drawn](particles-and-sound/drawing.md): `render`, instances and how many fit.
- [Control points, children and depth](particles-and-sound/control-points.md): points that steer a system and systems inside systems.
- [Sound](particles-and-sound/sound.md): a sound layer and a scene that reacts to sound.

## Not covered here

- Sounds fired from scripts: [scripts.md](scripts.md).
- The cover and title of the music that plays (`media`): [scene.md](scene.md).
