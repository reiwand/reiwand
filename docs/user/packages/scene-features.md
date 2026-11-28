# Scene features

Scenes now describe everything an author may want to redo by hand. This page lists those features,
then says which of them the desktop and the phone already draw. `docs/specs/scene.md` describes
each field and its limits.

## What scenes can describe

- 15 more blend modes (`linear_burn` … `negation`);
- layer tint and brightness;
- a transform parent without changing the draw order;
- tilt in depth with perspective;
- masks with intensity;
- layers that feed effects;
- videos as mask or texture, and playback control;
- sprite sheets and animated GIF or WebP;
- filtering, repeat and mipmaps of textures;
- text with line wrapping, vertical alignment, background box, outline, shadow and blur;
- font collections (`ttc`, `otc`, `woff2`) and fallback fonts;
- sound in MP3, sound lists in order or at random with pauses, and long sounds streamed and
  triggered by events;
- a camera with zoom, rotation and shake;
- parallax settings (also with the phone's tilt);
- scene glow, and HDR with tone mapping;
- frames drawn over the previous one;
- point lights, spotlights, tubes and directional ones with volume and projection;
- 2D lighting with normal maps and ambient light;
- capture of a region;
- named events in timelines;
- curves with handles, and per-component and relative animation;
- the time and date as values;
- frequency ranges with bounds and exponent;
- effects of the package's own in WGSL with passes, targets, variants, up to 8 textures and 24
  parameters, checked when the package is read against the contract of its prelude.

Each one has its name in `requires.features`. The desktop and the phone reject with the same notice
a scene that uses any that their engine does not draw yet (the phone already plays the new sound
too).

## Camera, glow and tilt

Of that list, the desktop and the phone already draw:

| Feature | Details |
| --- | --- |
| 2D camera | Center, zoom, rotation and a noise shake that is turned off by "reduce motion" and at the lowest quality. |
| Scene glow | With tint and its HDR play. Only at the highest quality, or the next one with power. It blurs at a quarter of the size, is added to the canvas with the blend without copying it and costs nothing at zero strength. |
| Frames drawn over the previous one | More expensive, because each change redraws the whole image. |
| Tilt in depth | Of layers and groups, with their mask projected the same. A tilted layer inside a tilted group turns as a single body seen by a single camera. |
| Capture of a region | By an adjustment layer, to change it in place or draw it again as a magnifier or mirror. |

## Text

Both platforms draw letter spacing, and the outline, shadow and blur of text. These come from a
distance field that the GPU builds once per change of the text in 8-bit textures. Every GPU can draw
them, also those without 16-bit floating-point textures, and they can be scaled without redrawing
the letters.

They also draw text with line wrapping, number of lines with ellipsis, line spacing, vertical
alignment, background box and margin, font collections, `woff2` and fallback fonts.

## Videos in scenes

Videos can go anywhere an image is accepted, with a decoder per video that pauses while nothing
shows it. They support the `playback` playback and control from scripts with `layer.video`: play,
pause, stop, seek, rate, volume from 0 to 1, time, duration and end notice.

> **Note:** Videos inside a scene have no sound on the phone, so the volume changes nothing there.

## Blend modes

Both platforms now compose layers with the 32 blend modes. To the 17 from before are added
`linear_burn`, `linear_dodge`, `darker_color`, `lighter_color`, `vivid_light`, `linear_light`,
`pin_light`, `hard_mix`, `subtract`, `divide`, `reflect`, `glow`, `phoenix`, `average` and
`negation`. They have published per-channel formulas that the engine, the built-in effects and
those of the package share.

## Layers and masks as effect inputs

A layer, even if hidden, can feed the texture of an effect:

```json
{ "layer": "<id>" }
```

It is drawn separately and enters aligned with the screen, so that each point of the effect reads
the point of the other layer that lies over it. It is only drawn again when that layer changes.

The masks of effects also accept a layer or a shape, with inversion, blur and intensity, like those
of layers. A pass can read `backdrop`, what is already drawn under the layer, both in the built-in
effects and in the package's.

Each layer that feeds an effect and each drawn mask count in the cost of the scene.

## New built-in effects

| Effect | What it does |
| --- | --- |
| `tint` | Tints the layer with a color and a blend mode, or colors it while keeping its luminosity. |
| `blend_texture` | Mixes another image or a layer with a blend mode, shifted and repeated. |
| `light_shafts` | Draws own shafts of light, in parallel, from a point or a corner, of a color or with a gradient. It only asks for frames while its speed is not zero. |

`docs/specs/scene.md` includes an equivalence table with the effects of other programs and how to
convert their parameters. See [Built-in effects](built-in-effects.md) for the rest.

## Layers, textures and animation

The desktop and the phone also already draw:

- layer tint and brightness;
- the transform parent;
- masks with intensity;
- parallax settings (the phone also with its tilt, with the same sensors as photo parallax and only
  while the scene is seen);
- sharp filtering, repeat, mirror and mipmaps of textures (mipmaps are created once, when an image
  is drawn at less than three quarters of its size);
- sprite sheets and animated GIF or WebP (the next frame is only requested when due and never with
  the layer hidden);
- curves with handles, and per-component and relative animation;
- the time and date as values, with month and day names in English or Spanish.

Keyframe colors are mixed in OKLab along the shortest hue path.

## Time in scripts

Scripts also:

- read the time of day (`engine.daytime`);
- use the computer's time zone in `Date` even if it changes, without reading any file;
- write dates with `time.format`.
