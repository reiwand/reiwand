# Lights on flat layers

A `light` layer illuminates the layers that have a `lighting` block. It draws nothing itself. A
scene has up to 12 lights; on phones and older computers only the first 8 light things up, in
layer order, and the rest are named in the notes about what was left out.

```json
    {
      "blend_space": "canvas", "id": "lantern", "type": "light", "light": "point", "color": "#ffd9a0",
      "intensity": 1.6, "radius": 700, "z": 160,
      "transform": {
        "position": {
          "timeline": "stroll",
          "keys": [
            [0, [400, 400]], [8, [1500, 700], "in_out_sine"], [16, [400, 400], "in_out_sine"]
          ],
          "invert": false
        },
        "origin": "anchor", "rotation": 0.0, "scale": [1.0, 1.0], "skew": [0.0, 0.0]
      },
      "blend": "normal", "cache": false, "cone": [30, 60], "depth_test": false, "effects": [],
      "length": 256, "opacity": 1.0, "parallax": [0, 0], "shadow_bias": 0.2,
      "shadow_resolution": null, "shadows": false, "solid": false, "visible": true
    }
```

The `light` layer carries only `id`, `type`, `transform` (its position, its rotation for the
direction of a spot, a tube or a directional light, and `tilt`), `anchor`, `visible`, `parent`,
`inherit` and:

> In these tables, *Prefill* is the value the editor and the templates write for you. The file still has to contain every field: the loader refuses a missing one and never fills it in (see `../specs/explicit-values.md`).

| Field | Prefill | Range | What it does |
|-------|---------|-------|--------------|
| `light` | `point` | `point`, `spot`, `tube`, `directional` | The type: point, spot, tube or directional. |
| `color` | white | | The color. |
| `intensity` | 1 | 0 to 100 | The strength. |
| `radius` | 512 | 1 to 100000 px | How far it reaches. |
| `z` | 100 | -100000 to 100000 px | The height above the canvas. |
| `cone` | `[30, 60]` | 0 to 180 degrees | Spots only: `[inner, outer]`; full in the middle, dark beyond the outer angle. |
| `length` | 256 | 1 to 100000 px | Tubes only. |
| `volumetric` | | see below | Light that shines in the air. Points and spots. |
| `texture` | | | Spots only: an image, a video or `{ "layer": id }` that the light projects. |
| `shadows` | `false` | | Casts shadows of models. See "Shadows". |
| `shadow_resolution` | by type | 256 to 4096 px | Size of the shadow map. |
| `shadow_bias` | 0.2 | 0 to 1 | How much surfaces are lifted off their own shadow. |
| `position3d` | | `[x, y, z]` | Where it is among the models. See "Lights among models". |
| `aim3d` | | `[x, y, z]` | Where a spot or a directional light aims among the models. |

`color`, `intensity`, `radius`, `z`, `position3d` and `aim3d` can follow values or keys. A light
follows its parents, groups and bones (`attach`) like the other layers. The `radius` and the
length of a light are always measured in pixels of the camera's focal plane, also with
`position3d`.

A spot or a directional light with no rotation points toward the bottom of the canvas and turns
clockwise with `rotation`; the first number of `transform.tilt` (from -89 to 89) tilts it into the
canvas, so `[90, 0]` would point straight ahead. A tube lies along the x axis of its layer, with
`length` as its length, and lights like a row of point lights. The light of a point, a spot or a
tube fades as (1 − d²/r²)², with d the distance in three dimensions (the height `z` included), and
ends at the radius; a directional light reaches everywhere with the same strength.

### Which layers receive light

An image, video, text, mesh, model or particles layer receives light if it has `lighting`:

```json
    {
      "blend_space": "canvas", "id": "wall", "type": "image", "image": "stone",
      "size": [1920, 1080],
      "lighting": {
        "normal": "relief", "roughness": 0.4, "strength": 1, "emissive": 0.0, "reflection": 0.0
      },
      "blend": "normal", "cache": false, "color": "#ffffff", "depth_test": false, "effects": [],
      "opacity": 1.0, "parallax": [0, 0], "solid": false,
      "transform": { "origin": "anchor", "rotation": 0.0, "scale": [1.0, 1.0], "skew": [0.0, 0.0] },
      "visible": true
    }
```

| Field | Prefill | Range | What it does |
|-------|---------|-------|--------------|
| `normal` | | | A normal map (a `linear` asset): red to the right, green up and blue toward the viewer, from 0 to 1 for -1 to 1. |
| `roughness` | 0.5 | 0 to 1 | The roughness: sharp shine on smooth surfaces, spread out on rough ones. |
| `roughness_map` | | | A `linear` grayscale image that multiplies the roughness. |
| `reflection` | 0 | 0 to 1 | How much it reflects of `env`, or of what is underneath if there is no `env`. |
| `env` | | | An image that is read as a sphere, with the normal. |
| `emissive` | 0 | 0 to 16 | Its own light, which shows even in the dark. |
| `emissive_map` | | | A `linear` grayscale image that multiplies its own light. |
| `strength` | 1 | 0 to 1 | How much of the lit color replaces the normal one. |

The numbers can follow values. For each light, every pixel adds the light's color times its
intensity, its attenuation, its cone and the cosine between the pixel's normal and the direction
of the light, plus a specular shine. The surrounding light arrives on top: the color of
`canvas.ambient` on surfaces that face down and that of `canvas.sky` (the ambient one if missing)
on those that face up, and black if you set neither. A layer with no normal map faces forward and
takes the mix of the two.

```json
  "canvas": {
    "size": [1920, 1080], "background": "#000000", "ambient": "#10131f", "sky": "#202a44",
    "blend_space": "linear", "clear": true, "fit": "fill", "hdr": false
  }
```

While the lights do not move, lit layers are drawn once and stored; when one moves, only the box
that covers its radius is redrawn. At the lowest quality level the lighting is computed once, at
the start, and lights that move stay where they were. A tilted (`tilt`) or raised (`z`) layer is
lit in its own plane. The depth of that plane does not count the narrowing of perspective or the
center of the camera: the light assumes the center of the canvas for the eye, so with a strong
tilt it may land somewhat off.

A normal map can be made with `reiwand-ctl scene act DIR normals` (from the edges of an image) and
the roughness and emissive maps are painted with `scene act DIR paint_map`.

### Light in the air

A point light or a spot with `volumetric` shines in the air within its radius:

```json
    {
      "blend_space": "canvas", "id": "spotlight", "type": "light", "light": "spot", "color": "#8fb4ff",
      "intensity": 1.2, "radius": 900, "cone": [20, 45], "z": 300,
      "transform": {
        "position": [1500, 150], "rotation": 20, "origin": "anchor", "scale": [1.0, 1.0],
        "skew": [0.0, 0.0]
      },
      "volumetric": { "density": 0.25, "samples": 16, "shafts": 0.3 }, "blend": "normal",
      "cache": false, "depth_test": false, "effects": [], "length": 256, "opacity": 1.0,
      "parallax": [0, 0], "shadow_bias": 0.2, "shadow_resolution": null, "shadows": false,
      "solid": false, "visible": true
    }
```

| Field | Prefill | Range | What it does |
|-------|---------|-------|--------------|
| `density` | 0.2 | 0 to 1 | How much of the path it covers and how strongly the air glows. |
| `samples` | 16 | 4 to 64 | How many points it takes per pixel. |
| `shafts` | 0 | 0 to 1 | With 1, the dark parts of the layers below hold back the glow and cast rays; with 0, the glow does not depend on them. |

The glow is added where the light's layer sits among the others: the layers below show through it
and the ones above cover it. It is computed at half the screen size (`q3`, `q2`) or at a quarter
(`q1`, and on computers of the two lowest classes), with half the points in `q1`, and it is not
computed in `q0`. These rays do not share the steps of the `god_rays` effect, and the editor's ray
curve is a bound computed over a dark background, not over what is underneath. Inside a tilted
group the air is drawn flat, without the group's perspective; inside a skipped or clipped group it
is drawn after the group.

### A spot that projects an image

A spot with `texture` multiplies its color by the image, which fills the outer cone like a
projector. A video keeps playing and asks for frames. `{ "layer": id }` projects that layer as it
is drawn, with its effects, even if it is hidden; the projected layer cannot receive light itself
or contain or read a layer that does. Up to 4 spots in a scene project.
