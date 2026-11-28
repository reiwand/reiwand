# Your own effect: the descriptor

On this page: what an own effect declares.

## Writing an effect of your own

A package can bring up to 32 effects of its own. Each one is two files of up to 64 KiB: a
**descriptor** (`.json`) that says what parameters it has and what it costs, and a **shader**
(`.wgsl`) that says what it does with each pixel. All the shaders of a scene add up to at most
256 KiB.

They are declared in `scene.json`, with an id (`[a-z0-9_]{1,32}`, different from those of the
program's effects), and layers use them like any other:

```json
  "effects": {
    "duotone": { "descriptor": "effects/duotone.json", "wgsl": "effects/duotone.wgsl" },
    "haze": { "descriptor": "effects/haze.json", "wgsl": "effects/haze.wgsl" }
  }
```

```json
    {
      "blend_space": "canvas", "id": "background", "type": "image", "image": "sky",
      "size": [1920, 1080],
      "effects": [
        {
          "effect": "duotone",
          "params": {
            "shadow": "#12122e", "light": "#ffcf8a",
            "amount": { "prop": "intensity", "invert": false }
          },
          "enabled": true, "optional": false
        },
        {
          "effect": "haze", "params": { "color": "#dfe6ff", "amount": 0.5 }, "enabled": true,
          "optional": false
        }
      ],
      "blend": "normal", "cache": false, "color": "#ffffff", "depth_test": false, "opacity": 1.0,
      "parallax": [0, 0], "solid": false,
      "transform": { "origin": "anchor", "rotation": 0.0, "scale": [1.0, 1.0], "skew": [0.0, 0.0] },
      "visible": true
    }
```

## The descriptor

```json
{
  "id": "duotone", "name": { "en": "Duotone" }, "mask_mode": "strength",
  "params": [
    {
      "id": "shadow", "type": "color", "channels": null, "help": {}, "label": {}, "max": null,
      "min": null, "paint": null, "prefill": "#1b1b3a", "tool": null, "tools": [], "unit": null,
      "uses": null, "values": [], "waits_for": null, "yields_to": null
    },
    {
      "id": "light", "type": "color", "channels": null, "help": {}, "label": {}, "max": null,
      "min": null, "paint": null, "prefill": "#ffd9a0", "tool": null, "tools": [], "unit": null,
      "uses": null, "values": [], "waits_for": null, "yields_to": null
    },
    {
      "id": "amount", "type": "float", "min": 0, "max": 1, "channels": null, "help": {},
      "label": {}, "paint": null, "prefill": 1, "tool": null, "tools": [], "unit": null,
      "uses": null, "values": [], "waits_for": null, "yields_to": null
    }
  ],
  "fuse": "color", "cost": { "taps": 1, "passes": 1, "scale": 1 }, "audio": [], "bounded": [],
  "category": null, "compose": false, "description": null, "lighter": null, "min_fps": null,
  "options": [], "pace": null, "passes": [], "quality": {}, "reach": null, "solve": null,
  "stage": "layer", "state": null, "targets": {}, "uses": [], "velocity": null
}
```

| Field | What it does |
|-------|--------------|
| `id` | The same as in `effects` of the scene. |
| `name` | The name, a text or one per language: `{ "en": "Duotone", "de": "Zweifarbig" }`. |
| `params` | Up to 24 parameters: `float`, `int`, `bool`, `color`, `vec2`, `texture`, `lut`, `enum`, `blend`, `position`, `mask` and `emitters`. Each with `id`, `prefill` (the value the editor fills it with), `min` and `max`. |
| `passes` | The drawing passes. Without `fuse`, at least one: `{ "entry": "fs_haze" }`. |
| `fuse` | `color` or `uv`, for single-pass effects that can be fused with their neighbors. |
| `cost` | **Required**: `{ "taps", "passes", "scale" }`: image reads per pixel, number of passes and size of the passes as a fraction of the layer. |
| `mask_mode` | `mix` (the usual: mixes with the input according to the mask) or `strength` (you scale with `mask_at(uv)`; suits distortions). |
| `uses` | `time`, `cursor` or `audio`: what the effect needs. The program also deduces it from your shader. |
| `audio` | Up to 2 sound readings, each with `kind` (`relative`, `absolute` or `surge`), `unit` (`db` or `amplitude`) and, with `surge`, `window` (from 0.05 to 60 s), `reference` (`band` or `all`) and `hold_silence`; without `surge` those three are not written. Your shader reads the 64 bands of reading `n` with `heard_band(n, i)`; a `surge` goes above 1 when the band rises after a silence and then falls. `band()` stays relative and in decibels. |
| `reach` | A `float` parameter, in pixels, by which the effect enlarges the layer. |
| `min_fps` | From 1 to 240: the rate at which the effect still looks smooth. |
| `pace` | When an effect that depends on time changes: `{ "rate": "<parameter>" }`, `{ "steps": [high, low] }`, `{ "sweep": [...] }` or `"outside"`. |
| `velocity` | A `bool` parameter that, when on, gives the shader the velocity of the layer. |
| `state` | `{ "scale", "settle", "fade" }`: textures that are kept between frames, for simulations. |
| `targets` | Up to 8 named textures `{ "scale", "format" }`, to chain passes. |
| `options` | Up to 8 variants of the shader (a WGSL `override`), compiled when the scene loads. |
| `compose`, `solve`, `bounded`, `quality` | Advanced options: draw the layer over what is underneath first, solve a homography, limit the work to the lit part of a mask, and change parameters by quality level. |
| `stage` | `material` or `particle`, for shaders of a 3D material or of a particle drawing (see [3d.md](../3d.md) and [particles-and-sound.md](../particles-and-sound.md)). |

A descriptor of an effect of your own does not admit `category` or `description`: they belong only
to the program's catalog, and writing them is an error.

Each pass can have `entry`, `scale` (fraction of the size of the layer), `inputs` (default
`["previous"]`: `previous`, `input`, `source`, `mask`, `pass:N`, `state`, `backdrop`,
`asset:<parameter>` or `target:<name>`), `output`, `target`, `blend` (`replace`, `normal`, `add`,
`multiply`), `copy` and `swap`. An effect has up to 24 passes.
