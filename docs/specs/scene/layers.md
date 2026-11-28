# Layers

Every layer has a unique `id` (among the layers) and a `type`. The scene only requires the ids of
layers to differ from each other, but the editor takes the ids of layers, assets and particle
systems as one namespace: a new layer, asset or system never reuses the id of another of them
(it adds a number to a name in use) and it refuses an explicit id that any of the three already
holds. Keep them apart in hand-written scenes too. A layer may set `transform` (`position`, `scale`,
`rotation`, `skew`, `origin`), `anchor` (`top_left` … `bottom_right`: the position is then
relative to that point of the visible part of the canvas), `opacity`, `visible`, `blend`, `mask`,
`effects` (at most 16), `parallax` (moves the layer with the pointer), `solid` and `cache`.
Layers without `anchor` are centered on the canvas and `origin` is a fraction of the
layer's own box (prefilled `[0.5, 0.5]`, or the anchor point).

| `type` | Fields |
|--------|--------|
| `group` | `layers`, `clip` (later children only show where the first one is) |
| `image` | `image` (asset id), `size` |
| `video` | `video` (asset id, or a value such as `{ "prop", "map" }`), `size`; plays in a loop |
| `text` | `text` (a text, a value, or a text per language such as `{ "en": "Hi", "es": "Hola" }`, which lists `text_languages`; the language is the one the player was given, whole and then its base, then the default language of the package, then `en`; each language is a tag such as `es` or `pt-BR`, its text is not empty and the manifest needs a text in its `default_locale`), `font` (asset id or system family), `size`, `color`, `align` |
| `shape` | `shape` (`rect` with `radius`, `ellipse`, `ring`, `polygon`, `star`, `line`), `fill`, `gradient` (`radial`, `from`, `to`, 2 to 4 `stops`), `stroke` (`color`, `width`) |
| `spectrum` | `size`, `style` (`bars`, `circle`, `wave`), `bands` (16, 32 or 64), `channel`, `color`, `gap` |
| `solid` | `color`, `size` (prefilled the canvas) |
| `particles` | `system`, a key of `particles` (see below) |
| `sound` | `sound` (asset id), `loop`, `play`, `volume`, `pan`, `fade_in`, `fade_out` (see "Sound"); no visual field |
| `light` | see "Lights"; draws nothing itself |
| `adjustment` | only `effects`, applied to everything below it; `size`, `redraw` and `rgb`, see below |
| `mesh` | a picture bent by a skeleton, see "Meshes" |
| `model` | a 3D model, see "Models and 3D" |

`blend` is `normal`, `add`, `multiply`, `screen`, `lighten`, `darken`, `overlay`, `soft_light`,
`hard_light`, `difference`, `exclusion`, `color_dodge`, `color_burn`, `hue`, `saturation`,
`color` or `luminosity`. Composition uses premultiplied alpha. `mask` is an asset id or
`{ "texture" | "layer" | "shape", "channel", "invert", "feather", "strength" }`: textures and layers mask by
luminance or alpha (`r`, `g`, `b`, `a`, `luminance`; without `channel`, a texture mask reads luminance and a layer or shape mask reads alpha), shapes by their outline. A layer used as a
mask (or a group holding one) may set `visible: false`: it is then not drawn but still masks. At
most 256 layers nested at most 8 levels; masks may not refer to each other in a cycle.

`blend` also takes these modes, computed per channel on the backdrop B and the layer S, both from
0 to 1, before the usual premultiplied composition (`blend_modes2`): `linear_burn` max(0, B + S − 1),
`linear_dodge` min(1, B + S), `darker_color` and `lighter_color` (the whole color of B or S with the
lower or higher luminance), `vivid_light` (color burn with 2S below one half, color dodge with
2S − 1 above), `linear_light` clamp(B + 2S − 1), `pin_light` (min(B, 2S) below one half, max(B,
2S − 1) above), `hard_mix` (1 where B + S ≥ 1, else 0), `subtract` max(0, B − S), `divide` (1 where S
is 0, else min(1, B / S)), `reflect` (1 where S is 1, else min(1, B² / (1 − S))), `glow` (reflect with
B and S swapped), `phoenix` min(B, S) − max(B, S) + 1, `average` (B + S) / 2, `negation`
1 − |1 − B − S|, `tint` max(B.r, B.g, B.b) × S, the layer color at the brightness of the backdrop,
and `diffuse_light` B + B × S, the layer as a light falling on the backdrop. `hue`, `saturation`,
`color` and `luminosity` swap the hue, saturation or lightness of B in HSL. Results are not clipped
at 1 while mixing, so `add` and `diffuse_light` keep light above white for later layers and for
`canvas.hdr` (a scene blending in linear space draws in 16 bits only when one of its layers uses
them or `canvas.hdr` is on, and in 8-bit sRGB otherwise); only the final picture is clipped. `darker_color` and `lighter_color` ignore the
layer `opacity`.

| Layer field | Meaning | Feature |
|-------------|---------|---------|
| `brightness` | 0 to 16 (prefilled 1), multiplies the color of the layer; above 1 it only shows with `canvas.hdr` | `layer_color` |
| `margin` | `[left, top, right, bottom]` in layer pixels (0 to 4096 each): room around a drawn layer for its effects, so a pass that moves the picture or its corners past the layer rectangle (a vertex stage that sways the top, a shift, a glow) is not cut at its edge; the layer and its anchor stay where they are, only the box the effects draw into grows, on each side by its own amount (on top of the `reach` of the effects, kept within the canvas); without effects it changes nothing, and groups and adjustment layers take their own box; may be absent | `layer_margin` |
| `color` | on `image`, `video`, `particles` and `group` layers, a color that multiplies the layer (prefilled white); a group applies it to its children together | `layer_color` |
| `parent` | the id of another drawn layer whose transform moves, turns and scales this one without changing the drawing order; the position of the layer is then relative to the position of its parent, in its turned and scaled frame (prefilled at the parent's position), instead of to its group; chains go at most 8 deep and never back to themselves | `parent` |
| `inherit` | what follows the parent: `transform`, `visible` and `opacity` (prefilled `transform` and `visible`), only with `parent` | `parent` |
| `transform.tilt` | `[x, y]` degrees (−89 to 89) of tilt in depth around the position of the layer, seen with `canvas.perspective`: a positive `x` turns the bottom edge towards the viewer, a positive `y` the left edge | `tilt` |
| `transform.z` | pixels towards the viewer (−100000 to 100000); a layer at or behind the viewer is not drawn | `tilt` |
| `parallax` | `[x, y]` (−10 to 10 each), with a different depth per axis, or a binding to a property, a timeline, the sound or anything a value may follow | `parallax2` when the two differ, `parallax_values` when bound |
| `lighting` | on `image`, `video`, `text`, `mesh`, `model` and `particles` layers, `{ "normal", "roughness", "roughness_map", "reflection", "env", "emissive", "emissive_map", "strength" }`: how the layer takes the light of `light` layers (see "Lights"); on `particles` layers it also names `particle_lighting` | `lighting2d` |
| `playback` | on `image` layers showing moving pictures or videos and on `video` layers, `{ "rate" (0 to 16; a video plays at 0 or from 0.1 to 4), "playing", "loop", "start", "end" (seconds), "frame", "own_clock" }`; `frame` picks a frame of a sprite sheet and may be bound to keyframes, the pointer, the audio or a script; layers that show the same video share one clock, led by the first of them, and a `video` layer with `own_clock: true` plays its video on a clock of its own, with its own rate, range, play and pause, seeks of scripts and decoder (`video_clocks`), so each such layer costs one more decoder, within the desktop limit of 16 decoders at once; a video picked by a property through a `map` keeps one own clock per video of the map (each runs while it is shown, and the seeks, rate and pauses of scripts reach all of them), and a video a `file` property that may hold a video (it accepts `video`, or its `default_asset` is a video) picks at run time (`{ "prop": "<id>" }`) keeps one own clock, `<id>@<layer>`, for whatever file the user picks, with a decoder of its own on the desktop and the phone; `own_clock: true` anywhere else (an image layer, a video bound without a `map` to a property that is no `file` or to a `file` property that only takes pictures, or a video layer showing a geometry pool) is refused, and the editor offers the switch only where it applies; its timeline shows a lane for each own clock, with a strip of frames of the video, the range of the video as a bar whose grips trim `start` and `end`, its speed, loop and play switches and a mark where the clock is at the playhead, and `reiwand-ctl scene ask DIR clocks` prints the same | `layer_playback`, `video_clocks` |
| mask `strength` | 0 to 1 (prefilled 1), weakens the mask: m′ = mix(1, m, strength) | `mask_strength` |
| adjustment `size` | `[width, height]` in pixels (at most twice the canvas): only that rectangle around the layer position (placed by `origin`, without the rotation, skew or scale of the layer) is captured and its effects change only that part, and `redraw: true` draws what was captured again with the whole transform of the layer, its opacity, blend mode and mask, as a magnifier or a mirror; without `size` the whole canvas is captured and turned around its middle. An adjustment layer with a `mask` is captured too and its effects change the picture only where the mask lets them, the mask laid over its box (the canvas without `size`). A capture is one copy of the rectangle and one pass more, and any change in the scene redraws the whole frame | `capture` |
| adjustment `rgb` | `true` sends what the layer captures, shrunk to at most 64 × 16 (keeping its shape), to the RGB lighting of the computer, at most 30 times a second and only when the frame changes; with `size` that region (placed like the capture), `null` the whole canvas; the picture below is not changed; the manifest must set `permissions.rgb`, the user must allow `rgb` for the package and turn RGB lighting on (`package.md`) | `rgb` |
| `attach` | `{ "layer", "bone" \| "point", "offset": [x, y], "inherit" }`: follows a bone or a node of a `mesh` or `model` layer instead of a `parent`, as posed in the same frame; `inherit` lists `position`, `rotation` and `scale` (all prefilled; without `position` the layer stays on the origin of the mesh layer, and what it does not take from the bone it still takes from the camera, which zooms and turns it like the rest of the scene); `offset` is in pixels along the bone; the layer hides with its mesh layer, and its own `transform` applies on top as under a parent; a layer may not follow itself, through other attachments either | `attach` |
| `solid` | `true` or `{ "alpha" (0 to 1) }`: the layer takes the pointer for `click:`, `enter:` and `leave:` triggers and script pointer events, over its box or, with `alpha`, only where it is at least that opaque (pictures by the alpha of their pixels, the other kinds by their color and opacity); a solid group takes the pointer over the boxes of its layers; layers are tested from the top down, as they were drawn in the last frame, with their transforms, parents and parallax, and hidden layers never take it | `layer_events` with `alpha` |
| `depth_test` | inside a group with `space: 3d`, a flat layer hides behind the models in front of it instead of covering them | `model3d` |

A layer with `tilt` or `z` is drawn as a projected quadrilateral: one 3 × 3 projective transform
instead of the flat one, so the GPU spreads its picture with the right perspective and it costs
nothing more. The projection follows the camera, so zooming the camera zooms a tilted layer as a
picture of it. The effects of a tilted layer run in its own plane before it is projected, and its
mask leans with it. A tilted group is drawn apart first (one more pass over its box) and then
projected, and its layers lean with it; a layer tilted inside a tilted group turns in the plane of the group as one rigid body seen by one camera (its tilt and `z` are 3D turns and moves within that plane, and the group's tilt then turns both), never as a picture of a picture. The pointer finds a tilted
layer where it shows.
