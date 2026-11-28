# The edit file

Edits never change the image. An edit lives in `<image file name>.reiwand.json` next to the image
(`hills.jpg.reiwand.json` for `hills.jpg`) or, when that folder cannot be written, in
`~/.local/share/reiwand/edits/<SHA-256 of the canonical image path>.json`, which is also
used when the file next to the image is a symbolic link; when both exist the newer one wins. Both
are written atomically with mode `0600` and read up to 2 MiB. `reiwand-ctl edit` changes them without
the daemon. The daemon reads the edit whenever it loads an image for an output (a file, a library
item, a package entry or a playlist item) and loads it again as soon as the file changes; an edit
that cannot be used is ignored with a warning.

```json
{
  "version": 1,
  "source": { "name": "hills.jpg", "bytes": 8421337, "sha256": "9f2c…", "pixels": [6000, 4000] },
  "geometry": { "crop": [0.0, 0.08, 1.0, 0.84], "rotate": 0, "straighten": -1.2, "flip": [false, false] },
  "ops": [
    { "id": "wb", "type": "white_balance", "temperature": 5600, "tint": 4 },
    { "id": "ev", "type": "exposure", "ev": 0.35 },
    { "id": "bl", "type": "blur", "radius": 18, "mask": "back" },
    { "id": "ov", "type": "overlay", "color": "#1a2b6d", "blend": "soft_light", "opacity": 0.35, "mask": "sky" }
  ],
  "masks": {
    "back": { "type": "luminance", "range": [0.0, 0.35], "soft": 0.05 },
    "sky": { "type": "linear", "from": [0.5, 0.0], "to": [0.5, 0.45], "invert": true }
  },
  "outputs": { "Maker Model 1234": { "connector": "DP-1", "crop": [0.0, 0.1, 0.5, 0.5625] } },
  "span": { "center": [0.5, 0.46], "zoom": 1.0 },
  "depth": { "model": "depth-anything-v2-small", "input": 518, "near": 1.0, "far": 0.0, "gamma": 1.0, "smooth": 2, "dilate": 3 },
  "parallax": { "enabled": true, "source": "auto", "strength": 0.02, "focus": 0.3, "steps": 16, "smoothing_ms": 150, "overscan": 1.04, "fill": "lama", "layers": 5 },
  "fit": "fill"
}
```

- `version` is `1`; newer files are refused. Unknown fields, values of the wrong type, out-of-range
  values and broken references are errors that name the field, such as
  `ops[1].ev: 7 is out of range -5 to 5` or `masks.sky.invert: invalid type: integer \`3\`, expected a boolean`.
- `source` holds the name, size, SHA-256 and pixel size of the image when the edit was created;
  when the file size or the pixel size changed, `reiwand-ctl edit` warns, the image editor of the app
  says so above the picture and the daemon logs a warning once for each change of the file.
- `geometry`: `crop` `[x, y, w, h]` in fractions of the rotated image (`null` for all of it),
  `rotate` clockwise `0`, `90`, `180` or `270`, `straighten` from `-45` to `45` degrees
  counterclockwise keeping the largest centred rectangle, `flip` `[horizontal, vertical]`.
- `ops` run in order, at most 24. Each has a unique `id` (1 to 64 letters, digits, `-` or `_`), a
  `type`, `enabled`, `mask` (one of `masks`, `""` for none) and `opacity` from `0` to `1`
  (a block: `null` or absent for a full-strength operation). `reiwand-ctl edit curve`, `blur`, `vignette` and `overlay`
  refuse an `--id` that names an operation of another type instead of changing its type.

| `type` | Fields |
|--------|--------|
| `exposure` | `ev`, `-5` to `5` |
| `white_balance` | `temperature`, `2000` to `12000` K with `6504` neutral; `tint`, `-100` to `100`, positive adds magenta |
| `contrast` | `amount`, `-100` to `100` |
| `saturation` | `saturation` and `vibrance`, `-100` to `100` |
| `curves` | `rgb`, `r`, `g` and `b`: empty or 2 to 16 `[x, y]` points from `0` to `1` with growing `x` |
| `blur` | `radius`, `0` to `200` pixels of a picture 1000 pixels long; an edge spreads as under a Gaussian blur whose standard deviation is one and a half times the radius |
| `overlay` | `color` `#rrggbb`; `blend` `normal`, `multiply`, `screen`, `overlay`, `soft_light` or `color` |
| `vignette` | `amount`, `-1` to `1`; `midpoint`, `roundness` and `feather`, `0` to `1` |

- `masks`, at most 8, use coordinates of the original image: `linear` (`from`, `to`), `radial`
  (`center`, `radius` `[rx, ry]`, `feather`), `luminance` and `depth` (`range` `[low, high]`,
  `soft`; `depth` reads the depth map, `1` near, and selects nothing while the image has none)
  and up to 4 `brush` masks (`strokes` of `radius`, `hardness`, `erase` and `points`, at most
  20 000 points in all), drawn into one map of at most 2048 pixels a side that keeps the whole
  width of very long pictures. Every mask takes `invert`. The `mask` action (`reiwand-ctl edit mask`)
  adds one stroke to a brush mask with the spec `brush:RADIUS,HARDNESS:X,Y;X,Y;…` or
  `erase:RADIUS,HARDNESS:X,Y;…`, creating the mask when it is not a brush yet; its `invert`,
  when given, also turns the brush mask over, and left out keeps it as it is; the app paints
  the same strokes on the picture. Any valid name can be a mask, such as
  `1`, `true` or `null`: `reiwand-ctl edit set FILE ops.<id>.mask=<name>` gives it to an operation
  and an empty name takes it away.
- `outputs` crops the edited image for one screen, keyed by its identity (as in
  `SetOutputInfo`) or output name; `connector` names the output as a hint. The Android app keeps
  the framing chosen on the phone under `Android`, in the shape of the phone's screen, inside the
  `geometry` crop; computers leave it to the phone.
- `span` places the image when the fit is `span`: `center` `[x, y]` and `zoom` from `1` to `8`.
- `depth` shapes the depth map: `near` and `far` (`0` to `1`) are the levels of the nearest and
  farthest points, `gamma` (`0.2` to `5`) curves the levels between them, `dilate` (`0` to `16`
  pixels of the map) grows near objects over their edges and `smooth` (`0` to `16`) blurs the
  result. `model` is `depth-anything-v2-small` and `input` (a multiple of 14 from `196` to `1036`)
  the short side given to it, which also names the generated map. `strokes` (the brush of
  `reiwand-ctl edit depth brush`) are lists of `{points, radius, falloff, by}` with `points` as `[x, y]`
  fractions of the original picture, `radius` a fraction (above 0, up to 1) of its longer side,
  `falloff` from `0` to `1` and `by` from `-1` to `1` added to the adjusted level at the middle
  of the stroke and fading to nothing at its edge; `fills` are marks of the same shape whose `by`
  is `1` (the band behind nearer layers is always filled there) or `-1` (never filled there);
  `cuts` (1 to 5 growing levels, 0.005 apart at least) sets where the depth layers end, grouping
  the regions of the picture by their mean depth, and none leaves it to the clustering; `joins`
  (at most 64 `{a, b}` pairs of `[x, y]` fractions) make the regions under `a` and `b` one region
  and `splits` (at most 64 strokes of the shape of `strokes`, whose `by` is not used) cut a region
  along the stroke; `moves` (at most 64 `{at, by}`, `at` an `[x, y]` fraction and `by` from `-1` to `1`)
  add `by` to the depth of the whole region under `at`, after the regions are found, so the region
  keeps its shape but changes layer and movement. Each list holds at most 256 entries and 3 000 points, and all of them are left
  out of the file while empty. See `editor.md`, "Picture depth tools".
- `parallax` moves the image in depth on the desktop while `enabled`: `source` is `auto` (a slow
  orbit), `pointer` (the pointer feed of `Inputs1`, back to the centre when the pointer leaves the
  outputs that show the image or rests for 3 s) or `accelerometer` (the IIO accelerometer of the
  computer, `(raw + offset) × scale` turned by its mount matrix, read 30 times per second only
  while the parallax is shown, looked for again every 5 s while none answers, with its resting
  tilt slowly recentred); a new source starts from the offset the image has;
  `strength` (`0` to `0.03`) is the largest shift as a fraction of the image width, `focus` (`0`
  far to `1` near) the depth that stays still, `steps` `1`, `8`, `16` or `32` (kept for older
  edits; the layers do not use it),
  `smoothing_ms` (`0` to `2000`) the time a critically damped spring takes to follow the source and
  `overscan` (`1` to `1.1`) the least zoom; the zoom that hides the edges is computed for each
  picture (see "Depth layers"),
  `fill` (`lama` or `simple`) how the layers are filled behind near objects (`lama` is the LaMa model
  when it and ONNX Runtime are installed, else `simple`, the push-pull extension of the nearest
  background) and `layers` (`4` to `6`) the number of depth layers. Both belong to the picture, are
  always written (`"auto"` for the standard value) and synced and undone with the rest of its edit; `"auto"` means,
  when it is read, `lama` on the computer, `simple` on the phone, and 5 layers. A picture that must
  not move turns `enabled` off; there is no machine-wide setting for either. Each choice has its own
  layer set (the key of the set includes both), so two pictures never share or delete each other's.
- `fit` (`fill`, `fit`, `stretch`, `center` or `tile`) shows this picture with its own fit instead
  of the fit of the output, except on `span` outputs; `GetState().outputs[x].fit` stays the fit of
  the output and `shown_fit` reports the fit in use (`span` also on outputs that show a scene its
  author spread over every screen, which each draw their part of one canvas on one shared clock).
