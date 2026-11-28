# The main fields

- `canvas.size` is the reference size in pixels (`1..=16384`); coordinates have their origin at
  the top left and angles are clockwise degrees. `fit` places the canvas on each screen (`fill`,
  `fit`, `stretch`, `center`). `background` is `#rrggbb[aa]`.
  `blend_space` is `linear` (prefilled) or `srgb`, which mixes encoded values as most editors do.
  Every layer carries a required `blend_space` of its own (`canvas`, `linear` or `srgb`; there is no default and an
  omitted value is an error; an unused optional block such as `mask` or `trail` may be absent). `canvas` follows `canvas.blend_space`; the other two decide how that layer composites
  onto what is below it, for every blend mode and every drawn type (particles included). A layer whose space differs
  from the canvas composites as a whole: a group is flattened first and then mixed in its own space, and its children
  use their own `blend_space` (`canvas` means the canvas, it is not inherited). An adjustment layer uses its space
  only for the final fade when its opacity is below 1. Effects always read `canvas.blend_space`. A layer without
  this field is refused naming it. The `layer` action edits it:
  `{"blend_space": {"layer": "smoke", "space": "srgb"}}`.
- `clock.max_fps` caps the frame rate (`1..=240`, or `"display"`, also `0`, to follow the refresh
  rate of each output for motion that gains from it; prefilled 60), `poster_time` is the instant
  shown where motion is not wanted or where the device cannot run the scene even at its lowest
  quality level, and the one drawn as the library thumbnail of a package without a preview picture (with clocks and
dates at the local time it is drawn), `seed` feeds procedural effects; `"seed": null` picks a new seed each time the
  scene starts, so its random patterns differ every time (`fresh_seed`, which every platform runs).
  Reaching an instant far into the scene (a poster or a held preview, up to a day) runs the
  timelines, videos and bones step by step but poses the bones every step only in the last 10
  seconds (before that, once every 10 minutes), so the
  springs of the bones settle as they would after a long pause; a looping timeline keeps its place
  exact however long it plays.
- `assets` maps at most 256 ids (`[a-z0-9_]{1,32}`) to `{ "path", "linear", "atlas" }`. Images
  (PNG, JPEG, WebP, GIF, BMP), fonts (TTF, OTF), videos (WebM, MP4, MKV, stored uncompressed),
  sounds (Ogg Vorbis, Opus, FLAC or WAV, see "Sound") and color lookup tables (`.cube`,
  `LUT_3D_SIZE` 2 to 65, domain 0 to 1, at most 16 MiB each and 64 MiB of table data per scene; a
  file named by several ids is read once) are used.
  `linear` marks data images such as depth or flow maps, which are read without color decoding.
  `scene.json` is at most 8 MiB and the images of a scene at most 512 MiB once decoded.
